use crate::node::{BoxedNode, Children, NodeType};

pub(in crate::node) const NODE256_CAPACITY: usize = 256;

pub(crate) struct Node256Children<K, V> {
  _children: [BoxedNode<K, V>; NODE256_CAPACITY],
}

impl<K, V> Default for Node256Children<K, V> {
  fn default() -> Self {
    todo!()
  }
}

impl<K, V> Children<K, V> for Node256Children<K, V> {
  const NODE_TYPE: NodeType = NodeType::Node256;
  fn insert(&mut self, _k: u8, _node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>> {
    todo!()
  }
}
