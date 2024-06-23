use types::token::*;
use types::location::*;


#[derive(Debug, Clone)]
pub enum Ast {
  Token(Token),
}


impl Ast {
  pub fn range(&self) -> &Range {
    match self {
      Ast::Token (t) => t.range(),
    }
  }
}


macro_rules! impl_commons {
  ($variant:ident) => {
    impl TryFrom<Ast> for $variant {
      type Error = Ast;

      fn try_from(token: Ast) -> Result<Self, Self::Error> {
        match token {
          Ast::$variant(value) => Ok(value),
          _ => Err(token),
        }
      }
    }

    impl<'a> TryFrom<&'a Ast> for &'a $variant {
      type Error = &'a Ast;

      fn try_from(token: &'a Ast) -> Result<Self, Self::Error> {
        match token {
          Ast::$variant(value) => Ok(value),
          _ => Err(token),
        }
      }
    }

    impl From<$variant> for Ast {
      fn from(value: $variant) -> Self {
        Ast::$variant(value)
      }
    }

    impl From<&$variant> for Ast {
      fn from(value: &$variant) -> Self {
        Ast::$variant(value.clone())
      }
    }
  };
}


impl_commons!(Token);
