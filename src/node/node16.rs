use crate::node::{BoxedNode, Children, NodeType};

const NODE16_CAPACITY: usize = 16;

pub(crate) struct Node16Children<K, V> {
  _keys: [u8; NODE16_CAPACITY],
  _children: [BoxedNode<K, V>; NODE16_CAPACITY],
}

impl<K, V> Default for Node16Children<K, V> {
  fn default() -> Self {
    todo!()
  }
}

impl<K, V> Children<K, V> for Node16Children<K, V> {
  const NODE_TYPE: NodeType = NodeType::Node16;

  fn insert(&mut self, _k: u8, _node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>> {
    todo!()
  }
}
