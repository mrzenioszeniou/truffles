use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
  msg: String,
}

impl Error {
  #[allow(dead_code)]
  pub fn from<T>(from: T) -> Self
  where
    T: Display,
  {
    Self {
      msg: format!("{}", from),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Error:{}", self.msg)
  }
}
