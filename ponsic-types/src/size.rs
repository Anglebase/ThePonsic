/// 尺寸结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size<T: Copy> {
    pub width: T,
    pub height: T,
}

impl<T: Copy> Size<T> {
    /// 创建尺寸结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ponsic_types::Size;
    ///
    /// let size = Size::new(10, 20);
    /// ```
    #[inline]
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

mod operator_overload {
    use super::*;

    use std::ops::Add;
    impl<T: Copy + Add<Output = T>> Add for Size<T> {
        type Output = Size<T>;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                width: self.width + rhs.width,
                height: self.height + rhs.height,
            }
        }
    }

    use std::ops::Sub;
    impl<T: Copy + Sub<Output = T>> Sub for Size<T> {
        type Output = Size<T>;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                width: self.width - rhs.width,
                height: self.height - rhs.height,
            }
        }
    }

    use std::ops::Mul;
    impl<U: Copy, T: Copy + Mul<U, Output = T>> Mul<U> for Size<T> {
        type Output = Size<T>;

        #[inline]
        fn mul(self, rhs: U) -> Self::Output {
            Self {
                width: self.width * rhs,
                height: self.height * rhs,
            }
        }
    }

    use std::ops::Div;
    impl<U: Copy, T: Copy + Div<U, Output = T>> Div<U> for Size<T> {
        type Output = Size<T>;

        #[inline]
        fn div(self, rhs: U) -> Self::Output {
            Self {
                width: self.width / rhs,
                height: self.height / rhs,
            }
        }
    }

    use std::ops::Neg;
    impl<T: Copy + Neg<Output = T>> Neg for Size<T> {
        type Output = Size<T>;

        #[inline]
        fn neg(self) -> Self::Output {
            Self {
                width: -self.width,
                height: -self.height,
            }
        }
    }

    use std::ops::AddAssign;
    impl<T: Copy + AddAssign> AddAssign for Size<T> {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.width += rhs.width;
            self.height += rhs.height;
        }
    }

    use std::ops::SubAssign;
    impl<T: Copy + SubAssign> SubAssign for Size<T> {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.width -= rhs.width;
            self.height -= rhs.height;
        }
    }

    use std::ops::MulAssign;
    impl<U: Copy, T: Copy + MulAssign<U>> MulAssign<U> for Size<T> {
        #[inline]
        fn mul_assign(&mut self, rhs: U) {
            self.width *= rhs;
            self.height *= rhs;
        }
    }

    use std::ops::DivAssign;
    impl<U: Copy, T: Copy + DivAssign<U>> DivAssign<U> for Size<T> {
        #[inline]
        fn div_assign(&mut self, rhs: U) {
            self.width /= rhs;
            self.height /= rhs;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_size_add() {
            let a = Size::new(1, 2);
            let b = Size::new(3, 4);
            let c = a + b;
            assert_eq!(c, Size::new(4, 6));
        }

        #[test]
        fn test_size_sub() {
            let a = Size::new(1, 2);
            let b = Size::new(3, 4);
            let c = a - b;
            assert_eq!(c, Size::new(-2, -2));
        }

        #[test]
        fn test_size_mul() {
            let a = Size::new(1, 2);
            let b = a * 3;
            assert_eq!(b, Size::new(3, 6));
        }

        #[test]
        fn test_size_div() {
            let a = Size::new(1, 2);
            let b = a / 2;
            assert_eq!(b, Size::new(0, 1));
        }

        #[test]
        fn test_size_neg() {
            let a = Size::new(1, 2);
            let b = -a;
            assert_eq!(b, Size::new(-1, -2));
        }

        #[test]
        fn test_size_add_assign() {
            let mut a = Size::new(1, 2);
            let b = Size::new(3, 4);
            a += b;
            assert_eq!(a, Size::new(4, 6));
        }

        #[test]
        fn test_size_sub_assign() {
            let mut a = Size::new(1, 2);
            let b = Size::new(3, 4);
            a -= b;
            assert_eq!(a, Size::new(-2, -2));
        }

        #[test]
        fn test_size_mul_assign() {
            let mut a = Size::new(1, 2);
            a *= 3;
            assert_eq!(a, Size::new(3, 6));
        }

        #[test]
        fn test_size_div_assign() {
            let mut a = Size::new(1, 2);
            a /= 2;
            assert_eq!(a, Size::new(0, 1));
        }
    }
}

impl<T: Copy> From<(T, T)> for Size<T> {
    #[inline]
    fn from((width, height): (T, T)) -> Self {
        Self { width, height }
    }
}

impl<T: Copy> Size<T> {
    pub fn convert<U: Copy>(self) -> Size<U>
    where
        T: Into<U>,
    {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}
