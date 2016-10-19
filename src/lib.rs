//! Efficient core numerical loops
//!
//! Crate Feature Flags
//! -------------------
//! 
//! The following crate feature flags are available. They are configured in
//! your `Cargo.toml`.
//! 
//! - ``use_nightly``
//! 
//!   - Optional, requires Recent nightly compiler
//!   - Enables optimizations using fast math flags and specialization
//! 
#![cfg_attr(feature = "use_nightly", feature(core_intrinsics, specialization))]

extern crate num_traits;

pub mod sum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
