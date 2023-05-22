/**
 * @filename: lib.rs
 * @author: Krisna Pranav
 * @copyright: COPYRIGHT (2023) MIT LICENSE Krisna Pranav
*/

use std::{borrow::Cow, fmt};

#[derive(Clone)]
pub enum Lang {
    English,
    French,
    Czech,
    Norwegian,
    Spanish,
    Swedish,
}

impl Lang {
    pub fn as_str(&self) -> &str {
        match self {
            Self::English => "en",
            Self::Czech => "cz",
            Self::French => "fr",
            Self::Norwegian => "no",
            Self::Spanish => "es",
            Self::Swedish => "sv",
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}