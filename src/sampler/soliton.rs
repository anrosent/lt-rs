pub struct RobustSolitonCDF {
  cdf: Vec<f64>    
}

impl RobustSolitonCDF {
  pub fn new(k: u32, c: f64, delta: f64) -> Self {
    let mu = gen_mu(k, c, delta);
    let mut cdf = vec!();
    for d in 0..k {
      let elt = mu.as_slice()[0 as usize..(d+1) as usize].iter().fold(0f64, |sum, v| sum + v);

      // Clone so we can get rid of the temporary slices
      cdf.push(elt.clone());
    }
    RobustSolitonCDF {
      cdf: cdf
    }
  }
  pub fn get_index(&self, sample: f64) -> usize {
    for (ix, &cdf_val) in self.cdf.iter().enumerate() {
      if cdf_val > sample {
        return ix
      }
    }
    self.cdf.len()
  }
}

fn gen_rho(k: u32) -> Vec<f64> {
  let mut res = vec!(1f64/k as f64);
  for d in 2..k+1 {
    res.push(1f64/(d * (d-1)) as f64);
  }
  res
}

fn gen_mu(k: u32, c: f64, delta: f64) -> Vec<f64> {
  let s = c * (k as f64/delta as f64).ln() * (k as f64).sqrt();
  let tau = gen_tau(k, s, delta);
  let rho = gen_rho(k);
  let normalizer = rho.iter().fold(0f64, |sum, v| sum + v) + tau.iter().fold(0f64, |sum, v| sum + v);
  let mut res = vec!();
  for d in 0..k {
    res.push((rho.as_slice()[d as usize] + tau.as_slice()[d as usize])/normalizer);
  }
  res
}

fn gen_tau(k: u32, s: f64, delta: f64) -> Vec<f64> {
  let pivot = (k as f64/s) as u32;
  let mut res = vec!();
  for d in 1..pivot {
    res.push((s/k as f64) * (1f64/(d as f64)));
  }
  res.push((s/k as f64) * (s as f64/delta).ln());
  for _ in pivot..k {
    res.push(0f64);
  }
  res
}
