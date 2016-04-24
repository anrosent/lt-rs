extern crate getopts;
extern crate lt;
use getopts::Options;
use std::env;
use std::fs::File;

use lt::sampler::params::LTBlockSamplerParams;
use lt::encode;


fn main () {
  let args: Vec<String> = env::args().collect();
  let exe_name = "encoder";
  let mut options = Options::new();
  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) }
  };
  let input_fn = match matches.free.len() {
    1 => matches.free[0].clone(),
    _ => panic!("Usage: encoder <input filename>")
  };
   
  let mut f = match File::open(input_fn) {
    Ok(file) => file,
    Err(e) => panic!("{}", e)
  };
  let num_blocks = 11;
  let params = LTBlockSamplerParams::new(num_blocks);
  let mut encoder = encode::LTEncoder::new(params, &mut f);
  for block in encoder.take(10) {
    println!("block");
  }
}
