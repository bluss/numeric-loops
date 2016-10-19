

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

float_impl!(f32);
float_impl!(f64);
