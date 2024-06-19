use types::token   ::*;
use types::location::*;

use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Lexer {
  file     : Arc<str>,
  source   : Arc<[char]>,
  line_idx : Arc<[usize]>,
  line     : usize,
  column   : usize,
  offset   : usize,
  was_space: bool,
  tokens   : Option<Arc<[Token]>>,
}


type PossibleChar = Option<(char, Location)>;
type Result<T>    = std::result::Result<T, LexError>;


#[derive(Debug, Clone)]
pub enum LexError {
  UnterminatedStringLiteral(Lexer, Range),
  InvalidEscapeSequence    (Lexer, Range),
  UnterminatedCharLiteral  (Lexer, Range),
}


use LexError::*;


impl Lexer {
  pub fn new(file: &str) -> std::io::Result<Self> {
    let mut file: String = std::fs::canonicalize(file)
      .unwrap().to_str().unwrap().into();

    // remove UNC prefix
    if file.starts_with("\\\\?\\") {
      file.drain(..4);
    }

    let source = std::fs::read_to_string(&file)?;
    Ok(Self::init(file.into(), source))
  }

  #[cfg(test)]
  pub fn from_source(source: &str) -> Self {
    Self::init(Arc::from("source"), source.into())
  }

  fn init(file: Arc<str>, src: String) -> Self {
    let mut source   = Vec::new();
    let mut line_idx = Vec::new();

    source  .reserve(src.len());
    line_idx.reserve(src.len() / 30);   // random guess

    line_idx.push(0);

    for ch in src.chars() {
      source.push(ch);

      if ch == '\n' {
        line_idx.push(source.len());
      }
    }

    Self {
      file,
      source   : source  .into(),
      line_idx : line_idx.into(),
      line     : 1,
      column   : 1,
      offset   : 0,
      was_space: false,
      tokens   : None,
    }
  }

  pub fn lex(&mut self) -> Result<Arc<[Token]>> {
    if let Some(tokens) = &self.tokens {
      return Ok(tokens.clone());
    }

    let mut tokens = Vec::new();

    loop {
      self.skip_whitespace();
      if self.is_eof() { break; }

      if let Some(t) = self.lex_lit_int   ()  { tokens.push(t); continue; }
      if let Some(t) = self.lex_lit_bool  ()  { tokens.push(t); continue; }
      if let Some(t) = self.lex_lit_char  ()? { tokens.push(t); continue; }
      if let Some(t) = self.lex_lit_str   ()? { tokens.push(t); continue; }
      if let Some(t) = self.lex_identifier()  { tokens.push(t); continue; }

      tokens.push(self.lex_punctuator());
    }

    self.tokens = Some(tokens.into());
    Ok(self.tokens.as_ref().unwrap().clone())
  }

  fn is_eof(&self) -> bool {
    self.offset >= self.source.len()
  }

  fn get_location(&self) -> Location {
    Location::new(
      self.file.clone(),
      self.line,
      self.column,
      self.offset)
  }

  /// Returns the spacing between the given range. The range should be end-inclusive.
  fn get_spacing(&self, range: &Range) -> Spacing {
    let s = if range.start.offset() == 0 { false } else {
      self.at(range.start.offset()-1).map_or(false, |c| c.is_ascii_whitespace())
    };

    let e = if range.end.offset() == self.source.len() { false } else {
      self.at(range.end.offset()+1).map_or(false, |c| c.is_ascii_whitespace())
    };

    match (s, e) {
      (true , true ) => Spacing::Both,
      (true , false) => Spacing::Leading,
      (false, true ) => Spacing::Trailing,
      (false, false) => Spacing::None,
    }
  }

  fn rollback(&mut self, loc: Location) {
    self.line   = loc.line  ();
    self.column = loc.column();
    self.offset = loc.offset();
  }

  fn at(&self, n: usize) -> Option<char> {
    self.source.get(n).copied()
  }

  fn peek(&self) -> Option<char> {
    self.at(self.offset)
  }

  fn peek_n(&self, n: usize) -> Option<char> {
    self.at(self.offset + n)
  }

