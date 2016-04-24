
// Parameters for PRNG
const PRNG_A : u32 = 16807;
const PRNG_M : u32 = (1 << 31) - 1;

pub struct LTPrng {
  state: u32
}

impl LTPrng {
  pub fn new(seed: u32) -> Self {
    LTPrng {
      state: seed
    }
  }

  pub fn upper_bound(&self) -> u32 {
    PRNG_M - 1
  }

  pub fn seed(&mut self, seed: u32) {
    self.state = seed;
  }

  pub fn next(&mut self) -> u32 {
    self.state = ((PRNG_A as u64 * self.state as u64) % PRNG_M as u64) as u32;
    self.state
  }

  pub fn current(&self) -> u32 {
    self.state
  }
}

impl Iterator for LTPrng {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    Some(self.next())
  }
}

#[cfg(test)]
mod test {
  use super::LTPrng;

  #[test]
  fn prng_sequence() {
    let prng = LTPrng::new(2067261);

    // Sequence from http://cs.brown.edu/courses/csci1680/f14/content/projects/lt.pdf, page 8.
    let expected = vec!(384717275, 2017463455,  888985702, 1138961335, 2001411634, 1688969677, 1074515293);
    for (pv, ev) in prng.zip(expected.iter()) {
      println!("{}:{}", pv, ev);
      assert_eq!(pv, *ev);
    }
  }
}
