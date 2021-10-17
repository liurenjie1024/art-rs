use std::marker::PhantomData;
use std::ptr::NonNull;

mod internal;
use internal::*;
mod node16;
mod node256;
mod node4;
mod node48;

mod leaf;

use leaf::*;

mod search;

pub(crate) const DEFAULT_TREE_DEPTH: usize = 16;

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

pub(crate) struct NodeRef<V> {
  inner: NonNull<NodeBase<V>>,
}

pub(crate) enum NodeKind<V> {
  Internal(InternalNodeRef<V>),
  Leaf(LeafNodeRef<V>),
}

impl<V> NodeRef<V> {
  pub(crate) fn downcast(self) -> NodeKind<V> {
    let node_base = unsafe { self.inner.as_ref() };

    match node_base.node_type {
      NodeType::Leaf => NodeKind::Leaf(LeafNodeRef::<V>::new(self.inner.cast())),
      _ => NodeKind::Internal(InternalNodeRef::<V>::new(self.inner.cast())),
    }
  }

  pub(crate) fn minimum_leaf(self) -> LeafNodeRef<V> {
    unimplemented!()
  }
}

impl NodeType {
  fn is_internal(self) -> bool {
    match self {
      NodeType::Leaf => false,
      _ => true,
    }
  }

  fn is_leaf(self) -> bool {
    !self.is_internal()
  }
}
