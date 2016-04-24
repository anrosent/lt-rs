extern crate rand;

pub struct LTBlockSamplerParams {
  pub k: u32,
  pub seed: u32,
  pub delta: f64,
  pub c: f64
}

impl LTBlockSamplerParams {
  pub fn new(num_blocks: u32) -> Self {
    return LTBlockSamplerParams {
      k: num_blocks,

      // Default parameters for Robust Soliton Distribution
      seed: rand::random::<u32>(),
      c: 0.1f64,
      delta: 0.5f64
    }
  }

  pub fn seed(&self, new_seed: u32) -> Self {
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
