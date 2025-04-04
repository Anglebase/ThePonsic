#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn pos(&self) -> Point {
        Point {
            x: self.left,
            y: self.top,
        }
    }

    pub fn size(&self) -> Size {
        Size {
            width: (self.right - self.left) as u32,
            height: (self.bottom - self.top) as u32,
        }
    }

    pub fn x(&self) -> i32 {
        self.left
    }

    pub fn y(&self) -> i32 {
        self.top
    }

    pub fn width(&self) -> u32 {
        self.size().width
    }

    pub fn height(&self) -> u32 {
        self.size().height
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }
}

impl Rect {
    pub fn from_pos_size(pos: Point, size: Size) -> Self {
        Self {
            left: pos.x,
            top: pos.y,
            right: pos.x + size.width as i32,
            bottom: pos.y + size.height as i32,
        }
    }

    pub fn from_ps(px: i32, py: i32, sw: u32, sh: u32) -> Self {
        Self {
            left: px,
            top: py,
            right: px + sw as i32,
            bottom: py + sh as i32,
        }
    }
}

impl Point {
    pub fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Size {
    pub fn from_wh(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

impl Rect {
    pub fn to_polyline(&self) -> [Point; 5] {
        [
            Point::from_xy(self.left, self.top),
            Point::from_xy(self.right, self.top),
            Point::from_xy(self.right, self.bottom),
            Point::from_xy(self.left, self.bottom),
            Point::from_xy(self.left, self.top),
        ]
    }

    pub fn to_polygon(&self) -> [Point; 4] {
        [
            Point::from_xy(self.left, self.top),
            Point::from_xy(self.right, self.top),
            Point::from_xy(self.right, self.bottom),
            Point::from_xy(self.left, self.bottom),
        ]
    }
}
