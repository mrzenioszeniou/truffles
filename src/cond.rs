
#[derive(Debug)]
pub enum Condition {
  New,
  Resale,
  UnderConstruction,
}

impl From<&str> for Condition {

  fn from(cond:&str) -> Self {
    match cond.to_lowercase().trim() {
      "new" | "brand new" => Self::New,
      "used" | "resale" => Self::Resale,
      "under construction" => Self::UnderConstruction,
      _ => panic!("Couldn't parse {} as a Condition", cond),
    }
  }

}