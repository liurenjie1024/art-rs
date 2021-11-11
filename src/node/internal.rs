use crate::node::internal::SearchResult::{Found, GoDown, GoUp};
use crate::node::leaf::LeafNodeRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node4::Node4Children;
use crate::node::node48::Node48Children;
use crate::node::NodeKind::Leaf;
use crate::node::{LeafNode, NodeBase, NodeRef, NodeType};
use crate::search::{SearchArgument, SearchResult};
use std::cmp::{min, Ordering};
use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::common_prefix_len;

const MAX_PREFIX_LEN: usize = 16;

#[derive(Default)]
struct PartialPrefixData {
  partial_prefix: [u8; MAX_PREFIX_LEN],
  partial_prefix_len: usize,
}

enum PartialKey<V> {
  Prefix(PartialPrefixData),
  Leaf {
    leaf_node: LeafNodeRef<V>,
    offset: usize,
  },
}

#[repr(C)]
pub(crate) struct InternalNodeBase<V> {
  node_base: NodeBase<V>,
  partial_key: PartialKey<V>,
  children_count: u8,
}

#[repr(C)]
pub(crate) struct InternalNode<C, V> {
  base: InternalNodeBase<C>,
  children: C,
  marker: PhantomData<V>,
}

#[repr(C)]
pub(crate) struct InternalNodeRef<V> {
  inner: NonNull<InternalNodeBase<V>>,
}

trait ChildrenContainer: Default {
  const NODE_TYPE: NodeType;
}

impl PartialPrefixData {
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

impl<V> InternalNodeRef<V> {
  pub(crate) fn new(inner: NonNull<InternalNodeBase<V>>) -> Self {
    Self { inner }
  }
}

impl<V> Clone for InternalNodeRef<V> {
  fn clone(&self) -> Self {
    Self { inner: self.inner }
  }
}

impl<V> Copy for InternalNodeRef<V> {}

impl<V> InternalNodeRef<V> {
  #[inline(always)]
  fn inner(&self) -> &InternalNodeBase<V> {
    unsafe { self.inner.as_ref() }
  }

  #[inline(always)]
  fn inner_mut(&mut self) -> &mut InternalNodeBase<V> {
    unsafe { self.inner.as_mut() }
  }

  pub(crate) fn find_lower_bound(self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
    match &self.inner().partial_key {
      PartialKey::Prefix(data) => self.lower_bound_with_partial_prefix(data, arg),
      PartialKey::Leaf(leaf) => self.lower_bound_with_leaf(*leaf, arg),
    }
  }

  pub(crate) fn find_child(self, _k: u8) -> Option<NodeRef<V>> {
    todo!()
  }

  pub(crate) fn find_next_child(self, _k: u8) -> Option<NodeRef<V>> {
    todo!()
  }

  /// Should only be called by entry
  pub(crate) fn upsert(mut self, input_key: &[u8], depth: usize, value: V) -> NodeRef<V> {
    let input_partial_key = &input_key[depth..];
    let mut this_partial_key = &mut self.inner_mut().partial_key;
    let this_partial_prefix = this_partial_key.partial_key();
    let same_prefix_len = common_prefix_len(this_partial_prefix, input_partial_key);

    if same_prefix_len < min(this_partial_prefix.len(), input_partial_key.len()) {
      // Create a new node as parent of these no
      let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
      new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);

      // Insert self to new node
      {
        match this_partial_key {
          PartialKey::Prefix(data) => {
            data.set_data(&this_partial_prefix[(same_prefix_len + 1)..]);
          }
          PartialKey::Leaf {
            leaf_node: _left_node,
            offset
          } => {
            *offset += (same_prefix_len + 1);
          }
        }
        new_node4.upsert_child(this_partial_prefix[same_prefix_len], self);
      }

      // Insert input key/value
      {
        let new_leaf_node = LeafNodeRef::<V>::with_data(input_key, value);
        new_node4.upsert_child(this_partial_key[same_prefix_len], new_leaf_node);
      }

      new_node4.into()
    } else {
      if input_partial_key.len() > this_partial_prefix.len() {
        unreachable!("This should not happen!");
      } else if input_partial_key.len() == this_partial_prefix.len() {
        match this_partial_key {
          PartialKey::Prefix(_) => {
            *this_partial_key = PartialKey::Leaf {
              leaf_node: LeafNodeRef::<V>::with_data(input_key, value),
              offset: depth,
            };
          }
          PartialKey::Leaf {
            leaf_node: left_node,
            offset: _offset
          } => {
            left_node.inner_mut().set_value(value)
          }
        }
        self.into()
      } else {
        // We need to split node
        let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();

        // Insert self to new node
        {
          match this_partial_key {
            PartialKey::Prefix(data) => {
              data.set_data(&this_partial_prefix[(same_prefix_len + 1)..]);
            }
            PartialKey::Leaf {
              leaf_node: _left_node,
              offset
            } => {
              *offset += (same_prefix_len + 1);
            }
          }
          new_node4.upsert_child(this_partial_prefix[same_prefix_len], self);
        }

        {
          new_node4.into().set_leaf(input_key, depth, value);
        }

        new_node4.into()
      }
    }
  }

