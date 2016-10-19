
mod unroll;
#[cfg(feature = "use_nightly")]
mod specialization;

use num_traits::Zero;

use std::ops::{
    Add,
    Mul,
};

/// Compute the sum of the values in `xs`
pub fn sum<A>(xs: &[A]) -> A
    where A: Clone + Add<Output=A> + Zero,
{
    Sum::sum(xs)
}

/// Compute the dot product.
///
/// `xs` and `ys` must be the same length
pub fn dot<A>(xs: &[A], ys: &[A]) -> A
    where A: Add<Output=A> + Mul<Output=A> + Zero + Copy,
{
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
