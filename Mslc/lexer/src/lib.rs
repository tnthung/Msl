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
}
