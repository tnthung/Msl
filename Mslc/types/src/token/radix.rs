use num::bigint::ParseBigIntError;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radix {
  /// prefixed with `0b`
  Binary,
  /// prefixed with `0o`
  Octal,
  /// no prefix
  Decimal,
  /// prefixed with `0x`
  Hexadecimal,
}


impl Radix {
  pub fn is_binary(&self) -> bool {
    matches!(self, Radix::Binary)
  }

  pub fn is_octal(&self) -> bool {
    matches!(self, Radix::Octal)
  }

  pub fn is_decimal(&self) -> bool {
    matches!(self, Radix::Decimal)
  }

  pub fn is_hexadecimal(&self) -> bool {
    matches!(self, Radix::Hexadecimal)
  }

  pub fn parse<T>(&self, s: &str) -> Result<T, ParseBigIntError>
  where
    T: std::str::FromStr + num::Num<FromStrRadixErr = ParseBigIntError>
  {
    match self {
      Radix::Binary      => T::from_str_radix(&s[2..], 2 ),
      Radix::Octal       => T::from_str_radix(&s[2..], 8 ),
      Radix::Decimal     => T::from_str_radix(&s     , 10),
      Radix::Hexadecimal => T::from_str_radix(&s[2..], 16),
    }
  }
}
