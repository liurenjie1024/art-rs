use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use crate::node::internal::SearchResult::{Found, FoundSelf, GoDown, GoUp};
use crate::node::leaf::LeafNodeRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node48::Node48Children;
use crate::node::node4::Node4Children;
use crate::node::{NodeRef, SearchArgument, SearchResult};

const MAX_PREFIX_LEN: usize = 16;

#[derive(Copy, Clone)]
pub(crate) struct InternalNodeRef<C, V> {
    inner: NonNull<InternalNode<C, V>>,
}

struct PartialPrefixData {
    partial_prefix: [u8; MAX_PREFIX_LEN],
    partial_prefix_len: usize,
}

enum PartialKey<V> {
    PartialPrefix(PartialPrefixData),
    Leaf(LeafNodeRef<V>),
}

pub(in crate::node) struct InternalNode<C, V> {
    partial_key: PartialKey<V>,
    children_count: u8,
    children: C,
}

trait ChildrenContainer {}


impl PartialPrefixData {
    #[inline(always)]
    fn partial_prefix(&self) -> &[u8] {
        &self.partial_prefix[0..self.partial_prefix_len]
    }
}

impl<C, V> InternalNodeRef<C, V> {
    #[inline(always)]
    fn inner(&self) -> &InternalNode<C, V> { unsafe { self.inner.as_ref() } }

    pub(crate) fn find_lower_bound(self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
        match &self.inner().partial_key {
            PartialKey::PartialPrefix(data) => self.lower_bound_with_partial_prefix(data, arg),
            PartialKey::Leaf(leaf) => self.lower_bound_with_leaf(*leaf, arg)
        }
    }

    fn lower_bound_with_partial_prefix(self,
                                       partial_prefix_data: &PartialPrefixData,
                                       arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
        let partial_key = arg.partial_key();
        let partial_prefix = partial_prefix_data.partial_prefix();
        if partial_key.len() <= partial_prefix.len() {
            match partial_key.cmp(partial_prefix) {
                Ordering::Greater => GoUp,
                _ => Found(self.minimum_leaf())
            }
        } else {
            let partial_key_of_prefix = partial_key[0..partial_prefix.len()];
            match partial_key_of_prefix.cmp(partial_prefix) {
                Ordering::Less => Found(self.minimum_leaf()),
                Ordering::Equal => GoDown,
                Ordering::Greater => GoUp
            }
        }
    }

    fn lower_bound_with_leaf(self,
                             leaf_ref: LeafNodeRef<V>,
                             arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
        let leaf_node = unsafe { leaf_ref.as_ref() };
        let partial_key = arg.partial_key();
        let partial_leaf_key = &leaf_node.key()[arg.depth..];
        if partial_key.len() <= partial_leaf_key.len() {
            match partial_key.cmp(partial_leaf_key) {
                Ordering::Greater => GoUp,
                _ => Found(leaf_ref)
            }
        } else {
            let partial_key_of_leaf = partial_key[0..partial_leaf_key.len()];
            match partial_key_of_leaf.cmp(partial_leaf_key) {
                Ordering::Greater => GoUp,
                Ordering::Equal => GoDown,
                Ordering::Less => Found(leaf_ref)
            }
        }
    }

    fn minimum_leaf(self) -> LeafNodeRef<V> {
        unimplemented!()
    }
}


impl<C, V> InternalNode<C, V> {

}
