use crate::node::{BoxedNode, Children, NodeType};

const NODE4_CAPACITY: usize = 4;

pub(crate) struct Node4Children<V> {
  _keys: [u8; NODE4_CAPACITY],
  _children: [BoxedNode<V>; NODE4_CAPACITY],
}

impl<V> Default for Node4Children<V> {
  fn default() -> Self {
    todo!()
  }
}

impl<V> Children for Node4Children<V> {
  const NODE_TYPE: NodeType = NodeType::Node4;
}
