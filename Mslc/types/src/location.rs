use std::sync::Arc;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
  file  : Arc<String>,
  line  : usize,
  column: usize,
  offset: usize,
}


impl Location {
  pub fn new(
    file  : impl Into<String>,
    line  : usize,
    column: usize,
    offset: usize
  ) -> Self {
    let file = Arc::new(file.into());
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
}


impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}
