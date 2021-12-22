use std::marker::PhantomData;
use std::ptr::NonNull;

pub(crate) use internal::*;
pub(crate) use leaf::*;

use crate::marker::{Immut, Internal, InternalOrLeaf, Leaf, Mut, Owned};

mod internal;
mod node16;
mod node256;
mod node4;
mod node48;

mod leaf;

pub(crate) const DEFAULT_TREE_DEPTH: usize = 16;

pub(crate) type Root<K, V> = NodeRef<Owned, K, V, InternalOrLeaf>;
pub(crate) type BoxedNode<K, V> = NonNull<NodeBase<K, V>>;
pub(crate) type Handle<K, V> = NonNull<Option<BoxedNode<K, V>>>;

#[repr(u8)]
pub(crate) enum NodeType {
  Node4,
  Node16,
  Node48,
  Node256,
  Leaf,
}

#[repr(C)]
pub(crate) struct NodeBase<K, V> {
  node_type: NodeType,
  _marker: PhantomData<(K, V)>,
}

pub(crate) struct NodeRef<BorrowType, K, V, NodeType> {
  inner: NonNull<NodeBase<K, V>>,
  /// Prefix length from root until this node.
  prefix_len: usize,
  /// Pointer holding this node.
  ///
  /// For non-root node, this is the pointer in parent node.
  /// For root node, this is the pointer in map.
  holder: NonNull<Option<BoxedNode<K, V>>>,
  _marker: PhantomData<(BorrowType, NodeType)>,
}

impl<'a, K, V, NodeType> Clone for NodeRef<Immut<'a>, K, V, NodeType> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner,
      prefix_len: self.prefix_len,
      holder: self.holder,
      _marker: PhantomData,
    }
  }
}

impl<'a, K, V, NodeType> Copy for NodeRef<Immut<'a>, K, V, NodeType> {}

pub(crate) enum NodeImpl<BorrowType, K, V> {
  Internal(NodeRef<BorrowType, K, V, Internal>),
  Leaf(NodeRef<BorrowType, K, V, Leaf>),
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

impl<K, V> NodeBase<K, V> {
  pub(crate) fn new(node_type: NodeType) -> Self {
    Self {
      node_type,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, K, V, NodeType> NodeRef<BorrowType, K, V, NodeType> {
  pub(crate) fn prefix_len(&self) -> usize {
    self.prefix_len
  }

  fn as_base_ref(&self) -> &NodeBase<K, V> {
    unsafe { self.inner.as_ref() }
  }

  /// Temporarily takes out another immutable reference to the same node.
  pub(crate) fn borrow(&self) -> NodeRef<Immut<'_>, K, V, NodeType> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      holder: self.holder,
      _marker: PhantomData,
    }
  }

  /// Takes a mutable reference.
  ///
  /// # Safety
  ///
  /// It should only be borrowed once.
  pub(crate) unsafe fn borrow_mut(&mut self) -> NodeRef<Mut<'_>, K, V, NodeType> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      holder: self.holder,
      _marker: PhantomData,
    }
  }

  pub(crate) fn forget_type(self) -> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      holder: self.holder,
      _marker: PhantomData,
    }
  }

  pub(crate) fn get_inner(&self) -> BoxedNode<K, V> {
    self.inner
  }

  pub(crate) fn root_node_ref(ptr: BoxedNode<K, V>, holder: NonNull<Option<BoxedNode<K, V>>>) -> Self {
    Self {
      inner: ptr,
      prefix_len: 0,
      holder,
      _marker: PhantomData,
    }
  }
}

impl<'a, K, V, NodeType> NodeRef<Mut<'a>, K, V, NodeType> {
  /// Temporarily takes out a mutable reference to the same node.
  pub(crate) fn reborrow(&mut self) -> NodeRef<Mut<'_>, K, V, NodeType> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      holder: self.holder,
      _marker: PhantomData,
    }
  }

  /// Write new pointer to holder of this node.
  pub(crate) unsafe fn replace_holder(mut self, new_ptr: Option<BoxedNode<K, V>>) {
    std::ptr::write(self.holder.as_ptr(), new_ptr)
  }
}

impl<'a, K, V, NodeType> NodeRef<Mut<'a>, K, V, NodeType> {
  fn as_base_mut(&mut self) -> &mut NodeBase<K, V> {
    unsafe { self.inner.as_mut() }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
  pub(crate) fn downcast(self) -> NodeImpl<BorrowType, K, V> {
    match self.as_base_ref().node_type {
      NodeType::Leaf => NodeImpl::Leaf(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        holder: self.holder,
        _marker: PhantomData,
      }),
      _ => NodeImpl::Internal(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        holder: self.holder,
        _marker: PhantomData,
      }),
    }
  }
}
