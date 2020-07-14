use chrono::{DateTime, Utc};
use csv::{Reader, Writer, WriterBuilder};
use reqwest::Url;

use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

use crate::listing::{Kind, Listing};
use crate::plot::Plot;
use crate::property::Property;

pub struct Cache {
  listings: Vec<Listing>,
  urls: HashMap<Url, Vec<usize>>,
  writers: HashMap<Kind, Writer<File>>,
}

impl Cache {
  pub fn get_last_timestamp(&self, url: &Url) -> Option<DateTime<Utc>> {
    self.urls.get(url).map(|indices| {
      let mut latest = self.listings[indices[0]].timestamp();

      for index in indices.into_iter() {
        let listing = &self.listings[*index];
        if listing.timestamp() > latest {
          latest = listing.timestamp();
        }
      }

      latest.clone()
    })
  }

  pub fn add(&mut self, listing: Listing) {
    let index = self.listings.len();

    match self.urls.get_mut(listing.url()) {
      Some(ref mut vec) => {
        vec.push(index);
      }
      None => {
        self.urls.insert(listing.url().clone(), vec![index]);
      }
    }

    match self.writers.get_mut(&listing.kind()) {
      Some(ref mut wrt) => {
        wrt.serialize(&listing).expect("Couldn't serialize listing");
        wrt.flush().expect("Couldn't flush writer");
      }
      None => panic!("No writer found for listing kind '{:?}'", listing.kind()),
    }

    self.listings.push(listing);
  }

  pub fn load() -> Self {
    let mut urls: HashMap<Url, Vec<usize>> = HashMap::new();
    let mut listings: Vec<Listing> = vec![];

    // Load up properties if any can be found
    let props_path = dirs::home_dir()
      .expect("Couldn't get home directory")
      .join(PathBuf::from(".truffles/properties.csv"));
    if props_path.exists() {
      let reader =
        Reader::from_path(props_path.clone()).expect("Couldn't open cached property listings");
      for prop in reader.into_deserialize() {
        let property: Property = match prop {
          Ok(property) => property,
          Err(e) => panic!("Couldn't deserialize property record:{}", e),
        };

        let index = listings.len();

        match urls.get_mut(&property.url) {
          Some(ref mut vec) => {
            vec.push(index);
          }
          None => {
            urls.insert(property.url.clone(), vec![index]);
          }
        }

        listings.push(Listing::Property(property));
      }
    } else {
      create_dir_all(
        props_path
          .parent()
          .expect("INTERNAL ERROR: Can't get path's parent"),
      )
      .expect("Can't create .truffles directory");
    }

    // Load up plots if any can be found
    let plots_path = dirs::home_dir()
      .expect("Couldn't get home directory")
      .join(PathBuf::from(".truffles/plots.csv"));
    if plots_path.exists() {
      let reader =
        Reader::from_path(plots_path.clone()).expect("Couldn't open cached plot listings");
      for plot in reader.into_deserialize() {
        let plot: Plot = match plot {
          Ok(plot) => plot,
          Err(e) => panic!("Couldn't deserialize plot record:{}", e),
        };

        let index = listings.len();

        match urls.get_mut(&plot.url) {
          Some(ref mut vec) => {
            vec.push(index);
          }
          None => {
            urls.insert(plot.url.clone(), vec![index]);
          }
        }

        listings.push(Listing::Plot(plot));
      }
    } else {
      create_dir_all(
        plots_path
          .parent()
          .expect("INTERNAL ERROR: Can't get path's parent"),
      )
      .expect("Can't create .truffles directory");
    }

    // Initialize writers
    let mut writers = HashMap::new();
    for kind in Kind::all().into_iter() {
      let path = dirs::home_dir()
        .expect("Couldn't get home directory")
        .join(PathBuf::from(format!(".truffles/{:?}.csv", kind)));

      let writer = WriterBuilder::new()
        .has_headers(!path.exists() || !listings.iter().any(|l| l.kind() == kind))
        .from_writer(
          OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Couldn't open cache file"),
        );
      assert!(writers.insert(kind, writer).is_none());
    }

    Self {
      listings,
      urls,
      writers,
    }
  }
}
