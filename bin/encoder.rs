extern crate getopts;
extern crate lt;
use getopts::Options;
use std::env;
use std::fs::File;

use lt::sampler::params::LTBlockSamplerParams;
use lt::encode;


fn main () {
  let args: Vec<String> = env::args().collect();
  let mut options = Options::new();
  options.reqopt("f", "input_file", "File to transmit", "FILE");
  options.reqopt("k", "num_blocks", "Number of blocks to divide the file into for transmission", "");
  options.optopt("c", "c", "C parameter to block sampler", "");
  options.optopt("d", "delta", "Delta parameter to block sampler", "");
  options.optopt("s", "seed", "Seed to the block sampler PRNG", "");

  // Validate args
  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) }
  };

  // Get input file from args
  let input_fn = matches.opt_str("f").unwrap();
  let mut f = match File::open(input_fn) {
    Ok(file) => file,
    Err(e) => panic!("{}", e)
  };

  // Parameterize LT sampler using options
  let num_blocks: u32 = match matches.opt_str("k").unwrap().parse() {
    Ok(k) => k,
    Err(e) => panic!("Error parsing k: {}", e)
  };
  let mut params = LTBlockSamplerParams::new(num_blocks);
  
  // Add c to opts
  params = match matches.opt_str("c") {
    Some(c) => params.c(c.parse::<f64>().unwrap()),
    None => params
  };
  // Add delta to opts
  params = match matches.opt_str("d") {
    Some(delta) => params.delta(delta.parse::<f64>().unwrap()),
    None => params
  };
  // Add seed to opts
  params = match matches.opt_str("s") {
    Some(s) => params.seed(s.parse::<u32>().unwrap()),
    None => params
  };
  let encoder = encode::LTEncoder::new(params, &mut f);
  for block in encoder.take(10) {
    println!("block {:?} ", block);
  }
}
