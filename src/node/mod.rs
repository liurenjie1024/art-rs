use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub(crate) use internal::*;
pub(crate) use leaf::*;

use crate::marker::{Immut, Internal, InternalOrLeaf, Leaf, Mut};

mod internal;
mod node16;
mod node256;
mod node4;
mod node48;

mod leaf;

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

/// Position of a child in parent node.
///
/// When `idx <= 0xFF`, it's an index in `Children` container.
/// Otherwise it's a leaf node.
#[derive(Debug, Copy, Clone)]
pub(super) struct ChildPos {
  idx: u16,
}

#[repr(C)]
pub(crate) struct NodeBase<K, V> {
  pub(super) node_type: NodeType,
  /// Pointer to parent node.
  pub(super) parent: Option<NonNull<InternalNodeBase<K, V>>>,
  /// Index in parent node. This is only inited when `parent` is not null.
  pub(super) idx: MaybeUninit<ChildPos>,
  pub(super) _marker: PhantomData<(K, V)>,
}

pub(crate) struct NodeRef<BorrowType, K, V, NodeType> {
  inner: NonNull<NodeBase<K, V>>,
  /// Prefix length from root until this node.
  prefix_len: usize,
  _marker: PhantomData<(BorrowType, NodeType)>,
}

impl<'a, K, V, NodeType> Clone for NodeRef<Immut<'a>, K, V, NodeType> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner,
      prefix_len: self.prefix_len,
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
  pub(super) fn new_root(node_type: NodeType) -> Self {
    Self {
      node_type,
      parent: None,
      idx: MaybeUninit::uninit(),
      _marker: PhantomData,
    }
  }
  pub(super) fn new(node_type: NodeType, parent: NonNull<InternalNodeBase<K, V>>, idx: ChildPos) -> Self {
    Self {
      node_type,
      parent: Some(parent),
      idx: MaybeUninit::new(idx),
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, K, V, NodeType> NodeRef<BorrowType, K, V, NodeType> {
  /// Returns parent node ref if exists.
  pub(crate) fn ascend(&self) -> Option<NodeRef<BorrowType, K, V, Internal>> {
    if let Some(parent_ptr) = self.as_base_ref().parent {
      let parent_prefix_len = self.prefix_len - unsafe {
        parent_ptr.as_ref().partial_key().len()
      } - 1;
      Some(NodeRef {
        inner: parent_ptr.cast(),
        prefix_len: parent_prefix_len,
        _marker: PhantomData,
      })
    } else {
      None
    }
  }

  pub(crate) fn prefix_len(&self) -> usize {
    self.prefix_len
  }

  fn as_base_ref(&self) -> &NodeBase<K, V> {
    unsafe { self.inner.as_ref() }
  }

  pub(crate) fn forget_type(self) -> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
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
      _marker: PhantomData,
    }
  }
}

impl<'a, K, V, NodeType> NodeRef<Mut<'a>, K, V, NodeType> {
  fn as_base_mut(&mut self) -> &mut NodeBase<K, V> {
    // SAFETY: Borrowed in mut ref.
    unsafe { self.inner.as_mut() }
  }

  /// Write new pointer to holder of this node.
  pub(crate) unsafe fn replace_self_in_parent(&mut self, new_ptr: Option<BoxedNode<K, V>>) {
    if let Some(mut parent) = self.ascend() {
      parent.update_child_at(self.inner.as_ref().idx.assume_init(), new_ptr);
    }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
  pub(crate) fn downcast(self) -> NodeImpl<BorrowType, K, V> {
    match self.as_base_ref().node_type {
      NodeType::Leaf => NodeImpl::Leaf(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        _marker: PhantomData,
      }),
      _ => NodeImpl::Internal(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        _marker: PhantomData,
      }),
    }
  }
}

impl ChildPos {
  /// Returns child index of this position. The result is `None` if it's leaf.
  fn to_idx(self) -> Option<u32> {
    if self.idx > 0xFF {
      None
    } else {
      Some(self.idx as u32)
    }
  }
}

impl From<Option<u8>> for ChildPos {
  fn from(input: Option<u8>) -> Self {
    match input {
      // An index in `Children` container.
      Some(idx) => Self { idx: idx as u16 },
      // Leaf
      None => Self { idx: 0xFFFF }
    }
  }
}
