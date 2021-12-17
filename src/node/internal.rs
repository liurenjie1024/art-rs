use crate::common_len;
use crate::marker::{Internal, InternalOrLeaf, Leaf, Mut, Owned};
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node4::Node4Children;
use crate::node::node48::Node48Children;
use crate::node::Handle;
use crate::node::InternalNodeImpl::{Node16, Node256, Node4, Node48};
use crate::node::NodeRef;
use crate::node::PartialKey::FixSized;
use crate::node::{BoxedNode, LeafNode, NodeBase, NodeType};
use std::marker::PhantomData;
use std::mem::swap;
use std::ptr::NonNull;

const MAX_PREFIX_LEN: usize = 16;

#[derive(Default)]
pub(crate) struct Fixed {
  partial_prefix: [u8; MAX_PREFIX_LEN],
  partial_prefix_len: usize,
}

pub(crate) enum PartialKey {
  FixSized(Fixed),
  VarSized(Vec<u8>),
}

pub(crate) struct InternalNodeBase<K, V> {
  node_base: NodeBase<K, V>,
  partial_key: PartialKey,
  leaf: Option<NonNull<LeafNode<K, V>>>,
  children_count: u8,
}

#[repr(C)]
pub(crate) struct InternalNode<C, K, V> {
  base: InternalNodeBase<K, V>,
  children: C,
}

pub(crate) trait Children<K, V>: Default {
  const NODE_TYPE: NodeType;

  unsafe fn set_child(&mut self, k: u8, node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>>;
  unsafe fn set_child_at(&mut self, idx: usize, node: BoxedNode<K, V>) -> Option<BoxedNode<K, V>>;
  fn child_at(&self, idx: usize) -> Option<BoxedNode<K, V>>;
}

impl Fixed {
  #[inline(always)]
  fn partial_prefix(&self) -> &[u8] {
    &self.partial_prefix[0..self.partial_prefix_len]
  }

