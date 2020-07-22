use chrono::Utc;
use regex::{Regex, RegexBuilder};
use reqwest::Url;
use scraper::{Html, Selector};

use crate::area::Area;
use crate::cond::Condition;
use crate::error::Error;
use crate::listing::{Kind as ListingKind, Listing};
use crate::lookup::Lookup;
use crate::plot::{Kind as PlotKind, Plot};
use crate::property::{Kind as PropertyKind, Property};
use crate::site::Website;

use std::iter::Iterator;

pub fn parse_bazaraki(html: &Html, url: &Url) -> Result<Listing, Error> {
  // Common regular expressions
  let re_int = Regex::new(r"[0-9]+").expect("INTERNAL ERROR: Couldn't build regex");

  // Common selectors
  let li_sel = Selector::parse("li").expect("INTERNAL ERROR: Couldn't parse selector");
  let a_sel = Selector::parse("a").expect("INTERNAL ERROR: Couldn't parse selector");
  let span_sel = Selector::parse("span").expect("INTERNAL ERROR: Couldn't parse selector");

  // Parse UID
  let id_sel: Selector =
    Selector::parse("span[itemprop=\"sku\"").expect("INTERNAL ERROR: Couldn't parse selector");
  let id: String = format!(
    "bazaraki_{}",
    html
      .select(&id_sel)
      .next()
      .ok_or(Error::from("Couldn't select UID element"))?
      .inner_html()
  );

  // Get timestamp
  let timestamp = Utc::now();

  // Parse price
  let price_sel: Selector =
    Selector::parse("meta[itemprop=\"price\"]").expect("INTERNAL ERROR: Couldn't parse selector");
  let price_str: &str = html
    .select(&price_sel)
    .next()
    .ok_or(Error::from("Couldn't select price element"))?
    .value()
    .attr("content")
    .ok_or(Error::from("Couldn't get price's 'content' attribute"))?;
  let price: u32 = price_str.parse::<f32>().map_err(|e| Error::from(e))? as u32;

  // Parse area
  let area_sel: Selector =
    Selector::parse("span[itemprop=\"address\"]").expect("INTERNAL ERROR: Couldn't parse selector");
  let area_str = html
    .select(&area_sel)
    .next()
    .ok_or(Error::from("Couldn't select area element"))?
    .inner_html();
  let area = match Area::lookup(&area_str) {
    Some(area) => area,
    None => return Err(Error::from("Couldn't parse area for")),
  };

  // Get useful html handles
  let breadcrumbs_sel =
    Selector::parse("ul.breadcrumbs").expect("INTERNAL ERROR: Couldn't parse selector");
  let breadcrumbs_html = html
    .select(&breadcrumbs_sel)
    .next()
    .ok_or(Error::from("Couldn't select breadcrumbs"))?
    .html();
  let chars_sel = Selector::parse("div.announcement-characteristics")
    .expect("INTERNAL ERROR: Couldn't parse selector");
  let chars = html
    .select(&chars_sel)
    .next()
    .ok_or(Error::from("Couldn't select characteristics"))?;
  let chars_html = chars.inner_html();
  let desc_sel = Selector::parse("div.announcement-description")
    .expect("INTERNAL ERROR: Couldn't parse selector");
  let desc = html
    .select(&desc_sel)
    .next()
    .ok_or(Error::from("Couldn't select description element"))?;
  let desc_html = desc.inner_html();

  // Parse size
  let re_size = Regex::new(r"([0-9]+) m²").expect("INTERNAL ERROR: Couldn't parse regex");
  let size = re_size
    .captures(&chars_html)
    .map(|g| g[1].parse::<f32>().map(|a| a as u32).ok())
    .flatten();

  // Figure out what kind of listing we're parsing
  let kind = ListingKind::lookup(&breadcrumbs_html)
    .ok_or(Error::from("Couldn't figure out listing kind"))?;

  match kind {
    ListingKind::Property => {
      // Parse property kind
      let kind = PropertyKind::lookup(&chars_html).ok_or(Error::from("Couldn't parse kind"))?;

      // Parse condition
      let cond = Condition::lookup(&chars_html);

      // Parse bedrooms
      let n_bedrooms = if Regex::new(r"[Ss]tudio")
        .expect("INTERNAL ERROR: Couldn't parse regex")
        .find(&chars_html)
        .is_some()
      {
        Some(0)
      } else {
        let re_bedrooms =
          Regex::new(r"[Bb]edrooms*").expect("INTERNAL ERROR: Couldn't parse regex");
        match chars
          .select(&li_sel)
          .filter(|li| re_bedrooms.find(&li.inner_html()).is_some())
          .next()
        {
          Some(li) => Some(
            li.select(&a_sel)
              .next()
              .ok_or(Error::from("Couldn't select bedrooms"))?
              .inner_html()
              .trim()
              .parse()
              .map_err(|e| Error::from(format!("Couldn't parse bedrooms:{}", e)))?,
          ),
          None => None,
        }
      };

      // Parse bathrooms
      let re_bathrooms =
        Regex::new(r"[Bb]athrooms*").expect("INTERNAL ERROR: Couldn't parse regex");
      let n_bathrooms = chars
        .select(&li_sel)
        .filter(|li| re_bathrooms.find(&li.inner_html()).is_some())
        .next()
        .map(|li| {
          li.select(&span_sel)
            .filter(|span| re_int.find(&span.inner_html()).is_some())
            .next()
            .map(|span| {
              let str_bathrooms = span.inner_html();
              str_bathrooms.trim().parse().ok()
            })
        })
        .flatten()
        .flatten();

      // Parse post code
      let re_post =
        Regex::new(r"[Pp]ostal\s+[Cc]ode").expect("INTERNAL ERROR: Couldn't parse regex");
      let post_code = chars
        .select(&li_sel)
        .filter(|li| re_post.find(&li.inner_html()).is_some())
        .next()
        .map(|li| {
          li.select(&span_sel)
            .filter(|span| re_int.find(&span.inner_html()).is_some())
            .next()
            .map(|span| {
              let str_post_code = span.inner_html();
              str_post_code.trim().parse().ok()
            })
        })
        .flatten()
        .flatten();

      // Parse year
      let re_year = Regex::new(r"(20[0-3][0-9])|(19[0-9][0-9])").expect("Couldn't parse regex");
      let year = re_year
        .captures_iter(&desc_html)
        .filter_map(|c| c[0].parse::<u32>().ok())
        .min();

      Ok(Listing::Property(Property::new(
        id,
        url.clone(),
        Website::Bazaraki,
        timestamp,
        kind,
        price,
        area,
        size,
        cond,
        year,
        n_bedrooms,
        n_bathrooms,
        post_code,
      )))
    }
    ListingKind::Plot => {
      // Parse plot kind
      let kind = PlotKind::lookup(&chars_html);

      // Parse coverage
      let coverage = parse_coverage(&desc_html)?;

      // TODO: Parse density
      let density = None;

      // TODO: Parse height
      let height = None;

      // TODO: Parse storeys
      let storeys = None;

      Ok(Listing::Plot(Plot::new(
        id,
        url.clone(),
        Website::Bazaraki,
        timestamp,
        price,
        area,
        kind,
        size,
        coverage,
        density,
        height,
        storeys,
      )))
    }
  }
}

