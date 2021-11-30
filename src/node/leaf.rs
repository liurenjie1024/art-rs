use crate::marker::{Immut, Mut};
use crate::node::{NodeBase, NodeRef, NodeType};
use std::marker::PhantomData;
use std::mem::swap;
use std::ptr::NonNull;

pub(crate) type BoxedLeafNode<V> = NonNull<LeafNode<V>>;

#[repr(C)]
pub(crate) struct LeafNode<V> {
  node_base: NodeBase<V>,
  key: Vec<u8>,
  value: V,
}

impl<V> LeafNode<V> {
  pub(crate) fn key(&self) -> &[u8] {
    &self.key
  }
}

pub(crate) struct LeafNodeRef<BorrowType, V> {
  inner: NonNull<LeafNode<V>>,
  _marker: PhantomData<BorrowType>,
}

impl<'a, V> LeafNodeRef<Immut<'a>, V> {
  pub(crate) fn inner(&self) -> &LeafNode<V> {
    unsafe { self.inner.as_ref() }
  }

  pub(crate) fn into_value_ref(self) -> &'a V {
    unsafe { &self.inner.as_ref().value }
  }
}

impl<'a, V> LeafNodeRef<Mut<'a>, V> {
  pub(crate) fn inner_mut(&mut self) -> &mut LeafNode<V> {
    unsafe { self.inner.as_mut() }
  }

  pub(crate) fn value_mut(&mut self) -> &'a mut V {
    unsafe { &mut self.inner.as_mut().value }
  }
}

impl<BorrowType, V> LeafNodeRef<BorrowType, V> {
  pub(crate) unsafe fn from_raw_ptr(ptr: BoxedLeafNode<V>) -> Self {
    Self {
      inner: ptr,
      _marker: PhantomData,
    }
  }

  pub(crate) unsafe fn from(node_ref: NodeRef<BorrowType, V>) -> Self {
    debug_assert!(node_ref.inner().node_type.is_leaf());

    Self {
      inner: node_ref.inner.cast(),
      _marker: PhantomData,
    }
  }

  pub(crate) unsafe fn to_ptr(self) -> BoxedLeafNode<V> {
    self.inner
  }

  /// Temporarily takes out an immutable reference of same node.
  pub(crate) fn reborrow(&self) -> LeafNodeRef<Immut<'_>, V> {
    LeafNodeRef::<Immut<'_>, V> {
      inner: self.inner,
      _marker: PhantomData,
    }
  }

  // pub(crate) fn is_lower_bound(self, arg: SearchArgument) -> bool {
  //   let partial_key = arg.partial_key();
  //   let partial_leaf_key = &self.inner().key()[arg.depth()..];
  //   if partial_key.len() <= partial_leaf_key.len() {
  //     partial_key <= partial_leaf_key
  //   } else {
  //     let partial_key_of_leaf = &partial_key[0..partial_leaf_key.len()];
  //     partial_key_of_leaf < partial_leaf_key
  //   }
  // }

  // pub(crate) fn update(mut self, key: &[u8], depth: usize, value: V) -> NodeRef<V> {
  //   let this_node = self.inner_mut();
  //   let this_partial_key = this_node.partial_key();
  //   let input_partial_key = &key[depth..];
  //
  //   let same_prefix_len = common_prefix_len(this_partial_key, input_partial_key);
  //   if same_prefix_len < min(this_partial_key.len(), input_partial_key.len()) {
  //     // Here we need to create a new node to hold them
  //     let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
  //     new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);
  //
  //     // Current leaf node just needs to adjust its `prefix_len` in parent.
  //     {
  //       this_node.prefix_len += same_prefix_len;
  //       new_node4.upsert_child(this_node.key[this_node.prefix_len], self);
  //     }
  //
  //     // Create a new leaf node for new value
  //     {
  //       let prefix_len = depth + same_prefix_len;
  //       let new_leaf_node = LeafNodeRef::with_data(key, prefix_len, value);
  //       let k = key[prefix_len];
  //       new_node4.upsert_child(k, new_leaf_node);
  //     }
  //
  //     new_node4.into()
  //   } else {
  //     if this_partial_key.len() == input_partial_key.len() {
  //       // Same key, just update value
  //       this_node.set_value(value);
  //       self.into()
  //     } else {
  //       // We need to create a new node to hold them
  //
  //       // Here we need to create a new node to hold them
  //       let mut new_node4 = InternalNodeRef::<V>::new::<Node4Children<V>>();
  //       new_node4.set_prefix(&this_partial_prefix[0..same_prefix_len]);
  //
  //       let new_prefix_len = this_node.prefix_len + same_prefix_len;
  //
  //       if this_partial_key.len() < input_partial_key.len() {
  //         this_node.prefix_len = new_prefix_len;
  //         new_node4.inner_mut().set_leaf(self);
  //
  //         let k = key[new_prefix_len];
  //         let new_leaf_node = LeafNodeRef::with_data(key, new_prefix_len + 1, value);
  //         new_node4.inner_mut().upsert_child(k, new_leaf_node);
  //       } else {
  //         let new_leaf_node = LeafNodeRef::with_data(key, new_prefix_len, value);
  //         new_node4.inner_mut().set_leaf(new_leaf_node);
  //
  //         let k = key[new_prefix_len];
  //         this_node.prefix_len = new_prefix_len + 1;
  //         new_node4.inner_mut().upsert_child(k, self);
  //       }
  //
  //       new_node4.into()
  //     }
  //   }
  // }
}

impl<V> LeafNode<V> {
  pub(crate) fn new(prefix_len: usize, key: &[u8], value: V) -> Box<Self> {
    Box::new(Self {
      node_base: NodeBase::new(NodeType::Leaf, prefix_len),
      key: Vec::from(key),
      value
    })
  }

  pub(crate) fn set_value(&mut self, mut value: V) -> V {
    swap(&mut self.value, &mut value);
    value
  }

  pub(crate) fn partial_key(&self) -> &[u8] {
    if self.prefix_len >= self.key.len() {
      return &[];
    }
    &self.key[self.prefix_len..]
  }
}
