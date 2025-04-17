use super::{Point, Size};
use std::ops::{Add, AddAssign, Div, Sub};

/// 矩形结构体
///
/// # Note
/// 若启用`perf`feature，所有方法都将不会进行标准化检查，以提高性能，
/// 该情况下，如有必要，需手动进行标准化检查，以保证方法计算结果的正确性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect<T: Copy> {
    left: T,
    top: T,
    right: T,
    bottom: T,
}

impl<T: Copy + Ord> Rect<T> {
    /// 创建一个新的矩形
    ///
    /// # Example
    /// ```
    /// use ponsic_types::Rect;
    /// let rect = Rect::new(1, 2, 3, 4);
    /// ```
    #[cfg(not(feature = "perf"))]
    #[inline]
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        let mut res = Self {
            left,
            top,
            right,
            bottom,
        };
        res.normalize();
        res
    }
}

impl<T: Copy> Rect<T> {
    /// 创建一个新的矩形
    ///
    /// # Example
    /// ```
    /// use wy_core::Rect;
    /// let rect = Rect::new(1, 2, 3, 4);
    /// ```
    #[cfg(feature = "perf")]
    #[inline]
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// 返回矩形的左边界坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的左边界坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn left(&self) -> T {
        self.left
    }

    /// 返回矩形的上边界坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的上边界坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn top(&self) -> T {
        self.top
    }

    /// 返回矩形的右边界坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的右边界坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn right(&self) -> T {
        self.right
    }

    /// 返回矩形的下边界坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的下边界坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn bottom(&self) -> T {
        self.bottom
    }

    /// 返回矩形的左上角坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的左上角坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn left_top(&self) -> Point<T> {
        Point::new(self.left, self.top)
    }

    /// 返回矩形的右上角坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的右上角坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn right_top(&self) -> Point<T> {
        Point::new(self.right, self.top)
    }

    /// 返回矩形的左下角坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的左下角坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn left_bottom(&self) -> Point<T> {
        Point::new(self.left, self.bottom)
    }

    /// 返回矩形的右下角坐标
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能不会返回实际的右下角坐标，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn right_bottom(&self) -> Point<T> {
        Point::new(self.right, self.bottom)
    }
}

// 启用`perf`feature时，实现以下方法
// 这些方法不进行标准化检查，以提高性能
#[cfg(feature = "perf")]
impl<T: Copy> Rect<T> {
    /// 设置矩形的左边界坐标
    #[inline]
    pub fn set_left(&mut self, left: T) {
        self.left = left;
    }

    /// 设置矩形的上边界坐标
    #[inline]
    pub fn set_top(&mut self, top: T) {
        self.top = top;
    }

    /// 设置矩形的右边界坐标
    #[inline]
    pub fn set_right(&mut self, right: T) {
        self.right = right;
    }

    /// 设置矩形的下边界坐标
    #[inline]
    pub fn set_bottom(&mut self, bottom: T) {
        self.bottom = bottom;
    }

    /// 设置矩形的左上角坐标
    #[inline]
    pub fn set_left_top(&mut self, left: T, top: T) {
        self.left = left;
        self.top = top;
    }

    /// 设置矩形的右上角坐标
    #[inline]
    pub fn set_right_top(&mut self, right: T, top: T) {
        self.right = right;
        self.top = top;
    }

    /// 设置矩形的左下角坐标
    #[inline]
    pub fn set_left_bottom(&mut self, left: T, bottom: T) {
        self.left = left;
        self.bottom = bottom;
    }

    /// 设置矩形的右下角坐标
    #[inline]
    pub fn set_right_bottom(&mut self, right: T, bottom: T) {
        self.right = right;
        self.bottom = bottom;
    }
}

/// 若启用`perf`feature，以下方法将不会进行标准化检查，以提高性能
#[cfg(not(feature = "perf"))]
impl<T: Copy + Ord> Rect<T> {
    /// 设置矩形的左边界坐标
    #[inline]
    pub fn set_left(&mut self, left: T) {
        self.left = left;
        self.normalize();
    }

    /// 设置矩形的上边界坐标
    #[inline]
    pub fn set_top(&mut self, top: T) {
        self.top = top;
        self.normalize();
    }

    /// 设置矩形的右边界坐标
    #[inline]
    pub fn set_right(&mut self, right: T) {
        self.right = right;
        self.normalize();
    }

