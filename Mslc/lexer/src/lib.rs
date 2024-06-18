use types::token   ::*;
use types::location::*;

use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Lexer {
  file     : Arc<str>,
  source   : Arc<[char]>,
  line_idx : Arc<[usize]>,
  line     : usize,
  column   : usize,
  offset   : usize,
  was_space: bool,
  tokens   : Option<Arc<[Token]>>,
}


impl Lexer {
  pub fn new(file: &str) -> std::io::Result<Self> {
    let mut file: String = std::fs::canonicalize(file)
      .unwrap().to_str().unwrap().into();

    // remove UNC prefix
    if file.starts_with("\\\\?\\") {
      file.drain(..4);
    }

    let source = std::fs::read_to_string(&file)?;
    Ok(Self::init(file.into(), source))
  }

  #[cfg(test)]
  pub fn from_source(source: &str) -> Self {
    Self::init(Arc::from("source"), source.into())
  }

  fn init(file: Arc<str>, src: String) -> Self {
    let mut source   = Vec::new();
    let mut line_idx = Vec::new();

    source  .reserve(src.len());
    line_idx.reserve(src.len() / 30);   // random guess

    line_idx.push(0);

    for ch in src.chars() {
      source.push(ch);

      if ch == '\n' {
        line_idx.push(source.len());
      }
    }

    Self {
      file,
      source   : source  .into(),
      line_idx : line_idx.into(),
      line     : 1,
      column   : 1,
      offset   : 0,
      was_space: false,
      tokens   : None,
    }
  }
}
