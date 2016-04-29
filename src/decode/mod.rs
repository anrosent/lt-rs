use std::io::Read;
use std::result::Result;
use super::sampler::{LTBlockSampler, LTBlockSpec};
use super::sampler::params::LTBlockSamplerParams;
use super::encode::block::LTBlock;
use self::graph::BlockGraphDecoder;

mod graph;

pub struct LTDecoder {
  filesize: u64,
  blocksize: usize,
  sampler: LTBlockSampler,
  decoder: BlockGraphDecoder
}

pub struct LTDecoderInitializer {
  params: LTBlockSamplerParams
}

impl LTDecoderInitializer {
  pub fn decoder(&self, block: &LTBlock) -> LTDecoder {
    let filesize = block.filesize;
    let blocksize = block.blocksize;
    let num_blocks = (block.filesize as f64/ block.blocksize as f64).ceil() as u32;
    let sampler = LTBlockSampler::new(self.params.k(num_blocks));
    let decoder = BlockGraphDecoder::new(num_blocks);
    LTDecoder {
      filesize: filesize,
      blocksize: blocksize,
      sampler: sampler,
      decoder: decoder
    }
  }
}

impl LTDecoder {
  pub fn initializer(params: LTBlockSamplerParams) -> LTDecoderInitializer {
    LTDecoderInitializer {
      params: params
    }
  }
  pub fn consume(&mut self, block: LTBlock) -> Result<Option<Vec<u8>>, &'static str> {
    try!(self.validate(&block));
    self.sampler.seed(block.blockseed);
    let blockspec: LTBlockSpec = self.sampler.next();
    self.decoder.consume(blockspec.srcblock_ixs, block.data);
    Ok(self.decoder.get())
  }

  fn validate(&self, block: &LTBlock) -> Result<(), &'static str> {
    if !(self.blocksize == block.blocksize && self.filesize == block.filesize) {
      Err("inconsistent block or file size")
    } else {
      Ok(())
    }
  }

  pub fn blocks<'a>(reader: &'a mut Read) -> LTBlockStream {
    LTBlockStream {
      reader: reader
    }
  }
}

pub struct LTBlockStream <'a> {
  reader: &'a mut Read
}
impl<'a> Iterator for LTBlockStream<'a> {
  type Item = LTBlock;
  fn next(&mut self) -> Option<LTBlock> {
    None
  }
}
