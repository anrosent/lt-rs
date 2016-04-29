use std::collections::HashMap;
use super::super::common;

struct CheckNode {
  srcblock_ixs: Vec<u32>,
  data: Vec<u8>
}

impl CheckNode {
  pub fn resolve(&mut self, ix: u32, data: &Vec<u8>) {
    let selfix = self.srcblock_ixs.iter().position(|i| *i == ix);
    assert!(selfix.is_some(), "resolved check with unrelated node");
    self.srcblock_ixs.remove(selfix.unwrap());
    common::vec_xor(self.data.as_mut_slice(), data.as_slice());
  }

  pub fn is_eliminable(&self) -> bool {
    self.srcblock_ixs.len() == 1
  }
}

pub struct BlockGraphDecoder {
  checks: HashMap<u32, Vec<CheckNode>>,
  eliminated: HashMap<u32, Vec<u8>>,
  num_blocks: u32,
  complete: bool
}

impl BlockGraphDecoder {

  pub fn new(num_blocks: u32) -> Self {
    BlockGraphDecoder {
      checks: HashMap::new(),
      eliminated: HashMap::new(),
      num_blocks: num_blocks,
      complete: false
    }
  }

  pub fn consume(&mut self, srcblock_ixs: Vec<u32>, mut data:  Vec<u8>) {
    let (eliminated, active) : (Vec<u32>, Vec<u32>) = srcblock_ixs.iter().partition(|ix| self.eliminated.contains_key(ix));
    for eix in eliminated.iter() {
      common::vec_xor(data.as_mut_slice(), self.eliminated.get(eix).unwrap());
    };
    let mut check = CheckNode {
      srcblock_ixs: active,
      data: data
    };

    match check.srcblock_ixs.len() {
      0 => (),
      1 => self.eliminate_node(check),
      _ => self.add_edges(check)
    }
  }

  fn eliminate_node(&mut self, check: CheckNode) {

    // We know this has exactly one element
    let ix = check.srcblock_ixs.first().unwrap();
    match self.checks.get(ix) {
      Some(checks) => (),
      None => {
        self.eliminated.insert(*ix, check.data);
      }
    };
  }

  fn add_edges(&mut self, check: CheckNode) {

  }

  // TODO:
  pub fn get(&self) -> Option<Vec<u8>> {
    if self.complete {
      Some(vec!())
    } else {
      None
    }
  }
}
