use std::sync::Arc;


pub type Range = std::ops::Range<Location>;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
  file  : Arc<str>,
  line  : usize,
  column: usize,
  offset: usize,
}


impl Location {
  pub fn new(
    file  : Arc<str>,
    line  : usize,
    column: usize,
    offset: usize
  ) -> Self {
    Self { file, line, column, offset }
  }

  pub fn file(&self) -> &str {
    &self.file
  }

  pub fn line(&self) -> usize {
    self.line
  }

  pub fn column(&self) -> usize {
    self.column
  }

  pub fn offset(&self) -> usize {
    self.offset
  }

  pub fn to_range(&self) -> Range {
    self.clone()..self.clone()
  }
}


impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}
