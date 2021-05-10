use crate::faker::color::raw::*;
use crate::{Dummy, Fake};
use crate::locales::Data;
use rand::Rng;
use random_color::{RandomColor};

impl<L: Data> Dummy<HexColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HexColor<L>, _rng: &mut R) -> Self {
        RandomColor::new().to_hex()
    }
}

impl<L: Data> Dummy<RgbColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RgbColor<L>, _rng: &mut R) -> Self {
        RandomColor::new().to_rgb_string()
    }
}

impl<L: Data> Dummy<RgbaColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RgbaColor<L>, rng: &mut R) -> Self {
        RandomColor::new()
        .alpha((0..10).fake_with_rng::<i8, _>(rng) as f32 / 10.)
        .to_rgba_string()
    }
}

impl<L: Data> Dummy<HslColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HslColor<L>, _rng: &mut R) -> Self {
        RandomColor::new().to_hsl_string()
    }
}

impl<L: Data> Dummy<HslaColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HslaColor<L>, rng: &mut R) -> Self {
        RandomColor::new()
        .alpha((0..10).fake_with_rng::<i8, _>(rng) as f32 / 10.)
        .to_hsla_string()
    }
}

impl<L: Data> Dummy<Color<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Color<L>, rng: &mut R) -> Self {
        let mut _color = RandomColor::new();
        let alpha = (0..10).fake_with_rng::<i8, _>(rng) as f32 / 10.;
        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            _color.to_hex(),
            _color.to_rgb_string(),
            _color.alpha(alpha).to_rgba_string(),
            _color.to_hsl_string(),
            _color.alpha(alpha).to_hsla_string()
        )
    }
}
