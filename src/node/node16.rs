use crate::node::{BoxedNode, Children, NodeType};

const NODE16_CAPACITY: usize = 16;

pub(crate) struct Node16Children<V> {
  _keys: [u8; NODE16_CAPACITY],
  _children: [BoxedNode<V>; NODE16_CAPACITY],
}

impl<V> Default for Node16Children<V> {
  fn default() -> Self {
    todo!()
  }
}

impl<V> Children<V> for Node16Children<V> {
  const NODE_TYPE: NodeType = NodeType::Node16;

  fn insert(&mut self, k: u8, node: BoxedNode<V>) -> Option<BoxedNode<V>> {
    todo!()
  }
}
