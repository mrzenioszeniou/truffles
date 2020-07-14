use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Url;
use scraper::Html;

use std::str::FromStr;

use crate::error::Error;
use crate::lookup::Lookup;
use crate::parse;
use crate::plot::Plot;
use crate::property::Property;
use crate::site::Website;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Listing {
  Plot(Plot),
  Property(Property),
}

impl Listing {
  pub fn kind(&self) -> Kind {
    match self {
      Self::Plot(_) => Kind::Plot,
      Self::Property(_) => Kind::Property,
    }
  }

  pub fn timestamp(&self) -> &DateTime<Utc> {
    match self {
      Self::Plot(plot) => &plot.timestamp,
      Self::Property(prop) => &prop.timestamp,
    }
  }

  pub fn url(&self) -> &Url {
    match self {
      Self::Plot(plot) => &plot.url,
      Self::Property(prop) => &prop.url,
    }
  }

  pub fn try_from_html(html: &Html, url: &Url, website: &Website) -> Result<Self, Error> {
    match website {
      Website::Bazaraki => parse::parse_bazaraki(html, url),
      _ => unimplemented!(),
    }
  }
}

impl Default for Listing {
  fn default() -> Self {
    Self::Property(Property::default())
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
  Plot,
  Property,
}

impl Kind {
  pub fn all() -> Vec<Kind> {
    vec![Self::Plot, Self::Property]
  }
}

impl FromStr for Kind {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_ascii_lowercase().as_str() {
      "plot" => Ok(Self::Plot),
      "property" => Ok(Self::Property),
      _ => Err(Error::from(format!(
        "Couldn't parse {} as a listing kind",
        s
      ))),
    }
  }
}

impl Lookup for Kind {
  fn lookup(from: &str) -> Option<Self> {
    if Regex::new(r"([Pp]lots*)|([Ll]and)")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Kind::Plot)
    } else if Regex::new(r"[Pp]ropert((y)|(ies))")
      .unwrap()
      .find(from)
      .is_some()
    {
      Some(Kind::Property)
    } else {
      None
    }
  }
}
