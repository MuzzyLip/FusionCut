use gpui::{Hsla, Rgba, rgb};

pub static BACKGROUND: u32 = 0x1C1C1E;
pub static TEXT_COLOR: u32 = 0xE1E1E3;

pub trait ColorExt {
    fn to_rgba(self) -> Rgba;
    fn to_hsla(self) -> Hsla;
    fn to_rgba_alpha(self, alpha: f32) -> Rgba;
    fn to_hsla_alpha(self, alpha: f32) -> Hsla;
}

impl ColorExt for u32 {
    #[inline]
    fn to_rgba(self) -> Rgba {
        rgb(self)
    }

    #[inline]
    fn to_hsla(self) -> Hsla {
        let Rgba { r, g, b, a } = self.to_rgba();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        // Saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        // Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Hsla { h, s, l, a }
    }

    #[inline]
    fn to_rgba_alpha(self, alpha: f32) -> Rgba {
        let mut rgba = self.to_rgba();
        rgba.a = alpha.clamp(0.0, 1.0);
        rgba
    }

    #[inline]
    fn to_hsla_alpha(self, alpha: f32) -> Hsla {
        let mut hsla = self.to_hsla();
        hsla.a = alpha.clamp(0.0, 1.0);
        hsla
    }
}
