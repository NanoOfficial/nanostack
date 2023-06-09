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

    pub const fn new_unchecked(h: f64, s: f64, l: f64, a: f64) -> Self {
        HSLuv { h, s, l, a }
    }

    pub fn to_rgb(&self) -> (f64, f64, f64) {
        hsluv_to_rgb((self.h, self.s, self.l))
    }

    pub fn set_h(mut self, h: impl Into<f64>) -> Self {
        self.h = h.into().clamp(0., 360.);
        self
    }

    pub fn set_s(mut self, s: impl Into<f64>) -> Self {
        self.s = s.into().clamp(0., 100.);
        self
    }

    pub fn set_l(mut self, l: impl Into<f64>) -> Self {
        self.l = l.into().clamp(0., 100.);
        self
    }

    pub fn set_a(mut self, a: impl Into<f64>) -> Self {
        self.a = a.into().clamp(0., 100.);
        self
    }

    pub fn h(&self) -> f64 {
        self.h
    }

    pub fn s(&self) -> f64 {
        self.s
    }

    pub fn l(&self) -> f64 {
        self.l
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn update_h(self, h: impl FnOnce(f64) -> f64) -> Self {
        self.set_h(h(self.h))
    }

    pub fn update_s(self, s: impl FnOnce(f64) -> f64) -> Self {
        self.set_s(s(self.s))
    }

    pub fn update_l(self, l: impl FnOnce(f64) -> f64) -> Self {
        self.set_l(l(self.l))
    }

    pub fn update_a(self, a: impl FnOnce(f64) -> f64) -> Self {
        self.set_a(a(self.a))
    }
}
