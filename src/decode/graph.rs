
pub struct BlockGraphDecoder {
  blocks: Vec<u8>,
  complete: bool
}

impl BlockGraphDecoder {

  // TODO:
  pub fn new(num_blocks: u32) -> Self {
    BlockGraphDecoder {
      blocks: vec![0u8; num_blocks as usize],
      complete: false
    }
  }

  // TODO:
  pub fn consume(&mut self, srcblock_ixs: &Vec<u32>, block: &Vec<u8>) -> bool {
    true 
  }

  // TODO:
  pub fn unwrap(self) -> Vec<u8> {
    if self.complete {
      self.blocks
    } else {
      panic!("Unwrapping a decoder that is not finished decoding");
    }
  }
}
