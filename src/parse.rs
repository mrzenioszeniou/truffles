use chrono::Utc;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};

use crate::area::Area;
use crate::cond::Condition;
use crate::error::Error;
use crate::kind::Kind;
use crate::listing::Listing;
use crate::lookup::Lookup;
use crate::site::Website;

use std::iter::Iterator;

pub fn parse_bazaraki(html: &Html, url: &Url) -> Result<Listing, Error> {
  // Common regular expressions
  let re_int = Regex::new(r"[0-9]+").unwrap();

  // Common selectors
  let li_sel = Selector::parse("li").unwrap();
  let a_sel = Selector::parse("a").unwrap();
  let span_sel = Selector::parse("span").unwrap();

  // Parse UID
  let id_sel: Selector = Selector::parse("span[itemprop=\"sku\"").unwrap();
  let id: String = format!(
    "bazaraki_{}",
    html.select(&id_sel).next().unwrap().inner_html()
  );

  // Get timestamp
  let timestamp = Utc::now();

  // Parse price
  let price_sel: Selector = Selector::parse("meta[itemprop=\"price\"]").unwrap();
  let price_str: &str = html
    .select(&price_sel)
    .next()
    .unwrap()
    .value()
    .attr("content")
    .unwrap();
  let price: u32 = price_str.parse::<f32>().unwrap() as u32;

  // Parse area
  let area_sel: Selector = Selector::parse("span[itemprop=\"address\"]").unwrap();
  let area_str = html.select(&area_sel).next().unwrap().inner_html();
  let area = match Area::lookup(&area_str) {
    Some(area) => area,
    None => panic!("Couldn't parse area for {}", url),
  };

  // Get useful html handles
  let chars_sel = Selector::parse("div.announcement-characteristics").unwrap();
  let chars = html.select(&chars_sel).next().unwrap();
  let chars_html = chars.inner_html();
  let desc_sel = Selector::parse("div.announcement-description").unwrap();
  let desc = html.select(&desc_sel).next().unwrap();
  let desc_html = desc.inner_html();

  // Parse property kind
  let kind = Kind::lookup(&chars_html)
    .ok_or("Couldn't parse kind")
    .unwrap();

  // Parse size
  let re_size = Regex::new(r"([0-9]+) m²").unwrap();
  let size = re_size
    .captures(&chars_html)
    .map(|g| g[1].parse::<f32>().map(|a| a as u32).ok())
    .flatten();

  // Parse condition
  let cond = Condition::lookup(&chars_html);

  // Parse bedrooms
  let n_bedrooms = if Regex::new(r"[Ss]tudio")
    .unwrap()
    .find(&chars_html)
    .is_some()
  {
    Some(0)
  } else {
    let re_bedrooms = Regex::new(r"[Bb]edrooms*").unwrap();
    chars
      .select(&li_sel)
      .filter(|li| re_bedrooms.find(&li.inner_html()).is_some())
      .next()
      .map(|li| {
        li.select(&a_sel)
          .next()
          .unwrap()
          .inner_html()
          .trim()
          .parse()
          .unwrap()
      })
  };

  // Parse bathrooms
  let re_bathrooms = Regex::new(r"[Bb]athrooms*").unwrap();
  let n_bathrooms = chars
    .select(&li_sel)
    .filter(|li| re_bathrooms.find(&li.inner_html()).is_some())
    .next()
    .map(|li| {
      li.select(&span_sel)
        .filter(|span| re_int.find(&span.inner_html()).is_some())
        .next()
        .map(|span| {
          let str_bathrooms = span.inner_html();
          str_bathrooms.trim().parse().unwrap()
        })
    })
    .flatten();

  // Parse post code
  let re_post = Regex::new(r"[Pp]ostal\s+[Cc]ode").unwrap();
  let post_code = chars
    .select(&li_sel)
    .filter(|li| re_post.find(&li.inner_html()).is_some())
    .next()
    .map(|li| {
      li.select(&span_sel)
        .filter(|span| re_int.find(&span.inner_html()).is_some())
        .next()
        .map(|span| {
          let str_post_code = span.inner_html();
          str_post_code.trim().parse().unwrap()
        })
    })
    .flatten();

  // Parse year
  let re_year = Regex::new(r"(20[0-3][0-9])|(19[0-9][0-9])").unwrap();
  let year = re_year
    .captures_iter(&desc_html)
    .filter_map(|c| c[0].parse::<u32>().ok())
    .min();

  Ok(Listing::new(
    id,
    url.clone(),
    Website::Bazaraki,
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
  ))
}

#[cfg(test)]
mod test {
  use super::*;
  use scraper::Html;
  use std::fs::File;
  use std::io::Read;
  use std::str::FromStr;

  #[test]
  fn bazaraki_parser() {
    let paths = vec![
      "res/listing_1.html",
      "res/listing_2.html",
      "res/listing_3.html",
      "res/listing_4.html",
      "res/listing_5.html",
      "res/listing_6.html",
      "res/listing_7.html",
    ];

    for path in paths.iter() {
      let mut content = String::new();
      let mut file = File::open(path)
        .or(Err(format!("Couldn't open {}", path)))
        .unwrap();
      file
        .read_to_string(&mut content)
        .or(Err(format!("Couldn't read {}", path)))
        .unwrap();
      let document = Html::parse_document(&content);

      println!(
        "{:?}\n",
        parse_bazaraki(&document, &Url::from_str("https://foo.bar").unwrap())
          .expect("Couldn't parse bazaraki listing")
      );
    }
  }
}
