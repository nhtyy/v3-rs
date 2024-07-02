/// Implements conversions and operations on the type
/// 
/// The type must impl `Add`, `Sub`, `Mul`, `Div`
macro_rules! impl_wrappers {
    (
        $(
            $(#[$attrs:meta])*
            pub struct $name:ident($vis:vis $inner:ty);
        )*
    ) => {
        $(
            $(#[$attrs])*
            pub struct $name($vis $inner);

            impl $name {
                #[inline]
                /// Get the wrapped value by consuming the wrapper
                pub fn into_inner(self) -> $inner {
                    self.0
                }
            }

            impl From<$name> for $inner {
                fn from(inner: $name) -> Self {
                    inner.0
                }
            }

            impl ::std::ops::Deref for $name {
                type Target = $inner;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl ::std::ops::Add for $name {
                type Output = $inner;

                fn add(self, rhs: Self) -> Self::Output {
                    self.0 + rhs.0
                }
            }

            impl ::std::ops::Add<$inner> for $name {
                type Output = $inner;

                fn add(self, rhs: $inner) -> Self::Output {
                    self.0 + rhs
                }
            }

            impl ::std::ops::Add<$name> for $inner {
                type Output = $inner;

                fn add(self, rhs: $name) -> Self::Output {
                    self + rhs.0
                }
            }

            impl ::std::ops::Sub for $name {
                type Output = $inner;

                fn sub(self, rhs: Self) -> Self::Output {
                    self.0 - rhs.0
                }
            }

            impl ::std::ops::Sub<$inner> for $name {
                type Output = $inner;

                fn sub(self, rhs: $inner) -> Self::Output {
                    self.0 - rhs
                }
            }

            impl ::std::ops::Sub<$name> for $inner {
                type Output = $inner;

                fn sub(self, rhs: $name) -> Self::Output {
                    self - rhs.0
                }
            }

            impl ::std::ops::Mul for $name {
                type Output = $inner;

                fn mul(self, rhs: Self) -> Self::Output {
                    self.0 * rhs.0
                }
            }

            impl ::std::ops::Mul<$inner> for $name {
                type Output = $inner;

                fn mul(self, rhs: $inner) -> Self::Output {
                   self.0 * rhs
                }
            }

            impl ::std::ops::Mul<$name> for $inner {
                type Output = $inner;

                fn mul(self, rhs: $name) -> Self::Output {
                    self * rhs.0
                }
            }

            impl ::std::ops::Div for $name {
                type Output = $inner;

                fn div(self, rhs: Self) -> Self::Output {
                    self.0 / rhs.0
                }
            }

            impl ::std::ops::Div<$inner> for $name {
                type Output = $inner;

                fn div(self, rhs: $inner) -> Self::Output {
                   self.0 / rhs
                }
            }

            impl ::std::ops::Div<$name> for $inner {
                type Output = $inner;

                fn div(self, rhs: $name) -> Self::Output {
                    self / rhs.0
                }
            }
        )*
    };
}