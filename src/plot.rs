use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Url;

use std::str::FromStr;

use crate::area::Area;
use crate::io::{timestamp_deserializer, timestamp_serializer, url_deserializer, url_serializer};
use crate::lookup::Lookup;
use crate::site::Website;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Plot {
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
  /// Price in EUR
  price: u32,
  /// Area
  pub area: Area,
  /// Property Type
  kind: Option<Kind>,
  /// Size in sq. meters
  size: Option<u32>,
  /// Coverage Factor (%)
  coverage: Option<u32>,
  /// Building/Density Factor (%)
  density: Option<u32>,
  /// Maximum permitted height in meters
  height: Option<f32>,
  /// Maximum permitted number of storeys
  storeys: Option<u32>,
}

impl Plot {
  pub fn new(
    id: String,
    url: Url,
    website: Website,
    timestamp: DateTime<Utc>,
    price: u32,
    area: Area,
    kind: Option<Kind>,
    size: Option<u32>,
    coverage: Option<u32>,
    density: Option<u32>,
    height: Option<f32>,
    storeys: Option<u32>,
  ) -> Self {
    Self {
      id,
      url,
      website,
      timestamp,
      price,
      area,
      kind,
      size,
      coverage,
      density,
      height,
      storeys,
    }
  }
}

impl Eq for Plot {}

impl Default for Plot {
  fn default() -> Self {
    return Self {
      id: String::from("FOOBAR"),
      url: Url::from_str("https://foo.bar").unwrap(),
      website: Website::Bazaraki,
      timestamp: Utc::now(),
      price: 42000,
      area: Area::Limassol,
      kind: Some(Kind::Agricultural),
      size: Some(4200),
      coverage: Some(20),
      density: Some(40),
      height: Some(10.2),
      storeys: Some(1),
    };
  }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Kind {
  Agricultural,
  Commercial,
  Industrial,
  Residential,
  Touristic,
}

impl Lookup for Kind {
  fn lookup(from: &str) -> Option<Self> {
    if Regex::new(r"[Aa]gricultural").unwrap().find(from).is_some() {
      Some(Kind::Agricultural)
    } else if Regex::new(r"[Cc]ommercial").unwrap().find(from).is_some() {
      Some(Kind::Commercial)
    } else if Regex::new(r"[Ii]ndustrial").unwrap().find(from).is_some() {
      Some(Kind::Industrial)
    } else if Regex::new(r"[Rr]esidential").unwrap().find(from).is_some() {
      Some(Kind::Residential)
    } else if Regex::new(r"[Tt]ourist(ic)*").unwrap().find(from).is_some() {
      Some(Kind::Touristic)
    } else {
      None
    }
  }
}
