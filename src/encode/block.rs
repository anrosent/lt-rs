use std::mem::size_of;

#[derive(Debug, Clone, PartialEq)]
pub struct LTBlock {
  pub filesize: u64,
  pub blocksize: usize,
  pub blockseed: u32,
  pub data: Vec<u8>
}

// TODO: needs testing
fn num_to_bytes(n: u64, nbytes: usize) -> Vec<u8> {
  let mut buf = vec!();
  for shifts in (0..nbytes).rev() {
    buf.push((n >> (8 * shifts)) as u8);
  }
  buf
}

// TODO: needs testing
fn bytes_to_num(bytes: Vec<&u8>) -> u64 {
  let mut res = 0u64;
  for (ix, &byte) in bytes.iter().rev().enumerate() {
    res |= (*byte as u64) << (8 * ix);
  }
  res
}

impl LTBlock {
  pub fn encode(&self) -> Vec<u8> {

    // Serialize the struct 
    let mut buf = vec!();
    buf.extend_from_slice(num_to_bytes(self.filesize, size_of::<u64>()).as_slice());
    buf.extend_from_slice(num_to_bytes(self.blocksize as u64, size_of::<usize>()).as_slice());
    buf.extend_from_slice(num_to_bytes(self.blockseed as u64, size_of::<u32>()).as_slice());
    buf.extend_from_slice(self.data.as_slice());

    // Set bytes to NETWORK BYTE ORDER
    for byte in buf.iter_mut() {
      *byte = byte.to_be();
    }
    buf
  }

  // TODO: should take Reader, not Vec
  pub fn decode(mut bytes: Vec<u8>) -> Result<Self, &'static str> {

    // Set from NETWORK to NATIVE byte order
    for byte in bytes.iter_mut() {
      *byte = u8::from_be(*byte);
    }

    let fs_bytes : Vec<&u8> = bytes.iter().take(size_of::<u64>()).collect();
    if fs_bytes.len() != size_of::<u64>() {
      return Err("Error unpacking filesize");
    }
    let filesize = bytes_to_num(fs_bytes);

    let bz_bytes : Vec<&u8> = bytes.iter().skip(size_of::<u64>()).take(size_of::<usize>()).collect();
    if bz_bytes.len() != size_of::<usize>() {
      return Err("Error unpacking blocksize");
    }
    let blocksize = bytes_to_num(bz_bytes) as usize;

    let bs_bytes : Vec<&u8> = bytes.iter().skip(size_of::<u64>() + size_of::<usize>()).take(size_of::<u32>()).collect();
    if bs_bytes.len() != size_of::<u32>() {
      return Err("Error unpacking blockseed");
    }
    let blockseed = bytes_to_num(bs_bytes) as u32;

    // TODO: should only take up to blocksize!
    let data = bytes.iter().skip(size_of::<u64>() + size_of::<usize>() + size_of::<u32>()).cloned().collect();

    Ok(LTBlock {
      filesize: filesize,
      blocksize: blocksize,
      blockseed: blockseed,
      data: data
    })
  }
}

#[test]
fn test_serialize(){
  let block = LTBlock {
    filesize: 100,
    blocksize: 10,
    blockseed: 123456,
    data: vec!(1,3,5,7,98,6,4,2)
  };
  let encoded = block.encode();
  assert_eq!(block, LTBlock::decode(encoded).unwrap());
}
