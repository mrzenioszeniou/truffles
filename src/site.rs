use reqwest::Url;

use std::fmt;
use std::str::FromStr;

use crate::area::Area;

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

  pub fn get_search_root(&self, area: &Area) -> Url {
    match self {
      Self::Bazaraki => match area {
        Area::Ammochostos => Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/ammochostos-district/?ordering=newest").unwrap(),
        Area::Larnaka => Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/larnaka-district-larnaca/?ordering=newest").unwrap(),
        Area::Lefkosia => Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lefkosia-district-nicosia/?ordering=newest").unwrap(),
        Area::Limassol => Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lemesos-district-limassol/?ordering=newest").unwrap(),
        Area::Paphos => Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/pafos-district-paphos/?ordering=newest").unwrap(),
      },
      _ => unimplemented!(),
    }
  }

  pub fn get_search_roots(&self) -> Vec<Url> {
    match self {
      Self::Bazaraki => Area::all()
        .iter()
        .map(|a| self.get_search_root(a))
        .collect(),
      _ => unimplemented!(),
    }
  }
}
