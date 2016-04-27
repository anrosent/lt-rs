extern crate getopts;
extern crate lt;
use std::env;
use std::io::{self, Write};
use std::fs::File;
use getopts::{Options, Matches};

use lt::sampler::params::LTBlockSamplerParams;
use lt::encode;


fn parameterize(default: LTBlockSamplerParams, matches: Matches) -> LTBlockSamplerParams {

  let mut params = default;
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
  params
}

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

  // Get standard out to stream blocks
  let stdout = io::stdout();
  let mut handle = stdout.lock();

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
  let params = parameterize(LTBlockSamplerParams::new(num_blocks), matches);
  let encoder = encode::LTEncoder::new(params, &mut f);
  for block in encoder.take(100) {
    match handle.write(block.encode().as_slice()) {
      Ok(_) => (),
      Err(e) => panic!(e)
    }
  }
}
