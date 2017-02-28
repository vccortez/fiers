//! A <b>f</b>uzzy <b>i</b>nference <b>e</b>ngine library.
//!
//! This crate aims to offer a simple declarative API to build fuzzy inference engines.
//!
//! ## Examples
//!
//! Start by defining a `Blueprint` inference engine:
//!
//! ```rust
//! # #![allow(unused_variables)]
//! use fiers::blueprint::MaxMin;
//!
//! let blueprint = MaxMin::default();
//! ```

#![warn(missing_docs)]

pub mod fuzzy;
pub mod blueprint;
pub mod util;

pub use blueprint::Blueprint;

#[cfg(test)]
mod tests {
  use super::blueprint::{MaxMin};

  #[test]
  fn test() {
    println!("{:?}", MaxMin::default());
  }
}
