//! This module defines an interface for fuzzy norms and exports common implementations.

/// This trait represents t-norms and t-conorms, or s-norms.
pub trait FuzzyNorm {
  /// Each norm is a binary function that operates over two membership degrees.
  fn calculate(a: f64, b: f64) -> f64;
}

/// A representation of the s-norm max.
#[derive(Debug)]
pub struct Maximum;

/// A representation of the t-norm min.
#[derive(Debug)]
pub struct Minimum;

impl FuzzyNorm for Maximum {
  fn calculate(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
  }
}

impl FuzzyNorm for Minimum {
  fn calculate(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
  }
}
