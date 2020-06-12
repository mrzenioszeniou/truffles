use regex::Regex;

use crate::lookup::Lookup;

#[derive(Debug, Serialize)]
pub enum Kind {
  House,
  Apartment,
  Duplex,
  Maisonette,
  Bungalow,
  Villa,
}

impl Default for Kind {
  fn default() -> Self {
    Kind::Villa
  }
}

impl Lookup for Kind {
  fn lookup(from: &str) -> Option<Self> {
    if Regex::new(r"([Aa]partment)|([Pp]enthouse)")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Kind::Apartment)
    } else if Regex::new(r"[Hh]ouse").unwrap().find(from).is_some() {
      Some(Kind::House)
    } else if Regex::new(r"[Ss]emi-*[Dd]etached")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Kind::Duplex)
    } else if Regex::new(r"[Mm]aisonette").unwrap().find(from).is_some() {
      Some(Kind::Maisonette)
    } else if Regex::new(r"[Bb]ungalow").unwrap().find(from).is_some() {
      Some(Kind::Bungalow)
    } else if Regex::new(r"[Vv]illa").unwrap().find(from).is_some() {
      Some(Kind::Villa)
    } else {
      None
    }
  }
}
