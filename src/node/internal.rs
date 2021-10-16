use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use crate::node::internal::SearchResult::{Found, FoundSelf, GoDown, GoUp};
use crate::node::leaf::LeafRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node48::Node48Children;
use crate::node::node4::Node4Children;
use crate::node::NodeRef;

const MAX_PREFIX_LEN: usize = 16;

pub(crate) type InternalNodeRef<C, V> = NonNull<InternalNode<C, V>>;

struct PartialPrefixData {
    partial_prefix: [u8; MAX_PREFIX_LEN],
    partial_prefix_len: usize,
}

enum PartialKey<V> {
    PartialPrefix(PartialPrefixData),
    Leaf(LeafRef<V>),
}

pub(in crate::node) struct InternalNode<C, V> {
    partial_key: PartialKey<V>,
    children_count: u8,
    children: C,
}

trait ChildrenContainer {}

pub(crate) enum SearchResult<R> {
    GoUp,
    GoDown,
    Found(R),
}

pub(crate) struct SearchArgument<'a> {
    key: &'a [u8],
    depth: usize,
}

impl<'a> SearchArgument<'a> {
    #[inline(always)]
    fn partial_key(&self) -> &[u8] {
        self.key[depth..]
    }
}

impl PartialPrefixData {
    #[inline(always)]
    fn partial_prefix(&self) -> &[u8] {
        &self.partial_prefix[0..self.partial_prefix_len]
    }
}

impl<C, V> InternalNode<C, V> {
    pub(crate) fn find_lower_bound(&self, arg: SearchArgument) -> SearchResult<LeafRef<V>> {
        match &self.partial_key {
            PartialKey::PartialPrefix(data) => self.lower_bound_with_partial_prefix(data, arg),
            PartialKey::Leaf(leaf) => self.lower_bound_with_leaf(*leaf, arg)
        }
    }

    fn lower_bound_with_partial_prefix<'a>(&self,
                                           partial_prefix_data: &PartialPrefixData,
                                           arg: SearchArgument<'a>) -> SearchResult<LeafRef<V>> {
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

    fn lower_bound_with_leaf(&self,
                             leaf_ref: LeafRef<V>,
                             arg: SearchArgument) -> SearchResult<LeafRef<V>> {
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

    fn minimum_leaf(&self) -> LeafRef<V> {
        unimplemented!()
    }
}
