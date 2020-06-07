use regex::Regex;
use scraper::{
  Html,
  Selector
};

use crate::cond::Condition;
use crate::kind::Kind;
use crate::lookup::Lookup;

use std::iter::Iterator;

#[derive(Debug, Serialize)]
pub struct Listing {

  /// Unique Identifier
  pub id: String,
  /// Property Type
  kind: Kind,
  /// Price in EUR
  price: u32,
  /// Area in sq. meters
  area: Option<u32>,
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
      kind: Kind::Villa,
      price: 42000,
      area: None,
      cond: None,
      year: None,
      n_bedrooms: None,
      n_bathrooms: None,
      post_code: None,
    };
  }

}

impl From<&Html> for Listing {

  fn from(html :&Html) -> Self {

    // Common regular expressions
    let re_int = Regex::new(r"[0-9]+").unwrap();

    // Common selectors
    let li_sel = Selector::parse("li").unwrap();
    let a_sel = Selector::parse("a").unwrap();
    let span_sel = Selector::parse("span").unwrap();

    // Parse UID
    let id_sel:Selector = Selector::parse("span[itemprop=\"sku\"").unwrap();
    let id:String = html.select(&id_sel).next().unwrap().inner_html();

    // Parse price
    let price_sel:Selector = Selector::parse("meta[itemprop=\"price\"]").unwrap();
    let price_str:&str= html.select(&price_sel).next().unwrap().value().attr("content").unwrap();
    let price:u32 = price_str.parse::<f32>().unwrap() as u32;

    // Get useful html handles
    let chars_sel = Selector::parse("div.announcement-characteristics").unwrap();
    let chars = html.select(&chars_sel).next().unwrap();
    let chars_html = chars.inner_html();
    let desc_sel = Selector::parse("div.announcement-description").unwrap();
    let desc = html.select(&desc_sel).next().unwrap();
    let desc_html = desc.inner_html();

    // Parse property kind
    let kind = Kind::lookup(&chars_html).ok_or("Couldn't parse kind").unwrap();

    // Parse size
    let re_size = Regex::new(r"([0-9]+) m²").unwrap();
    let area = re_size.captures(&chars_html).map(|g| g[1].parse::<f32>().map(|a| a as u32).ok()).flatten();

    // Parse condition
    let cond = Condition::lookup(&chars_html);

    // Parse bedrooms
    let n_bedrooms = if Regex::new(r"[Ss]tudio").unwrap().find(&chars_html).is_some() {
      Some(0)
    } else {
      let re_bedrooms = Regex::new(r"[Bb]edrooms*").unwrap();
      chars.select(&li_sel)
          .filter( |li| re_bedrooms.find(&li.inner_html()).is_some())
          .next()
          .map(|li| li.select(&a_sel).next().unwrap().inner_html().trim().parse().unwrap())
    };

    // Parse bathrooms
    let re_bathrooms = Regex::new(r"[Bb]athrooms*").unwrap();
    let n_bathrooms = chars.select(&li_sel)
        .filter(|li| re_bathrooms.find(&li.inner_html()).is_some())
        .next()
        .map(|li| 
          li.select(&span_sel)
            .filter(|span| re_int.find(&span.inner_html()).is_some())
            .next()
            .map(|span| {
              let str_bathrooms = span.inner_html();
              str_bathrooms.trim().parse().unwrap()
            })
          ).flatten();

    // Parse post code
    let re_post = Regex::new(r"[Pp]ostal\s+[Cc]ode").unwrap();
    let post_code = chars.select(&li_sel)
        .filter(|li| re_post.find(&li.inner_html()).is_some())
        .next()
        .map(|li| 
          li.select(&span_sel)
          .filter(|span| re_int.find(&span.inner_html()).is_some())
          .next()
          .map(|span| {
            let str_post_code = span.inner_html();
            str_post_code.trim().parse().unwrap()
          })
        ).flatten();

    // Parse year
    let re_year = Regex::new(r"(20[0-9][0-9])|(19[0-9][0-9])").unwrap();
    let year = re_year.captures_iter(&desc_html).filter_map(|c| c[0].parse::<u32>().ok()).min();

    return Listing {
      id,
      kind,
      price,
      area,
      cond,
      n_bedrooms,
      n_bathrooms,
      post_code,
      year: year,
    };
  }

}
#[cfg(test)]
mod test {
  use super::*;
  use std::fs::File;
  use std::io::Read;

  use scraper::Html;

  #[test]
  fn parse() {
    
    let paths = vec![
      "res/listing_1.html",
      "res/listing_2.html",
      "res/listing_3.html",
    ];

    for path in paths.iter() {
      let mut content = String::new();
      let mut file = File::open(path).or(Err(format!("Couldn't open {}", path))).unwrap();
      file.read_to_string(&mut content).or(Err(format!("Couldn't read {}", path))).unwrap();
      let document = Html::parse_document(&content);
      println!("{}: {:?}\n", path,  Listing::from(&document));
    }

  }

}