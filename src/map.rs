use crate::node::{NodeRef, Root};
use std::marker::PhantomData;

pub struct ARTMap<K, V> {
  root: Option<Root<V>>,
  _phantom: PhantomData<K>,
}

impl<K, V> ARTMap<K, V> {
  pub fn new() -> Self {
    Self {
      root: None,
      _phantom: PhantomData,
    }
  }

  pub fn get(&self, key: &K) -> Option<&V>
  where
    K: AsRef<[u8]>,
  {
    self
      .root
      .as_ref()?
      .reborrow()
      .search_tree(key.as_ref())
      .map(|leaf| leaf.into_value_ref())
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
  where
    K: AsRef<[u8]>,
  {
    self
      .root
      .as_mut()?
      .borrow_mut()
      .search_tree(key.as_ref())
      .map(|mut leaf| leaf.into_value_mut())
  }

  pub fn insert(&mut self, _key: &K, _value: V) -> Option<V>
  where
    K: AsRef<[u8]>,
  {
    todo!()
  }

  pub fn remove(&mut self, _key: &K) -> Option<V>
  where
    K: AsRef<[u8]>,
  {
    todo!()
  }
}
