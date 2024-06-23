use std::sync::Arc;


pub type Range = std::ops::Range<Location>;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
  /// The file path
  file  : Arc<str>,
  /// The line number
  line  : usize,
  /// The column number
  column: usize,
  /// The character index
  index : usize,
  /// The byte offset
  offset: usize,
}


impl Location {
  pub fn new(
    file  : Arc<str>,
    line  : usize,
    column: usize,
    index : usize,
    offset: usize,
  ) -> Self {
    Self { file, line, column, index, offset }
  }

  /// Create a fake location
  pub fn fake() -> Self {
    Self::new("".into(), 1, 1, 0, 0)
  }

  /// The file path
  pub fn file(&self) -> &str { &self.file }

  /// The line number
  pub fn line(&self) -> usize { self.line }

  /// The column number
  pub fn column(&self) -> usize { self.column }

  /// The character index
  pub fn index(&self) -> usize { self.index }

  /// The byte offset
  pub fn offset(&self) -> usize { self.offset }

  /// Create a range from self to the given location
  pub fn to(&self, end: &Self) -> Range {
    self.clone()..end.clone()
  }
}


impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}


impl From<&Location> for std::ops::Range<Location> {
  fn from(loc: &Location) -> Self {
    loc.clone()..loc.clone()
  }
}
