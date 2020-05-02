use regex::Regex;
use scraper::{
  Html,
  Selector
};

use crate::cond::Condition;

#[derive(Debug)]
pub struct Listing {

  /// Unique Identifier
  id: String,
  /// Price in EUR
  price: u32,
  /// Area in sq. meters
  area: u32,
  /// Condition
  cond: Option<Condition>,
  /// Floor #
  floor: Option<u8>,
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
      price: 42000,
      area: 42,
      cond: None,
      floor: None,
      year: None,
      n_bedrooms: None,
      n_bathrooms: None,
      post_code: None,
    };
  }

}

impl From<&Html> for Listing {

  fn from(html :&Html) -> Self {

    // Common selectors
    let ul_sel = Selector::parse("ul").unwrap();
    let li_sel = Selector::parse("li").unwrap();
    let a_sel = Selector::parse("a").unwrap();

    // Parse UID
    let id_sel:Selector = Selector::parse("span[itemprop=\"sku\"").unwrap();
    let id:String = html.select(&id_sel).next().unwrap().inner_html();

    // Parse price
    let price_sel:Selector = Selector::parse("meta[itemprop=\"price\"]").unwrap();
    let price_str:&str= html.select(&price_sel).next().unwrap().value().attr("content").unwrap();
    let price:u32 = price_str.parse::<f32>().unwrap() as u32;

    let size_sel = Selector::parse("div.announcement-characteristics").unwrap();
    let div = html.select(&size_sel).next().unwrap().inner_html();

    // Parse size
    let re = Regex::new(r"([0-9]+) m²").unwrap();
    let area_str = &re.captures(&div).unwrap()[1];
    let area:u32 = area_str.parse::<f32>().unwrap() as u32;

    // Parse condition
    let cond = if Regex::new(r"[Rr]esale").unwrap().find(&div).is_some() {
      Some(Condition::Resale)
    } else if Regex::new(r"[Bb]rand\s+[Nn]ew").unwrap().find(&div).is_some() {
      Some(Condition::New)
    } else if Regex::new(r"[Uu]nder\s+[Cc]onstruction").unwrap().find(&div).is_some() {
      Some(Condition::UnderConstruction)
    } else {
      None
    };

    // Parse 


    return Listing {
      id,
      price,
      area,
      cond,
      floor: None,
      n_bathrooms: None,
      n_bedrooms: None,
      post_code: None,
      year: None,
    };
  }

}
