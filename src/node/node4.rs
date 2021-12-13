use crate::node::{BoxedNode, Children, NodeType};

const NODE4_CAPACITY: usize = 4;

pub(crate) struct Node4Children<K, V> {
  _keys: [u8; NODE4_CAPACITY],
  _children: [BoxedNode<K, V>; NODE4_CAPACITY],
}

impl<K, V> Default for Node4Children<K, V> {
  fn default() -> Self {
    todo!()
  }
}

impl<K, V> Children<K, V> for Node4Children<K, V> {
  const NODE_TYPE: NodeType = NodeType::Node4;

  fn insert(&mut self, _k: u8, _node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>> {
    todo!()
  }
}
