use crate::common_len;
use crate::marker::{Immut, Mut};
use crate::node::leaf::LeafNodeRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node4::Node4Children;
use crate::node::node48::Node48Children;
use crate::node::{BoxedLeafNode, BoxedNode, Handle, NodeBase, NodeRef, NodeType};
use either::Either;
use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::node::PartialKey::FixSized;

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

impl<'a, V> InternalNodeRef<Mut<'a>, V> {
}

impl<BorrowType, V> InternalNodeRef<BorrowType, V> {
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

  // #[inline(always)]
  // fn inner_mut(&mut self) -> &mut InternalNodeBase<V> {
  //   unsafe { self.inner.as_mut() }
  // }

  // pub(crate) fn partial_prefix(&self) -> &[u8] {}

  // pub(crate) fn find_lower_bound(self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
  //   match &self.inner().partial_key {
  //     PartialKey::Prefix(data) => self.lower_bound_with_partial_prefix(data, arg),
  //     PartialKey::Leaf(leaf) => self.lower_bound_with_leaf(*leaf, arg),
  //   }
  // }

  pub(crate) fn reborrow(&self) -> InternalNodeRef<Immut<'_>, V> {
    InternalNodeRef {
      inner: self.inner,
      _marker: PhantomData,
    }
  }

  // /// Should only be called by entry
  // pub(crate) fn upsert(mut self, input_key: &[u8], depth: usize, value: V) -> NodeRef<V> {
  //   let input_partial_key = &input_key[depth..];
  //   let mut this_partial_key = &mut self.inner_mut().partial_key;
  //   let this_partial_prefix = this_partial_key.partial_key();
  //   let same_prefix_len = common_prefix_len(this_partial_prefix, input_partial_key);
  //
  //   if same_prefix_len < min(this_partial_prefix.len(), input_partial_key.len()) {
  //     // Create a new node as parent of these no
  //     let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
  //     new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);
  //
  //     // Insert self to new node
  //     {
  //       match this_partial_key {
  //         PartialKey::Prefix(data) => {
  //           data.set_data(&this_partial_prefix[(same_prefix_len + 1)..]);
  //         }
  //         PartialKey::Leaf {
  //           leaf_node: _left_node,
  //           offset
  //         } => {
  //           *offset += (same_prefix_len + 1);
  //         }
  //       }
  //       new_node4.upsert_child(this_partial_prefix[same_prefix_len], self);
  //     }
  //
  //     // Insert input key/value
  //     {
  //       let new_leaf_node = LeafNodeRef::<V>::with_data(input_key, value);
  //       new_node4.upsert_child(this_partial_key[same_prefix_len], new_leaf_node);
  //     }
  //
  //     new_node4.into()
  //   } else {
  //     if input_partial_key.len() > this_partial_prefix.len() {
  //       unreachable!("This should not happen!");
  //     } else if input_partial_key.len() == this_partial_prefix.len() {
  //       match this_partial_key {
  //         PartialKey::Prefix(_) => {
  //           *this_partial_key = PartialKey::Leaf {
  //             leaf_node: LeafNodeRef::<V>::with_data(input_key, value),
  //             offset: depth,
  //           };
  //         }
  //         PartialKey::Leaf {
  //           leaf_node: left_node,
  //           offset: _offset
  //         } => {
  //           left_node.inner_mut().set_value(value)
  //         }
  //       }
  //       self.into()
  //     } else {
  //       // We need to split node
  //       let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
  //
  //       // Insert self to new node
  //       {
  //         match this_partial_key {
  //           PartialKey::Prefix(data) => {
  //             data.set_data(&this_partial_prefix[(same_prefix_len + 1)..]);
  //           }
  //           PartialKey::Leaf {
  //             leaf_node: _left_node,
  //             offset
  //           } => {
  //             *offset += (same_prefix_len + 1);
  //           }
  //         }
  //         new_node4.upsert_child(this_partial_prefix[same_prefix_len], self);
  //       }
  //
  //       {
  //         new_node4.into().set_leaf(input_key, depth, value);
  //       }
  //
  //       new_node4.into()
  //     }
  //   }
  // }
  //
  // pub(crate) fn upsert_child(mut self, k: u8, child: NodeRef<V>) -> Option<NodeRef<V>> {
  //   todo!()
  // }
  //
  // fn lower_bound_with_partial_prefix(
  //   self,
  //   partial_prefix_data: &PrefixData,
  //   arg: SearchArgument,
  // ) -> SearchResult<LeafNodeRef<V>> {
  //   let partial_key = arg.partial_key();
  //   let partial_prefix = partial_prefix_data.partial_prefix();
  //   if partial_key.len() <= partial_prefix.len() {
  //     match partial_key.cmp(partial_prefix) {
  //       Ordering::Greater => GoUp,
  //       _ => Found(self.minimum_leaf()),
  //     }
  //   } else {
  //     let partial_key_of_prefix = &partial_key[0..partial_prefix.len()];
  //     match partial_key_of_prefix.cmp(partial_prefix) {
  //       Ordering::Less => Found(self.minimum_leaf()),
  //       Ordering::Equal => GoDown(arg.depth() + partial_prefix.len()),
  //       Ordering::Greater => GoUp,
  //     }
  //   }
  // }
  //
  // fn lower_bound_with_leaf(
  //   self,
  //   leaf_ref: LeafNodeRef<V>,
  //   arg: SearchArgument,
  // ) -> SearchResult<LeafNodeRef<V>> {
  //   let leaf_node = leaf_ref.inner();
  //   let partial_key = arg.partial_key();
  //   let partial_leaf_key = &leaf_node.key()[arg.depth()..];
  //   if partial_key.len() <= partial_leaf_key.len() {
  //     match partial_key.cmp(partial_leaf_key) {
  //       Ordering::Greater => GoUp,
  //       _ => Found(leaf_ref),
  //     }
  //   } else {
  //     let partial_key_of_leaf = &partial_key[0..partial_leaf_key.len()];
  //     match partial_key_of_leaf.cmp(partial_leaf_key) {
  //       Ordering::Greater => GoUp,
  //       Ordering::Equal => GoDown(arg.depth() + partial_leaf_key.len()),
  //       Ordering::Less => Found(leaf_ref),
  //     }
  //   }
  // }
  //
  //
  // pub(crate) fn set_prefix(mut self, new_prefix: &[u8]) {
  //   let mut prefix_data = PrefixData::default();
  //   prefix_data.set_data(new_prefix);
  //
  //   self.inner_mut().partial_key = PartialKey::Prefix(prefix_data);
  // }
  //
  // pub(crate) fn set_leaf_data(mut self, full_key: &[u8], depth: usize, value: V) {
  //   let leaf_node = LeafNodeRef::with_data(full_key, value);
  //   self.inner_mut().partial_key = PartialKey::Leaf {
  //     leaf_node: leaf_node,
  //     offset: depth,
  //   };
  // }
  //
  // pub(crate) fn partial_key(&self) -> &PartialKey<V> {
  //   self.inner().partial_key()
  // }
  //
  // pub(crate) fn child_at(self, idx: usize) -> NodeRef<BorrowType, V> {
  //   todo!()
  // }
  //
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
      node_base: NodeBase::new(C::NODE_TYPE, prefix_len),
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

  pub(crate) fn set_leaf(&mut self, )
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
  pub(crate) unsafe fn insert_child(&mut self, k: u8, node_ptr: BoxedNode<V>) -> Option<BoxedNode<V>> {
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
      },
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
