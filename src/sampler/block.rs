use super::params::LTBlockSamplerParams;
use super::prng::LTPrng;
use super::soliton::RobustSolitonCDF;

pub struct LTBlockSampler {
  k: u32,
  prng: LTPrng,
  cdf: RobustSolitonCDF
}

#[derive(Debug)]
pub struct LTBlockSpec {
  pub seed: u32,
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

  pub fn seed(&mut self, seed: u32) {
    self.prng.seed(seed);
  }

  pub fn next(&mut self) -> LTBlockSpec {
    let seed = self.prng.current();
    let degree = self.sample_degree();
    println!("Degree is {}", degree);

    let mut srcblock_ixs : Vec<u32> = vec!();
    while srcblock_ixs.len() < degree as usize {
      let block_id = (self.prng.next() % self.k);
      println!("Gen block {}", block_id);
      if !srcblock_ixs.contains(&block_id) {
        println!("adding block {}", block_id);
        srcblock_ixs.push(block_id);
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

