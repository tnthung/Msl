

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
  /// `(...)`
  Parentheses,
  /// `[...]`
  Brackets,
  /// `{...}`
  Braces,
}


impl Delimiter {
  pub fn is_parentheses(&self) -> bool {
    matches!(self, Self::Parentheses)
  }

  pub fn is_brackets(&self) -> bool {
    matches!(self, Self::Brackets)
  }

  pub fn is_braces(&self) -> bool {
    matches!(self, Self::Braces)
  }
}
