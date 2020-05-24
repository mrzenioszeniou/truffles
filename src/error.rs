struct Error {
  msg: String,
}

impl From<reqwest::Error> for Error {
  
  fn from(from: reqwest::Error) -> Self {
    Error {
      msg: format!("{}", from),
    }
  }
}