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
      Radix::Binary      => T::from_str_radix(&s, 2 ),
      Radix::Octal       => T::from_str_radix(&s, 8 ),
      Radix::Decimal     => T::from_str_radix(&s, 10),
      Radix::Hexadecimal => T::from_str_radix(&s, 16),
    }
  }

  pub fn get_digit_checker(&self) -> fn(char) -> bool {
    match self {
      Radix::Binary      => |c| c == '0' || c == '1',
      Radix::Octal       => |c| c.is_ascii_digit() && c <= '7',
      Radix::Decimal     => |c| c.is_ascii_digit(),
      Radix::Hexadecimal => |c| c.is_ascii_hexdigit(),
    }
  }
}


impl From<u32> for Radix {
  fn from(n: u32) -> Self {
    match n {
      2  => Radix::Binary,
      8  => Radix::Octal,
      10 => Radix::Decimal,
      16 => Radix::Hexadecimal,
      _  => unreachable!(),
    }
  }
}


impl From<Radix> for u32 {
  fn from(radix: Radix) -> Self {
    match radix {
      Radix::Binary      => 2,
      Radix::Octal       => 8,
      Radix::Decimal     => 10,
      Radix::Hexadecimal => 16,
    }
  }
}
