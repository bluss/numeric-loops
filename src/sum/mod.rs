//! Summation loops

mod unroll;
#[cfg(feature = "use_nightly")]
mod specialization;

use num_traits::Zero;

use std::ops::{
    Add,
    Mul,
};

/// Compute the sum of the values in `xs`
///
/// With `"use_nightly"`, this is special cased for `f32, f64`.
pub fn sum<A>(xs: &[A]) -> A
    where A: Clone + Add<Output=A> + Zero,
{
    Sum::sum(xs)
}

/// Compute the dot product.
///
/// `xs` and `ys` should be the same length and there *may* be a panic if they
/// are not.
///
/// With `"use_nightly"`, this is special cased for `f32, f64`.
pub fn dot<A>(xs: &[A], ys: &[A]) -> A
    where A: Add<Output=A> + Mul<Output=A> + Zero + Copy,
{
    debug_assert_eq!(xs.len(), ys.len());
    Dot::dot(xs, ys)
}

trait Sum : Sized {
    fn sum(lhs: &[Self]) -> Self;
}

trait Dot : Sized {
    fn dot(lhs: &[Self], rhs: &[Self]) -> Self;
}

impl<A> Sum for A
    where A: Clone + Add<Output=A> + Zero,
{
    #[cfg(feature = "use_nightly")]
    default fn sum(lhs: &[A]) -> A {
        unroll::sum(lhs)
    }
    #[cfg(not(feature = "use_nightly"))]
    fn sum(lhs: &[A]) -> A {
        unroll::sum(lhs)
    }
}

impl<A> Dot for A
    where A: Add<Output=A> + Mul<Output=A> + Zero + Copy,
{
    #[cfg(feature = "use_nightly")]
    default fn dot(lhs: &[A], rhs: &[A]) -> A {
        unroll::dot(lhs, rhs)
    }
    #[cfg(not(feature = "use_nightly"))]
    fn dot(lhs: &[A], rhs: &[A]) -> A {
        unroll::dot(lhs, rhs)
    }
}
