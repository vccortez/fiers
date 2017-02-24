mod blueprint;

pub use blueprint::Blueprint;

#[cfg(test)]
mod tests {
  use super::Blueprint;

  #[test]
  fn test() {
    println!("{:?}", Blueprint{})
  }
}
