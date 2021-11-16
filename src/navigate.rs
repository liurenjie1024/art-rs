use std::cmp::Ordering;
use crate::node::{BoxedLeafNode, Handle, InternalNodeRef, PartialKey};
use crate::node::LeafNodeRef;
use crate::node::{NodeKind, NodeRef, DEFAULT_TREE_DEPTH};

pub(crate) enum SearchResult<BorrowType, V> {
  GoUp,
  Found(LeafNodeRef<BorrowType, V>),
  GoDown(InternalNodeRef<BorrowType, V>),
}

#[derive(Copy, Clone)]
pub(crate) struct SearchArgument<'a> {
  key: &'a [u8],
  depth: usize,
}

impl<BorrowType, V> NodeRef<BorrowType, V> {
  /// Find first leaf node, whose keys is not less than input key.
  pub(crate) fn find_lower_bound_node(self, key: &[u8]) -> Option<LeafNodeRef<BorrowType, V>> {
    let mut stack = Vec::<Handle<BorrowType, V>>::with_capacity(DEFAULT_TREE_DEPTH);

    let mut cur = self;
    let mut depth = 0usize;
    let mut arg = SearchArgument { key, depth: 0 };

    loop {
      match cur.downcast() {
        NodeKind::Internal(node) => match node.search_lower_bound(key, depth) {
          SearchResult::GoUp => break,
          SearchResult::Found(ret) => {
            return Some(ret);
          }
          SearchResult::GoDown(origin_node) => {
            let new_depth = depth + origin_node.partial_key().len();
            let k = key[new_depth];
            if let Some(c) = node.find_child(k) {
              stack.push(c.reborrow());
              depth = new_depth + 1;
              cur = c.reborrow().into();
            } else {
              break;
            }
          }
        },
        NodeKind::Leaf(node) => {
          if node.is_lower_bound(arg) {
            return Some(node);
          } else {
            break;
          }
        }
      }
    }

    while let Some(entry) = stack.pop() {
      if let Some(c) = entry.node.find_next_child(entry.key) {
        return Some(c.minimum_leaf());
      }
    }

    None
  }
}

impl<'a> SearchArgument<'a> {
  #[inline(always)]
  pub(crate) fn partial_key(&self) -> &[u8] {
    // This is possible when that last node of key matches some internal node,
    // and we need to go down.
    if self.depth >= self.key.len() {
      &[0u8; 0]
    } else {
      &self.key[self.depth..]
    }
  }

  #[inline(always)]
  pub(crate) fn depth(&self) -> usize {
    self.depth
  }
}

impl<BorrowType, V> InternalNodeRef<BorrowType, V> {
  fn search_lower_bound(self, key: &[u8], depth: usize) -> SearchResult<LeafNodeRef<BorrowType, V>> {
    let this_partial_prefix = self.partial_prefix();
    let input_partial_key = &key[depth..];

    match input_partial_key.cmp(this_partial_prefix) {
      Ordering::Greater => SearchResult::GoUp,
      Ordering::Equal => {
        match self.partial_key() {
          PartialKey::Prefix(_) => SearchResult::GoDown(depth + this_partial_prefix.len()),
          PartialKey::Leaf(leaf_node) => SearchResult::Found(leaf_node.into())
        }
      }
      Ordering::Less => {
        match self.partial_key() {
          PartialKey::Prefix(_) => SearchResult::Found(self.minimum_leaf()),
          PartialKey::Leaf(leaf_node) => SearchResult::Found(leaf_node.into())
        }
      }
    }
  }

  pub(crate) fn minimum_leaf(self) -> Option<LeafNodeRef<BorrowType, V>> {
    todo!()
  }

  pub(crate) fn maximum_leaf(self) -> Option<LeafNodeRef<BorrowType, V>> {
    todo!()
  }

  fn search_upper_bound(self, key: &[u8], depth: usize) -> SearchResult<LeafNodeRef<BorrowType, V>> {
    let this_partial_prefix = self.partial_prefix();
    let input_partial_key = &key[depth..];

    match input_partial_key.cmp(this_partial_prefix) {
      Ordering::Less => SearchResult::GoUp,
      Ordering::Equal => {
        match self.partial_key() {
          PartialKey::Prefix(_) => SearchResult::GoDown(depth + this_partial_prefix.len()),
          PartialKey::Leaf(leaf_node) => SearchResult::Found(leaf_node.into())
        }
      }
      Ordering::Greater => {
        match self.partial_key() {
          PartialKey::Prefix(_) => SearchResult::Found(self.maximum_leaf()),
          PartialKey::Leaf(leaf_node) => SearchResult::Found(leaf_node.into())
        }
      }
    }
  }

  pub(crate) fn find_child(&self, _k: u8) -> Option<Handle<BorrowType, V>> {
    todo!()
  }

  pub(crate) fn find_next_child(&self, _k: u8) -> Option<NodeRef<BorrowType, V>> {
    todo!()
  }
}
