use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Stream<T: Clone> {
  nodes: Arc<[T]>,
  index: usize,
}


impl<T: Clone> Stream<T> {
  pub fn new(nodes: Vec<T>) -> Self {
    Self { nodes: nodes.into(), index: 0 }
  }

  pub fn is_exhausted(&self) -> bool {
    self.index >= self.nodes.len()
  }

  pub fn at(&self, index: usize) -> Option<&T> {
    self.nodes.get(index)
  }

  pub fn peek(&self) -> Option<&T> {
    self.at(self.index)
  }

  pub fn peek_n(&self, n: usize) -> Option<&T> {
    self.at(self.index + n)
  }

  pub fn peek_next_n<const N: usize>(&self) -> [Option<&T>; N] {
    std::array::from_fn(|i| self.peek_n(i))
  }

  pub fn next(&mut self) -> Option<&T> {
    let index = self.index;

    if !self.is_exhausted() {
      self.index += 1;
    }

    self.at(index)
  }

  pub fn advance(&mut self) {
    self.index += 1;
  }

  pub fn advance_n(&mut self, n: usize) {
    self.index += n;
  }
}


impl<T: Clone> From<Vec<T>> for Stream<T> {
  fn from(nodes: Vec<T>) -> Self {
    Self::new(nodes)
  }
}


impl<T: Clone> From<Arc<[T]>> for Stream<T> {
  fn from(nodes: Arc<[T]>) -> Self {
    Self { nodes, index: 0 }
  }
}


impl<T: Clone> Iterator for Stream<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next().cloned()
  }
}
