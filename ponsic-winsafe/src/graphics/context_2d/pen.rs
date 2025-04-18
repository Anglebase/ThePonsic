use std::ptr::null;
use winapi::shared::windef::HPEN;
use winapi::um::wingdi::*;

use crate::{SystemError, check_error};

use ponsic_color::Color;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub enum LineStyle<'a> {
    #[default]
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
    UserDef(&'a [u32]),
}

impl LineStyle<'_> {
    fn to_sys_enum(self) -> u32 {
        match self {
            LineStyle::Solid => PS_SOLID,
            LineStyle::Dash => PS_DASH,
            LineStyle::Dot => PS_DOT,
            LineStyle::DashDot => PS_DASHDOT,
            LineStyle::DashDotDot => PS_DASHDOTDOT,
            LineStyle::Null => PS_NULL,
            LineStyle::UserDef(_) => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub enum JoinStyle {
    Bevel,
    #[default]
    Miter,
    Round,
}

impl JoinStyle {
    fn to_sys_enum(self) -> u32 {
        match self {
            JoinStyle::Bevel => PS_JOIN_BEVEL,
            JoinStyle::Miter => PS_JOIN_MITER,
            JoinStyle::Round => PS_JOIN_ROUND,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub enum CapStyle {
    Round,
    Square,
    #[default]
    Flat,
}

impl CapStyle {
    fn to_sys_enum(self) -> u32 {
        match self {
            CapStyle::Round => PS_ENDCAP_ROUND,
            CapStyle::Square => PS_ENDCAP_SQUARE,
            CapStyle::Flat => PS_ENDCAP_FLAT,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub enum PenStyle {
    Null,
    #[default]
    Solid,
    Hollow,
}

impl PenStyle {
    fn to_sys_enum(self) -> u32 {
        match self {
            PenStyle::Null => BS_NULL,
            PenStyle::Solid => BS_SOLID,
            PenStyle::Hollow => BS_HOLLOW,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pen {
    pub(crate) pen: HPEN,
}

impl Drop for Pen {
    fn drop(&mut self) {
        unsafe { DeleteObject(self.pen as _) };
    }
}

impl Pen {
    pub unsafe fn handle(&self) -> HPEN {
        self.pen
    }
}

#[derive(Debug, Clone)]
pub struct GenPen<'a> {
    pub line_style: LineStyle<'a>,
    pub join_style: JoinStyle,
    pub cap_style: CapStyle,
    pub pen_style: PenStyle,
    pub width: usize,
    pub color: Color,
}

impl Default for GenPen<'static> {
    fn default() -> Self {
        GenPen {
            line_style: Default::default(),
            join_style: Default::default(),
            cap_style: Default::default(),
            pen_style: Default::default(),
            width: Default::default(),
            color: Color::BLACK,
        }
    }
}

impl<'a> GenPen<'a> {
    pub fn set_line_style(mut self, line_style: LineStyle<'a>) {
        self.line_style = line_style;
    }
}

impl GenPen<'_> {
    pub fn set_cap_style(mut self, cap_style: CapStyle) -> Self {
        self.cap_style = cap_style;
        self
    }
    pub fn set_join_style(mut self, join_style: JoinStyle) -> Self {
        self.join_style = join_style;
        self
    }
    pub fn set_pen_style(mut self, pen_style: PenStyle) -> Self {
        self.pen_style = pen_style;
        self
    }
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn create(self) -> Pen {
        self.create_by_ref().unwrap()
    }

    pub(crate) fn create_by_ref(&self) -> Result<Pen, SystemError> {
        let brush = LOGBRUSH {
            lbStyle: self.pen_style.to_sys_enum(),
            lbColor: self.color.into(),
            lbHatch: BS_SOLID as _,
        };
        let hpen = if let LineStyle::UserDef(array) = self.line_style {
            unsafe {
                ExtCreatePen(
                    PS_GEOMETRIC
                        | self.line_style.to_sys_enum()
                        | self.join_style.to_sys_enum()
                        | self.cap_style.to_sys_enum(),
                    self.width.max(1) as _,
                    &brush as _,
                    array.len() as _,
                    array.as_ptr() as _,
                )
            }
        } else {
            unsafe {
                ExtCreatePen(
                    PS_GEOMETRIC
                        | self.line_style.to_sys_enum()
                        | self.join_style.to_sys_enum()
                        | self.cap_style.to_sys_enum(),
                    self.width.max(1) as _,
                    &brush as _,
                    0,
                    null(),
                )
            }
        };

        if hpen.is_null() {
            check_error()?;
        }
        Ok(Pen { pen: hpen })
    }
}
