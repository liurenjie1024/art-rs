// use crate::entry::Entry::{Occupied, Vacant};
// use crate::node::{Handle, LeafNodeRef, NodeRef};

use crate::map::ARTMap;
use crate::marker::Mut;
use crate::node::{Handle, LeafNodeRef};
use crate::DormantMutRef;
use either::Either;

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
  node: LeafNodeRef<Mut<'a>, V>,
}

impl<'a, K, V> Entry<'a, K, V> {
  pub(crate) fn new_vacant(
    key: K,
    handle: Either<DormantMutRef<'a, ARTMap<K, V>>, Handle<Mut<'a>, V>>,
  ) -> Self {
    Entry::Vacant(VacantEntry { key, handle })
  }

  pub(crate) fn new_occupied(key: K, node: LeafNodeRef<Mut<'a>, V>) -> Self {
    Entry::Occupied(OccupiedEntry { key, node })
  }
}
//
// impl<'a, K: AsRef<[u8]>, V> Entry<'a, K, V> {
//   pub fn or_insert(self, default: V) -> &'a mut V {
//     todo!()
//   }
//
//   pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
//     todo!()
//   }
//
//   pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
//     todo!()
//   }
//
//   pub fn key(&self) -> &K {
//     match self {
//       Occupied(ref entry) => &entry.key,
//       Vacant(ref entry) => &entry.key,
//     }
//   }
//
//   pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
//     todo!()
//   }
// }
//
// impl<'a, K: AsRef<u8>, V: Default> Entry<'a, K, V> {
//   pub fn or_default(self) -> &'a mut V {
//     todo!()
//   }
// }
//
// impl<'a, K: AsRef<[u8]>, V> VacantEntry<'a, K, V> {
//   pub fn key(&self) -> &K {
//     &self.key
//   }
//
//   pub fn into_key(self) -> K {
//     self.key
//   }
//
//   pub fn insert(self, value: V) -> &'a mut V {
//     todo!()
//   }
// }
