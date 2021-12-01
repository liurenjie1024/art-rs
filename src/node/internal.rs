use crate::common_len;
use crate::marker::{Immut, Mut, Owned};
use crate::node::leaf::LeafNodeRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node4::Node4Children;
use crate::node::node48::Node48Children;
use crate::node::PartialKey::FixSized;
use crate::node::{BoxedLeafNode, BoxedNode, Handle, LeafNode, NodeBase, NodeRef, NodeType};
use either::Either;
use std::marker::PhantomData;
use std::ptr::NonNull;

const MAX_PREFIX_LEN: usize = 16;

/// This pointer doesn't have to actually point to a `Node4`, but also possible to point to
/// internal nodes with other children type. We can infer actual node type from `node_type` in
/// `node_base`.
pub(crate) type BoxedInternalNode<V> = NonNull<InternalNodeBase<V>>;

#[derive(Default)]
pub(crate) struct Fixed {
  partial_prefix: [u8; MAX_PREFIX_LEN],
  partial_prefix_len: usize,
}

pub(crate) enum PartialKey {
  FixSized(Fixed),
  VarSized(Vec<u8>),
}

pub(crate) struct InternalNodeBase<V> {
  node_base: NodeBase<V>,
  partial_key: PartialKey,
  leaf: Option<BoxedLeafNode<V>>,
  children_count: u8,
}

#[repr(C)]
pub(crate) struct InternalNode<C, V> {
  base: InternalNodeBase<V>,
  children: C,
}

pub(crate) trait Children<V>: Default {
  const NODE_TYPE: NodeType;

  fn insert(&mut self, k: u8, node: BoxedNode<V>) -> Option<BoxedNode<V>>;
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

pub(crate) struct InternalNodeRef<BorrowType, V> {
  inner: BoxedInternalNode<V>,
  _marker: PhantomData<BorrowType>,
}

impl<BorrowType, V> InternalNodeRef<BorrowType, V> {
  pub(crate) fn new(inner: BoxedInternalNode<V>) -> Self {
    Self {
      inner,
      _marker: PhantomData,
    }
  }
}

impl<'a, V> Clone for InternalNodeRef<Immut<'a>, V> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner,
      _marker: PhantomData,
    }
  }
}

impl<'a, V> Copy for InternalNodeRef<Immut<'a>, V> {}

impl<BorrowType, V> InternalNodeRef<BorrowType, V> {
  pub(crate) fn from_new_node<C: Children<V>>(node: Box<InternalNode<C, V>>) -> Self {
    Self {
      // SAFETY: `Box` guarantee that it's nonnull.
      inner: unsafe { NonNull::new_unchecked(Box::into_raw(node)).cast() },
      _marker: PhantomData,
    }
  }

  pub(crate) unsafe fn from(node_ref: NodeRef<BorrowType, V>) -> Self {
    Self {
      inner: node_ref.inner.cast(),
      _marker: PhantomData,
    }
  }

  pub(crate) unsafe fn to_ptr(self) -> BoxedInternalNode<V> {
    self.inner
  }

  #[inline(always)]
  pub(crate) fn inner(&self) -> &InternalNodeBase<V> {
    unsafe { self.inner.as_ref() }
  }

  #[inline(always)]
  pub(crate) fn inner_mut(&mut self) -> &mut InternalNodeBase<V> {
    unsafe { self.inner.as_mut() }
  }

  pub(crate) fn reborrow(&self) -> InternalNodeRef<Immut<'_>, V> {
    InternalNodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }

  pub(crate) fn find_child(self, _k: u8) -> Either<Handle<BorrowType, V>, Self> {
    todo!()
  }

  pub(crate) fn get_leaf(&self) -> Option<LeafNodeRef<BorrowType, V>> {
    self
      .inner()
      .leaf
      .map(|ptr| unsafe { LeafNodeRef::<BorrowType, V>::from_raw_ptr(ptr) })
  }

  pub(crate) fn child_at(self, _idx: usize) -> NodeRef<BorrowType, V> {
    todo!()
  }
}

pub(crate) type InternalNode4<V> = InternalNode<Node4Children<V>, V>;
pub(crate) type InternalNode16<V> = InternalNode<Node16Children<V>, V>;
pub(crate) type InternalNode48<V> = InternalNode<Node48Children<V>, V>;
pub(crate) type InternalNode256<V> = InternalNode<Node256Children<V>, V>;

impl<V> InternalNodeBase<V> {
  /// Creates an boxed internal node.
  ///
  /// # Safety
  ///
  /// A valid internal node should have at least one child, and this method doesn't enforce this
  /// guarantee.
  unsafe fn new(node_type: NodeType, prefix_len: usize) -> Self {
    debug_assert!(node_type.is_internal());
    Self {
      node_base: NodeBase::new(node_type, prefix_len),
      partial_key: PartialKey::default(),
      leaf: None,
      children_count: 0,
    }
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    self.partial_key.as_slice()
  }

  pub(crate) fn prefix_ken(&self) -> usize {
    self.node_base.prefix_len
  }

  pub(crate) fn set_partial_key(&mut self, partial_key: &[u8]) {
    self.partial_key.update(partial_key);
  }

  pub(crate) fn set_leaf(&mut self, leaf_node: Box<LeafNode<V>>) {
    // SAFETY: `Box` guarantee it's not null.
    unsafe {
      self.leaf = Some(BoxedLeafNode::new_unchecked(Box::into_raw(leaf_node)));
    }
  }

  pub(crate) unsafe fn insert_child(
    &mut self,
    k: u8,
    node_ptr: BoxedNode<V>,
  ) -> Option<BoxedNode<V>> {
    todo!()
  }
}

impl<C: Children<V>, V> InternalNode<C, V> {
  pub(crate) unsafe fn new(prefix_len: usize) -> Box<Self> {
    Box::new(Self {
      base: InternalNodeBase::new(C::NODE_TYPE, prefix_len),
      children: C::default(),
    })
  }

  pub(crate) fn base(&self) -> &InternalNodeBase<V> {
    &self.base
  }

  pub(crate) fn base_mut(&mut self) -> &mut InternalNodeBase<V> {
    &mut self.base
  }

  /// Insert node with k and return previous node pointer.
  ///
  /// # Safety
  ///
  /// This method accepts a raw pointer and owns it afterwards. If a child node with same key
  /// already exists, it's returned and the caller has its ownership.
  pub(crate) unsafe fn insert_child(
    &mut self,
    k: u8,
    node_ptr: BoxedNode<V>,
  ) -> Option<BoxedNode<V>> {
    self.children.insert(k, node_ptr)
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
