use crate::node::internal::SearchResult::{Found, GoDown, GoUp};
use crate::node::leaf::LeafNodeRef;
use crate::node::{NodeBase, NodeRef};
use crate::search::{SearchArgument, SearchResult};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ptr::NonNull;

const MAX_PREFIX_LEN: usize = 16;

struct PartialPrefixData {
  partial_prefix: [u8; MAX_PREFIX_LEN],
  partial_prefix_len: usize,
}

enum PartialKey<V> {
  PartialPrefix(PartialPrefixData),
  Leaf(LeafNodeRef<V>),
}

#[repr(C)]
pub(crate) struct InternalNodeBase<V> {
  node_base: NodeBase<V>,
  partial_key: PartialKey<V>,
  children_count: u8,
}

#[repr(C)]
pub(crate) struct InternalNode<C, V> {
  base: InternalNodeBase<C>,
  children: C,
  marker: PhantomData<V>,
}

#[repr(C)]
pub(crate) struct InternalNodeRef<V> {
  inner: NonNull<InternalNodeBase<V>>,
}

trait ChildrenContainer {}

impl PartialPrefixData {
  #[inline(always)]
  fn partial_prefix(&self) -> &[u8] {
    &self.partial_prefix[0..self.partial_prefix_len]
  }
}

impl<V> InternalNodeRef<V> {
  pub(crate) fn new(inner: NonNull<InternalNodeBase<V>>) -> Self {
    Self { inner }
  }
}

impl<V> Clone for InternalNodeRef<V> {
  fn clone(&self) -> Self {
    Self { inner: self.inner }
  }
}

impl<V> Copy for InternalNodeRef<V> {}

impl<V> InternalNodeRef<V> {
  #[inline(always)]
  fn inner(&self) -> &InternalNodeBase<V> {
    unsafe { self.inner.as_ref() }
  }

  pub(crate) fn find_lower_bound(self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
    match &self.inner().partial_key {
      PartialKey::PartialPrefix(data) => self.lower_bound_with_partial_prefix(data, arg),
      PartialKey::Leaf(leaf) => self.lower_bound_with_leaf(*leaf, arg),
    }
  }

  pub(crate) fn find_child(self, _k: u8) -> Option<NodeRef<V>> {
    unimplemented!()
  }

  pub(crate) fn find_next_child(self, _k: u8) -> Option<NodeRef<V>> {
    unimplemented!()
  }

  fn lower_bound_with_partial_prefix(
    self,
    partial_prefix_data: &PartialPrefixData,
    arg: SearchArgument,
  ) -> SearchResult<LeafNodeRef<V>> {
    let partial_key = arg.partial_key();
    let partial_prefix = partial_prefix_data.partial_prefix();
    if partial_key.len() <= partial_prefix.len() {
      match partial_key.cmp(partial_prefix) {
        Ordering::Greater => GoUp,
        _ => Found(self.minimum_leaf()),
      }
    } else {
      let partial_key_of_prefix = &partial_key[0..partial_prefix.len()];
      match partial_key_of_prefix.cmp(partial_prefix) {
        Ordering::Less => Found(self.minimum_leaf()),
        Ordering::Equal => GoDown(arg.depth() + partial_prefix.len()),
        Ordering::Greater => GoUp,
      }
    }
  }

  fn lower_bound_with_leaf(
    self,
    leaf_ref: LeafNodeRef<V>,
    arg: SearchArgument,
  ) -> SearchResult<LeafNodeRef<V>> {
    let leaf_node = leaf_ref.inner();
    let partial_key = arg.partial_key();
    let partial_leaf_key = &leaf_node.key()[arg.depth()..];
    if partial_key.len() <= partial_leaf_key.len() {
      match partial_key.cmp(partial_leaf_key) {
        Ordering::Greater => GoUp,
        _ => Found(leaf_ref),
      }
    } else {
      let partial_key_of_leaf = &partial_key[0..partial_leaf_key.len()];
      match partial_key_of_leaf.cmp(partial_leaf_key) {
        Ordering::Greater => GoUp,
        Ordering::Equal => GoDown(arg.depth() + partial_leaf_key.len()),
        Ordering::Less => Found(leaf_ref),
      }
    }
  }

  fn minimum_leaf(self) -> LeafNodeRef<V> {
    unimplemented!()
  }
}
