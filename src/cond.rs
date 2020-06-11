use regex::Regex;

use crate::lookup::Lookup;

#[derive(Debug, Serialize)]
pub enum Condition {
    New,
    Resale,
    UnderConstruction,
}

impl From<&str> for Condition {
    fn from(cond: &str) -> Self {
        match cond.to_lowercase().trim() {
            "new" | "brand new" => Self::New,
            "used" | "resale" => Self::Resale,
            "under construction" => Self::UnderConstruction,
            _ => panic!("Couldn't parse {} as a Condition", cond),
        }
    }
}

impl Lookup for Condition {
    fn lookup(from: &str) -> Option<Self> {
        if Regex::new(r"[Rr]esale").unwrap().find(from).is_some() {
            Some(Condition::Resale)
        } else if Regex::new(r"[Bb]rand\s+[Nn]ew")
            .unwrap()
            .find(from)
            .is_some()
        {
            Some(Condition::New)
        } else if Regex::new(r"[Uu]nder\s+[Cc]onstruction")
            .unwrap()
            .find(from)
            .is_some()
        {
            Some(Condition::UnderConstruction)
        } else {
            None
        }
    }
}
