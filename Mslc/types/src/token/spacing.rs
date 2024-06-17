

#[derive(Debug, Clone)]
pub enum Spacing {
  /// `+` in the middle of `_+_`
  Both,
  /// `+` in the middle of `_++`
  Leading,
  /// `+` in the middle of `++_`
  Trailing,
  /// `+` in the middle of `+++`
  None,
}


impl Spacing {
  pub fn is_both(&self) -> bool {
    matches!(self, Spacing::Both)
  }

  pub fn is_leading(&self) -> bool {
    matches!(self, Spacing::Leading)
  }

  pub fn is_trailing(&self) -> bool {
    matches!(self, Spacing::Trailing)
  }

  pub fn is_none(&self) -> bool {
    matches!(self, Spacing::None)
  }
}
