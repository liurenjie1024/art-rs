use crate::node::node256::NODE256_CAPACITY;
use crate::node::{BoxedNode, Children, NodeRef, NodeType};

const NODE48_CAPACITY: usize = 256;

pub(crate) struct Node48Children<V> {
  keys: [u8; NODE256_CAPACITY],
  children: [BoxedNode<V>; NODE48_CAPACITY],
}

impl<V> Default for Node48Children<V> {
  fn default() -> Self {
    todo!()
  }
}

impl<V> Children for Node48Children<V> {
  const NODE_TYPE: NodeType = NodeType::Node48;
}
