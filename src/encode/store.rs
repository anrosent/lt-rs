use std::iter::FromIterator;
pub trait LTBlockStore {
  fn get(&self, ix: usize) -> Option<&Vec<u8>>;
}

pub struct InMemoryBlockStore {
  blocks: Vec<Vec<u8>>
}

impl InMemoryBlockStore {
  pub fn new(blocksize: usize, bytes: Vec<u8>) -> Self {
    let mut blocks = vec!();
    for chunk in bytes.chunks(blocksize) {
      let mut block = vec!();
      block.extend_from_slice(chunk);
      if block.len() < blocksize {
        let npad = blocksize - block.len();
        block.extend_from_slice(vec![0u8; npad].as_slice());
        blocks.push(block);

        // We've hit the end of the file
        break;
      } else {
        blocks.push(Vec::from_iter(block));
      }
    }
    return InMemoryBlockStore {
      blocks: blocks
    }
  }
}

impl LTBlockStore for InMemoryBlockStore {
  fn get(&self, ix: usize) -> Option<&Vec<u8>> {
    self.blocks.get(ix)
  }
}

