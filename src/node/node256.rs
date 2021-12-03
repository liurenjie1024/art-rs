use crate::node::{BoxedNode, Children, NodeType};

pub(in crate::node) const NODE256_CAPACITY: usize = 256;

pub(crate) struct Node256Children<V> {
  _children: [BoxedNode<V>; NODE256_CAPACITY],
}

impl<V> Default for Node256Children<V> {
  fn default() -> Self {
    todo!()
  }
}

impl<V> Children<V> for Node256Children<V> {
  const NODE_TYPE: NodeType = NodeType::Node256;
  fn insert(&mut self, _k: u8, _node: BoxedNode<V>) -> Option<BoxedNode<V>> {
    todo!()
  }
}
