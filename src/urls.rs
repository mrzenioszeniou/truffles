use reqwest::Url;

use std::collections::HashSet;
use std::iter::FromIterator;

use crate::area::Area;
use crate::kind::Kind;
use crate::site::Website;

const URLS: &[(&str, Website, Option<Area>, Option<Kind>)] = &[
  ("https://www.bazaraki.com/real-estate/houses-and-villas-sale/ammochostos-district/?ordering=newest", Website::Bazaraki, Some(Area::Ammochostos), None),
  ("https://www.bazaraki.com/real-estate/houses-and-villas-sale/larnaka-district-larnaca/?ordering=newest", Website::Bazaraki, Some(Area::Larnaka), None),
  ("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lefkosia-district-nicosia/?ordering=newest", Website::Bazaraki, Some(Area::Lefkosia), None),
  ("https://www.bazaraki.com/real-estate/houses-and-villas-sale/lemesos-district-limassol/?ordering=newest", Website::Bazaraki, Some(Area::Limassol), None),
  ("https://www.bazaraki.com/real-estate/houses-and-villas-sale/pafos-district-paphos/?ordering=newest", Website::Bazaraki, Some(Area::Paphos), None),
  ("https://www.bazaraki.com/real-estate/land-and-plot/ammochostos-district/?ordering=newest", Website::Bazaraki, Some(Area::Ammochostos), Some(Kind::Plot)),
  ("https://www.bazaraki.com/real-estate/land-and-plot/larnaka-district-larnaca/?ordering=newest", Website::Bazaraki, Some(Area::Larnaka), Some(Kind::Plot)),
  ("https://www.bazaraki.com/real-estate/land-and-plot/lefkosia-district-nicosia/?ordering=newest", Website::Bazaraki, Some(Area::Lefkosia), Some(Kind::Plot)),
  ("https://www.bazaraki.com/real-estate/land-and-plot/lemesos-district-limassol/?ordering=newest", Website::Bazaraki, Some(Area::Limassol), Some(Kind::Plot)),
  ("https://www.bazaraki.com/real-estate/land-and-plot/pafos-district-paphos/?ordering=newest", Website::Bazaraki, Some(Area::Paphos), Some(Kind::Plot)),
];

pub fn get_search_roots(
  website: Option<Website>,
  area: Option<Area>,
  kind: Option<Kind>,
) -> Vec<Url> {
  let mut indices: HashSet<usize> = HashSet::from_iter(0..URLS.len());

  if let Some(website) = website {
    indices.retain(|&i| URLS[i].1 == website);
  }

  if let Some(area) = area {
    indices.retain(|&i| URLS[i].2.as_ref() == Some(&area));
  }

  if let Some(kind) = kind {
    indices.retain(|&i| URLS[i].3.as_ref() == Some(&kind));
  }

  indices
    .into_iter()
    .map(|i| Url::parse(URLS[i].0).unwrap())
    .collect()
}
