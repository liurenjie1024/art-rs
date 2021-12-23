use std::mem::swap;
use std::ptr::NonNull;

use crate::marker::{Immut, Leaf, Mut};
use crate::node::NodeRef;
use crate::node::{NodeBase, NodeType};


// pub(crate) type BoxedLeafNode<V> = NonNull<LeafNode<V>>;

#[repr(C)]
pub(crate) struct LeafNode<K, V> {
  node_base: NodeBase<K, V>,
  key: K,
  value: V,
}

impl<K, V> LeafNode<K, V> {
  pub(crate) fn new(key: K, value: V) -> Box<Self> {
    Box::new(Self {
      node_base: NodeBase::new(NodeType::Leaf),
      key,
      value,
    })
  }

  pub(crate) fn set_value(&mut self, mut value: V) -> V {
    swap(&mut self.value, &mut value);
    value
  }

  pub(crate) fn key_ref(&self) -> &K {
    &self.key
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

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, Leaf> {
  fn as_leaf_ptr(&self) -> *mut LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    self.inner.cast().as_ptr()
  }

  pub(crate) fn as_leaf_ref(&self) -> &LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_ref() }
  }

  pub(crate) fn as_leaf_mut(&mut self) -> &mut LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_mut() }
  }
}

impl<BorrowType, K: AsRef<[u8]>, V> NodeRef<BorrowType, K, V, Leaf> {
  pub(crate) fn partial_key(&self) -> &[u8] {
    let leaf = self.as_leaf_ref();
    let leaf_key_bytes = leaf.key_ref().as_ref();
    if self.prefix_len >= leaf_key_bytes.len() {
      &[]
    } else {
      &leaf_key_bytes[self.prefix_len..]
    }
  }
}

impl<'a, K: 'a + AsRef<[u8]>, V: 'a> NodeRef<Mut<'a>, K, V, Leaf> {
  pub(crate) fn value_mut(self) -> &'a mut V {
    unsafe { (&mut *self.as_leaf_ptr()).value_mut() }
  }

  pub(crate) fn set_prefix_len(&mut self, new_prefix_len: usize) {
    debug_assert!(self.as_leaf_ref().key_ref().as_ref().len() >= new_prefix_len);
    self.prefix_len = new_prefix_len;
  }

  pub(crate) fn into_kv(self) -> (K, V) {
    let leaf = unsafe { Box::from_raw(self.inner.cast::<LeafNode<K, V>>().as_ptr()) };
    (leaf.key, leaf.value)
  }
}

impl<'a, K: 'a, V: 'a> NodeRef<Immut<'a>, K, V, Leaf> {
  pub(crate) fn value_ref(&self) -> &'a V {
    unsafe { (&*self.as_leaf_ptr()).value_ref() }
  }
}

// impl<K, V> NodeRef<Owned, K, V, Leaf> {
//   pub(crate) fn from_new_leaf_node(prefix_len: usize, leaf: Box<LeafNode<K, V>>) -> Self {
//     Self {
//       inner: NonNull::from(Box::leak(leaf)).cast(),
//       prefix_len,
//       _marker: PhantomData,
//     }
//   }
// }
