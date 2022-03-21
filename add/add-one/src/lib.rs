use rand::prelude::*;

pub fn add_one(x: i32) -> i32 {
  x + 1
}

pub fn rand() -> f64 {
  let mut rng = thread_rng();
  // random number in range [0, 1)
  let x: f64 = rng.gen();
  x
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(3, add_one(2));
  }
}
