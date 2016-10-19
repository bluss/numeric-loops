#![cfg_attr(feature = "use_nightly", feature(core_intrinsics, specialization))]

extern crate num_traits;

#[path="sum/sum.rs"]
pub mod sum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
