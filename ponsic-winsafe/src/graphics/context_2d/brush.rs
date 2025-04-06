use winapi::shared::windef::HBRUSH;
use winapi::um::wingdi::{
    CreateHatchBrush, CreateSolidBrush, HS_BDIAGONAL, HS_CROSS, HS_DIAGCROSS, HS_FDIAGONAL,
    HS_HORIZONTAL, HS_VERTICAL, RGB,
};

use super::Color;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HatchStyle {
    BDiagonal,
    Cross,
    DiagCross,
    FDiagonal,
    Horizontal,
    Vertical,
}

impl HatchStyle {
    fn to_sys_enum(self) -> u32 {
        match self {
            HatchStyle::BDiagonal => HS_BDIAGONAL,
            HatchStyle::Cross => HS_CROSS,
            HatchStyle::DiagCross => HS_DIAGCROSS,
            HatchStyle::FDiagonal => HS_FDIAGONAL,
            HatchStyle::Horizontal => HS_HORIZONTAL,
            HatchStyle::Vertical => HS_VERTICAL,
        }
    }
}

pub struct Brush {
    pub(crate) brush: HBRUSH,
}

impl Brush {
    pub unsafe fn handle(&self) -> HBRUSH {
        self.brush
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GenBrush {
    Solid(Color),
    Hatch(Color, HatchStyle),
}

impl GenBrush {
    pub fn create(self) -> Brush {
        self.create_by_ref()
    }

    pub(crate) fn create_by_ref(&self) -> Brush {
        match self {
            GenBrush::Solid(Color {
                red, green, blue, ..
            }) => Brush {
                brush: unsafe { CreateSolidBrush(RGB(*red, *green, *blue)) },
            },
            GenBrush::Hatch(
                Color {
                    red, green, blue, ..
                },
                hatch,
            ) => Brush {
                brush: unsafe {
                    CreateHatchBrush(hatch.to_sys_enum() as _, RGB(*red, *green, *blue))
                },
            },
        }
    }
}
