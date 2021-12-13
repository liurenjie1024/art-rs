use crate::node::node256::NODE256_CAPACITY;
use crate::node::{BoxedNode, Children, NodeType};

const NODE48_CAPACITY: usize = 256;

pub(crate) struct Node48Children<K, V> {
  _keys: [u8; NODE256_CAPACITY],
  _children: [BoxedNode<K, V>; NODE48_CAPACITY],
}

impl<K, V> Default for Node48Children<K, V> {
  fn default() -> Self {
    todo!()
  }
}

impl<K, V> Children<K, V> for Node48Children<K, V> {
  const NODE_TYPE: NodeType = NodeType::Node48;

  fn insert(&mut self, _k: u8, _node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>> {
    todo!()
  }
}
