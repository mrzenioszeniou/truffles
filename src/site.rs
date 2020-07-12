use reqwest::Url;

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Website {
  Bazaraki,
  Spitogatos,
  ImmobilienScout24,
}

impl FromStr for Website {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_ascii_lowercase().as_str() {
      "bazaraki" => Ok(Self::Bazaraki),
      "spitogatos" => Ok(Self::Spitogatos),
      "immobilienscout24" => Ok(Self::ImmobilienScout24),
      _ => Err(format!("Couldn't parse '{}' as a Website instance", s)),
    }
  }
}

impl fmt::Display for Website {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Website {
  pub fn get_root(&self) -> Url {
    match self {
      Self::Bazaraki => Url::parse("https://www.bazaraki.com").unwrap(),
      _ => unimplemented!(),
    }
  }
}
