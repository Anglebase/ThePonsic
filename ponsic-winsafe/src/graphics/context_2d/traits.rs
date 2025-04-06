use super::{Brush, Color, DrawTextMode, Font, Pen, Point, Rect};
use std::ptr::null_mut;
use winapi::{
    shared::windef::{HDC, HWND},
    um::{wingdi::*, winuser::*},
};

pub trait Context2DData {
    unsafe fn handle(&self) -> HWND;
    unsafe fn hdc(&self) -> HDC;
    unsafe fn ps(&self) -> PAINTSTRUCT;
}

pub trait DrawOpen: Context2DData {
    fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc(), p1.x, p1.y, null_mut());
            LineTo(self.hdc(), p2.x, p2.y);
        }
    }

    fn polyline(&self, points: &[Point]) {
        unsafe {
            Polyline(self.hdc(), points.as_ptr() as _, points.len() as _);
        }
    }

    fn arc(&self, border: Rect, p1: Point, p2: Point) {
        unsafe {
            Arc(
                self.hdc(),
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

    fn angle_arc(&self, orgin: Point, radius: u32, start: f32, end: f32) {
        unsafe {
            AngleArc(self.hdc(), orgin.x, orgin.y, radius, start, end);
        }
    }

    fn poly_bezier(&self, points: &[Point]) {
        unsafe {
            PolyBezier(self.hdc(), points.as_ptr() as _, points.len() as _);
        }
    }
}

pub trait DrawClose: Context2DData {
    fn rectangle(&self, rect: Rect) {
        unsafe {
            Rectangle(self.hdc(), rect.left, rect.top, rect.right, rect.bottom);
        }
    }

    fn ellipse(&self, rect: Rect) {
        unsafe {
            Ellipse(self.hdc(), rect.left, rect.top, rect.right, rect.bottom);
        }
    }

    fn polygon(&self, points: &[Point]) {
        unsafe {
            Polygon(self.hdc(), points.as_ptr() as _, points.len() as _);
        }
    }

    fn pie(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            Pie(
                self.hdc(),
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

    fn chord(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            Chord(
                self.hdc(),
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

    fn round_rect(&self, rect: Rect, dx: u32, dy: u32) {
        unsafe {
            RoundRect(
                self.hdc(),
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

pub trait DrawPath: Context2DData {
    fn begin_path(&self) {
        unsafe {
            BeginPath(self.hdc());
        }
    }

    fn end_path(&self) {
        unsafe {
            EndPath(self.hdc());
        }
    }

    fn move_to(&self, p: Point) {
        unsafe {
            MoveToEx(self.hdc(), p.x, p.y, std::ptr::null_mut());
        }
    }

    fn line_to(&self, p: Point) {
        unsafe {
            LineTo(self.hdc(), p.x, p.y);
        }
    }

    fn arc_to(&self, rect: Rect, p1: Point, p2: Point) {
        unsafe {
            ArcTo(
                self.hdc(),
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

    fn poly_bezier_to(&self, points: &[Point]) {
        unsafe {
            PolyBezierTo(self.hdc(), points.as_ptr() as _, points.len() as _);
        }
    }

    fn polyline_to(&self, points: &[Point]) {
        unsafe {
            PolylineTo(self.hdc(), points.as_ptr() as _, points.len() as _);
        }
    }

    fn close_figure(&self) {
        unsafe {
            CloseFigure(self.hdc());
        }
    }
}

pub trait DrawPolygon: Context2DData {
    fn poly_polygon(&self, points: &[Point], count: &[u32]) {
        unsafe {
            PolyPolygon(
                self.hdc(),
                points.as_ptr() as _,
                count.as_ptr() as _,
                count.len() as _,
            );
        }
    }

    fn poly_polyline(&self, points: &[Point], count: &[u32]) {
        unsafe {
            PolyPolyline(
                self.hdc(),
                points.as_ptr() as _,
                count.as_ptr() as _,
                count.len() as _,
            );
        }
    }
}

pub trait DrawText: Context2DData {
    fn out_text(&self, text: &str, p: Point) {
        let text: Vec<u16> = text.encode_utf16().collect();
        unsafe {
            TextOutW(self.hdc(), p.x, p.y, text.as_ptr(), text.len() as _);
        }
    }

    fn draw_text(&self, text: &str, rect: &mut Rect, mode: &[DrawTextMode]) -> i32 {
        let mut text_utf16: Vec<u16> = text.encode_utf16().collect();
        let mut format = 0;
        mode.iter().for_each(|m| format |= m.into_sys());
        unsafe {
            DrawTextW(
                self.hdc(),
                text_utf16.as_mut_ptr() as _,
                text_utf16.len() as _,
                rect as *mut _ as _,
                format,
            )
        }
    }
}

pub trait PenSetter<'a, 'b: 'a>: Context2DData {
    fn set_pen(&'a mut self, pen: &'b Pen) {
        unsafe {
            SelectObject(self.hdc(), pen.handle() as _);
        }
    }
}

pub trait BrushSetter<'a, 'b: 'a>: Context2DData {
    fn set_brush(&'a mut self, brush: &'b Brush) {
        unsafe {
            SelectObject(self.hdc(), brush.handle() as _);
        }
    }
}

pub trait FontSetter<'a, 'b: 'a>: Context2DData {
    fn set_font(&'a mut self, font: &'b Font) {
        unsafe {
            SelectObject(self.hdc(), font.handle() as _);
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackMode {
    Transparent = TRANSPARENT,
    Opaque = OPAQUE,
}

pub trait EnvironmentSetter: Context2DData {
    fn set_back_color(&self, color: Color) {
        unsafe {
            SetBkColor(self.hdc(), color.into());
        }
    }

    fn set_text_color(&self, color: Color) {
        unsafe {
            SetTextColor(self.hdc(), color.into());
        }
    }

    fn set_back_mode(&self, mode: BackMode) {
        unsafe {
            SetBkMode(self.hdc(), mode as _);
        }
    }
}
