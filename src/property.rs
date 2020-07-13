use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Url;

use std::str::FromStr;

use crate::area::Area;
use crate::cond::Condition;
use crate::io::{timestamp_deserializer, timestamp_serializer, url_deserializer, url_serializer};
use crate::lookup::Lookup;
use crate::site::Website;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Property {
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
  pub timestamp: DateTime<Utc>,
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

impl Property {
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

impl Default for Property {
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Kind {
  Apartment,
  Bungalow,
  Duplex,
  House,
  Maisonette,
  Villa,
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
