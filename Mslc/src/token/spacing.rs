

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
  pub fn has_both(&self) -> bool {
    matches!(self, Spacing::Both)
  }

  pub fn has_none(&self) -> bool {
    matches!(self, Spacing::None)
  }

  pub fn is_leading(&self) -> bool {
    matches!(self, Spacing::Leading)
  }

  pub fn is_trailing(&self) -> bool {
    matches!(self, Spacing::Trailing)
  }

  pub fn has_leading(&self) -> bool {
    matches!(self, Spacing::Both | Spacing::Leading)
  }

  pub fn has_trailing(&self) -> bool {
    matches!(self, Spacing::Both | Spacing::Trailing)
  }

  pub fn has_no_leading(&self) -> bool {
    matches!(self, Spacing::None | Spacing::Trailing)
  }

  pub fn has_no_trailing(&self) -> bool {
    matches!(self, Spacing::None | Spacing::Leading)
  }
}
