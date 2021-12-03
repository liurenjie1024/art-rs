use crate::marker;
use crate::marker::{Internal, InternalOrLeaf, Leaf};
use crate::node::{Handle, NodeKind, NodeRef};
use crate::search::SearchResult::{Found, GoDown, NotFound};
use either::{Either};
use std::cmp::Ordering;

pub(crate) enum SearchResult<BorrowType, V> {
  Found(NodeRef<BorrowType, V, Leaf>),
  GoDown(Handle<BorrowType, V>),
  NotFound(NodeRef<BorrowType, V, InternalOrLeaf>),
}

impl<BorrowType: marker::BorrowType, V> NodeRef<BorrowType, V, InternalOrLeaf> {
  pub(crate) fn search_tree(self, key: &[u8]) -> Option<NodeRef<BorrowType, V, Leaf>> {
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
  ) -> Either<NodeRef<BorrowType, V, Leaf>, Handle<BorrowType, V>> {
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

impl<BorrowType: marker::BorrowType, V> NodeRef<BorrowType, V, InternalOrLeaf> {
  fn search_node(self, key: &[u8], depth: &mut usize) -> SearchResult<BorrowType, V> {
    match self.downcast() {
      NodeKind::Internal(internal_ref) => internal_ref.search_node(key, depth),
      NodeKind::Leaf(leaf_ref) => leaf_ref.search_node(key, *depth),
    }
  }
}

impl<BorrowType: marker::BorrowType, V> NodeRef<BorrowType, V, Internal> {
  fn search_node(self, key: &[u8], depth: &mut usize) -> SearchResult<BorrowType, V> {
    if *depth >= key.len() {
      return NotFound(self.forget_type());
    }

    let input_partial_prefix = &key[*depth..];
    let this_partial_prefix = self.as_internal_ref().partial_key();

    if input_partial_prefix.len() > this_partial_prefix.len() {
      match &input_partial_prefix[0..this_partial_prefix.len()].cmp(this_partial_prefix) {
        Ordering::Equal => {
          let new_depth = *depth + self.as_internal_ref().partial_key().len();

          let k = key[new_depth];

          match self.find_child(k) {
            Some(handle) => {
              *depth = new_depth + 1;
              GoDown(handle)
            }
            None => NotFound(self.forget_type()),
          }
        }
        _ => NotFound(self.forget_type()),
      }
    } else if input_partial_prefix.len() == this_partial_prefix.len() {
      match input_partial_prefix.cmp(this_partial_prefix) {
        Ordering::Equal => match self.get_leaf() {
          Some(leaf) => Found(leaf),
          None => NotFound(self.forget_type()),
        },
        _ => NotFound(self.forget_type()),
      }
    } else {
      NotFound(self.forget_type())
    }
  }
}

impl<BorrowType, V> NodeRef<BorrowType, V, Leaf> {
  fn search_node(self, key: &[u8], depth: usize) -> SearchResult<BorrowType, V> {
    if depth >= key.len() && self.reborrow().as_leaf_ref().partial_key().len() == 0 {
      return Found(self);
    }
    let input_partial_prefix = &key[depth..];
    match input_partial_prefix.cmp(self.reborrow().as_leaf_ref().partial_key()) {
      Ordering::Equal => Found(self),
      _ => NotFound(self.forget_type()),
    }
  }
}
