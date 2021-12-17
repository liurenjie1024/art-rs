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

pub(crate) type Root<K, V> = NodeRef<Owned, K, V, InternalOrLeaf>;
pub(crate) type BoxedNode<K, V> = NonNull<NodeBase<K, V>>;

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

pub(crate) enum NodeKind<BorrowType, K, V> {
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
  pub(crate) unsafe fn inner(&self) -> BoxedNode<K, V> {
    self.inner
  }

  pub(crate) fn prefix_len(&self) -> usize {
    self.prefix_len
  }

  pub(crate) fn as_base_ref(&self) -> &NodeBase<K, V> {
    unsafe { self.inner.as_ref() }
  }

  /// Temporarily takes out another immutable reference to the same node.
  pub(crate) fn reborrow(&self) -> NodeRef<Immut<'_>, K, V, NodeType> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      _marker: PhantomData,
    }
  }

  /// Temporarily takes out a mutable reference to the same node.
  pub(crate) fn borrow_mut(&mut self) -> NodeRef<Mut<'_>, K, V, NodeType> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      _marker: PhantomData,
    }
  }

  pub(crate) fn forget_type(self) -> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    NodeRef {
      inner: self.inner,
      prefix_len: self.prefix_len,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
  pub(crate) fn downcast(self) -> NodeKind<BorrowType, K, V> {
    match self.as_base_ref().node_type {
      NodeType::Leaf => NodeKind::Leaf(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        _marker: PhantomData,
      }),
      _ => NodeKind::Internal(NodeRef {
        inner: self.inner,
        prefix_len: self.prefix_len,
        _marker: PhantomData,
      }),
    }
  }
}

impl<'a, K, V, NodeType> NodeRef<Mut<'a>, K, V, NodeType> {
  pub(crate) fn as_base_mut(&mut self) -> &mut NodeBase<K, V> {
    unsafe { self.inner.as_mut() }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, Internal> {
  pub(crate) fn as_internal_ref(&self) -> &InternalNodeBase<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_internal());
    // SAFETY: This is internal node.
    unsafe { self.inner.cast().as_ref() }
  }

  pub(crate) fn as_internal_mut(&mut self) -> &mut InternalNodeBase<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_internal());
    // SAFETY: This is internal node.
    unsafe { self.inner.cast().as_mut() }
  }

  fn as_internal_impl(&self) -> InternalNodeImpl<'_, K, V> {
    // SAFETY: This is internal node.
    unsafe {
      match self.as_base_ref().node_type {
        NodeType::Node4 => {
          InternalNodeImpl::Node4(self.inner.cast::<InternalNode4<K, V>>().as_ref())
        }
        NodeType::Node16 => {
          InternalNodeImpl::Node16(self.inner.cast::<InternalNode16<K, V>>().as_ref())
        }
        NodeType::Node48 => {
          InternalNodeImpl::Node48(self.inner.cast::<InternalNode48<K, V>>().as_ref())
        }
        NodeType::Node256 => {
          InternalNodeImpl::Node256(self.inner.cast::<InternalNode256<K, V>>().as_ref())
        }
        NodeType::Leaf => panic!("This should not happen!"),
      }
    }
  }

  pub(crate) fn find_child(&self, _k: u8) -> Option<Handle<BorrowType, K, V>> {
    todo!()
  }

  pub(crate) fn get_leaf(&self) -> Option<NodeRef<BorrowType, K, V, Leaf>> {
    let internal_ref = self.as_internal_ref();
    let leaf_prefix_len = self.prefix_len + internal_ref.partial_prefix().len();
    internal_ref.get_leaf().map(|leaf_ptr| NodeRef {
      inner: leaf_ptr,
      prefix_len: leaf_prefix_len,
      _marker: PhantomData,
    })
  }

  pub(crate) fn child_at(&self, idx: usize) -> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    let internal_ref = self.as_internal_impl();
    let child_prefix_len = self.prefix_len + internal_ref.partial_prefix().len() + 1;
    NodeRef {
      inner: internal_ref.child_at(idx),
      prefix_len: child_prefix_len,
      _marker: PhantomData,
    }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, Leaf> {
  fn as_leaf_ptr(&self) -> *mut LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    self.inner.cast().as_ptr()
  }

  pub(crate) fn as_leaf_ref(&self) -> &LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_ref() }
  }

  pub(crate) fn as_leaf_mut(&mut self) -> &mut LeafNode<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_leaf());
    // SAFETY: This is leaf node.
    unsafe { self.inner.cast().as_mut() }
  }
}

impl<BorrowType, K: AsRef<[u8]>, V> NodeRef<BorrowType, K, V, Leaf> {
  pub(crate) fn partial_key(&self) -> &[u8] {
    let leaf = self.as_leaf_ref();
    let leaf_key_bytes = leaf.key_ref().as_ref();
    if self.prefix_len >= leaf_key_bytes.len() {
      &[]
    } else {
      &leaf_key_bytes[self.prefix_len..]
    }
  }
}

impl<'a, K: 'a, V: 'a> NodeRef<Mut<'a>, K, V, Internal> {
  pub(crate) unsafe fn set_child_at(
    &mut self,
    idx: usize,
    ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    self.as_internal_impl().set_child_at(idx, ptr)
  }
}

impl<'a, K: 'a, V: 'a> NodeRef<Mut<'a>, K, V, Leaf> {
  pub(crate) fn value_mut(self) -> &'a mut V {
    unsafe { (&mut *self.as_leaf_ptr()).value_mut() }
  }

  pub(crate) fn set_prefix_len(&mut self, new_prefix_len: usize) {
    assert!(self.prefix_len() >= new_prefix_len);
    unsafe {
      self.set_prefix_len(new_prefix_len);
    }
  }
}

impl<'a, K, V> NodeRef<Immut<'a>, K, V, Leaf> {
  pub(crate) fn value_ref(self) -> &'a V {
    unsafe { (&*self.as_leaf_ptr()).value_ref() }
  }
}

impl<K, V> NodeRef<Owned, K, V, Leaf> {
  pub(crate) fn from_new_leaf_node(prefix_len: usize, leaf: Box<LeafNode<K, V>>) -> Self {
    Self {
      inner: NonNull::from(Box::leak(leaf)).cast(),
      prefix_len,
      _marker: PhantomData,
    }
  }
}

impl<K, V> NodeRef<Owned, K, V, Internal> {
  pub(crate) fn from_new_internal_node<C>(
    prefix_len: usize,
    leaf: Box<InternalNode<C, K, V>>,
  ) -> Self {
    Self {
      inner: NonNull::from(Box::leak(leaf)).cast(),
      prefix_len,
      _marker: PhantomData,
    }
  }
}
