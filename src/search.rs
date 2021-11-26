use crate::marker;
use crate::node::{Handle, InternalNodeRef, LeafNodeRef, NodeKind, NodeRef};
use crate::search::SearchResult::{Found, GoDown, NotFound};
use either::{Either, Left, Right};
use std::cmp::Ordering;

pub(crate) enum SearchResult<BorrowType, V> {
  Found(LeafNodeRef<BorrowType, V>),
  GoDown(Handle<BorrowType, V>),
  NotFound(NodeRef<BorrowType, V>),
}

impl<BorrowType: marker::BorrowType, V> NodeRef<BorrowType, V> {
  pub(crate) fn search_tree(self, key: &[u8]) -> Option<LeafNodeRef<BorrowType, V>> {
    let mut cur = self;
    let mut depth: usize = 0;
    loop {
      match cur.search_node(key, &mut depth) {
        Found(leaf) => return Some(leaf),
        GoDown(handle) => {
          cur = handle.node_ref;
        }
        NotFound(_) => return None,
      }
    }
  }

  pub(crate) fn search_tree_for_insertion(
    self,
    key: &[u8],
  ) -> Either<LeafNodeRef<BorrowType, V>, Handle<BorrowType, V>> {
    let mut cur_parent_ref = None;
    let mut cur = self;
    let mut depth: usize = 0;
    loop {
      match cur.search_node(key, &mut depth) {
        Found(leaf) => return Either::Left(leaf),
        GoDown(handle) => {
          cur_parent_ref = handle.parent_ref;
          cur = handle.node_ref;
        }
        NotFound(node) => return Either::Right(Handle::new(cur_parent_ref, node)),
      }
    }
  }
}

impl<BorrowType: marker::BorrowType, V> NodeRef<BorrowType, V> {
  fn search_node(self, key: &[u8], depth: &mut usize) -> SearchResult<BorrowType, V> {
    match self.downcast() {
      NodeKind::Internal(internal_ref) => internal_ref.search_node(key, depth),
      NodeKind::Leaf(leaf_ref) => leaf_ref.search_node(key, *depth),
    }
  }
}

impl<BorrowType: marker::BorrowType, V> InternalNodeRef<BorrowType, V> {
  fn search_node(self, key: &[u8], depth: &mut usize) -> SearchResult<BorrowType, V> {
    if *depth >= key.len() {
      return NotFound(self.into());
    }

    let input_partial_prefix = &key[*depth..];
    let this_partial_prefix = self.inner().partial_key();

    if input_partial_prefix.len() > this_partial_prefix.len() {
      match &input_partial_prefix[0..this_partial_prefix.len()].cmp(this_partial_prefix) {
        Ordering::Equal => {
          let new_depth = *depth + self.inner().partial_key().len();

          let k = key[new_depth];

          match self.find_child(k) {
            Left(handle) => {
              *depth = new_depth + 1;
              GoDown(handle)
            }
            Right(this) => NotFound(this.into()),
          }
        }
        _ => NotFound(self.into()),
      }
    } else if input_partial_prefix.len() == this_partial_prefix.len() {
      match input_partial_prefix.cmp(this_partial_prefix) {
        Ordering::Equal => match self.get_leaf() {
          Some(leaf) => Found(leaf),
          None => NotFound(self.into()),
        },
        _ => NotFound(self.into()),
      }
    } else {
      NotFound(self.into())
    }
  }
}

impl<BorrowType, V> LeafNodeRef<BorrowType, V> {
  fn search_node(self, key: &[u8], depth: usize) -> SearchResult<BorrowType, V> {
    if depth >= key.len() && self.reborrow().inner().partial_key().len() == 0 {
      return Found(self);
    }
    let input_partial_prefix = &key[depth..];
    match input_partial_prefix.cmp(self.reborrow().inner().partial_key()) {
      Ordering::Equal => Found(self),
      _ => NotFound(self.into()),
    }
  }
}
