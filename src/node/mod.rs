use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub(crate) use internal::*;
pub(crate) use leaf::*;
pub(crate) use handle::*;
use crate::marker::Immut;

mod internal;
mod node16;
mod node256;
mod node4;
mod node48;
mod handle;

mod leaf;

pub(crate) const DEFAULT_TREE_DEPTH: usize = 16;

pub(crate) type BoxedNode<V> = NonNull<NodeBase<V>>;

#[repr(u8)]
enum NodeType {
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
}

impl<'a, V> Clone for NodeRef<Immut<'a>, V> {
  fn clone(&self) -> Self {
    Self { inner: self.inner }
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
        NodeType::Leaf => NodeKind::Leaf(LeafNodeRef::<BorrowType, V>::from_node_ref_unchecked(self)),
        _ => NodeKind::Internal(InternalNodeRef::<BorrowType, V>::from_node_ref_unchecked(self)),
      }
    }
  }

  pub(crate) fn minimum_leaf(self) -> LeafNodeRef<BorrowType, V> {
    unimplemented!()
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
    unsafe {
      self.inner.as_ref()
    }
  }
}
