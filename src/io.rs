use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserializer, Serializer};

use std::str::FromStr;

pub fn timestamp_serializer<S>(val: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  s.serialize_str(&format!("{}", val))
}

pub fn timestamp_deserializer<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = serde::de::Deserialize::deserialize(d)?;
  Ok(DateTime::from_str(s).expect("Couldn't parse datetime"))
}

pub fn url_serializer<S>(val: &Url, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  s.serialize_str(val.as_str())
}

pub fn url_deserializer<'de, D>(d: D) -> Result<Url, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = serde::de::Deserialize::deserialize(d)?;
  Ok(Url::from_str(s).expect("Couldn't parse Url"))
}
