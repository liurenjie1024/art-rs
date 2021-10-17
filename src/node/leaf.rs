use std::cmp::Ordering;
use std::ptr::NonNull;
use crate::node::{SearchArgument, SearchResult};
use crate::node::SearchResult::{Found, GoDown, GoUp};

pub struct LeafRange<V> {
    start: Option<LeafNodeRef<V>>,
    end: Option<LeafNodeRef<V>>
}

#[derive(Copy, Clone)]
pub(crate) struct LeafNodeRef<V> {
    inner: NonNull<LeafNode<V>>
}

pub struct LeafNode<V> {
    key: Vec<u8>,
    value: V,
    prev: Option<NonNull<LeafNode<V>>>,
    next: Option<NonNull<LeafNode<V>>>
}

impl<V> LeafNode<V> {
    pub(crate) fn key(&self) -> &[u8] {
        &self.key
    }
}

impl<V> LeafNodeRef<V> {
    pub(crate) fn find_lower_bound(self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
        let partial_key = arg.partial_key();
        let partial_leaf_key = &self.key()[arg.depth..];
        if partial_key.len() <= partial_leaf_key.len() {
            match partial_key.cmp(partial_leaf_key) {
                Ordering::Greater => GoUp,
                _ => Found(leaf_ref)
            }
        } else {
            let partial_key_of_leaf = partial_key[0..partial_leaf_key.len()];
            match partial_key_of_leaf.cmp(partial_leaf_key) {
                Ordering::Less => unsafe {
                    Found(LeafNodeRef::new_unchecked(&mut *self))
                },
                _ => GoUp,
            }
        }
    }

    fn inner(&self) -> &LeafNode<V> {
        unsafe {
            self.inner.as_ref()
        }
    }
}