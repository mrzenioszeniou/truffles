extern crate csv;
extern crate futures;
extern crate indicatif;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate tokio;

mod area;
mod cache;
mod cond;
mod engine;
mod error;
mod kind;
mod listing;
mod lookup;
mod parse;
mod site;
mod throttle;

use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

use std::collections::HashSet;

use crate::area::Area;
use crate::cache::Cache;
use crate::engine::Engine;
use crate::site::Website;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "truffles",
  about = "\ntruffles is a command-line tool that scrapes listings off of real estate websites."
)]
struct Args {
  #[structopt(short = "a", long = "area", help = "Only fetch listings in this area.")]
  area: Option<Area>,
  #[structopt(
    short = "f",
    long = "force",
    help = "Fetch all listings regardless of their latest timestamp."
  )]
  force: bool,
}

#[tokio::main]
async fn main() -> Result<(), String> {
  let args: Args = Args::from_args();

  let mut engine = Engine::new();
  let mut cache = Cache::load();

  // Get result URLs
  let bar = ProgressBar::new(1);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("{spinner} Snooping result pages ... {percent:>3}% (ETA ~{eta})")
      .tick_chars("|/-\\-"),
  );
  bar.enable_steady_tick(250);
  let result_urls = engine
    .get_result_urls(Website::Bazaraki, args.area.clone())
    .await;
  bar.inc(1);
  bar.finish();

  // Get listing URLs
  let mut listing_urls = HashSet::new();
  let bar = ProgressBar::new(result_urls.len() as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("{spinner} Getting listings URLs ... {percent:>3}% (ETA ~{eta})")
      .tick_chars("|/-\\-"),
  );
  bar.enable_steady_tick(250);
  for result_url in result_urls.into_iter() {
    for url in engine.get_listing_urls(result_url, Website::Bazaraki).await {
      listing_urls.insert(url);
    }
    bar.inc(1);
  }
  bar.finish();

  // Only fetch "stale" listings
  if args.force {
    let now = Utc::now();
    listing_urls.retain(|url| {
      cache
        .get_last_timestamp(url)
        .map(|timestamp| (now - timestamp).num_days() >= 30)
        .unwrap_or(true)
    });
  }

  // Get listing pages, parse them and cache the results
  let bar = ProgressBar::new(listing_urls.len() as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("{spinner} Getting listings      ... {percent:>3}% (ETA ~{eta})")
      .tick_chars("|/-\\-"),
  );
  bar.enable_steady_tick(250);
  for listing_url in listing_urls.iter() {
    match engine.get_listing(listing_url, &Website::Bazaraki).await {
      Some(listing) => {
        cache.add(listing);
      }
      None => continue,
    }
    bar.inc(1);
  }
  bar.finish();

  Ok(())
}
