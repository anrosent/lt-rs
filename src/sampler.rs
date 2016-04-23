extern crate rand; 
use super::soliton::RobustSolitonCDF;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

// Parameters for PRNG
const PRNG_A : u32 = 16807;
const PRNG_M : u32 = (1 << 31) - 1;
const PRNG_MAX_RAND : u32 = PRNG_M - 1;

pub struct LTBlockGenerator {
  k: u32,
  prng: LTPrng,
  cdf: RobustSolitonCDF
}

pub struct LTBlockSpec {
  seed: u32,
  degree: u32,
  srcblock_ids: Vec<u32>
}

impl LTBlockGenerator {
  pub fn new(params: LTBlockGeneratorParams) -> Self {
    return LTBlockGenerator {
      k: params.k,
      prng: LTPrng::new(params.seed),
      cdf: RobustSolitonCDF::new(params.k, params.c, params.delta)
    }
  }

  pub fn seed(&mut self, seed: u32) {
    self.prng.seed(seed);
  }

  pub fn next(&mut self) -> LTBlockSpec {
    let seed = self.prng.current();
    let degree = self.sample_degree();

    let mut n = 0;
    let mut srcblock_ids = vec!();
    while n < degree {
      let block_id = self.prng.next() % self.k;
      if srcblock_ids.contains(&block_id) {
        srcblock_ids.push(block_id);
        n += 1;
      }
    }
    LTBlockSpec {
      seed: seed,
      degree: degree,
      srcblock_ids: srcblock_ids
    }
  }

  fn sample_degree(&mut self) -> u32 {
    let sample = self.prng.next() as f64 / PRNG_MAX_RAND as f64;
    self.cdf.get_index(sample) as u32
  }
}

struct LTPrng {
  state: u32
}

impl LTPrng {
  fn new(seed: u32) -> Self {
    LTPrng {
      state: seed
    }
  }

  fn seed(&mut self, seed: u32) {
    self.state = seed;
  }

  fn next(&mut self) -> u32 {
    self.state = (PRNG_A * self.state) % PRNG_M;
    self.state
  }

  fn current(&self) -> u32 {
    self.state
  }
}

pub struct LTBlockGeneratorParams {
  k: u32,
  seed: u32,
  delta: f64,
  c: f64
}

impl LTBlockGeneratorParams {
  pub fn new(num_blocks: u32) -> Self {
    return LTBlockGeneratorParams {
      k: num_blocks,

      // Default parameters for Robust Soliton Distribution
      seed: rand::random::<u32>(),
      c: 0.1f64,
      delta: 0.5f64
    }
  }
  pub fn seed(&self, new_seed: u32) -> Self {
    return LTBlockGeneratorParams {
      seed: new_seed,
      .. *self
    }
  }
  pub fn c(&self, new_c: f64) -> Self {
   return LTBlockGeneratorParams {
    c: new_c,
    .. *self
   }
  }
  pub fn delta(&self, new_delta: f64) -> Self {
   return LTBlockGeneratorParams {
    delta: new_delta,
    .. *self
   }
  }
}
