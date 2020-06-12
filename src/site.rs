use reqwest::Url;

use std::fmt;

use crate::area::Area;

#[derive(Debug)]
pub enum Website {
  Bazaraki,
  _Spitogatos,
  _ImmobilienScout,
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
