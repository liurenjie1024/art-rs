use std::mem::swap;
use std::ptr::NonNull;

use crate::node::{NodeBase, NodeType};

// pub(crate) type BoxedLeafNode<V> = NonNull<LeafNode<V>>;

#[repr(C)]
pub(crate) struct LeafNode<V> {
  node_base: NodeBase<V>,
  key: Vec<u8>,
  value: V,
}

impl<V> LeafNode<V> {
  pub(crate) fn key(&self) -> &[u8] {
    &self.key
  }
}

impl<V> LeafNode<V> {
  pub(crate) fn new(prefix_len: usize, key: &[u8], value: V) -> Box<Self> {
    Box::new(Self {
      node_base: NodeBase::new(NodeType::Leaf, prefix_len),
      key: Vec::from(key),
      value,
    })
  }

  pub(crate) fn set_value(&mut self, mut value: V) -> V {
    swap(&mut self.value, &mut value);
    value
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    if self.node_base.prefix_len >= self.key.len() {
      return &[];
    }
    &self.key[self.node_base.prefix_len..]
  }

  pub(crate) fn value_mut(&mut self) -> &mut V {
    &mut self.value
  }

  pub(crate) fn value_ref(&self) -> &V {
    &self.value
  }

  pub(crate) fn value_ptr(&mut self) -> NonNull<V> {
    NonNull::from(&mut self.value)
  }
}
