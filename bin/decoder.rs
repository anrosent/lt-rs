extern crate lt;
use std::io;

use lt::sampler;
use lt::decode;

fn main () {
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  // TODO: k should not be required
  let num_blocks = 10;
  let params = sampler::params::LTBlockSamplerParams::new(num_blocks);
  let factory = decode::LTDecoder::initializer(params);
  let mut block_stream = decode::LTDecoder::blocks(&mut handle).peekable();

  let mut decoder = match block_stream.peek() {
    Some(block) => factory.decoder(block),
    None => panic!("Empty stream cannot be decoded")
  };
  for block in block_stream {
    match decoder.consume(block) {
      Ok(opt) => match opt {
        Some(file) => println!("file is : {:?}", file),
        None => ()
      },
      Err(e) => {
        println!("{}", e)
      }
    }
  }
}
