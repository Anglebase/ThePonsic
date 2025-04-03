use winapi::{shared::windef::COLORREF, um::wingdi::RGB};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub fn from_gray(gray: u8) -> Self {
        Self {
            red: gray,
            green: gray,
            blue: gray,
            alpha: 255,
        }
    }
}

impl Into<COLORREF> for Color {
    fn into(self) -> COLORREF {
        RGB(self.red, self.green, self.blue)
    }
}
