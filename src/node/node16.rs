use crate::node::{BoxedNode, Children, NodeRef, NodeType};

const NODE16_CAPACITY: usize = 16;

pub(crate) struct Node16Children<V> {
  keys: [u8; NODE16_CAPACITY],
  children: [BoxedNode<V>; NODE16_CAPACITY],
}

impl<V> Default for Node16Children<V> {
  fn default() -> Self {
    todo!()
  }
}

impl<V> Children for Node16Children<V> {
  const NODE_TYPE: NodeType = NodeType::Node16;
}
