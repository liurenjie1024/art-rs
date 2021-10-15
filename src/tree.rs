use std::marker::PhantomData;
use crate::node::{NodeRef, RawNode};
use std::borrow::Borrow;

pub struct AdaptiveRadixTreeMap<K, V> {
    root: Option<NodeRef<V>>,
    _phantom: PhantomData<K>,
}

impl<K, V> AdaptiveRadixTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, _key: &K) -> Option<&V>
        where K: Borrow<[u8]>
    {
        todo!()
    }

    pub fn get_mut(&mut self, _key: &K) -> Option<&mut V>
        where K: Borrow<[u8]>
    {
        todo!()
    }

    pub fn insert(&mut self, _key: &K, _value: V) -> Option<V>
        where K: Borrow<[u8]>
    {
        todo!()
    }

    pub fn remove(&mut self, _key: &K) -> Option<V>
        where K: Borrow<[u8]>
    {
        todo!()
    }
}