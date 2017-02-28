//! Fuzzy membership functions.

use std::fmt;

/// General representation of a membership function.
pub trait FuzzyMembershipFn: fmt::Debug {
  /// Returns the degree of membership of a value `x`.
  fn eval(&self, x: f64) -> f64;
}

/// Triangular membership function formed by three vertices.
pub struct Triangular(pub f64, pub f64, pub f64);

impl fmt::Debug for Triangular {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "TriangularMF(a: {}, b: {}, c: {})",
           self.0,
           self.1,
           self.2)
  }
}

impl FuzzyMembershipFn for Triangular {
  fn eval(&self, x: f64) -> f64 {
    (((x - self.0) / (self.1 - self.0)).min((self.2 - x) / (self.2 - self.1))).max(0f64)
  }
}
