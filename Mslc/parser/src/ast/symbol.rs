use types::token::*;
use types::stream::*;
use types::location::*;

use super::Ast;


#[derive(Debug, Clone)]
pub struct Symbol {
  pub range : Range,
  pub symbol: String,
}


impl Symbol {
  pub fn parse(ts: &Stream<Ast>, symbol: &str) -> Option<(Symbol, Stream<Ast>)> {
    let mut ts = ts.clone();

    let start = ts.peek()?.range().start.clone();
    let mut end = start.clone();

    for (i, c) in symbol.to_string().chars().enumerate() {
      match ts.peek()? {
        Ast::Token(Token::Punct(Punct { range, spacing, punctuator }))
          if *punctuator == c && (i == 0 || spacing.has_no_leading())
        => {
          end = range.end.clone();
          ts.advance();
        }

        _ => return None,
      }
    }

    Some((
      Symbol {
        range : start..end,
        symbol: symbol.to_string(),
      },
      ts,
    ))
  }

  pub fn parse_in_place(ts: &mut Stream<Ast>, symbol: &str) -> Option<Symbol> {
    Symbol::parse(ts, symbol).map (|(s, t)| { *ts = t; s })
  }
}
