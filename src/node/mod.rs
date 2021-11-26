use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::marker::{Immut, Mut, Owned};
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
pub(crate) type Root<V> = NodeRef<Owned, V>;
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
  _marker: PhantomData<V>,
}

pub(crate) struct NodeRef<BorrowType, V> {
  inner: NonNull<NodeBase<V>>,
  _marker: PhantomData<BorrowType>,
}

impl<'a, V> Clone for NodeRef<Immut<'a>, V> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<'a, V> Copy for NodeRef<Immut<'a>, V> {}

pub(crate) enum NodeKind<BorrowType, V> {
  Internal(InternalNodeRef<BorrowType, V>),
  Leaf(LeafNodeRef<BorrowType, V>),
}

impl<BorrowType, V> NodeRef<BorrowType, V> {
  pub(crate) fn downcast(self) -> NodeKind<BorrowType, V> {
    unsafe {
      match self.inner().node_type {
        NodeType::Leaf => NodeKind::Leaf(LeafNodeRef::<BorrowType, V>::from(self)),
        _ => NodeKind::Internal(InternalNodeRef::<BorrowType, V>::from(self)),
      }
    }
  }

  pub(crate) unsafe fn from_leaf_node_ref(leaf_node: BoxedLeafNode<V>) -> Self {
    Self {
      inner: leaf_node.cast(),
      _marker: PhantomData,
    }
  }

  pub(crate) fn minimum_leaf(self) -> LeafNodeRef<BorrowType, V> {
    unimplemented!()
  }

  /// Temporarily takes out another immutable reference to the same node.
  pub(crate) fn reborrow(&self) -> NodeRef<Immut<'_>, V> {
    NodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }

  /// Temporarily takes out a mutable reference to the same node.
  pub(crate) fn borrow_mut(&mut self) -> NodeRef<Mut<'_>, V> {
    NodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
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
  pub(crate) fn new(node_type: NodeType) -> Self {
    Self {
      node_type,
      _marker: PhantomData,
    }
  }
}

impl<V> From<NodeType> for NodeBase<V> {
  fn from(node_type: NodeType) -> Self {
    Self::new(node_type)
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V> {
  fn inner(&self) -> &NodeBase<V> {
    unsafe { self.inner.as_ref() }
  }
}

impl<BorrowType, V> From<InternalNodeRef<BorrowType, V>> for NodeRef<BorrowType, V> {
  fn from(internal: InternalNodeRef<BorrowType, V>) -> Self {
    Self {
      inner: unsafe { internal.to_ptr().cast() },
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, V> From<LeafNodeRef<BorrowType, V>> for NodeRef<BorrowType, V> {
  fn from(leaf: LeafNodeRef<BorrowType, V>) -> Self {
    Self {
      inner: unsafe { leaf.to_ptr().cast() },
      _marker: PhantomData,
    }
  }
}
