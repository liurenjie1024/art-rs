use std::mem;
use either::Either;

use crate::entry::Entry::{Occupied, Vacant};
use crate::map::ARTMap;
use crate::marker::{Leaf, Mut};
use crate::node::{Handle, LeafNode, NodeRef};
use crate::DormantMutRef;

pub enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
    key: K,
    /// Left when the tree is empty, and right when it's not.
    handle: Either<DormantMutRef<'a, ARTMap<K, V>>, Handle<Mut<'a>, K, V>>,
}

pub struct OccupiedEntry<'a, K, V> {
    key: K,
    node: NodeRef<Mut<'a>, K, V, Leaf>,
}

impl<'a, K, V> Entry<'a, K, V> {
    pub(crate) fn new_vacant(
        key: K,
        handle: Either<DormantMutRef<'a, ARTMap<K, V>>, Handle<Mut<'a>, K, V>>,
    ) -> Self {
        Entry::Vacant(VacantEntry { key, handle })
    }

    pub(crate) fn new_occupied(key: K, node: NodeRef<Mut<'a>, K, V, Leaf>) -> Self {
        Entry::Occupied(OccupiedEntry { key, node })
    }
}

impl<'a, K: AsRef<[u8]>, V> Entry<'a, K, V> {
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(value)
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(f())
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

    pub fn insert(self, value: V) -> &'a mut V {
        match self.handle {
            Either::Left(tree_ref) => {
                // An empty tree, just create a node and modify root
                let mut root = NodeRef::from_new_leaf_node(LeafNode::new(self.key.as_ref(), value));
                // SAFETY: `DormantMutRef` only appears when tree is empty
                let mut ptr = root.as_leaf_mut().value_ptr();
                unsafe {
                    tree_ref.awaken().init_root(root.forget_type());
                    ptr.as_mut()
                }
            }
            Either::Right(handle) => {
                unsafe { handle.insert_node(self.key.as_ref(), value).as_mut() }
            }
        }
    }
}

impl<'a, K: AsRef<[u8]>, V> OccupiedEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        &self.key
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
}


