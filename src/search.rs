use std::cmp::Ordering;

use crate::marker;
use crate::marker::{Internal, InternalOrLeaf, Leaf};
use crate::node::{NodeImpl, NodeRef};
use crate::search::SearchResult::{Found, GoDown, NotFound};

pub(crate) enum SearchResult<BorrowType, K, V> {
  Found(NodeRef<BorrowType, K, V, Leaf>),
  GoDown(NodeRef<BorrowType, K, V, InternalOrLeaf>),
  NotFound(NodeRef<BorrowType, K, V, InternalOrLeaf>),
}

impl<BorrowType: marker::BorrowType, K: AsRef<[u8]>, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
  pub(crate) fn search_tree(self, key: &K) -> SearchResult<BorrowType, K, V> {
    let mut cur = self;

    loop {
      match cur.downcast() {
        NodeImpl::Internal(internal) => match internal.search_node(key.as_ref()) {
          SearchResult::Found(node) => return SearchResult::Found(node),
          SearchResult::GoDown(node) => {
            cur = node;
          }
          SearchResult::NotFound(ret) => return SearchResult::NotFound(ret)
        },
        NodeImpl::Leaf(leaf) => {
          return leaf.search_node(key);
        }
      }
    }
  }
}

impl<BorrowType: marker::BorrowType, K, V> NodeRef<BorrowType, K, V, Internal> {
  fn search_node(self, key: &[u8]) -> SearchResult<BorrowType, K, V> {
    if self.prefix_len() >= key.len() {
      return NotFound(self.forget_type());
    }

    let input_partial_prefix = &key[self.prefix_len()..];
    let this_partial_prefix = self.partial_key();

    if input_partial_prefix.len() > this_partial_prefix.len() {
      match &input_partial_prefix[0..this_partial_prefix.len()].cmp(this_partial_prefix) {
        Ordering::Equal => {
          let new_prefix_len = self.prefix_len() + self.partial_key().len();

          let k = key[new_prefix_len];

          match self.find_child(k) {
            Some(handle) => GoDown(handle),
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

impl<BorrowType, K: AsRef<[u8]>, V> NodeRef<BorrowType, K, V, Leaf> {
  fn search_node(self, key: &K) -> SearchResult<BorrowType, K, V> {
    let key = key.as_ref();
    if self.prefix_len() >= key.len() && self.partial_key().len() == 0 {
      return SearchResult::Found(self);
    }
    let input_partial_prefix = &key[self.prefix_len()..];
    if input_partial_prefix == self.partial_key() {
      SearchResult::Found(self)
    } else {
      SearchResult::NotFound(self.forget_type())
    }
  }
}
