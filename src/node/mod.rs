use std::marker::PhantomData;
use std::ptr::NonNull;

pub(crate) use handle::*;
pub(crate) use internal::*;
pub(crate) use leaf::*;

use crate::marker::{Immut, Internal, InternalOrLeaf, Leaf, Mut, Owned};

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
  // TODO: We should remove this to save memory
  prefix_len: usize,
  _marker: PhantomData<V>,
}

pub(crate) struct NodeRef<BorrowType, V, NodeType> {
  inner: NonNull<NodeBase<V>>,
  _marker: PhantomData<(BorrowType, NodeType)>,
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

  pub(crate) fn prefix_len(&self) -> usize {
    self.prefix_len
  }

  /// Sets the new prefix length of this node.
  ///
  /// # Safety
  ///
  /// For leaf node, this should not be larger than length.
  unsafe fn set_prefix_len(&mut self, new_prefix_len: usize) {
    self.prefix_len = new_prefix_len;
  }
}

impl<BorrowType, V, NodeType> NodeRef<BorrowType, V, NodeType> {
  pub(crate) unsafe fn inner(&self) -> BoxedNode<V> {
    self.inner
  }

  pub(crate) fn as_base_ref(&self) -> &NodeBase<V> {
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

  pub(crate) fn forget_type(self) -> NodeRef<BorrowType, V, InternalOrLeaf> {
    NodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, InternalOrLeaf> {
  pub(crate) fn downcast(self) -> NodeKind<BorrowType, V> {
    match self.as_base_ref().node_type {
      NodeType::Leaf => NodeKind::Leaf(NodeRef {
        inner: self.inner,
        _marker: PhantomData,
      }),
      _ => NodeKind::Internal(NodeRef {
        inner: self.inner,
        _marker: PhantomData,
      }),
    }
  }
}

impl<'a, V, NodeType> NodeRef<Mut<'a>, V, NodeType> {
  pub(crate) fn as_base_mut(&mut self) -> &mut NodeBase<V> {
    unsafe { self.inner.as_mut() }
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, Internal> {
  pub(crate) fn as_internal_ref(&self) -> &InternalNodeBase<V> {
    debug_assert!(self.as_base_ref().node_type.is_internal());
    // SAFETY: This is internal node.
    unsafe { self.inner.cast().as_ref() }
  }

  pub(crate) fn as_internal_mut(&mut self) -> &mut InternalNodeBase<V> {
    debug_assert!(self.as_base_ref().node_type.is_internal());
    // SAFETY: This is internal node.
    unsafe { self.inner.cast().as_mut() }
  }

  pub(crate) fn find_child(&self, _k: u8) -> Option<Handle<BorrowType, V>> {
    todo!()
  }

  pub(crate) fn get_leaf(&self) -> Option<NodeRef<BorrowType, V, Leaf>> {
    todo!()
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, Leaf> {
  fn as_leaf_ptr(&self) -> *mut LeafNode<V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    self.inner.cast().as_ptr()
  }

  pub(crate) fn as_leaf_ref(&self) -> &LeafNode<V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_ref() }
  }

  pub(crate) fn as_leaf_mut(&mut self) -> &mut LeafNode<V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_mut() }
  }
}

impl<'a, V> NodeRef<Mut<'a>, V, Leaf> {
  pub(crate) fn value_mut(self) -> &'a mut V {
    unsafe { (&mut *self.as_leaf_ptr()).value_mut() }
  }

  pub(crate) fn set_prefix_len(&mut self, new_prefix_len: usize) {
    assert!(self.as_base_ref().prefix_len() >= new_prefix_len);
    unsafe {
      self.as_base_mut().set_prefix_len(new_prefix_len);
    }
  }
}

impl<'a, V> NodeRef<Immut<'a>, V, Leaf> {
  pub(crate) fn value_ref(self) -> &'a V {
    unsafe { (&*self.as_leaf_ptr()).value_ref() }
  }
}

impl<V> NodeRef<Owned, V, Leaf> {
  pub(crate) fn from_new_leaf_node(leaf: Box<LeafNode<V>>) -> Self {
    Self {
      inner: NonNull::from(Box::leak(leaf)).cast(),
      _marker: PhantomData,
    }
  }
}

impl<V> NodeRef<Owned, V, Internal> {
  pub(crate) fn from_new_internal_node<C>(leaf: Box<InternalNode<C, V>>) -> Self {
    Self {
      inner: NonNull::from(Box::leak(leaf)).cast(),
      _marker: PhantomData,
    }
  }
}
