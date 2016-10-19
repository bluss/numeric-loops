

use std::intrinsics::{fadd_fast, fmul_fast};

use super::{Sum, Dot};

macro_rules! float_impl {
    ($f:ty) => {
        impl Sum for $f {
            fn sum(lhs: &[Self]) -> Self {
                unsafe {
                    lhs.iter()
                       .fold(0., |acc, &z| fadd_fast(acc, z))
                }
            }
        }

        impl Dot for $f {
            #[inline]
            fn dot(xs: &[Self], ys: &[Self]) -> Self {
                unsafe {
                    xs.iter()
                      .zip(ys)
                      .map(|(&x, &y)| fmul_fast(x, y))
                      .fold(0., |acc, z| fadd_fast(acc, z))
                }
            }
        }
    }
}

macro_rules! int_impl {
    ($($f:ty)*) => {
        $(
        impl Sum for $f {
            fn sum(lhs: &[Self]) -> Self {
                lhs.iter()
                   .fold(0, |acc, &z| acc + z)
            }
        }

        impl Dot for $f {
            fn dot(xs: &[Self], ys: &[Self]) -> Self {
                xs.iter()
                  .zip(ys)
                  .map(|(&x, &y)| x + y)
                  .fold(0, |acc, z| acc + z)
            }
        }
        )*
    }
}

float_impl!(f32);
float_impl!(f64);

int_impl!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
