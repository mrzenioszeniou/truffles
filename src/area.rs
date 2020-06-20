use regex::Regex;

use std::str::FromStr;

use crate::lookup::Lookup;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Area {
  Ammochostos,
  Larnaka,
  Lefkosia,
  Limassol,
  Paphos,
}

impl FromStr for Area {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_ascii_lowercase().as_str() {
      "ammochostos" | "famagusta" => Ok(Self::Ammochostos),
      "larnaka" | "larnaca" => Ok(Self::Larnaka),
      "lefkosia" | "nicosia" => Ok(Self::Lefkosia),
      "limassol" | "lemesos" => Ok(Self::Limassol),
      "paphos" | "pafos" => Ok(Self::Paphos),
      _ => Err(format!("Couldn't parse '{}' as an area", s)),
    }
  }
}

impl Area {
  pub fn all() -> Box<[Self]> {
    Box::new([
      Area::Ammochostos,
      Area::Larnaka,
      Area::Lefkosia,
      Area::Limassol,
      Area::Paphos,
    ])
  }
}

impl Lookup for Area {
  fn lookup(from: &str) -> Option<Self> {
    if Regex::new(r"[Ff]amagusta").unwrap().find(from).is_some() {
      Some(Area::Ammochostos)
    } else if Regex::new(r"[Ll]arna[kc]a").unwrap().find(from).is_some() {
      Some(Area::Larnaka)
    } else if Regex::new(r"([Ll]efkosia)|([Nn]icosia)")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Area::Lefkosia)
    } else if Regex::new(r"([Ll]imassol)|([Ll]emesos)")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Area::Limassol)
    } else if Regex::new(r"([Pp]a((f)|(ph))os)")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Area::Paphos)
    } else {
      None
    }
  }
}
