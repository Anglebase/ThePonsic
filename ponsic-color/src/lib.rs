mod const_color;

#[repr(C, align(4))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    _ph: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

pub struct ColorHSV {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

pub struct ColorHSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            _ph: 0,
            blue,
            green,
            red,
        }
    }
}

impl Color {
    pub const fn into_hsv(self) -> ColorHSV {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else if max == r {
            (g - b) / delta
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let value = max;

        ColorHSV {
            hue: hue * 60.0,
            saturation,
            value,
        }
    }

    pub const fn into_hsl(self) -> ColorHSL {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let lightness = (max + min) / 2.0;
        let saturation = if lightness == 0.0 || lightness == 1.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        let hue = if delta == 0.0 {
            0.0
        } else if max == r {
            (g - b) / delta
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        ColorHSL {
            hue: hue * 60.0,
            saturation,
            lightness,
        }
    }

    pub const fn from_hsv(hsv: ColorHSV) -> Color {
        let chroma = hsv.value * hsv.saturation;
        let hue_prime = hsv.hue / 60.0;
        let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());

        let (r, g, b) = if hue_prime < 1.0 {
            (chroma, x, 0.0)
        } else if hue_prime < 2.0 {
            (x, chroma, 0.0)
        } else if hue_prime < 3.0 {
            (0.0, chroma, x)
        } else if hue_prime < 4.0 {
            (0.0, x, chroma)
        } else if hue_prime < 5.0 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

        let m = hsv.value - chroma;

        Color::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    pub const fn from_hsl(hsl: ColorHSL) -> Color {
        let chroma = (1.0 - (2.0 * hsl.lightness - 1.0).abs()) * hsl.saturation;
        let hue_prime = hsl.hue / 60.0;
        let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());

        let (r, g, b) = if hue_prime < 1.0 {
            (chroma, x, 0.0)
        } else if hue_prime < 2.0 {
            (x, chroma, 0.0)
        } else if hue_prime < 3.0 {
            (0.0, chroma, x)
        } else if hue_prime < 4.0 {
            (0.0, x, chroma)
        } else if hue_prime < 5.0 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

        let m = hsl.lightness - chroma / 2.0;

        Color::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        unsafe { std::mem::transmute(color & 0x00ffffff) }
    }
}

impl Into<ColorHSV> for Color {
    fn into(self) -> ColorHSV {
        self.into_hsv()
    }
}

impl From<ColorHSV> for Color {
    fn from(hsv: ColorHSV) -> Self {
        Self::from_hsv(hsv)
    }
}

impl Into<ColorHSL> for Color {
    fn into(self) -> ColorHSL {
        self.into_hsl()
    }
}

impl From<ColorHSL> for Color {
    fn from(hsl: ColorHSL) -> Self {
        Self::from_hsl(hsl)
    }
}