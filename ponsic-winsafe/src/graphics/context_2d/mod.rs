mod brush;
mod color;
mod pen;
mod text;
mod traits;
mod types;
use std::fmt::Debug;
pub use traits::*;

use super::context::Context;
pub use brush::*;
pub use color::*;
pub use pen::*;
pub use text::*;
pub use types::*;
use winapi::{
    shared::windef::{HDC, HWND},
    um::{wingdi::*, winuser::*},
};

/// 窗口 2D 绘图上下文
pub struct Context2D<'a> {
    hwnd: HWND,
    hdc: HDC,
    ps: PAINTSTRUCT,
    pen_data: GenPen<'a>,
    pen: Pen,
    brush_data: GenBrush,
    brush: Brush,
    font_data: GenFont,
    font: Font,
}

/// 高性能 2D 绘图上下文
pub struct FastContext2D {
    hwnd: HWND,
    hdc: HDC,
    ps: PAINTSTRUCT,
}

impl FastContext2D {
    pub fn new(context: Context) -> Self {
        let mut ps = unsafe { std::mem::zeroed::<PAINTSTRUCT>() };
        let hwnd = unsafe { context.hwnd() };
        let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
        Self { hwnd, hdc, ps }
    }
}

impl Drop for FastContext2D {
    fn drop(&mut self) {
        unsafe {
            EndPaint(self.hwnd, &self.ps);
        }
    }
}

impl Context2DData for Context2D<'_> {
    unsafe fn handle(&self) -> HWND {
        self.hwnd
    }

    unsafe fn hdc(&self) -> HDC {
        self.hdc
    }

    unsafe fn ps(&self) -> PAINTSTRUCT {
        self.ps
    }
}

impl Context2DData for FastContext2D {
    unsafe fn handle(&self) -> HWND {
        self.hwnd
    }

    unsafe fn hdc(&self) -> HDC {
        self.hdc
    }

    unsafe fn ps(&self) -> PAINTSTRUCT {
        self.ps
    }
}

impl Debug for Context2D<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context2D")
            .field("hwnd", &self.hwnd)
            .field("hdc", &self.hdc)
            .finish()
    }
}

impl Context2D<'_> {
    pub fn new(context: Context) -> Self {
        let mut ps = unsafe { std::mem::zeroed::<PAINTSTRUCT>() };
        let hwnd = unsafe { context.hwnd() };
        let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
        Self {
            hwnd,
            hdc,
            ps,
            pen_data: GenPen::default(),
            pen: GenPen::default().create(),
            brush_data: GenBrush::Solid(Color::from_gray(255)),
            brush: GenBrush::Solid(Color::from_gray(255)).create(),
            font_data: GenFont::default(),
            font: GenFont::default().create(),
        }
    }
}

impl Drop for Context2D<'_> {
    fn drop(&mut self) {
        unsafe {
            EndPaint(self.hwnd, &self.ps);
        }
    }
}

impl Context2D<'_> {
    pub fn clear(&self) {
        unsafe {
            FillRect(self.hdc, &self.ps.rcPaint, (COLOR_WINDOW + 1) as _);
        }
    }
}

impl<'a> Context2D<'a> {
    fn update_pen(&mut self) {
        self.pen = self.pen_data.create_by_ref().unwrap();
        unsafe {
            SelectObject(self.hdc(), self.pen.handle() as _);
        }
    }

    pub fn set_line_width(&mut self, width: usize) {
        self.pen_data.width = width;
        self.update_pen();
    }

    pub fn set_line_color(&mut self, color: Color) {
        self.pen_data.color = color;
        self.update_pen();
    }

    pub fn set_line_style(&mut self, style: LineStyle<'a>) {
        self.pen_data.line_style = style;
        self.update_pen();
    }

    pub fn set_cap_style(&mut self, style: CapStyle) {
        self.pen_data.cap_style = style;
        self.update_pen();
    }

    pub fn set_join_style(&mut self, style: JoinStyle) {
        self.pen_data.join_style = style;
        self.update_pen();
    }

    pub fn set_pen_style(&mut self, style: PenStyle) {
        self.pen_data.pen_style = style;
        self.update_pen();
    }

