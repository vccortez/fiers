//! Set of utility functions.

/// Absolute difference between two `f64`.
pub fn range(a: f64, b: f64) -> f64 {
  ((a) - (b)).abs()
}

/// Generates a `(f64,f64)` tuple with the smaller value first.
pub fn min_max(a: f64, b: f64) -> (f64, f64) {
  if a >= b { (b, a) } else { (a, b) }
}
