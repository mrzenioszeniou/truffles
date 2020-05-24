extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate tokio;

mod listing;
mod cond;
mod kind;
mod scrape;
mod search;
mod site;
mod throttle;

use std::fs::File;
use std::io::Read;

use scraper::Html;

use listing::Listing;



#[tokio::main]
async fn main() -> Result<(), String> {

  let paths = vec![
    "res/listing_1.html",
    "res/listing_2.html",
    "res/listing_3.html",
  ];

  for path in paths.iter() {
    let mut content = String::new();
    let mut file = File::open(path).or(Err(format!("Couldn't open {}", path)))?;
    file.read_to_string(&mut content).or(Err(format!("Couldn't read {}", path)))?;
    let document = Html::parse_document(&content);
    println!("{}: {:?}\n", path,  Listing::from(&document));
  }

  // scrape::scrape(Website::Bazaraki, None).await;

  Ok(())
}