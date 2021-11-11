use std::cmp::min;
use std::mem::swap;
use crate::node::{InternalNodeRef, NodeRef};
use crate::search::SearchArgument;
use std::ptr::NonNull;
use crate::common_prefix_len;
use crate::node::node4::Node4Children;

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
  /// Prefix length in parent.
  prefix_len: usize,
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

  pub(crate) fn with_data(key: &[u8], prefix_len: usize, value: V) -> Self {
    let node = Box::new(LeafNode::<V> {
      key: Vec::from(key),
      prefix_len,
      value,
      prev: None,
      next: None,
    });

    Self {
      inner: NonNull::from(Box::leak(node)),
    }
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

  pub(crate) fn update(mut self, key: &[u8], depth: usize, value: V) -> NodeRef<V> {
    let this_node = self.inner_mut();
    let this_partial_key = this_node.partial_key();
    let input_partial_key = &key[depth..];

    let same_prefix_len = common_prefix_len(this_partial_key, input_partial_key);
    if same_prefix_len < min(this_partial_key.len(), input_partial_key.len()) {
      // Here we need to create a new node to hold them
      let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
      new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);

      // Current leaf node just needs to adjust its `prefix_len` in parent.
      {
        this_node.prefix_len += same_prefix_len;
        new_node4.upsert_child(this_node.key[this_node.prefix_len], self);
      }


      // Create a new leaf node for new value
      {
        let prefix_len = depth + same_prefix_len;
        let new_leaf_node = LeafNodeRef::with_data(key, prefix_len, value);
        let k = key[prefix_len];
        new_node4.upsert_child(k, new_leaf_node);
      }

      new_node4.into()
    } else {
      if this_partial_key.len() == input_partial_key.len() {
        // Same key, just update value
        this_node.set_value(value);
        self.into()
      } else {
        // We need to create a new node to hold them

        // Here we need to create a new node to hold them
        let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
        new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);

        let new_prefix_len = this_node.prefix_len + same_prefix_len;

        if this_partial_key.len() < input_partial_key.len() {
          this_node.prefix_len = new_prefix_len;
          new_node4.inner_mut().set_leaf(self);

          let k = key[new_prefix_len];
          let new_leaf_node = LeafNodeRef::with_data(key, new_prefix_len + 1, value);
          new_node4.inner_mut().upsert_child(k, new_leaf_node);
        } else {
          let new_leaf_node = LeafNodeRef::with_data(key, new_prefix_len, value);
          new_node4.inner_mut().set_leaf(new_leaf_node);


          let k = key[new_prefix_len];
          this_node.prefix_len = new_prefix_len + 1;
          new_node4.inner_mut().upsert_child(k, self);
        }

        new_node4.into()
      }
    }
  }

  pub(crate) fn inner(&self) -> &LeafNode<V> {
    unsafe { self.inner.as_ref() }
  }

  pub(crate) fn inner_mut(&mut self) -> &mut LeafNode<V> {
    unsafe { self.inner.as_mut() }
  }
}

impl<V> Into<NodeRef<V>> for LeafNodeRef<V> {
  fn into(self) -> NodeRef<V> {
    NodeRef::<V> {
      inner: self.inner.cast(),
    }
  }
}

impl<V> LeafNode<V> {
  pub(crate) fn set_value(&mut self, mut value: V) -> V {
    swap(&mut self.value, &mut value);
    value
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    &self.key[self.prefix_len..]
  }
}
