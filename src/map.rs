use std::ptr::NonNull;
use crate::entry::Entry;
use crate::node::{BoxedNode, NodeRef, Root};
use crate::{DormantMutRef, marker};
use either::Either;
use crate::marker::{Immut, InternalOrLeaf, Mut};
use crate::search::SearchResult;

pub struct ARTMap<K, V> {
  root: Option<BoxedNode<K, V>>,
}

impl<K, V> ARTMap<K, V> {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn get(&self, key: &K) -> Option<&V>
    where
        K: AsRef<[u8]>,
  {
    match self.root_node_ref()?.search_tree(key) {
      SearchResult::Found(leaf) => Some(leaf.value_ref()),
      SearchResult::NotFound(_) => None,
      _ => unreachable!()
    }
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: AsRef<[u8]>,
  {
    match self.root_node_mut()?.search_tree(key) {
      SearchResult::Found(leaf) => Some(leaf.value_mut()),
      SearchResult::NotFound(_) => None,
      _ => unreachable!()
    }
  }

  pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: AsRef<[u8]>,
  {
    let (map, dormant_ref) = DormantMutRef::new(self);
    match map.root_node_mut() {
      Some(node) => match node.search_tree(&key) {
        SearchResult::Found(leaf) => Entry::new_occupied(key, leaf),
        SearchResult::NotFound(node) => Entry::new_vacant(key, Either::Right(node)),
        _ => unreachable!()
      },
      None => Entry::new_vacant(key, Either::Left( NonNull::from(unsafe { &mut dormant_ref.awaken().root }) ))
    }
  }

  pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: AsRef<[u8]>,
  {
    match self.entry(key) {
      Entry::Occupied(mut entry) => Some(entry.insert(value)),
      Entry::Vacant(entry) => {
        entry.insert(value);
        None
      }
    }
  }

  pub fn remove(&mut self, _key: &K) -> Option<V>
    where
        K: AsRef<[u8]>,
  {
    todo!()
  }
}

impl<K, V> ARTMap<K, V> {
  fn root_node_ref(&self) -> Option<NodeRef<Immut<'_>, K, V, InternalOrLeaf>> {
    self.root.map(|ptr| NodeRef::root_node_ref(ptr, NonNull::from(&self.root)))
  }

  fn root_node_mut(&mut self) -> Option<NodeRef<Mut<'_>, K, V, InternalOrLeaf>> {
    self.root.map(|ptr| NodeRef::root_node_ref(ptr, NonNull::from(&self.root)))
  }
}
