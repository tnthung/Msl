mod ast;


use types::stream::*;


pub trait Parsable: Sized {
  fn parse(ts: &Stream<ast::Ast>) -> Option<(Self, Stream<ast::Ast>)>;

  fn parse_in_place(ts: &mut Stream<ast::Ast>) -> Option<Self> {
    Self::parse(ts).map(|(x, n)| { *ts = n; x })
  }
}
