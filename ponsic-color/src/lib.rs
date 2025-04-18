mod const_color;

/// RGB 颜色类型
#[repr(C, align(4))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    _ph: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

/// HSV 颜色类型
pub struct ColorHSV {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

/// HSL 颜色类型
pub struct ColorHSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Color {
    /// 创建一个颜色
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
    /// 将 RGB 颜色转换为 HSV 颜色
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

        let mut hue = hue * 60.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        ColorHSV {
            hue,
            saturation,
            value,
        }
    }

    /// 将 RGB 颜色转换为 HSL 颜色
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

        let mut hue = hue * 60.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        ColorHSL {
            hue,
            saturation,
            lightness,
        }
    }

    /// 将 HSV 颜色转换为 RGB 颜色
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

    /// 将 HSL 颜色转换为 RGB 颜色
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hsv() {
        let color = Color::new(255, 0, 0);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 0.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);

        let color = Color::new(0, 255, 0);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 120.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);

        let color = Color::new(0, 0, 255);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 240.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);

        let color = Color::new(255, 255, 0);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 60.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);

        let color = Color::new(0, 255, 255);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 180.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);

        let color = Color::new(255, 0, 255);
        let hsv = color.into_hsv();
        assert_eq!(hsv.hue, 300.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);
    }

    #[test]
    fn test_color_hsl() {
        let color = Color::new(255, 0, 0);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 0.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);

        let color = Color::new(0, 255, 0);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 120.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);

        let color = Color::new(0, 0, 255);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 240.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);

        let color = Color::new(255, 255, 0);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 60.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);
        let color = Color::new(0, 255, 255);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 180.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);

        let color = Color::new(255, 0, 255);
        let hsl = color.into_hsl();
        assert_eq!(hsl.hue, 300.0);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!(hsl.lightness, 0.5);
    }

    #[test]
    fn test_color_from_hsv() {
        let color = Color::from_hsv(ColorHSV {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(255, 0, 0));

        let color = Color::from_hsv(ColorHSV {
            hue: 120.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(0, 255, 0));

        let color = Color::from_hsv(ColorHSV {
            hue: 240.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(0, 0, 255));

        let color = Color::from_hsv(ColorHSV {
            hue: 60.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(255, 255, 0));

        let color = Color::from_hsv(ColorHSV {
            hue: 180.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(0, 255, 255));

        let color = Color::from_hsv(ColorHSV {
            hue: 300.0,
            saturation: 1.0,
            value: 1.0,
        });
        assert_eq!(color, Color::new(255, 0, 255));
    }

    #[test]
    fn test_color_from_hsl() {
        let color = Color::from_hsl(ColorHSL {
            hue: 0.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(255, 0, 0));

        let color = Color::from_hsl(ColorHSL {
            hue: 120.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(0, 255, 0));

        let color = Color::from_hsl(ColorHSL {
            hue: 240.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(0, 0, 255));

        let color = Color::from_hsl(ColorHSL {
            hue: 60.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(255, 255, 0));

        let color = Color::from_hsl(ColorHSL {
            hue: 180.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(0, 255, 255));

        let color = Color::from_hsl(ColorHSL {
            hue: 300.0,
            saturation: 1.0,
            lightness: 0.5,
        });
        assert_eq!(color, Color::new(255, 0, 255));
    }
}