  pub(crate) fn upsert_child(mut self, k: u8, child: NodeRef<V>) -> Option<NodeRef<V>> {
    todo!()
  }

  fn lower_bound_with_partial_prefix(
    self,
    partial_prefix_data: &PartialPrefixData,
    arg: SearchArgument,
  ) -> SearchResult<LeafNodeRef<V>> {
    let partial_key = arg.partial_key();
    let partial_prefix = partial_prefix_data.partial_prefix();
    if partial_key.len() <= partial_prefix.len() {
      match partial_key.cmp(partial_prefix) {
        Ordering::Greater => GoUp,
        _ => Found(self.minimum_leaf()),
      }
    } else {
      let partial_key_of_prefix = &partial_key[0..partial_prefix.len()];
      match partial_key_of_prefix.cmp(partial_prefix) {
        Ordering::Less => Found(self.minimum_leaf()),
        Ordering::Equal => GoDown(arg.depth() + partial_prefix.len()),
        Ordering::Greater => GoUp,
      }
    }
  }

  fn lower_bound_with_leaf(
    self,
    leaf_ref: LeafNodeRef<V>,
    arg: SearchArgument,
  ) -> SearchResult<LeafNodeRef<V>> {
    let leaf_node = leaf_ref.inner();
    let partial_key = arg.partial_key();
    let partial_leaf_key = &leaf_node.key()[arg.depth()..];
    if partial_key.len() <= partial_leaf_key.len() {
      match partial_key.cmp(partial_leaf_key) {
        Ordering::Greater => GoUp,
        _ => Found(leaf_ref),
      }
    } else {
      let partial_key_of_leaf = &partial_key[0..partial_leaf_key.len()];
      match partial_key_of_leaf.cmp(partial_leaf_key) {
        Ordering::Greater => GoUp,
        Ordering::Equal => GoDown(arg.depth() + partial_leaf_key.len()),
        Ordering::Less => Found(leaf_ref),
      }
    }
  }

  fn minimum_leaf(self) -> LeafNodeRef<V> {
    todo!()
  }

  pub(crate) fn set_prefix(mut self, new_prefix: &[u8]) {
    let mut prefix_data = PartialPrefixData::default();
    prefix_data.set_data(new_prefix);

    self.inner_mut().partial_key = PartialKey::Prefix(prefix_data);
  }

  pub(crate) fn set_leaf_data(mut self, full_key: &[u8], depth: usize, value: V) {
    let leaf_node = LeafNodeRef::with_data(full_key, value);
    self.inner_mut().partial_key = PartialKey::Leaf {
      leaf_node: leaf_node,
      offset: depth,
    };
  }
}

pub(crate) type InternalNode4<V> = InternalNode<Node4Children<V>, V>;
pub(crate) type InternalNode16<V> = InternalNode<Node16Children<V>, V>;
pub(crate) type InternalNode48<V> = InternalNode<Node48Children<V>, V>;
pub(crate) type InternalNode256<V> = InternalNode<Node256Children<V>, V>;

impl<V> InternalNodeBase<V> {
  pub(crate) fn new(node_type: NodeType) -> Self {
    assert!(node_type.is_internal());
    Self {
      node_base: NodeBase::new(node_type),
      partial_key: PartialKey::Prefix(PartialPrefixData::default()),
      children_count: 0,
    }
  }

  pub(crate) fn set_leaf(&mut self, leaf_node: LeafNodeRef<V>) {
    self.partial_key = PartialKey::Leaf {
      offset: 0,
      leaf_node
    };
  }
}

impl<C: ChildrenContainer, V> Default for InternalNode<C, V> {
  fn default() -> Self {
    Self {
      base: InternalNodeBase::new(C::NODE_TYPE),
      children: C::default(),
      marker: PhantomData,
    }
  }
}

impl<V> InternalNodeRef<V> {
  pub(crate) fn new<C: ChildrenContainer>() -> Self {
    let new_node = Box::new(InternalNode::<C, V>::default());
    let inner = NonNull::from(Box::leak(new_node)).cast();

    Self { inner }
  }

}

impl<V> Into<NodeRef<V>> for InternalNodeRef<V> {
  fn into(self) -> NodeRef<V> {
    NodeRef::<V> {
      inner: self.inner.cast(),
    }
  }
}

impl<V> PartialKey<V> {
  fn common_prefix_len(&self, key: &[u8]) -> usize {
    common_prefix_len(self.partial_key(), key)
  }

  fn partial_key(&self) -> &[u8] {
    match self {
      PartialKey::Prefix(prefix) => prefix.partial_prefix(),
      PartialKey::Leaf {
        leaf_node: left_node,
        offset
      } => left_node.inner().key()[offset..]
    }
  }
}
