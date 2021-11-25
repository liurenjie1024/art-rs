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

  pub fn get(&self, _key: &K) -> Option<&V>
  where
    K: AsRef<[u8]>,
  {
    todo!()
  }

  pub fn get_mut(&mut self, _key: &K) -> Option<&mut V>
  where
    K: AsRef<[u8]>,
  {
    todo!()
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
