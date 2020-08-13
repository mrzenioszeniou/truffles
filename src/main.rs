extern crate csv;
extern crate futures;
extern crate indicatif;
#[macro_use]
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate simplelog;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate tokio;

mod area;
mod cache;
mod cond;
mod engine;
mod error;
mod io;
mod listing;
mod lookup;
mod parse;
mod plot;
mod property;
mod site;
mod throttle;
mod urls;

use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter;
use structopt::StructOpt;

use std::collections::HashSet;
use std::time::Duration;

use crate::area::Area;
use crate::cache::Cache;
use crate::engine::Engine;
use crate::listing::Kind;
use crate::site::Website;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "truffles",
  about = "\ntruffles is a command-line tool that scrapes listings off of real estate websites."
)]
struct Args {
  #[structopt(
    short = "a",
    long = "area",
    help = "Only fetch listings in this area [options: famagusta|larnaka|lefkosia|limassol|paphos]"
  )]
  area: Option<Area>,
  #[structopt(
    short = "f",
    long = "force",
    help = "Fetch all listings regardless of their latest timestamp"
  )]
  force: bool,
  #[structopt(
    short = "l",
    long = "level",
    help = "Logging level",
    default_value = "warn"
  )]
  level: LevelFilter,
  #[structopt(
    short = "k",
    long = "kind",
    help = "Only fetch listings of a specific kind [options: plot|property]"
  )]
  kind: Option<Kind>,

  #[structopt(
    short = "t",
    long = "throttle",
    help = "An interval (in milliseconds) to wait for between HTTP requests. Defaults to 1000ms"
  )]
  throttling: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
  // Parse arguments
  let args: Args = Args::from_args();

  // Initial engine
  let mut engine = Engine::new(
    args.level,
    args.throttling.map(|ms| Duration::from_millis(ms)),
  );

  // Load cache
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
    .get_result_urls(Website::Bazaraki, args.area, args.kind)
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
  if !args.force {
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
