mod delimiter;
mod spacing;
mod radix;
mod sign;


pub use delimiter::*;
pub use spacing::*;
pub use radix::*;
pub use sign::*;


use std::sync::Arc;
use num::{ BigInt, BigRational };

use crate::location::*;


#[derive(Debug, Clone)]
pub enum Token {
  Group   (Group   ),
  Ident   (Ident   ),
  Punct   (Punct   ),
  LitStr  (LitStr  ),
  LitChar (LitChar ),
  LitInt  (LitInt  ),
  LitFloat(LitFloat),
  LitBool (LitBool ),
}


#[derive(Debug, Clone)]
pub struct Group {
  pub range    : Range,
  pub spacing  : Spacing,
  pub delimiter: Delimiter,
  pub tokens   : Vec<Token>,
}


#[derive(Debug, Clone)]
pub struct Ident {
  pub range     : Range,
  pub spacing   : Spacing,
  pub identifier: String,
}


#[derive(Debug, Clone)]
pub struct Punct {
  pub range     : Range,
  pub spacing   : Spacing,
  pub punctuator: char,
}


#[derive(Debug, Clone)]
pub struct LitStr {
  pub range  : Range,
  pub spacing: Spacing,
  pub prefix : Option<Ident>,
  pub value  : String,
}


#[derive(Debug, Clone)]
pub struct LitChar {
  pub range  : Range,
  pub spacing: Spacing,
  pub prefix : Option<Ident>,
  pub value  : char,
}


#[derive(Debug, Clone)]
pub struct LitInt {
  pub range   : Range,
  pub spacing : Spacing,
  pub sign    : Sign,
  pub radix   : Radix,
  pub value   : String,
  pub exponent: Option<usize>,  // only for decimal
  pub suffix  : Option<Ident>,
}


#[derive(Debug, Clone)]
pub struct LitFloat {
  pub range  : Range,
  pub spacing: Spacing,
  pub sign   : Sign,
  pub value  : String,
  pub suffix : Option<Ident>,
}


#[derive(Debug, Clone)]
pub struct LitBool {
  pub range  : Range,
  pub spacing: Spacing,
  pub value  : bool,
}


impl Token {
  pub fn range(&self) -> &Range {
    match self {
      Token::Group   (group    ) => &group    .range,
      Token::Ident   (ident    ) => &ident    .range,
      Token::Punct   (punct    ) => &punct    .range,
      Token::LitStr  (lit_str  ) => &lit_str  .range,
      Token::LitChar (lit_char ) => &lit_char .range,
      Token::LitInt  (lit_int  ) => &lit_int  .range,
      Token::LitFloat(lit_float) => &lit_float.range,
      Token::LitBool (lit_bool ) => &lit_bool .range,
    }
  }

  pub fn spacing(&self) -> Spacing {
    match self {
      Token::Group   (group    ) => group    .spacing,
      Token::Ident   (ident    ) => ident    .spacing,
      Token::Punct   (punct    ) => punct    .spacing,
      Token::LitStr  (lit_str  ) => lit_str  .spacing,
      Token::LitChar (lit_char ) => lit_char .spacing,
      Token::LitInt  (lit_int  ) => lit_int  .spacing,
      Token::LitFloat(lit_float) => lit_float.spacing,
      Token::LitBool (lit_bool ) => lit_bool .spacing,
    }
  }

  pub fn is_group(&self) -> bool {
    matches!(self, Token::Group(_))
  }

  pub fn is_ident(&self) -> bool {
    matches!(self, Token::Ident(_))
  }

  pub fn is_punct(&self) -> bool {
    matches!(self, Token::Punct(_))
  }

  pub fn is_lit_str(&self) -> bool {
    matches!(self, Token::LitStr(_))
  }

  pub fn is_lit_char(&self) -> bool {
    matches!(self, Token::LitChar(_))
  }

  pub fn is_lit_int(&self) -> bool {
    matches!(self, Token::LitInt(_))
  }

  pub fn is_lit_float(&self) -> bool {
    matches!(self, Token::LitFloat(_))
  }

  pub fn is_lit_bool(&self) -> bool {
    matches!(self, Token::LitBool(_))
  }
}


macro_rules! impl_commons {
  ($variant:ident) => {
    impl TryFrom<Token> for $variant {
      type Error = Token;

      fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
          Token::$variant(value) => Ok(value),
          _ => Err(token),
        }
      }
    }

    impl<'a> TryFrom<&'a Token> for &'a $variant {
      type Error = &'a Token;

      fn try_from(token: &'a Token) -> Result<Self, Self::Error> {
        match token {
          Token::$variant(value) => Ok(value),
          _ => Err(token),
        }
      }
    }

    impl From<$variant> for Token {
      fn from(value: $variant) -> Self {
        Token::$variant(value)
      }
    }

    impl From<&$variant> for Token {
      fn from(value: &$variant) -> Self {
        Token::$variant(value.clone())
      }
    }
  };
}


impl_commons!(Group   );
impl_commons!(Ident   );
impl_commons!(Punct   );
impl_commons!(LitStr  );
impl_commons!(LitChar );
impl_commons!(LitInt  );
impl_commons!(LitFloat);
impl_commons!(LitBool );


impl Group {
  pub fn open_location(&self) -> &Location {
    &self.range.start
  }

  pub fn close_location(&self) -> &Location {
    &self.range.end
  }

  pub fn inner_range(&self) -> Option<Range> {
    if self.tokens.is_empty() {
      return None;
    }

    let start = &self.tokens.first()?.range().start;
    let end   = &self.tokens.last ()?.range().end  ;

    Some(start.clone()..end.clone())
  }