    /// 设置矩形的下边界坐标
    #[inline]
    pub fn set_bottom(&mut self, bottom: T) {
        self.bottom = bottom;
        self.normalize();
    }

    /// 设置矩形的左上角坐标
    #[inline]
    pub fn set_left_top(&mut self, left: T, top: T) {
        self.left = left;
        self.top = top;
        self.normalize();
    }

    /// 设置矩形的右上角坐标
    #[inline]
    pub fn set_right_top(&mut self, right: T, top: T) {
        self.right = right;
        self.top = top;
        self.normalize();
    }

    /// 设置矩形的左下角坐标
    #[inline]
    pub fn set_left_bottom(&mut self, left: T, bottom: T) {
        self.left = left;
        self.bottom = bottom;
        self.normalize();
    }

    /// 设置矩形的右下角坐标
    #[inline]
    pub fn set_right_bottom(&mut self, right: T, bottom: T) {
        self.right = right;
        self.bottom = bottom;
        self.normalize();
    }
}

impl<T: Copy + Sub<Output = T>> Rect<T> {
    /// 返回矩形的宽度
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能会返回负值，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn width(&self) -> T {
        self.right - self.left
    }

    /// 返回矩形的高度
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数可能会返回负值，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn height(&self) -> T {
        self.bottom - self.top
    }

    /// 返回矩形的尺寸
    ///
    /// # Note
    /// 若矩形不是标准化的，该函数返回值可能包含负值，
    /// 若要标准化矩形，请使用 \[`normalize()`\]
    #[inline]
    pub fn size(&self) -> Size<T> {
        Size::new(self.width(), self.height())
    }
}

impl<T: Copy + Ord> Rect<T> {
    /// 判断矩形是否标准
    ///
    /// # Note
    /// 若矩形不是标准的，可以通过 \[`normalize()`\] 进行标准化
    #[cfg(feature = "perf")]
    #[inline]
    pub fn is_normalized(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    /// 将矩形标准化
    #[cfg(feature = "perf")]
    #[inline]
    pub fn normalize(&mut self) {
        if self.is_normalized() {
            return;
        }
        if self.left > self.right {
            std::mem::swap(&mut self.left, &mut self.right);
        }
        if self.top > self.bottom {
            std::mem::swap(&mut self.top, &mut self.bottom);
        }
    }

    #[allow(dead_code)]
    #[cfg(not(feature = "perf"))]
    #[inline]
    fn is_normalized(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    #[allow(dead_code)]
    #[cfg(not(feature = "perf"))]
    #[inline]
    fn normalize(&mut self) {
        if self.is_normalized() {
            return;
        }
        if self.left > self.right {
            std::mem::swap(&mut self.left, &mut self.right);
        }
        if self.top > self.bottom {
            std::mem::swap(&mut self.top, &mut self.bottom);
        }
    }

    /// 判断点是否在矩形区域内
    ///
    /// # Note
    /// 如果点在矩形的边界上，此函数也会返回 `false`；
    /// 如果需要包含点在矩形边界上的情况，请使用 \[`contains_with_bound()`\]
    #[inline]
    pub fn contains(&self, point: Point<T>) -> bool {
        point.x > self.left && point.x < self.right && point.y > self.top && point.y < self.bottom
    }

    /// 判断点是否在矩形区域内
    ///
    /// # Note
    /// 如果点在矩形的边界上，此函数也会返回 `true`；
    /// 如果需要排除点在矩形边界上的情况，请使用 \[`contains_with_bound()`\]
    #[inline]
    pub fn contains_with_bound(&self, point: Point<T>) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }

    /// 判断此矩形与另一个矩形是否存在交集
    ///
    /// 如果另一个矩形与此矩形存在公共区域，则返回 `true`
    #[inline]
    pub fn intersects(&self, other: &Rect<T>) -> bool {
        self.left <= other.right
            && self.top <= other.bottom
            && self.right >= other.left
            && self.bottom >= other.top
    }

    /// 计算两个矩形的交集
    ///
    /// 此函数会计算两个矩形的重叠部分，并返回一个包含该信息的新矩形
    ///
    /// # Note
    /// 若两个矩形没有交集，则返回 `None`
    #[inline]
    pub fn intersected(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(Self {
                left: self.left.max(other.left),
                top: self.top.max(other.top),
                right: self.right.min(other.right),
                bottom: self.bottom.min(other.bottom),
            })
        } else {
            None
        }
    }

