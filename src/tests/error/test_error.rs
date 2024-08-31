use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TestError {
  pub message :String
}

impl TestError {
  pub fn new(
    message :String
  ) -> Self {
    Self {
      message
    }
  }
}

impl Display for TestError {
  fn fmt(
    &self,
    f: &mut Formatter<'_>
  ) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self.message
    )
  }
}

impl Error for TestError {
}