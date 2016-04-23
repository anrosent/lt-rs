
// Parameters for PRNG
const PRNG_A : u64 = 16807;
const PRNG_M : u64 = (1 << 31) - 1;
const PRNG_MAX_RAND : u64 = PRNG_M - 1;

struct LTPrng {
  state: u64
}

impl LTPrng {
  fn new(seed: u64) -> Self {
    LTPrng {
      state: seed
    }
  }

  fn seed(&mut self, seed: u64) {
    self.state = seed;
  }

  fn next(&mut self) -> u64 {
    self.state = (PRNG_A * self.state) % PRNG_M;
    self.state
  }

  fn current(&self) -> u64 {
    self.state
  }
}

impl Iterator for LTPrng {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
    Some(self.next())
  }
}

#[cfg(test)]
mod test {
  use super::LTPrng;

  #[test]
  fn prng_sequence() {
    let mut prng = LTPrng::new(2067261);

    // Sequence from http://cs.brown.edu/courses/csci1680/f14/content/projects/lt.pdf, page 8.
    let expected = vec!(384717275, 2017463455,  888985702, 1138961335, 2001411634, 1688969677, 1074515293);
    for (pv, ev) in prng.zip(expected.iter()) {
      assert_eq!(pv, *ev);
    }
  }
}
