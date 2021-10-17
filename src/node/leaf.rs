use crate::node::search::SearchArgument;
use std::ptr::NonNull;

pub struct LeafRange<V> {
  start: Option<LeafNodeRef<V>>,
  end: Option<LeafNodeRef<V>>,
}

pub(crate) struct LeafNodeRef<V> {
  inner: NonNull<LeafNode<V>>,
}

impl<V> Clone for LeafNodeRef<V> {
  fn clone(&self) -> Self {
    Self { inner: self.inner }
  }
}

impl<V> Copy for LeafNodeRef<V> {}

pub struct LeafNode<V> {
  key: Vec<u8>,
  value: V,
  prev: Option<NonNull<LeafNode<V>>>,
  next: Option<NonNull<LeafNode<V>>>,
}

impl<V> LeafNode<V> {
  pub(crate) fn key(&self) -> &[u8] {
    &self.key
  }
}

impl<V> LeafNodeRef<V> {
  pub(crate) fn new(inner: NonNull<LeafNode<V>>) -> Self {
    Self { inner }
  }

  pub(crate) fn is_lower_bound(self, arg: SearchArgument) -> bool {
    let partial_key = arg.partial_key();
    let partial_leaf_key = &self.inner().key()[arg.depth()..];
    if partial_key.len() <= partial_leaf_key.len() {
      partial_key <= partial_leaf_key
    } else {
      let partial_key_of_leaf = &partial_key[0..partial_leaf_key.len()];
      partial_key_of_leaf < partial_leaf_key
    }
  }

  pub(crate) fn inner(&self) -> &LeafNode<V> {
    unsafe { self.inner.as_ref() }
  }
}
