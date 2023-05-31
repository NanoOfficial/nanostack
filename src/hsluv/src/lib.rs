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

impl fmt::Display for HSLuv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.to_rgb();
        let (r, g, b, a) = (r * 100., g * 100., b * 100., self.a);
        write!(f, "rgba({r}% {g}% {b}% / {a}%)")
    }
}

impl HSLuv {
    pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> Self {
        Self::hsla(h, s, l, 100)
    }

    pub fn hsla(
        h: impl Into<f64>,
        s: impl Into<f64>,
        l: impl Into<f64>,
        a: impl Into<f64>,
    ) -> Self {
        Self {
            h: h.into().clamp(0., 360.),
            s: s.into().clamp(0., 100.),
            l: l.into().clamp(0., 100.),
            a: a.into().clamp(0., 100.),
        }
    }
}
