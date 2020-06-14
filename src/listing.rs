use chrono::{DateTime, Utc};
use reqwest::Url;
use scraper::Html;
use serde::{Deserializer, Serializer};

use std::str::FromStr;

use crate::area::Area;
use crate::cond::Condition;
use crate::error::Error;
use crate::kind::Kind;
use crate::parse;
use crate::site::Website;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Listing {
  /// Unique Identifier
  id: String,
  /// URL
  #[serde(
    serialize_with = "url_serializer",
    deserialize_with = "url_deserializer"
  )]
  pub url: Url,
  /// Website,
  pub website: Website,
  /// Timestamp
  #[serde(
    serialize_with = "timestamp_serializer",
    deserialize_with = "timestamp_deserializer"
  )]
  timestamp: DateTime<Utc>,
  /// Property Type
  kind: Kind,
  /// Price in EUR
  price: u32,
  /// Area
  pub area: Area,
  /// Size in sq. meters
  size: Option<u32>,
  /// Condition
  cond: Option<Condition>,
  /// Year of constructon
  year: Option<u32>,
  /// # of bedrooms
  n_bedrooms: Option<u8>,
  /// # of bathrooms
  n_bathrooms: Option<u8>,
  /// Postal Code
  post_code: Option<u32>,
}

impl Default for Listing {
  fn default() -> Self {
    return Self {
      id: String::from("FOOBAR"),
      url: Url::from_str("https://foo.bar").unwrap(),
      website: Website::Bazaraki,
      timestamp: Utc::now(),
      kind: Kind::Villa,
      price: 42000,
      area: Area::Limassol,
      size: Some(42),
      cond: Some(Condition::Resale),
      year: Some(1992),
      n_bedrooms: Some(1),
      n_bathrooms: Some(1),
      post_code: Some(2020),
    };
  }
}

impl Listing {
  pub fn try_from_html(html: &Html, url: &Url, website: &Website) -> Result<Self, Error> {
    match website {
      Website::Bazaraki => parse::parse_bazaraki(html, url),
      _ => unimplemented!(),
    }
  }

  pub fn new(
    id: String,
    url: Url,
    website: Website,
    timestamp: DateTime<Utc>,
    kind: Kind,
    price: u32,
    area: Area,
    size: Option<u32>,
    cond: Option<Condition>,
    year: Option<u32>,
    n_bedrooms: Option<u8>,
    n_bathrooms: Option<u8>,
    post_code: Option<u32>,
  ) -> Self {
    Self {
      id,
      url,
      website,
      timestamp,
      kind,
      price,
      area,
      size,
      cond,
      year,
      n_bedrooms,
      n_bathrooms,
      post_code,
    }
  }
}

fn timestamp_serializer<S>(val: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  s.serialize_str(&format!("{}", val))
}

fn timestamp_deserializer<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = serde::de::Deserialize::deserialize(d)?;
  Ok(DateTime::from_str(s).expect("Couldn't parse datetime"))
}

fn url_serializer<S>(val: &Url, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  s.serialize_str(val.as_str())
}

fn url_deserializer<'de, D>(d: D) -> Result<Url, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = serde::de::Deserialize::deserialize(d)?;
  Ok(Url::from_str(s).expect("Couldn't parse Url"))
}

#[cfg(test)]
mod test {
  extern crate serde_json;
  use super::*;
  use serde_json::{from_str, to_string};

  #[test]
  fn listing_serde() {
    let listing = Listing::default();
    let json = to_string(&listing).expect("Couldn't serialize listing");
    let parsed: Listing = from_str(&json).expect("Couldn't deserialize listing");

    assert_eq!(listing, parsed);
  }
}
