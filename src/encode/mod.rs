use std::io::prelude::*;
use std::fs::File;

use super::common::vec_xor;
use super::sampler::{LTBlockSpec, LTBlockSampler};
use super::sampler::params::LTBlockSamplerParams;

use self::store::{LTBlockStore, InMemoryBlockStore};
use self::block::LTBlock;

mod store;
pub mod block;

pub struct LTEncoder {
  sampler: LTBlockSampler,
  filesize: u64,
  blocksize: usize,
  blocks: Box<LTBlockStore>
}

impl LTEncoder {
  pub fn new(params: LTBlockSamplerParams, file: &mut File) -> Self {
    let meta_res = file.metadata();
    let file_bytes = file.bytes().map(|byte| byte.unwrap()).collect();
    match meta_res {
      Err(e) => panic!("{}", e),
      Ok(meta) => {
        let blocksize = (meta.len()/params.k as u64) as usize;
        LTEncoder {
          filesize: meta.len(),
          blocksize: blocksize,
          sampler: LTBlockSampler::new(params),
          blocks: Box::new(InMemoryBlockStore::new(blocksize, file_bytes))
        }
      }
    }
  }
}

impl Iterator for LTEncoder {
  type Item = LTBlock;

  fn next(&mut self) -> Option<LTBlock> {
    let block = self.sampler.next();
    let mut data : Vec<u8> = vec![0u8; self.blocksize];
    for block_ix in block.srcblock_ixs.iter() {
      match self.blocks.get(*block_ix as usize) {
        Some(block) => vec_xor(data.as_mut_slice(), block.as_slice()),
        None => panic!("Invalid block index generated by PRNG")
      }
    }
    return Some(LTBlock {
      filesize: self.filesize,
      blocksize: self.blocksize,
      blockseed: block.seed,
      data: data
    })
  }
}
