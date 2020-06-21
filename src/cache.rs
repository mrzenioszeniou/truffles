use chrono::{DateTime, Utc};
use csv::{Reader, Writer, WriterBuilder};
use reqwest::Url;

use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

use crate::listing::Listing;

pub struct Cache {
  listings: Vec<Listing>,
  urls: HashMap<Url, Vec<usize>>,
  writter: Writer<File>,
}

impl Cache {
  pub fn get_last_timestamp(&self, url: &Url) -> Option<DateTime<Utc>> {
    self.urls.get(url).map(|indices| {
      let mut latest = self.listings[indices[0]].timestamp;

      for index in indices.into_iter() {
        let listing = &self.listings[*index];
        if listing.timestamp > latest {
          latest = listing.timestamp;
        }
      }

      latest
    })
  }

  pub fn add(&mut self, listing: Listing) {
    let index = self.listings.len();

    match self.urls.get_mut(&listing.url) {
      Some(ref mut vec) => {
        vec.push(index);
      }
      None => {
        self.urls.insert(listing.url.clone(), vec![index]);
      }
    }

    self
      .writter
      .serialize(listing.clone())
      .expect("Couldn't serialize listing");
    self.writter.flush().expect("Couldn't flush listing");

    self.listings.push(listing);
  }

  pub fn load() -> Self {
    let path = dirs::home_dir()
      .expect("Couldn't get home directory")
      .join(PathBuf::from(".truffles/listings.csv"));

    let mut urls: HashMap<Url, Vec<usize>> = HashMap::new();
    let mut listings: Vec<Listing> = vec![];
    if path.exists() {
      let reader = Reader::from_path(path.clone()).expect("Couldn't open cached listings");
      for listing in reader.into_deserialize() {
        let listing: Listing = match listing {
          Ok(listing) => listing,
          Err(e) => panic!("Couldn't deserialize listing record:{}", e),
        };

        let index = listings.len();

        match urls.get_mut(&listing.url) {
          Some(ref mut vec) => {
            vec.push(index);
          }
          None => {
            urls.insert(listing.url.clone(), vec![index]);
          }
        }

        listings.push(listing);
      }
    } else {
      create_dir_all(
        path
          .parent()
          .expect("INTERNAL ERROR: Can't get path's parent"),
      )
      .expect("Can't create .truffles directory");
    }

    let writter = WriterBuilder::new()
      .has_headers(!path.exists())
      .from_writer(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(path)
          .expect("Couldn't open cache file"),
      );

    Self {
      listings,
      urls,
      writter,
    }
  }
}
