use csv::{Reader, Writer, WriterBuilder};
use reqwest::Url;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use crate::area::Area;
use crate::listing::Listing;
use crate::site::Website;

pub struct Cache {
  listings: Vec<Listing>,
  urls: HashMap<Url, Vec<usize>>,
  websites: HashMap<Website, Vec<usize>>,
  areas: HashMap<Area, Vec<usize>>,
  writter: Writer<File>,
}

impl Cache {
  pub fn load() -> Self {
    let path = dirs::home_dir()
      .expect("Couldn't get home directory")
      .join(PathBuf::from(".truffles/listings.csv"));

    let mut urls = HashMap::new();
    let mut websites = HashMap::new();
    let mut areas = HashMap::new();
    let mut listings: Vec<Listing> = vec![];
    if path.exists() {
      let reader = Reader::from_path(path.clone()).expect("Couldn't open cached listings");
      for listing in reader.into_deserialize() {
        let listing: Listing = match listing {
          Ok(listing) => listing,
          Err(e) => panic!("Couldn't deserialize listing record:{}", e),
        };

        let index = listings.len();

        if !urls.contains_key(&listing.url) {
          urls.insert(listing.url.clone(), vec![]);
        }
        urls
          .get_mut(&listing.url)
          .expect("Can't find URL entry. This shouldn't be possible")
          .push(index);

        if !websites.contains_key(&listing.website) {
          websites.insert(listing.website.clone(), vec![]);
        }
        websites
          .get_mut(&listing.website)
          .expect("Can't find website entry. This shouldn't be possible")
          .push(index);

        if !areas.contains_key(&listing.area) {
          areas.insert(listing.area.clone(), vec![]);
        }
        areas
          .get_mut(&listing.area)
          .expect("Can't find website entry. This shouldn't be possible")
          .push(index);

        listings.push(listing);
      }
    };

    let writter = WriterBuilder::new()
      .has_headers(!path.exists())
      .from_writer(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(path)
          .unwrap(),
      );

    Self {
      listings,
      urls,
      websites,
      areas,
      writter,
    }
  }
}
