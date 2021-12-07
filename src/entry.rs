use either::Either;

use crate::entry::Entry::{Occupied, Vacant};
use crate::map::ARTMap;
use crate::marker::{Leaf, Mut};
use crate::node::{Handle, NodeRef};
use crate::DormantMutRef;

pub enum Entry<'a, K, V> {
  Vacant(VacantEntry<'a, K, V>),
  Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
  key: K,
  /// Left when the tree is empty, and right when it's not.
  handle: Either<DormantMutRef<'a, ARTMap<K, V>>, Handle<Mut<'a>, V>>,
}

pub struct OccupiedEntry<'a, K, V> {
  key: K,
  node: NodeRef<Mut<'a>, V, Leaf>,
}

impl<'a, K, V> Entry<'a, K, V> {
  pub(crate) fn new_vacant(
    key: K,
    handle: Either<DormantMutRef<'a, ARTMap<K, V>>, Handle<Mut<'a>, V>>,
  ) -> Self {
    Entry::Vacant(VacantEntry { key, handle })
  }

  pub(crate) fn new_occupied(key: K, node: NodeRef<Mut<'a>, V, Leaf>) -> Self {
    Entry::Occupied(OccupiedEntry { key, node })
  }
}

impl<'a, K: AsRef<[u8]>, V> Entry<'a, K, V> {
  pub fn or_insert(self, _default: V) -> &'a mut V {
    todo!()
  }

  pub fn or_insert_with<F: FnOnce() -> V>(self, _default: F) -> &'a mut V {
    todo!()
  }

  pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, _default: F) -> &'a mut V {
    todo!()
  }

  pub fn key(&self) -> &K {
    match self {
      Occupied(ref entry) => &entry.key,
      Vacant(ref entry) => &entry.key,
    }
  }

  pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
    match self {
      Entry::Occupied(mut entry) => {
        f(entry.node.as_leaf_mut().value_mut());
        Entry::Occupied(entry)
      }
      Entry::Vacant(e) => Entry::Vacant(e),
    }
  }
}

impl<'a, K: AsRef<u8>, V: Default> Entry<'a, K, V> {
  pub fn or_default(self) -> &'a mut V {
    todo!()
  }
}

impl<'a, K: AsRef<[u8]>, V> VacantEntry<'a, K, V> {
  pub fn key(&self) -> &K {
    &self.key
  }

  pub fn into_key(self) -> K {
    self.key
  }

  // pub fn insert(self, value: V) -> &'a mut V {
  //   match self.handle {
  //     Either::Left(tree_ref) => {
  //       // An empty tree, just create a node and modify root
  //       let mut root= NodeRef::from_new_leaf_node(LeafNode::new(0, self.key.as_ref(), value));
  //       // SAFETY: `DormantMutRef` only appears when tree is empty
  //       let mut ptr = root.as_leaf_mut().value_ptr();
  //       unsafe {
  //         tree_ref.awaken().init_root(root.forget_type());
  //         ptr.as_mut()
  //       }
  //     },
  //     Either::Right(handle) => {
  //       handle.insert_node()
  //     }
  //   }
  // }
  pub fn insert(self, _value: V) -> &'a mut V {
    todo!()
  }
}
