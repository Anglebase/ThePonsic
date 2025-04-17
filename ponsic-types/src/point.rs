/// 点结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Point<T> {
    /// 创建一个新的点
    ///
    /// # Examples
    ///
    /// ```
    /// use ponsic_types::Point;
    ///
    /// let p = Point::new(1, 2);
    /// ```
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

mod operator_overload {
    use super::*;

    use std::ops::Add;
    impl<T: Copy + Add<Output = T>> Add for Point<T> {
        type Output = Point<T>;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    use std::ops::Sub;
    impl<T: Copy + Sub<Output = T>> Sub for Point<T> {
        type Output = Point<T>;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    use std::ops::Mul;
    impl<U: Copy, T: Copy + Mul<U, Output = T>> Mul<U> for Point<T> {
        type Output = Point<T>;

        #[inline]
        fn mul(self, rhs: U) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    use std::ops::Div;
    impl<U: Copy, T: Copy + Div<U, Output = T>> Div<U> for Point<T> {
        type Output = Point<T>;

        #[inline]
        fn div(self, rhs: U) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    use std::ops::Neg;
    impl<T: Copy + Neg<Output = T>> Neg for Point<T> {
        type Output = Point<T>;

        #[inline]
        fn neg(self) -> Self::Output {
            Self {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    use std::ops::AddAssign;
    impl<T: Copy + AddAssign> AddAssign for Point<T> {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    use std::ops::SubAssign;
    impl<T: Copy + SubAssign> SubAssign for Point<T> {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    use std::ops::MulAssign;
    impl<U: Copy, T: Copy + MulAssign<U>> MulAssign<U> for Point<T> {
        #[inline]
        fn mul_assign(&mut self, rhs: U) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }

    use std::ops::DivAssign;
    impl<U: Copy, T: Copy + DivAssign<U>> DivAssign<U> for Point<T> {
        #[inline]
        fn div_assign(&mut self, rhs: U) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn point_add_test() {
            let p1 = Point::new(1, 2);
            let p2 = Point::new(3, 4);
            let p3 = p1 + p2;
            assert_eq!(p3, Point::new(4, 6));
        }

        #[test]
        fn point_sub_test() {
            let p1 = Point::new(1, 2);
            let p2 = Point::new(3, 4);
            let p3 = p1 - p2;
            assert_eq!(p3, Point::new(-2, -2));
        }

        #[test]
        fn point_mul_test() {
            let p1 = Point::new(1, 2);
            let p2 = p1 * 3;
            assert_eq!(p2, Point::new(3, 6));
        }

        #[test]
        fn point_div_test() {
            let p1 = Point::new(1, 2);
            let p2 = p1 / 2;
            assert_eq!(p2, Point::new(0, 1));
        }

        #[test]
        fn point_neg_test() {
            let p1 = Point::new(1, 2);
            let p2 = -p1;
            assert_eq!(p2, Point::new(-1, -2));
        }

        #[test]
        fn point_add_assign_test() {
            let mut p1 = Point::new(1, 2);
            let p2 = Point::new(3, 4);
            p1 += p2;
            assert_eq!(p1, Point::new(4, 6));
        }

        #[test]
        fn point_sub_assign_test() {
            let mut p1 = Point::new(1, 2);
            let p2 = Point::new(3, 4);
            p1 -= p2;
            assert_eq!(p1, Point::new(-2, -2));
        }

        #[test]
        fn point_mul_assign_test() {
            let mut p1 = Point::new(1, 2);
            p1 *= 3;
            assert_eq!(p1, Point::new(3, 6));
        }

        #[test]
        fn point_div_assign_test() {
            let mut p1 = Point::new(1, 2);
            p1 /= 2;
            assert_eq!(p1, Point::new(0, 1));
        }
    }
}

impl<T: Copy> From<(T, T)> for Point<T> {
    #[inline]
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}
