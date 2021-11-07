use crate::node::NodeRef;

const NODE4_CAPACITY: usize = 4;

#[derive(Default)]
pub(in crate::node) struct Node4Children<V> {
  keys: [u8; NODE4_CAPACITY],
  children: [NodeRef<V>; NODE4_CAPACITY],
}
