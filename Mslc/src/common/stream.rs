use std::sync::Arc;


#[derive(Clone, Debug)]
pub struct Stream<T: Clone> {
  nodes: Arc<[T]>,    // The stream provider
  count: usize,       // The number of nodes
  index: usize,       // The current index
}


impl<T: Clone> Stream<T> {
  pub fn new(nodes: Arc<[T]>) -> Self {
    let index = 0;
    let count = nodes.len();

    Self { nodes, count, index }
  }

  /// Get the length of elements haven't been consumed
  pub fn len(&self) -> usize {
    self.count - self.index
  }

  /// Get the total number of elements
  pub fn count(&self) -> usize {
    self.count
  }

  /// Check if the stream is exhausted
  pub fn is_exhausted(&self) -> bool {
    self.index >= self.count
  }

  /// Get the current index without consuming
  pub fn peek(&self) -> Option<&T> {
    self.nodes.get(self.index)
  }

  /// Get the next nth element without consuming
  pub fn peek_n(&self, n: usize) -> Option<&T> {
    self.nodes.get(self.index + n)
  }

  /// Get an array of future elements without consuming
  pub fn peek_next_n<const N: usize>(&self) -> [Option<&T>; N] {
    std::array::from_fn(|i| self.nodes.get(self.index + i))
  }

  /// Advance the stream by n elements
  pub fn advance(&mut self, n: usize) {
    self.index = (self.index + n).min(self.count);
  }
}


impl<T: Clone> Iterator for Stream<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.is_exhausted() { self.index += 1; }
    self.nodes.get(self.index - 1).cloned()
  }
}


impl<T: Clone> From<Vec<T>> for Stream<T> {
  fn from(nodes: Vec<T>) -> Self {
    Self::new(nodes.into())
  }
}


impl<T: Clone> From<&[T]> for Stream<T> {
  fn from(nodes: &[T]) -> Self {
    Self::new(nodes.into())
  }
}


impl<T: Clone> From<Arc<[T]>> for Stream<T> {
  fn from(nodes: Arc<[T]>) -> Self {
    Self::new(nodes)
  }
}
