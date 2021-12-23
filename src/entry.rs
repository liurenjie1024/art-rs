use either::Either;
use std::mem;
use std::ptr::NonNull;

use crate::entry::Entry::{Occupied, Vacant};

use crate::marker::{InternalOrLeaf, Leaf, Mut};
use crate::node::{Handle, LeafNode, NodeRef};


pub enum Entry<'a, K, V> {
  Vacant(VacantEntry<'a, K, V>),
  Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
  pub(crate) key: K,
  /// When map is empty, it should be a pointer to new node.
  /// When the map is not empty, it's a node ref.
  pub(crate) node: Either<Handle<K, V>, NodeRef<Mut<'a>, K, V, InternalOrLeaf>>,
}

pub struct OccupiedEntry<'a, K, V> {
  pub(crate) node: NodeRef<Mut<'a>, K, V, Leaf>,
}

impl<'a, K, V> Entry<'a, K, V> {
  pub(crate) fn new_vacant(
    key: K,
    node: Either<Handle<K, V>, NodeRef<Mut<'a>, K, V, InternalOrLeaf>>,
  ) -> Self {
    Entry::Vacant(VacantEntry { key, node })
  }

  pub(crate) fn new_occupied(node: NodeRef<Mut<'a>, K, V, Leaf>) -> Self {
    Entry::Occupied(OccupiedEntry { node })
  }
}

impl<'a, K: AsRef<[u8]> + 'a, V: 'a> Entry<'a, K, V> {
  pub fn or_insert(self, value: V) -> &'a mut V {
    match self {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => entry.insert(value),
    }
  }

  pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
    match self {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => entry.insert(f()),
    }
  }

  pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, f: F) -> &'a mut V {
    match self {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => {
        let new_value = f(entry.key());
        entry.insert(new_value)
      }
    }
  }

  pub fn key(&self) -> &K {
    match self {
      Occupied(ref entry) => &entry.key(),
      Vacant(ref entry) => &entry.key(),
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

impl<'a, K: AsRef<[u8]> + 'a, V: 'a> VacantEntry<'a, K, V> {
  pub fn key(&self) -> &K {
    &self.key
  }

  pub fn into_key(self) -> K {
    self.key
  }

  pub fn insert(self, value: V) -> &'a mut V {
    match self.node {
      Either::Left(mut handle) => {
        let new_leaf = LeafNode::new(self.key, value);
        let mut leaf_node = NonNull::from(Box::leak(new_leaf));
        unsafe {
          std::ptr::write(handle.as_mut(), Some(leaf_node.cast()));
          leaf_node.as_mut().value_mut()
        }
      }
      Either::Right(node) => {
        unsafe {
          node.insert_node(self.key, value).as_mut()
        }
      }
    }
  }
}

impl<'a, K: AsRef<[u8]>, V> OccupiedEntry<'a, K, V> {
  pub fn key(&self) -> &K {
    self.node.as_leaf_ref().key_ref()
  }

  pub fn get(&self) -> &V {
    self.node.as_leaf_ref().value_ref()
  }

  pub fn get_mut(&mut self) -> &mut V {
    self.node.as_leaf_mut().value_mut()
  }

  pub fn into_mut(mut self) -> &'a mut V {
    unsafe { self.node.as_leaf_mut().value_ptr().as_mut() }
  }

  pub fn insert(&mut self, value: V) -> V {
    mem::replace(self.get_mut(), value)
  }

  pub fn remove_kv(mut self) -> (K, V) {
    // TODO: This is not enough, parent should change
    unsafe {
      self.node.replace_holder(None);
    }
    self.node.into_kv()
  }
}