    pub fn replace_pen(&mut self, pen: Pen) {
        self.pen = pen;
        unsafe {
            SelectObject(self.hdc(), self.pen.handle() as _);
        }
    }
}

impl Context2D<'_> {
    fn update_brush(&mut self) {
        self.brush = self.brush_data.create_by_ref();
        unsafe {
            SelectObject(self.hdc, self.brush.handle() as _);
        }
    }

    pub fn set_brush_color(&mut self, color: Color) {
        self.brush_data = match self.brush_data {
            GenBrush::Solid(_) => GenBrush::Solid(color),
            GenBrush::Hatch(_, style) => GenBrush::Hatch(color, style),
        };
        self.update_brush();
    }

    pub fn set_brush_hatch(&mut self, style: HatchStyle) {
        self.brush_data = match self.brush_data {
            GenBrush::Solid(color) => GenBrush::Hatch(color, style),
            GenBrush::Hatch(color, _) => GenBrush::Hatch(color, style),
        };
        self.update_brush();
    }

    pub fn replace_brush(&mut self, brush: Brush) {
        self.brush = brush;
        unsafe {
            SelectObject(self.hdc, self.brush.handle() as _);
        }
    }
}

impl Context2D<'_> {
    fn update_font(&mut self) {
        self.font = self.font_data.create_by_ref();
        unsafe {
            SelectObject(self.hdc, self.font.handle() as _);
        }
    }

    pub fn set_font_height(&mut self, height: i32) {
        self.font_data.height = height;
        self.update_font();
    }

    pub fn set_font_width(&mut self, width: i32) {
        self.font_data.widht = width;
        self.update_font();
    }

    pub fn set_font_escapement(&mut self, escapement: i32) {
        self.font_data.escapement = escapement;
        self.update_font();
    }

    pub fn set_font_orientation(&mut self, orientation: i32) {
        self.font_data.orientation = orientation;
        self.update_font();
    }

    pub fn set_font_weight(&mut self, weight: FontWeight) {
        self.font_data.weight = weight;
        self.update_font();
    }

    pub fn set_font_italic(&mut self, italic: bool) {
        self.font_data.italic = italic;
        self.update_font();
    }

    pub fn set_font_underline(&mut self, underline: bool) {
        self.font_data.underline = underline;
        self.update_font();
    }

    pub fn set_font_strikeout(&mut self, strikeout: bool) {
        self.font_data.strikeout = strikeout;
        self.update_font();
    }

    pub fn set_font_charset(&mut self, charset: CharSet) {
        self.font_data.charset = charset;
        self.update_font();
    }

    pub fn set_font_outprecision(&mut self, outprecision: OutPrecision) {
        self.font_data.outprecision = outprecision;
        self.update_font();
    }

    pub fn set_font_clipprecision(&mut self, clipprecision: ClipPrecision) {
        self.font_data.clipprecision = clipprecision;
        self.update_font();
    }

    pub fn set_font_quality(&mut self, quality: Quality) {
        self.font_data.quality = quality;
        self.update_font();
    }

    pub fn set_font_pitch(&mut self, pitch: Pitch) {
        self.font_data.pitch = pitch;
        self.update_font();
    }

    pub fn set_font_family(&mut self, family: FontFamily) {
        self.font_data.family = family;
        self.update_font();
    }

    pub fn set_font_name(&mut self, name: &str) {
        self.font_data.name = String::from(name);
        self.update_font();
    }

    pub fn replace_font(&mut self, font: Font) {
        self.font = font;
        unsafe {
            SelectObject(self.hdc, self.font.handle() as _);
        }
    }
}

impl<T: Context2DData> DrawOpen for T {}
impl<T: Context2DData> DrawClose for T {}
impl<T: Context2DData> DrawPath for T {}
impl<T: Context2DData> DrawPolygon for T {}
impl<T: Context2DData> DrawText for T {}
impl<'a, 'b: 'a, T: Context2DData> PenSetter<'a, 'b> for T {}
impl<'a, 'b: 'a, T: Context2DData> BrushSetter<'a, 'b> for T {}
impl<'a, 'b: 'a, T: Context2DData> FontSetter<'a, 'b> for T {}
impl<T: Context2DData> EnvironmentSetter for T {}
impl<T: Context2DData> DrawPixel for T {}
