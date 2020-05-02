mod list;
mod cond;

extern crate regex;
extern crate scraper;

use std::fs::File;
use std::io::Read;

use scraper::Html;


use list::Listing;


fn main() {

  let mut file : File = File::open("res/listing_1.html").unwrap();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  let document : Html = Html::parse_document(&content);

  println!("{:?}", Listing::from(&document));
}