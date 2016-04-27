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
  pub fn new(params: LTBlockSamplerParams) -> Self {
    LTDecoderInitializer {
      params: params
    }
  }

  pub fn consume(self, block: LTBlock) -> LTDecoder {
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
  pub fn consume(mut self, block: LTBlock) -> Option<Vec<u8>> {
    self.sampler.seed(block.blockseed);
    let blockspec: LTBlockSpec = self.sampler.next();
    if self.decoder.consume(&blockspec.srcblock_ixs, &block.data) {
      Some(self.decoder.unwrap())
    } else {
      None
    }
  }
}

pub fn decode() {
  println!("decoded!");
}
