extern crate rand; 
use super::prng::LTPrng;
use super::soliton::RobustSolitonCDF;

pub struct LTBlockSampler {
  k: u32,
  prng: LTPrng,
  cdf: RobustSolitonCDF
}

#[derive(Debug)]
pub struct LTBlockSpec {
  pub seed: u64,
  pub degree: u32,
  pub srcblock_ixs: Vec<u32>
}

impl LTBlockSampler {
  pub fn new(params: LTBlockSamplerParams) -> Self {
    return LTBlockSampler {
      k: params.k,
      prng: LTPrng::new(params.seed),
      cdf: RobustSolitonCDF::new(params.k, params.c, params.delta)
    }
  }

  pub fn seed(&mut self, seed: u64) {
    self.prng.seed(seed);
  }

  pub fn next(&mut self) -> LTBlockSpec {
    let seed = self.prng.current();
    let degree = self.sample_degree();

    let mut n = 0;
    let mut srcblock_ixs : Vec<u32> = vec!();
    while n < degree {
      let block_id = (self.prng.next() as u32 % self.k) as u32;
      if !srcblock_ixs.contains(&block_id) {
        srcblock_ixs.push(block_id);
        n += 1;
      }
    }
    LTBlockSpec {
      seed: seed,
      degree: degree,
      srcblock_ixs: srcblock_ixs
    }
  }

  fn sample_degree(&mut self) -> u32 {
    let sample = self.prng.next() as f64 / self.prng.upper_bound() as f64;

    // Add 1 to 0-indexed CDF
    self.cdf.get_index(sample) as u32 + 1
  }
}

impl Iterator for LTBlockSampler {
  type Item = LTBlockSpec;

  fn next(&mut self) -> Option<LTBlockSpec> {
    Some(self.next())
  }
}

pub struct LTBlockSamplerParams {
  pub k: u32,
  pub seed: u64,
  pub delta: f64,
  pub c: f64
}

impl LTBlockSamplerParams {
  pub fn new(num_blocks: u32) -> Self {
    return LTBlockSamplerParams {
      k: num_blocks,

      // Default parameters for Robust Soliton Distribution
      seed: rand::random::<u64>(),
      c: 0.1f64,
      delta: 0.5f64
    }
  }

  pub fn seed(&self, new_seed: u64) -> Self {
    return LTBlockSamplerParams {
      seed: new_seed,
      .. *self
    }
  }
  pub fn c(&self, new_c: f64) -> Self {
   return LTBlockSamplerParams {
    c: new_c,
    .. *self
   }
  }
  pub fn delta(&self, new_delta: f64) -> Self {
   return LTBlockSamplerParams {
    delta: new_delta,
    .. *self
   }
  }
}


#[test]
fn blocks_sequence() {
  // Test sequence from http://cs.brown.edu/courses/csci1680/f14/content/projects/lt.pdf, page 9
  let num_blocks = 571;
  let seed = 166362120;
  let expected : Vec<Vec<u32>> = vec!(vec!(98), 
                                      vec!(400, 62),
                                      vec!(49, 385),
                                      vec!(421, 541),
                                      vec!(336, 109, 412, 410, 463, 231, 319, 564, 417, 305, 313, 461));
  let params = LTBlockSamplerParams::new(num_blocks).seed(seed);
  let sampler = LTBlockSampler::new(params);
  for (block, exp_block) in sampler.zip(expected.iter()) {
    assert_eq!(block.srcblock_ixs, *exp_block);
  }
}

