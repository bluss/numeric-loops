#![feature(core_intrinsics)]
#![feature(specialization)]

extern crate num_traits;

use std::intrinsics::{fadd_fast, fmul_fast};

#[path="sum/sum.rs"]
pub mod sum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/// T must be a floating point type
unsafe fn float_dot<T: Copy>(zero: T, xs: &[T], ys: &[T]) -> T {
    xs.iter().zip(ys).map(|(&x, &y)| fmul_fast(x, y)).fold(zero, |acc, z| fadd_fast(acc, z))
}

/// T must be a floating point type
unsafe fn float_sum<T: Copy>(zero: T, xs: &[T]) -> T {
    xs.iter().fold(zero, |acc, &z| fadd_fast(acc, z))
}
