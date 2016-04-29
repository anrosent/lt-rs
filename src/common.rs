// TODO: needs testing
pub fn num_to_bytes(n: u64, nbytes: usize) -> Vec<u8> {
  let mut buf = vec!();
  for shifts in (0..nbytes).rev() {
    buf.push((n >> (8 * shifts)) as u8);
  }
  buf
}

// TODO: needs testing
pub fn bytes_to_num(bytes: Vec<&u8>) -> u64 {
  let mut res = 0u64;
  for (ix, &byte) in bytes.iter().rev().enumerate() {
    res |= (*byte as u64) << (8 * ix);
  }
  res
}

// TODO: needs testing
pub fn vec_xor(v1: &mut [u8], v2: &[u8]) {
  assert_eq!(v1.len(), v2.len());
  for (ix, b) in v2.iter().enumerate() {
    v1[ix] ^= *b;
  }
}
