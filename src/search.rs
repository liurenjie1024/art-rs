use crate::marker;
use crate::marker::{Internal, InternalOrLeaf, Leaf};
use crate::node::{Handle, NodeKind, NodeRef};
use crate::search::SearchResult::{Found, GoDown, NotFound};
use either::{Either};
use std::cmp::Ordering;

pub(crate) enum SearchResult<BorrowType, K, V> {
    Found(NodeRef<BorrowType, K, V, Leaf>),
    GoDown(Handle<BorrowType, K, V>),
    NotFound(NodeRef<BorrowType, K, V, InternalOrLeaf>),
}

impl<BorrowType: marker::BorrowType, K: AsRef<[u8]>, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    pub(crate) fn search_tree(self, key: K) -> Option<NodeRef<BorrowType, K, V, Leaf>> {
        let mut cur = self;

        loop {
            match cur.search_node(key.as_ref()) {
                Found(leaf) => return Some(leaf),
                GoDown(handle) => {
                    cur = handle.resolve_node();
                }
                NotFound(_) => return None,
            }
        }
    }

    pub(crate) fn search_tree_for_insertion(
        self,
        key: K,
    ) -> Either<NodeRef<BorrowType, K, V, Leaf>, Handle<BorrowType, K, V>> {
        let mut cur_parent_ref = None;
        let mut cur = self;
        let mut depth: usize = 0;
        loop {
            match cur.search_node(key.as_ref()) {
                Found(leaf) => return Either::Left(leaf),
                GoDown(handle) => {
                    cur_parent_ref = handle.parent_ref;
                    cur = handle.node;
                }
                NotFound(node) => return Either::Right(Handle::new(cur_parent_ref, node)),
            }
        }
    }
}

impl<BorrowType: marker::BorrowType, K, V> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    fn search_node(self, key: &[u8]) -> SearchResult<BorrowType, K, V> {
        match self.downcast() {
            NodeKind::Internal(internal_ref) => internal_ref.search_node(key),
            NodeKind::Leaf(leaf_ref) => leaf_ref.search_node(key),
        }
    }
}

impl<BorrowType: marker::BorrowType, K, V> NodeRef<BorrowType, K, V, Internal> {
    fn search_node(self, key: &[u8]) -> SearchResult<BorrowType, K, V> {
        if self.prefix_len() >= key.len() {
            return NotFound(self.forget_type());
        }

        let input_partial_prefix = &key[self.prefix_len()..];
        let this_partial_prefix = self.as_internal_ref().partial_prefix();

        if input_partial_prefix.len() > this_partial_prefix.len() {
            match &input_partial_prefix[0..this_partial_prefix.len()].cmp(this_partial_prefix) {
                Ordering::Equal => {
                    let new_prefix_len = self.prefix_len() + self.as_internal_ref().partial_prefix().len();

                    let k = key[new_prefix_len];

                    match self.find_child(k) {
                        Some(handle) => {
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

impl<BorrowType, K, V> NodeRef<BorrowType, K, V, Leaf> {
    fn search_node(self, key: &[u8]) -> SearchResult<BorrowType, K, V> {
        if self.prefix_len() >= key.len() && self.as_leaf_ref().partial_key().len() == 0 {
            return Found(self);
        }
        let input_partial_prefix = &key[self.prefix_len()..];
        match input_partial_prefix.cmp(self.partial_key()) {
            Ordering::Equal => Found(self),
            _ => NotFound(self.forget_type()),
        }
    }
}
