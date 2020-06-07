extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate tokio;
extern crate indicatif;

mod area;
mod listing;
mod cond;
mod engine;
mod kind;
mod lookup;
mod site;
mod throttle;

use indicatif::{
  ProgressBar,
  ProgressStyle,
};

use crate::area::Area;
use crate::engine::Engine;
use crate::site::Website;

#[tokio::main]
async fn main() -> Result<(), String> {
  
  let mut engine = Engine::new();

  // Get result URLs
  let bar = ProgressBar::new(1);
  bar.set_style(ProgressStyle::default_bar()
    .template("{spinner} Snooping result pages ... {percent:>3}%")
    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈⠐"));
  bar.enable_steady_tick(250);
  let result_urls = engine.get_result_urls(Website::Bazaraki, Some(Area::Ammochostos)).await;
  bar.inc(1);
  bar.finish();

  // Get listing URLs
  let mut listing_urls = vec![];
  let bar = ProgressBar::new(result_urls.len() as u64);
  bar.set_style(ProgressStyle::default_bar()
    .template("{spinner} Getting listings URLs ... {percent:>3}%")
    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈⠐"));
  bar.enable_steady_tick(250);
  for result_url in result_urls.into_iter() {
    listing_urls.append(&mut engine.get_listing_urls(result_url, Website::Bazaraki).await);
    bar.inc(1);
  }
  bar.finish();

  // TODO: Get listings

  Ok(())
}