  fn peek_next_n<const N: usize>(&self) -> [Option<char>; N] {
    let mut chars = [None; N];

    for i in 0..N {
      chars[i] = self.peek_n(i);
    }

    chars
  }

  fn next(&mut self) -> PossibleChar {
    let ch  = self.peek()?;
    let loc = self.get_location();

    self.offset += 1;

    if ch == '\n' {
      self.line   += 1;
      self.column  = 1;
    }

    else {
      self.column += 1;
    }

    self.was_space =
         ch == ' '
      || ch == '\t'
      || ch == '\n'
      || ch == '\r';

    Some((ch, loc))
  }

  fn advance(&mut self) -> Option<Location> {
    self.next().map(|(_, loc)| loc)
  }

  fn advance_n(&mut self, n: usize) -> Option<Location> {
    for _ in 0..n-1 {
      self.advance();
    }

    self.advance()
  }

  fn skip_whitespace(&mut self) {
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_whitespace() {
        break;
      }

      self.advance();
    }
  }

  /// `max_input_len` is used to prevent collecting too many unnecessary characters.
  /// If not set, the default value is `500` as one token rarely exceeds that length.
  fn lex_by_regex(&mut self, regex: &'static str, max_input_len: Option<usize>) -> Option<(Range, HashMap<String, String>)> {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static mut REGEX: Lazy<HashMap<&'static str, regex::Regex>> =
      Lazy::new(HashMap::new);

    if !regex.starts_with('^') {
      panic!("The regex must start with '^'");
    }

    let regex = unsafe {
      REGEX.get(regex).unwrap_or_else(|| {
        let rule = regex::Regex::new(regex).unwrap();
        REGEX.insert(regex, rule);
        REGEX.get(regex).unwrap()
      })
    };

    let range  = self.offset..max_input_len.unwrap_or(500).min(self.source.len());
    let slice  = self.source[range].iter().collect::<String>();
    let result = regex.captures(&slice)?;

    let mut map = HashMap::new();

    let mut idx = 0;
    for cap in regex.capture_names() {
      if let Some(group) = result.get(idx) {
        let group = group.as_str().to_string();

        if let Some(name) = cap {
          map.insert(name.into(), group.clone());
        }

        map.insert(format!("<{idx}>"), group);
      }

      idx += 1;
    }

    let start = self.get_location();
    let end   = self.advance_n(map.get("<0>").unwrap().chars().count())?;

    Some((start..end, map))
  }

  fn lex_identifier(&mut self) -> Option<Token> {
    let start = self.get_location();
    let mut end        = start.clone();
    let mut identifier = String::new();

    while let Some(ch) = self.peek() {
      if !ch.is_ascii_alphanumeric() && ch != '_' {
        break;
      }

      identifier.push(ch);
      end = self.advance()?;
    }

    if identifier.is_empty() {
      return None;
    }

    let range   = start..end;
    let spacing = self.get_spacing(&range);

    Some(Token::Ident(Ident { range, spacing, identifier }))
  }

  fn lex_punctuator(&mut self) -> Token {
    let (ch, start) = self.next().unwrap();

    let range   = start.clone()..start;
    let spacing = self.get_spacing(&range);

    Token::Punct(Punct { range, spacing, punctuator: ch })
  }

  fn lex_lit_str(&mut self) -> Result<Option<Token>> {
    let before = self.get_location();
    let prefix = self.lex_identifier();

    let start = if let Some(ref p) = prefix {
      p.range().start.clone()
    } else {
      self.get_location()
    };

    match self.peek() {
      Some('"') => { self.advance(); }
      _         => {
        self.rollback(before);
        return Ok(None);
      },
    }

    let mut value  = String::new();
    let mut escape = None;

    loop {
      match self.peek() {
        None => {
          let range = start..self.get_location();
          return Err(UnterminatedStringLiteral(self.clone(), range));
        }

        Some('"') if escape.is_none() => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance().unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitStr(LitStr { range, spacing, value, prefix })))
        }

        Some('\\') if escape.is_none() => {
          escape = self.advance();
        }

        Some('n' ) if escape.is_some() => { value.push('\n'); escape = None; self.advance(); }
        Some('r' ) if escape.is_some() => { value.push('\r'); escape = None; self.advance(); }
        Some('t' ) if escape.is_some() => { value.push('\t'); escape = None; self.advance(); }
        Some('0' ) if escape.is_some() => { value.push('\0'); escape = None; self.advance(); }
        Some('\"') if escape.is_some() => { value.push('\"'); escape = None; self.advance(); }
        Some('\'') if escape.is_some() => { value.push('\''); escape = None; self.advance(); }
        Some('\\') if escape.is_some() => { value.push('\\'); escape = None; self.advance(); }

        Some('u') if escape.is_some() => {
          let mut code_point = 0;
          let mut count      = 0;
          let mut closed     = false;

          match self.next() {
            Some(('{', _)) => {}

            None => {
              let range = start..self.get_location();
              return Err(UnterminatedStringLiteral(self.clone(), range));
            }

            Some((_, loc)) => {
              let range = start..loc;
              return Err(InvalidEscapeSequence(self.clone(), range));
            }
          }

          for _ in 0..7 {
            match self.next() {
              None => {
                let range = start..self.get_location();
                return Err(UnterminatedStringLiteral(self.clone(), range));
              }

              Some(('}', _)) => {
                closed = true;
                break;
              }

              Some((ch @ ('0'..='9' | 'A'..='F'), _)) => {
                code_point = code_point<<4 | ch.to_digit(16).unwrap();
                count += 1;
              }

              Some((_, loc)) => {
                let range = start..loc;
                return Err(InvalidEscapeSequence(self.clone(), range));
              }
            }
          }

          if !closed || count < 1 || count > 6 {
            let range = start..self.get_location();
            return Err(InvalidEscapeSequence(self.clone(), range));
          }

          if let Some(ch) = std::char::from_u32(code_point) {
            value.push(ch);
            escape = None;
            continue;
          }

          let range = start..self.get_location();
          return Err(InvalidEscapeSequence(self.clone(), range));
        }

        Some(_) if escape.is_some() => {
          let range = start..escape.unwrap();
          return Err(InvalidEscapeSequence(self.clone(), range));
        }

        Some(ch) => {
          value.push(ch);
          self.advance();
        }
      }
    }
  }

  fn lex_lit_char(&mut self) -> Result<Option<Token>> {
    let before = self.get_location();
    let prefix = self.lex_identifier();

    let start = if let Some(ref p) = prefix {
      p.range().start.clone()
    } else {
      self.get_location()
    };

    let [l, a, r] = self.peek_next_n();

    if l != Some('\'') {
      self.rollback(before);
      return Ok(None);
    }

    if a.is_none() {
      let range = start..self.advance().unwrap();
      return Err(UnterminatedCharLiteral(self.clone(), range));
    }

    if a == Some('\\') {
      if r.is_none() {
        let range = start..self.advance_n(2).unwrap();
        return Err(UnterminatedCharLiteral(self.clone(), range));
      }

      self.advance_n(2);

      match self.peek_next_n() {
        [Some('n'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\n', prefix })));
        }

        [Some('r'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\r', prefix })));
        }

        [Some('t'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\t', prefix })));
        }

        [Some('0'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\0', prefix })));
        }

        [Some('\''), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\'', prefix })));
        }

        [Some('\"'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\"', prefix })));
        }

        [Some('\\'), Some('\'')] => {
          let prefix  = prefix.map(|t| t.try_into().unwrap());
          let range   = start..self.advance_n(2).unwrap();
          let spacing = self.get_spacing(&range);

          return Ok(Some(Token::LitChar(LitChar { range, spacing, value: '\\', prefix })));
        }

        [Some('u'), Some('{')] => {
          self.advance_n(2);

          let mut code_point = 0;

          for i in 0..7 {
            match self.next() {
              Some(('}', _)) => {
                if i == 0 {
                  let range = start..self.get_location();
                  return Err(InvalidEscapeSequence(self.clone(), range));
                }

                let prefix  = prefix.map(|t| t.try_into().unwrap());
                let range   = start..self.advance().unwrap();
                let spacing = self.get_spacing(&range);

                if let Some(value) = std::char::from_u32(code_point) {
                  return Ok(Some(Token::LitChar(LitChar { range, spacing, value, prefix })));
                }

                return Err(InvalidEscapeSequence(self.clone(), range));
              }

              Some((ch @ ('0'..='9' | 'A'..='F'), _)) => {
                if i > 5 {
                  let range = start..self.get_location();
                  return Err(InvalidEscapeSequence(self.clone(), range));
                }

                code_point = code_point<<4 | ch.to_digit(16).unwrap();
              }

              Some((_, loc)) => {
                let range = start..loc;
                return Err(InvalidEscapeSequence(self.clone(), range));
              }

              None => {
                let range = start..self.get_location();
                return Err(UnterminatedCharLiteral(self.clone(), range));
              }
            }
          }
        }

        _ => {
          let range = start..self.get_location();
          return Err(UnterminatedCharLiteral(self.clone(), range));
        }
      }
    }

    if r == Some('\'') {
      let prefix  = prefix.map(|t| t.try_into().unwrap());
      let range   = start..self.advance_n(3).unwrap();
      let spacing = self.get_spacing(&range);

      return Ok(Some(Token::LitChar(LitChar { range, spacing, value: a.unwrap(), prefix })));
    }

    let range = start..self.advance_n(3).unwrap();
    Err(UnterminatedCharLiteral(self.clone(), range))
  }

  fn lex_lit_int(&mut self) -> Option<Token> {
    let (
      mut range,
      value,
      radix,
      exponent,
    ) =
      if let Some((r, m)) = self.lex_by_regex(r"^[+-]?0b[01_]*[01]", None) {
        (r, m.get("<0>").unwrap().clone(), Radix::Binary, None)
      }

      else if let Some((r, m)) = self.lex_by_regex(r"^[+-]?0o[0-7_]*[0-7]", None) {
        (r, m.get("<0>").unwrap().clone(), Radix::Octal, None)
      }

      else if let Some((r, m)) = self.lex_by_regex(r"^[+-]?0x[0-9a-fA-F_]*[0-9a-fA-F]", None) {
        (r, m.get("<0>").unwrap().clone(), Radix::Hexadecimal, None)
      }

      else if let Some((r, m)) = self.lex_by_regex(r"^[+-]?[0-9](?:[0-9_]*[0-9])?(?:[eE](?<exp>[0-9]+))?", None) {
        let all = m.get("<0>").unwrap().clone();
        let exp = m.get("exp").map(|v| v.parse().unwrap());
        (r, all, Radix::Decimal, exp)
      }

      else { return None; };

    let suffix  = self.lex_identifier();

    range.end = suffix.as_ref().map_or(
      range.end, |t| t.range().end.clone());

    let suffix  = suffix.map(|t| t.try_into().unwrap());
    let sign    = if value.starts_with('-') { Sign::Negative } else { Sign::Positive };
    let spacing = self.get_spacing(&range);

    return Some(Token::LitInt(LitInt {
      range,
      spacing,
      value,
      sign,
      radix,
      exponent,
      suffix,
    }));
  }

  fn lex_lit_bool(&mut self) -> Option<Token> {
    let start = self.get_location();

    let [a, b, c, d, e] = self.peek_next_n();

    if a == Some('t')
    && b == Some('r')
    && c == Some('u')
    && d == Some('e') {
      let range = start..self.advance_n(4).unwrap();
      let spacing = self.get_spacing(&range);

      return Some(Token::LitBool(LitBool { range, spacing, value: true }));
    }

    if a == Some('f')
    && b == Some('a')
    && c == Some('l')
    && d == Some('s')
    && e == Some('e') {
      let range = start..self.advance_n(5).unwrap();
      let spacing = self.get_spacing(&range);

      return Some(Token::LitBool(LitBool { range, spacing, value: false }));
    }

    None
  }
}
