use reqwest::Url;
use scraper::Html;

use crate::area::Area;
use crate::cond::Condition;
use crate::error::Error;
use crate::kind::Kind;
use crate::parse;
use crate::site::Website;

#[derive(Debug, Serialize)]
pub struct Listing {
    /// Unique Identifier
    id: String,
    /// URL
    url: String,
    /// Property Type
    kind: Kind,
    /// Price in EUR
    price: u32,
    /// Area
    area: Area,
    /// Size in sq. meters
    size: Option<u32>,
    /// Condition
    cond: Option<Condition>,
    /// Year of constructon
    year: Option<u32>,
    /// # of bedrooms
    n_bedrooms: Option<u8>,
    /// # of bathrooms
    n_bathrooms: Option<u8>,
    /// Postal Code
    post_code: Option<u32>,
}

impl Default for Listing {
    fn default() -> Self {
        return Self {
            id: String::from("FOOBAR"),
            url: String::from("https://foo.bar"),
            kind: Kind::Villa,
            price: 42000,
            area: Area::Limassol,
            size: None,
            cond: None,
            year: None,
            n_bedrooms: None,
            n_bathrooms: None,
            post_code: None,
        };
    }
}

impl Listing {
    pub fn try_from_html(html: &Html, url: &Url, website: &Website) -> Result<Self, Error> {
        match website {
            Website::Bazaraki => parse::parse_bazaraki(html, url.as_str()),
            _ => unimplemented!(),
        }
    }

    pub fn new(
        id: String,
        url: String,
        kind: Kind,
        price: u32,
        area: Area,
        size: Option<u32>,
        cond: Option<Condition>,
        year: Option<u32>,
        n_bedrooms: Option<u8>,
        n_bathrooms: Option<u8>,
        post_code: Option<u32>,
    ) -> Self {
        Self {
            id,
            url,
            kind,
            price,
            area,
            size,
            cond,
            year,
            n_bedrooms,
            n_bathrooms,
            post_code,
        }
    }
}