  pub fn is_empty(&self) -> bool {
    self.tokens.is_empty()
  }

  pub fn as_slice(&self) -> &[Token] {
    &self.tokens
  }

  pub fn is_by_parentheses(&self) -> bool {
    self.delimiter == Delimiter::Parentheses
  }

  pub fn is_by_brackets(&self) -> bool {
    self.delimiter == Delimiter::Brackets
  }

  pub fn is_by_braces(&self) -> bool {
    self.delimiter == Delimiter::Braces
  }
}


impl Ident {
  pub fn as_str(&self) -> &str {
    &self.identifier
  }
}


impl Punct {
  pub fn as_char(&self) -> char {
    self.punctuator
  }
}


impl LitStr {
  pub fn as_str(&self) -> &str {
    &self.value
  }

  pub fn prefix_range(&self) -> Option<Range> {
    self.prefix.as_ref().map(|ident| ident.range.clone())
  }
}


impl LitChar {
  pub fn as_char(&self) -> char {
    self.value
  }

  pub fn prefix_range(&self) -> Option<Range> {
    self.prefix.as_ref().map(|ident| ident.range.clone())
  }
}


impl LitInt {
  pub fn as_str(&self) -> &str {
    &self.value
  }

  pub fn radix(&self) -> Radix {
    self.radix
  }

  pub fn suffix_range(&self) -> Option<Range> {
    self.suffix.as_ref().map(|ident| ident.range.clone())
  }

  pub fn is_negative(&self) -> bool { self.sign == Sign::Negative }
  pub fn is_positive(&self) -> bool { self.sign == Sign::Positive }

  pub fn to_int(&self) -> BigInt {
    let start = {
      let mut start = 0;

      let first = self.value.chars().nth(0);

      if matches!(first, Some('+' | '-')) { start += 1; }
      if !self.radix.is_decimal()         { start += 2; }

      start
    };

    let end = self.value.len() - self.exponent
      .map_or(0, |e| e.to_string().len() + 1);

    let mut ret = self.radix.parse(&self.value[start..end]).unwrap();

    ret *= match self.sign {
      Sign::Negative => -1,
      Sign::Positive =>  1,
    };

    if let Some(exp) = self.exponent {
      ret *= BigInt::from(10).pow(exp as u32);
    }

    return ret;
  }
}


impl LitFloat {
  pub fn as_str(&self) -> &str {
    &self.value
  }

  pub fn suffix_range(&self) -> Option<Range> {
    self.suffix.as_ref().map(|ident| ident.range.clone())
  }

  pub fn is_negative(&self) -> bool { self.sign == Sign::Negative }
  pub fn is_positive(&self) -> bool { self.sign == Sign::Positive }

  pub fn to_float(&self) -> BigRational {
    self.value.parse().unwrap()
  }
}


impl LitBool {
  pub fn as_str(&self) -> &str {
    if self.value { "true"  }
    else          { "false" }
  }

  pub fn as_bool(&self) -> bool {
    self.value
  }

  pub fn is_true (&self) -> bool {  self.value }
  pub fn is_false(&self) -> bool { !self.value }
}


impl From<Group> for Vec<Token> {
  fn from(group: Group) -> Self {
    group.tokens
  }
}


impl From<LitBool> for bool {
  fn from(lit_bool: LitBool) -> Self {
    lit_bool.value
  }
}


#[derive(Debug, Clone)]
pub struct TokenStream {
  tokens: Arc<[Token]>,
  index : usize,
}


impl From<Group> for TokenStream {
  fn from(group: Group) -> Self {
    TokenStream {
      tokens: Arc::from(group.tokens.into_boxed_slice()),
      index : 0,
    }
  }
}


impl From<Vec<Token>> for TokenStream {
  fn from(tokens: Vec<Token>) -> Self {
    TokenStream {
      tokens: Arc::from(tokens.into_boxed_slice()),
      index : 0,
    }
  }
}


impl Iterator for TokenStream {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.tokens.len() {
      return None;
    }

    let token = self.tokens[self.index].clone();
    self.index += 1;
    Some(token)
  }
}


impl TokenStream {
  pub fn range(&self) -> Option<Range> {
    let start = &self.tokens.first()?.range().start;
    let end   = &self.tokens.last ()?.range().end  ;
    Some(start.clone()..end.clone())
  }

  pub fn is_empty(&self) -> bool {
    self.tokens.is_empty()
  }

  pub fn as_slice(&self) -> &[Token] {
    &self.tokens
  }

  pub fn extend_from_slice(&mut self, rest: &[Token]) {
    let mut tokens = Vec::new();

    tokens.reserve(self.tokens.len() + rest.len());
    tokens.extend_from_slice(&self.tokens[..self.index]);
    tokens.extend_from_slice(rest);
    tokens.extend_from_slice(&self.tokens[self.index..]);

    self.tokens = Arc::from(tokens.into_boxed_slice());
  }

  pub fn extend(&mut self, rest: TokenStream) {
    self.extend_from_slice(rest.as_slice());
  }

  pub fn peek_n(&self, n: usize) -> Option<&Token> {
    self.tokens.get(self.index + n)
  }

  pub fn peek(&self) -> Option<&Token> {
    self.peek_n(0)
  }

  pub fn advance_n(&mut self, n: usize) {
    self.index += n;
  }

  pub fn advance(&mut self) {
    self.advance_n(1);
  }
}
