use chrono::{DateTime, Utc};
use reqwest::Url;
use scraper::Html;

use crate::error::Error;
use crate::parse;
use crate::plot::Plot;
use crate::property::Property;
use crate::site::Website;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Listing {
  Plot(Plot),
  Property(Property),
}

impl Listing {
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