  fn set_data(&mut self, new_data: &[u8]) {
    assert!(new_data.len() <= MAX_PREFIX_LEN);
    (&mut self.partial_prefix[0..new_data.len()]).copy_from_slice(new_data);
    self.partial_prefix_len = new_data.len();
  }
}

pub(crate) type InternalNode4<K, V> = InternalNode<Node4Children<K, V>, K, V>;
pub(crate) type InternalNode16<K, V> = InternalNode<Node16Children<K, V>, K, V>;
pub(crate) type InternalNode48<K, V> = InternalNode<Node48Children<K, V>, K, V>;
pub(crate) type InternalNode256<K, V> = InternalNode<Node256Children<K, V>, K, V>;

pub(crate) enum InternalNodeImpl<'a, K, V> {
  Node4(&'a InternalNode4<K, V>),
  Node16(&'a InternalNode16<K, V>),
  Node48(&'a InternalNode48<K, V>),
  Node256(&'a InternalNode256<K, V>),
}

impl<K, V> InternalNodeBase<K, V> {
  /// Creates an boxed internal node.
  ///
  /// # Safety
  ///
  /// A valid internal node should have at least one child, and this method doesn't enforce this
  /// guarantee.
  unsafe fn new(node_type: NodeType) -> Self {
    debug_assert!(node_type.is_internal());
    Self {
      node_base: NodeBase::new(node_type),
      partial_key: PartialKey::default(),
      leaf: None,
      children_count: 0,
    }
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    self.partial_key.as_slice()
  }

  pub(crate) fn set_partial_key(&mut self, partial_key: &[u8]) {
    self.partial_key.update(partial_key);
  }

  pub(crate) unsafe fn set_leaf(
    &mut self,
    leaf_node: NonNull<LeafNode<K, V>>,
  ) -> Option<NonNull<LeafNode<K, V>>> {
    let mut ret = Some(leaf_node.cast());
    swap(&mut self.leaf, &mut ret);
    ret
  }

  pub(crate) fn get_leaf(&self) -> Option<NonNull<LeafNode<K, V>>> {
    self.leaf
  }
}

impl<K, V, C: Children<K, V>> InternalNode<C, K, V> {
  pub(crate) unsafe fn new() -> Box<Self> {
    Box::new(Self {
      base: InternalNodeBase::new(C::NODE_TYPE),
      children: C::default(),
    })
  }

  pub(crate) fn base(&self) -> &InternalNodeBase<K, V> {
    &self.base
  }

  pub(crate) fn base_mut(&mut self) -> &mut InternalNodeBase<K, V> {
    &mut self.base
  }

  /// Insert node with k and return previous node pointer.
  ///
  /// # Safety
  ///
  /// This method accepts a raw pointer and owns it afterwards. If a child node with same key
  /// already exists, it's returned and the caller has its ownership.
  pub(crate) unsafe fn set_child(
    &mut self,
    k: u8,
    node_ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    self.children.set_child(k, node_ptr)
  }

  pub(crate) unsafe fn set_child_at(
    &mut self,
    idx: usize,
    node_ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    self.children.set_child_at(idx, node_ptr)
  }

  pub(crate) fn child_at(&self, idx: usize) -> Option<BoxedNode<K, V>> {
    self.children.child_at(idx)
  }
}

impl PartialKey {
  fn common_prefix_len(&self, key: &[u8]) -> usize {
    common_len(self.as_slice(), key)
  }

  fn as_slice(&self) -> &[u8] {
    match self {
      PartialKey::FixSized(prefix) => prefix.partial_prefix(),
      PartialKey::VarSized(key) => key.as_slice(),
    }
  }

  pub(crate) fn len(&self) -> usize {
    self.as_slice().len()
  }

  fn update(&mut self, new_partial_key: &[u8]) {
    match self {
      PartialKey::FixSized(cur_key) => {
        if new_partial_key.len() > MAX_PREFIX_LEN {
          *self = PartialKey::VarSized(Vec::from(new_partial_key));
        } else {
          cur_key.update(new_partial_key);
        }
      }
      PartialKey::VarSized(cur_key) => {
        if new_partial_key.len() > MAX_PREFIX_LEN {
          cur_key.copy_from_slice(new_partial_key);
        } else {
          let fixed_key = FixSized(Fixed::new(new_partial_key));
          *self = fixed_key;
        }
      }
    }
  }
}

impl Default for PartialKey {
  fn default() -> Self {
    PartialKey::FixSized(Fixed::default())
  }
}

impl Fixed {
  fn new(slice: &[u8]) -> Self {
    debug_assert!(slice.len() < MAX_PREFIX_LEN);

    let mut ret = Self::default();
    ret.update(slice);

    ret
  }

  fn update(&mut self, slice: &[u8]) {
    debug_assert!(slice.len() < MAX_PREFIX_LEN);
    self.partial_prefix[0..slice.len()].copy_from_slice(slice);
    self.partial_prefix_len = slice.len();
  }
}

impl<'a, K, V> InternalNodeImpl<'a, K, V> {
  pub(crate) fn child_at(&self, idx: usize) -> Option<BoxedNode<K, V>> {
    match self {
      Node4(n) => n.child_at(idx),
      Node16(n) => n.child_at(idx),
      Node48(n) => n.child_at(idx),
      Node256(n) => n.child_at(idx),
    }
  }

  unsafe fn set_child_at(
    &mut self,
    idx: usize,
    node_ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    match self {
      Node4(n) => n.set_child_at(idx, node_ptr),
      Node16(n) => n.set_child_at(idx, node_ptr),
      Node48(n) => n.set_child_at(idx, node_ptr),
      Node256(n) => n.set_child_at(idx, node_ptr),
    }
  }
}

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, Internal> {
  fn as_internal_ref(&self) -> &InternalNodeBase<K, V> {
    debug_assert!(self.as_base_ref().node_type.is_internal());
    // SAFETY: This is internal node.
    unsafe { self.inner.cast().as_ref() }
  }

  fn as_internal_mut(&mut self) -> &mut InternalNodeBase<K, V> {
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
    let leaf_prefix_len = self.prefix_len + internal_ref.partial_key().len();
    internal_ref.get_leaf().map(|leaf_ptr| NodeRef {
      inner: leaf_ptr.cast(),
      prefix_len: leaf_prefix_len,
      _marker: PhantomData,
    })
  }

  pub(crate) fn child_at(&self, idx: usize) -> Option<NodeRef<BorrowType, K, V, InternalOrLeaf>> {
    let internal_ref = self.as_internal_impl();
    if let Some(child_ptr) = internal_ref.child_at(idx) {
      let child_prefix_len = self.prefix_len + self.as_internal_ref().partial_key.len() + 1;
      Some(NodeRef {
        inner: child_ptr,
        prefix_len: child_prefix_len,
        _marker: PhantomData,
      })
    } else {
      None
    }
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    self.as_internal_ref().partial_key()
  }
}

impl<'a, K: 'a, V: 'a> NodeRef<Mut<'a>, K, V, Internal> {
  pub(crate) unsafe fn insert_child(
    &mut self,
    _k: u8,
    _node_ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    todo!()
  }

  pub(crate) unsafe fn set_child_at(
    &mut self,
    idx: usize,
    ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    self.as_internal_impl().set_child_at(idx, ptr)
  }

  pub(crate) unsafe fn set_leaf(
    &mut self,
    ptr: NonNull<LeafNode<K, V>>,
  ) -> Option<NonNull<LeafNode<K, V>>> {
    self.as_internal_mut().set_leaf(ptr)
  }

  pub(crate) fn set_partial_key(&mut self, new_partial_key: &[u8]) {
    self.as_internal_mut().set_partial_key(new_partial_key)
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
