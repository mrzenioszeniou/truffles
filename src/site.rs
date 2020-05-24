use std::fmt;
use reqwest::Url;
use scraper::{
  Html,
  Selector,
};
use crate::throttle::Throttler;

#[derive(Debug)]
pub enum Website {
  Bazaraki,
  Spitogatos,
  ImmobilienScout,
}

impl fmt::Display for Website {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}


impl Website {

  fn get_search_roots(&self) -> Vec<Url> {
    match self {
      Self::Bazaraki => vec![
        Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lemesos-district-limassol/?ordering=newest").unwrap(),
        Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/ammochostos-district/?ordering=newest").unwrap(),
        Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/larnaka-district-larnaca/?ordering=newest").unwrap(),
        Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lefkosia-district-nicosia/?ordering=newest").unwrap(),
        Url::parse("https://www.bazaraki.com/real-estate/houses-and-villas-sale/pafos-district-paphos/?ordering=newest").unwrap(),
      ],
      Self::ImmobilienScout | Self::Spitogatos => unimplemented!(),
    }
  }

  pub async fn get_listing_urls(&self) -> Vec<Url> {
    let mut ret = vec![];
    let mut throttler = Throttler::new(None);

    for search_url in self.get_search_roots().into_iter() {
      throttler.tick();
      println!("INFO: Expanding root result page '{}'", search_url);
      match reqwest::get(search_url.clone()).await {
        Ok(response) => {
          let content = response.text().await.expect("Couldn't get text from response");
          let html = Html::parse_document(&content);
          match self {
            Website::Bazaraki => {
              let sel = Selector::parse("a.page-number.js-page-filter").expect("");
              match html.select(&sel).filter_map(|a| a.inner_html().parse::<u32>().ok()).max() {
                Some(n_pages) => for i in 1..=n_pages {
                  ret.push(Url::parse(&format!("{}&page={}", search_url, i)).expect("Couldn't construct URL"));
                },
                None => println!("Couldn't get number of result pages from {}", search_url),
              }
            },
            _ => unimplemented!(),
          }
        },
        Err(e) => println!("Couldn't get response from {}:{}", self, e),
      }
    }

    ret
  }



}