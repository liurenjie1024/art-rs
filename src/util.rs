use std::cmp::min;

pub(crate) fn common_prefix_len(left: &[u8], right: &[u8]) -> usize {
  if let Some(pos) = left
    .iter()
    .zip(right)
    .position(|(left, right)| *left != *right)
  {
    pos + 1
  } else {
    min(left.len(), right.len())
  }
}
