use rust_hsluv::hsluv_to_rgb;
use std::fmt;

#[cfg(feature = "hsluv_macro")]
pub use hsluv_macro::hsluv;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLuv {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

// impl fmt::Display for HSLuv {}

impl HSLuv {}
