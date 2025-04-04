mod brush;
mod color;
mod pen;
mod text;
mod types;
use std::{fmt::Debug, ptr::null_mut};

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

impl Context2D<'_> {
    pub fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc, p1.x, p1.y, null_mut());
            LineTo(self.hdc, p2.x, p2.y);
        }
    }

    pub fn polyline(&self, points: &[Point]) {
        unsafe {
            Polyline(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }

    pub fn arc(&self, border: Rect, p1: Point, p2: Point) {
        unsafe {
            Arc(
                self.hdc,
                border.left,
                border.top,
                border.right,
                border.bottom,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            );
        }
    }

    pub fn angle_arc(&self, orgin: Point, radius: u32, start: f32, end: f32) {
        unsafe {
            AngleArc(self.hdc, orgin.x, orgin.y, radius, start, end);
        }
    }

    pub fn poly_bezier(&self, points: &[Point]) {
        unsafe {
            PolyBezier(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }
}

impl<'a> Context2D<'a> {
    fn update_pen(&mut self) {
        self.pen = self.pen_data.create_by_ref().unwrap();
        unsafe {
            SelectObject(self.hdc, self.pen.handle() as _);
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

    pub fn set_pen(&mut self, pen: Pen) {
        self.pen = pen;
        unsafe {
            SelectObject(self.hdc, self.pen.handle() as _);
        }
    }

    pub unsafe fn set_pen_uncatch(&mut self, pen: &Pen) {
        unsafe {
            SelectObject(self.hdc, pen.handle() as _);
        }
    }
}

impl Context2D<'_> {
    pub fn rectangle(&self, rect: Rect) {
        unsafe {
            Rectangle(self.hdc, rect.left, rect.top, rect.right, rect.bottom);
        }
    }

    pub fn ellipse(&self, rect: Rect) {
        unsafe {
            Ellipse(self.hdc, rect.left, rect.top, rect.right, rect.bottom);
        }
    }

    pub fn polygon(&self, points: &[Point]) {
        unsafe {
            Polygon(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }

    pub fn pie(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            Pie(
                self.hdc,
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            );
        }
    }

    pub fn chord(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            Chord(
                self.hdc,
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            );
        }
    }

    pub fn round_rect(&self, rect: Rect, dx: u32, dy: u32) {
        unsafe {
            RoundRect(
                self.hdc,
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                dx as _,
                dy as _,
            );
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

    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
        unsafe {
            SelectObject(self.hdc, self.brush.handle() as _);
        }
    }

    pub unsafe fn set_brush_uncatch(&mut self, brush: &Brush) {
        unsafe {
            SelectObject(self.hdc, brush.handle() as _);
        }
    }
}

impl Context2D<'_> {
    pub fn begin_path(&self) {
        unsafe {
            BeginPath(self.hdc);
        }
    }

    pub fn end_path(&self) {
        unsafe {
            EndPath(self.hdc);
        }
    }

    pub fn move_to(&self, p: Point) {
        unsafe {
            MoveToEx(self.hdc, p.x, p.y, std::ptr::null_mut());
        }
    }

    pub fn line_to(&self, p: Point) {
        unsafe {
            LineTo(self.hdc, p.x, p.y);
        }
    }

    pub fn arc_to(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            ArcTo(
                self.hdc,
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            );
        }
    }

    pub fn poly_bezier_to(&self, points: &[Point]) {
        unsafe {
            PolyBezierTo(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }

    pub fn polyline_to(&self, points: &[Point]) {
        unsafe {
            PolylineTo(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }

    pub fn close_figure(&self) {
        unsafe {
            CloseFigure(self.hdc);
        }
    }
}

impl Context2D<'_> {
    pub fn poly_polygon(&self, points: &[Point], count: &[u32]) {
        unsafe {
            PolyPolygon(
                self.hdc,
                points.as_ptr() as _,
                count.as_ptr() as _,
                count.len() as _,
            );
        }
    }

    pub fn poly_polyline(&self, points: &[Point], count: &[u32]) {
        unsafe {
            PolyPolyline(
                self.hdc,
                points.as_ptr() as _,
                count.as_ptr() as _,
                count.len() as _,
            );
        }
    }
}

impl Context2D<'_> {
    pub fn out_text(&self, text: &str, p: Point) {
        let text: Vec<u16> = text.encode_utf16().collect();
        unsafe {
            TextOutW(self.hdc, p.x, p.y, text.as_ptr(), text.len() as _);
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

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
        unsafe {
            SelectObject(self.hdc, self.font.handle() as _);
        }
    }

    pub unsafe fn set_font_uncatch(&mut self, font: &Font) {
        unsafe {
            SelectObject(self.hdc, font.handle() as _);
        }
    }
}
