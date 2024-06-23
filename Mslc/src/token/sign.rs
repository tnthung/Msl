

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
  /// `+` or no sign
  Positive,
  /// `-`
  Negative,
}


impl Sign {
  pub fn is_positive(&self) -> bool {
    matches!(self, Sign::Positive)
  }

  pub fn is_negative(&self) -> bool {
    matches!(self, Sign::Negative)
  }
}
