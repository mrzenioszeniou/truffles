use crate::site::Website;
use std::time::Duration;
use reqwest::Url;

pub async fn scrape(site: Website, interval: Option<Duration>) -> Vec<String> {

  // Produced page contents
  let mut _ret = Vec::new();

  // Collect a list of result page URLs. Each one contains URLs to actual
  // listing pages which we need to parse.
  let res_pages:Vec<Url> = site.get_listing_urls().await;

  for each in res_pages.iter() {
    println!("{}", each);
  }

  _ret
}