fn parse_coverage(from: &str) -> Result<Option<u32>, Error> {
  Ok(
    if let Some(caps) = RegexBuilder::new(
      r"([0-9]+)\s*%(\s+max(imum)?)?(\s+build(ing)?)?\s+cover(age)?(\s+((coefficient)|(factor)))?",
    )
    .case_insensitive(true)
    .build()
    .expect("Couldn't parse regex")
    .captures(from)
    {
      let cover_str = caps
        .get(1)
        .ok_or(Error::from("INTERNAL ERROR: Matched regex but not group"))?
        .as_str();
      Some(cover_str.parse().map_err(|e| Error::from(e))?)
    } else if let Some(caps) = RegexBuilder::new(
      r"(max(imum)?\s+)?(build(ing)?\s+)?cover(age)?\s*(((coefficient)|(factor))\s+)?(is\s+)?(of\s+)?(:\s*)?([0-9]+)\s*%",
    )
    .case_insensitive(true)
    .build()
    .expect("Couldn't parse regex")
    .captures(from)
    {
      let cover_str = caps
        .get(13)
        .ok_or(Error::from("INTERNAL ERROR: Matched regex but not group"))?
        .as_str();
      Some(cover_str.parse().map_err(|e| Error::from(e))?)
    } else if let Some(caps) = RegexBuilder::new(
      r"(μ[έε]γιστο(ς)?\s+)?(συντελεστ[ήη](ς)?\s+)?κ[αά]λυψη(ς)?\s*(:\s*)?([0-9]+)\s*%",
    )
    .case_insensitive(true)
    .build()
    .expect("Couldn't parse regex")
    .captures(from)
    {
      let cover_str = caps
        .get(7)
        .ok_or(Error::from("INTERNAL ERROR: Matched regex but not group"))?
        .as_str();
      Some(cover_str.parse().map_err(|e| Error::from(e))?)
    } else if let Some(caps) = RegexBuilder::new(
      r"([0-9]+)\s*%(\s+μ[έε]γιστο(ς)?)?(\s+συντελεστ[ήη](ς)?)?\s+κ[αά]λυψη(ς)?",
    )
    .case_insensitive(true)
    .build()
    .expect("Couldn't parse regex")
    .captures(from)
    {
      let cover_str = caps
        .get(1)
        .ok_or(Error::from("INTERNAL ERROR: Matched regex but not group"))?
        .as_str();
      Some(cover_str.parse().map_err(|e| Error::from(e))?)
    } else {
      None
    },
  )
}

