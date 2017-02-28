//! Declarative API to build inference engines.

use fuzzy::membership::FuzzyMembershipFn;
use fuzzy::norm::{FuzzyNorm, Maximum, Minimum};
use util::{range, min_max};

/// Public representation of an inference engine.
#[derive(Debug)]
pub struct Blueprint<T: FuzzyNorm, S: FuzzyNorm, I: FuzzyNorm> {
  /// Optional engine name.
  pub name: Option<String>,
  /// Conjunction t-norm.
  pub conjunction: T,
  /// Disjunction s-norm.
  pub disjunction: S,
  /// Implication t-norm.
  pub implication: I,
  /// Set of fuzzy inputs.
  pub inputs: Vec<FuzzyInput>,
  /// Set of fuzzy outputs.
  pub outputs: Vec<FuzzyOutput>,
  /// Inference rules.
  pub rule_base: Vec<FuzzyRule>,
}

/// Blueprint of common Mamdani inference engine.
pub type MaxMin = Blueprint<Minimum, Maximum, Minimum>;

impl MaxMin {
  /// Generates a default instance of a `MaxMin` blueprint.
  pub fn default() -> Self {
    Blueprint {
      name: None,
      conjunction: Minimum {},
      disjunction: Maximum {},
      implication: Minimum {},
      inputs: Vec::new(),
      outputs: Vec::new(),
      rule_base: Vec::new(),
    }
  }
}

/// Input variable of inference engine.
#[derive(Debug)]
pub struct FuzzyInput {
  /// Unique input variable name.
  pub name: String,
  /// Universe of discourse.
  pub range: DiscreteRange,
  /// Set of linguistic values.
  pub terms: Vec<Term>,
}

/// Output variable of inference engine.
#[derive(Debug)]
pub struct FuzzyOutput/*<A, D> where A: FuzzyNorm*/ {
  /// Unique output variable name.
  pub name: String,
  /// Universe of discourse.
  pub range: DiscreteRange,
  // TODO: define aggregation and defuzzifier types
  //pub aggregation: A,
  //pub defuzzifier: D,
  /// Set of linguistic values.
  pub terms: Vec<Term>,
}

/// Inference rule representation.
#[derive(Debug)]
pub struct FuzzyRule {}

/// Iterator over a finite set of `f64` values.
#[derive(Debug, Clone)]
pub struct DiscreteRange {
  discrete_points: usize,
  point: usize,
  start: f64,
  end: f64,
  step: f64,
}

impl DiscreteRange {
  /// Creates a new `DiscreteRange` instance.
  pub fn from(a: f64, b: f64, pts: usize) -> Self {
    let (a, b) = min_max(a, b);

    DiscreteRange {
      discrete_points: pts,
      start: a,
      end: b,
      point: 0,
      step: range(a, b) / (pts - 1) as f64,
    }
  }
}

/// Fuzzy linguistic term.
#[derive(Debug)]
pub struct Term {
  /// Unique name.
  pub name: String,
  /// Membership function.
  pub mf: Box<FuzzyMembershipFn>,
}

impl Iterator for DiscreteRange {
  type Item = f64;

  fn next(&mut self) -> Option<f64> {
    self.point += 1;

    if self.point > self.discrete_points {
      None
    } else {
      Some(self.start + ((self.point - 1) as f64) * self.step)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{DiscreteRange, Term};
  use fuzzy::membership::Triangular;

  #[test]
  fn test() {
    let (start, end) = (1f64, 10f64);
    let total_points: usize = 20;

    let iter = DiscreteRange::from(start, end, total_points);

    assert_eq!(iter.clone().count(), total_points);

    println!("Range: {:?}",
             iter.enumerate().collect::<Vec<(usize, f64)>>());
  }

  #[test]
  fn debug_mf() {
    let name = String::from("test");
    let term = Term {
      name: name.clone(),
      mf: Box::new(Triangular(1., 2., 3.)),
    };

    assert_eq!(name, term.name);

    println!("{:?}", term);
  }
}
