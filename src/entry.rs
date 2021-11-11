use crate::entry::Entry::{Occupied, Vacant};
use crate::node::{Edge, LeafNodeRef, NodeRef};

pub enum Entry<'a, K, V> {
  Vacant(VacantEntry<'a, K, V>),
  Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
  key: K,
  parent: Option<Edge<V>>,
  current: Option<NodeRef<V>>,
}

pub struct OccupiedEntry<'a, K, V> {
  key: K,
  current: LeafNodeRef<V>,
}

impl<'a, K: AsRef<[u8]>, V> Entry<'a, K, V> {
  pub fn or_insert(self, default: V) -> &'a mut V {
    todo!()
  }

  pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
    todo!()
  }

  pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
    todo!()
  }

  pub fn key(&self) -> &K {
    match self {
      Occupied(ref entry) => &entry.key,
      Vacant(ref entry) => &entry.key,
    }
  }

  pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
    todo!()
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

  pub fn insert(self, value: V) -> &'a mut V {
    todo!()
  }
}