#[cfg(test)]
mod test {
  use super::*;
  use scraper::Html;
  use std::fs::File;
  use std::io::Read;
  use std::str::FromStr;

  #[test]
  fn bazaraki_parser() {
    let paths = vec![
      "res/listing_1.html",
      "res/listing_2.html",
      "res/listing_3.html",
      "res/listing_4.html",
      "res/listing_5.html",
      "res/listing_6.html",
      "res/listing_7.html",
      "res/listing_8.html",
      "res/listing_9.html",
      "res/listing_10.html",
      "res/listing_11.html",
      "res/listing_12.html",
      "res/listing_13.html",
    ];

    for path in paths.iter() {
      let mut content = String::new();
      let mut file = File::open(path)
        .or(Err(format!("Couldn't open {}", path)))
        .unwrap();
      file
        .read_to_string(&mut content)
        .or(Err(format!("Couldn't read {}", path)))
        .unwrap();
      let document = Html::parse_document(&content);

      println!(
        "{:?}\n",
        parse_bazaraki(
          &document,
          &Url::from_str(&format!("https://foo.bar/{}", path)).unwrap()
        )
        .expect("Couldn't parse bazaraki listing")
      );
    }
  }

  #[test]
  fn coverage_parser() {
    for max in vec!["", "max", "maximum"] {
      for build in vec!["", "build", "building"] {
        for cover in vec!["cover", "coverage"] {
          for factor in vec!["", "coefficient", "factor "] {
            for space in vec!["", " ", "\t"] {
              for of in vec!["", "of", "is"] {
                for colon in vec!["", ":"] {
                  let lowercase = format!(
                    "{} {} {} {} {} {} 42{}%",
                    max, build, cover, factor, of, colon, space
                  );
                  let uppercase = lowercase.to_uppercase();
                  assert_eq!(parse_coverage(&lowercase).expect("Whoops.."), Some(42));
                  assert_eq!(parse_coverage(&uppercase).expect("Whoops.."), Some(42));
                }
              }
              let lowercase = format!("42{}% {} {} {} {}", space, max, build, cover, factor,);
              let uppercase = lowercase.to_uppercase();
              assert_eq!(parse_coverage(&lowercase).expect("Whoops.."), Some(42));
              assert_eq!(parse_coverage(&uppercase).expect("Whoops.."), Some(42));
            }
          }
        }
      }
    }

    for max in vec!["", "μεγιστο", "μέγιστο", "μεγιστος", "μέγιστος"]
    {
      for factor in vec!["", "συντελεστη", "συντελεστή", "συντελεστης", "συντελεστής"]
      {
        for cover in vec!["καλυψη", "κάλυψη", "καλυψης", "κάλυψης"] {
          for space in vec!["", " ", "\t"] {
            for colon in vec!["", ":"] {
              let lowercase = format!("{} {} {} {} 42{}%", max, factor, cover, colon, space);
              let uppercase = lowercase.to_uppercase();
              assert_eq!(parse_coverage(&lowercase).expect("Whoops.."), Some(42));
              assert_eq!(parse_coverage(&uppercase).expect("Whoops.."), Some(42));
            }
            let lowercase = format!("42{}% {} {} {}", space, max, factor, cover);
            let uppercase = lowercase.to_uppercase();
            assert_eq!(parse_coverage(&lowercase).expect("Whoops.."), Some(42));
            assert_eq!(parse_coverage(&uppercase).expect("Whoops.."), Some(42));
          }
        }
      }
    }
  }
}
