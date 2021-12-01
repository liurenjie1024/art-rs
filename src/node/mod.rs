use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::marker::{Immut, Internal, InternalOrLeaf, Leaf, Mut, Owned};
pub(crate) use handle::*;
pub(crate) use internal::*;
pub(crate) use leaf::*;

mod handle;
mod internal;
mod node16;
mod node256;
mod node4;
mod node48;

mod leaf;

pub(crate) const DEFAULT_TREE_DEPTH: usize = 16;
pub(crate) type Root<V> = NodeRef<Owned, V, InternalOrLeaf>;
pub(crate) type BoxedNode<V> = NonNull<NodeBase<V>>;

#[repr(u8)]
pub(crate) enum NodeType {
  Node4,
  Node16,
  Node48,
  Node256,
  Leaf,
}

#[repr(C)]
pub(crate) struct NodeBase<V> {
  node_type: NodeType,
  /// Prefix length from root until this node.
  prefix_len: usize,
  _marker: PhantomData<V>,
}

pub(crate) struct NodeRef<BorrowType, V, NodeType> {
  inner: NonNull<NodeBase<V>>,
  _marker: PhantomData<BorrowType>,
}

impl<'a, V, NodeType> Clone for NodeRef<Immut<'a>, V, NodeType> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<'a, V, NodeType> Copy for NodeRef<Immut<'a>, V, NodeType> {}

pub(crate) enum NodeKind<BorrowType, V> {
  Internal(NodeRef<BorrowType, V, Internal>),
  Leaf(NodeRef<BorrowType, V, Leaf>),
}

impl NodeType {
  fn is_internal(&self) -> bool {
    match self {
      NodeType::Leaf => false,
      _ => true,
    }
  }

  fn is_leaf(&self) -> bool {
    !self.is_internal()
  }
}

impl<V> NodeBase<V> {
  pub(crate) fn new(node_type: NodeType, prefix_len: usize) -> Self {
    Self {
      node_type,
      prefix_len,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, V, NodeType> NodeRef<BorrowType, V, NodeType> {
  fn inner(&self) -> &NodeBase<V> {
    unsafe { self.inner.as_ref() }
  }

  /// Temporarily takes out another immutable reference to the same node.
  pub(crate) fn reborrow(&self) -> NodeRef<Immut<'_>, V, NodeType> {
    NodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }

  /// Temporarily takes out a mutable reference to the same node.
  pub(crate) fn borrow_mut(&mut self) -> NodeRef<Mut<'_>, V, NodeType> {
    NodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, InternalOrLeaf> {
  pub(crate) fn downcast(self) -> NodeKind<BorrowType, V> {
    unsafe {
      match self.inner().node_type {
        NodeType::Leaf => NodeKind::Leaf(NodeRef {
          inner: self.inner,
          _marker: PhantomData,
        }),
        _ => NodeKind::Internal(NodeRef {
          inner: self.inner,
          _marker,
        }),
      }
    }
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, Internal> {}
