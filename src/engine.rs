use log::LevelFilter;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use simplelog::{ConfigBuilder, LevelPadding, WriteLogger};

use std::fs::{create_dir_all, OpenOptions};
use std::path::PathBuf;
use std::time::Duration;

use crate::area::Area;
use crate::listing::{Kind, Listing};
use crate::site::Website;
use crate::throttle::Throttler;
use crate::urls;

const HEADER_KEY: &str = "User-Agent";
const HEADER_VALUE:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36";

pub struct Engine {
  client: Client,
  throttler: Throttler,
}

impl Engine {
  pub fn new(log_level: LevelFilter, throttling: Option<Duration>) -> Self {
    let client = Client::new();
    let throttler = Throttler::new(throttling);

    let path = dirs::home_dir()
      .expect("Couldn't get home directory")
      .join(PathBuf::from(".truffles/truffles.log"));

    if !path.exists() {
      create_dir_all(
        path
          .parent()
          .expect("INTERNAL ERROR: Can't get path's parent"),
      )
      .expect("INTERNAL ERROR: Can't create directory");
    }

    let log_file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(path)
      .expect(&format!("Couldn't open log file"));

    let _ = WriteLogger::init(
      log_level,
      ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_time_format_str("%FT%T")
        .build(),
      log_file,
    );

    Self { client, throttler }
  }

  async fn get(&mut self, url: &Url) -> Option<String> {
    self.throttler.tick();
    match self
      .client
      .get(url.clone())
      .header(HEADER_KEY, HEADER_VALUE)
      .send()
      .await
    {
      Ok(response) => match response.text().await {
        Ok(text) => Some(text),
        Err(e) => {
          error!("Couldn't get text from {}:{}", url, e);
          None
        }
      },
      Err(e) => {
        error!("Couldn't get response from {}:{}", url, e);
        None
      }
    }
  }

  pub async fn get_result_urls(
    &mut self,
    site: Website,
    area: Option<Area>,
    kind: Option<Kind>,
  ) -> Vec<Url> {
    let mut result_urls = vec![];
    let search_roots = urls::get_search_roots(Some(site.clone()), area, kind);
    for search_url in search_roots.into_iter() {
      let html = match self.get(&search_url).await {
        Some(content) => Html::parse_document(&content),
        None => continue,
      };
      match site {
        Website::Bazaraki => {
          let sel =
            Selector::parse("a.page-number.js-page-filter").expect("Couldn't parse selector");
          match html
            .select(&sel)
            .filter_map(|a| a.inner_html().parse::<u32>().ok())
            .max()
          {
            Some(n_pages) => {
              for i in 1..=n_pages {
                result_urls.push(
                  Url::parse(&format!("{}&page={}", search_url, i))
                    .expect("Couldn't construct URL"),
                );
              }
            }
            None => error!("Couldn't get number of result pages from {}\n", search_url),
          }
        }
        _ => unimplemented!(),
      }
    }

    result_urls
  }

  pub async fn get_listing_urls(&mut self, result_url: Url, site: Website) -> Vec<Url> {
    let mut listing_urls = vec![];
    let html = match self.get(&result_url).await {
      Some(content) => Html::parse_document(&content),
      None => return listing_urls,
    };
    let root_url = site.get_root();
    match site {
      Website::Bazaraki => {
        let sel = Selector::parse("a.announcement-block__title").unwrap();
        for selection in html.select(&sel) {
          let url_str = selection
            .value()
            .attr("href")
            .expect("No 'href' found <a> element");
          match root_url.join(url_str) {
            Ok(url) => listing_urls.push(url),
            Err(e) => error!("Couldn't parse {} as URL:{}\n", url_str, e),
          }
        }
      }
      _ => unimplemented!(),
    }

    listing_urls
  }

  pub async fn get_listing(&mut self, url: &Url, website: &Website) -> Option<Listing> {
    self
      .get(url)
      .await
      .map(
        |c| match Listing::try_from_html(&Html::parse_document(&c), url, website) {
          Ok(listing) => Some(listing),
          Err(err) => {
            error!("Couldn't parse {} : {}", url, err);
            None
          }
        },
      )
      .flatten()
  }
}
