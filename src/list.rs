use regex::Regex;
use scraper::{
  Html,
  Selector
};

use crate::cond::Condition;


#[derive(Debug)]
pub enum Listing {

  Apartment {
    /// Unique Identifier
    id: String,
    /// Price in EUR
    price: u32,
    /// Size in sq. meters
    size: u32,
    /// Condition
    cond: Option<Condition>,
    /// Floor #
    floor: Option<u8>,
    /// Year of constructon
    year: Option<u32>,
  }

}

impl Default for Listing {
  
  fn default() -> Self {
    return Self::Apartment {
      id: String::from("FOOBAR"),
      price: 42000,
      size: 42,
      cond: Some(Condition::New),
      floor: Some(42),
      year: Some(4242),
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


    // Get characteristics div
    let size_sel = Selector::parse("div.announcement-characteristics").unwrap();
    let div = html.select(&size_sel).next().unwrap().inner_html();


    // Parse size
    let re = Regex::new(r"([0-9]+) m²").unwrap();
    let size_str = &re.captures(&div).unwrap()[1];
    let size:u32 = size_str.parse::<f32>().unwrap() as u32;

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

    
    println!("id: {:?}", id);
    println!("price: {:?}", price);
    println!("size: {:?}", size);
    println!("cond: {:?}", cond);


    return Listing::default();
  }

}
