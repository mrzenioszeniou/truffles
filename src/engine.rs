use reqwest::{
  Client,
  Url,
};
use scraper::{
  Html,
  Selector,
};

use crate::area::Area;
use crate::site::Website;
use crate::throttle::Throttler;

const HEADER_KEY:&str = "User-Agent";
const HEADER_VALUE:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36";

pub struct Engine {
  client: Client,
  throttler: Throttler,
}

impl Engine {

  pub fn new() -> Self {
    let client = Client::new();
    let throttler = Throttler::new(None);
    Self {
      client,
      throttler,
    }
  }

  async fn get(&mut self, url:&Url) -> Option<String> {
    self.throttler.tick();
    match self.client.get(url.clone()).header(HEADER_KEY, HEADER_VALUE).send().await {
      Ok(response) => match response.text().await {
        Ok(text) => Some(text),
        Err(e) => {
          println!("Couldn't get text from {}:{}\n", url, e);
          None
        },
      },
      Err(e) => {
        println!("Couldn't get response from {}:{}\n", url, e);
        None
      },
    }
  }

  pub async fn get_result_urls(&mut self, site: Website, area: Option<Area>) -> Vec<Url> {
    let mut result_urls = vec![];
    let search_roots = match area {
      Some(area) => vec![site.get_search_root(&area)],
      None => site.get_search_roots(),
    };
    for search_url in search_roots.into_iter() {
      let html = match self.get(&search_url).await {
        Some(content) => Html::parse_document(&content),
        None => continue,
      };
      match site {
        Website::Bazaraki => {
          let sel = Selector::parse("a.page-number.js-page-filter").expect("");
          match html.select(&sel).filter_map(|a| a.inner_html().parse::<u32>().ok()).max() {
            Some(n_pages) => for i in 1..=n_pages {
              result_urls.push(Url::parse(&format!("{}&page={}", search_url, i)).expect("Couldn't construct URL"));
            },
            None => println!("Couldn't get number of result pages from {}\n", search_url),
          }
        }
        _ => unimplemented!(),
      }
    }

    result_urls
  }

  pub async fn get_listing_urls(&mut self, result_url:Url, site: Website) -> Vec<Url> {
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
          let url_str = selection.value().attr("href").expect("No 'href' found <a> element");
          match root_url.join(url_str) {
            Ok(url) => listing_urls.push(url),
            Err(e) => println!("Couldn't parse {} as URL:{}\n",url_str,e),
          }
        }
      },
      _ => unimplemented!(),
    }

    listing_urls
  }

}