    /// 计算两个矩形的并集
    ///
    /// 此函数会计算两个矩形所在位置的最小轮廓矩形，并返回一个包含该信息的新矩形
    #[inline]
    pub fn united(&self, other: &Rect<T>) -> Rect<T> {
        Self {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }
}

#[cfg(feature = "perf")]
impl<T: Copy + AddAssign> Rect<T> {
    /// 调整矩形的位置
    #[inline]
    pub fn adjust(&mut self, left: T, top: T, right: T, bottom: T) {
        self.left += left;
        self.top += top;
        self.right += right;
        self.bottom += bottom;
    }
}

#[cfg(not(feature = "perf"))]
impl<T: Copy + AddAssign + Ord> Rect<T> {
    /// 调整矩形的位置
    #[inline]
    pub fn adjust(&mut self, left: T, top: T, right: T, bottom: T) {
        self.left += left;
        self.top += top;
        self.right += right;
        self.bottom += bottom;
        self.normalize();
    }
}

impl<T: Copy + Add<Output = T> + Div<i32, Output = T>> Rect<T> {
    /// 返回矩形的中心点坐标
    #[inline]
    pub fn center(&self) -> Point<T> {
        Point::new((self.right + self.left) / 2, (self.bottom + self.top) / 2)
    }
}

impl<T: Copy + Add<Output = T>> From<(Point<T>, Size<T>)> for Rect<T> {
    #[inline]
    fn from((pos, size): (Point<T>, Size<T>)) -> Self {
        Self {
            left: pos.x,
            top: pos.y,
            right: pos.x + size.width,
            bottom: pos.y + size.height,
        }
    }
}

mod operator_overload {
    use super::*;

    use std::ops::BitAnd;
    impl<T: Copy + Ord> BitAnd for Rect<T> {
        type Output = Option<Self>;

        fn bitand(self, rhs: Self) -> Self::Output {
            self.intersected(&rhs)
        }
    }

    use std::ops::BitOr;
    impl<T: Copy + Ord> BitOr for Rect<T> {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            self.united(&rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "perf")]
    fn is_normalized_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert!(rect.is_normalized());
        let rect = Rect::new(3, 2, 1, 4);
        assert!(!rect.is_normalized());
    }

    #[test]
    fn normalize_test() {
        let mut rect = Rect::new(3, 2, 1, 4);
        rect.normalize();
        assert_eq!(rect, Rect::new(1, 2, 3, 4));
    }

    #[test]
    fn contains_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert!(rect.contains(Point::new(2, 3)));
        assert!(!rect.contains(Point::new(0, 0)));
    }

    #[test]
    fn contains_with_bound_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert!(rect.contains_with_bound(Point::new(2, 3)));
        assert!(rect.contains_with_bound(Point::new(1, 2)));
        assert!(!rect.contains_with_bound(Point::new(0, 0)));
    }

    #[test]
    fn intersects_test() {
        let rect1 = Rect::new(1, 2, 3, 4);
        let rect2 = Rect::new(2, 3, 4, 5);
        assert!(rect1.intersects(&rect2));
        let rect3 = Rect::new(4, 5, 6, 7);
        assert!(!rect1.intersects(&rect3));
    }

    #[test]
    fn intersected_test() {
        let rect1 = Rect::new(1, 2, 3, 4);
        let rect2 = Rect::new(2, 3, 4, 5);
        assert_eq!(rect1.intersected(&rect2), Some(Rect::new(2, 3, 3, 4)));
        let rect3 = Rect::new(4, 5, 6, 7);
        assert_eq!(rect1.intersected(&rect3), None);
    }

    #[test]
    fn united_test() {
        let rect1 = Rect::new(1, 2, 3, 4);
        let rect2 = Rect::new(2, 3, 4, 5);
        assert_eq!(rect1.united(&rect2), Rect::new(1, 2, 4, 5));
    }

    #[test]
    fn adjust_test() {
        let mut rect = Rect::new(1, 2, 3, 4);
        rect.adjust(1, 2, 3, 4);
        assert_eq!(rect, Rect::new(2, 4, 6, 8));
    }

    #[test]
    fn center_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert_eq!(rect.center(), Point::new(2, 3));
    }
}
