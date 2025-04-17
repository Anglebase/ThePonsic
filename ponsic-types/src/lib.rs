mod point;
mod rect;
mod size;

pub use point::Point;
pub use rect::Rect;
pub use size::Size;

pub type Pointi = Point<i32>;
pub type Pointf = Point<f32>;
pub type Sizei = Size<u32>;
pub type Sizef = Size<f32>;
pub type Recti = Rect<i32>;
pub type Rectf = Rect<f32>;
