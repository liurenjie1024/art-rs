use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use crate::node::internal::SearchResult::{FoundSelf, GoUp};
use crate::node::leaf::LeafRef;
use crate::node::node16::Node16Children;
use crate::node::node256::Node256Children;
use crate::node::node48::Node48Children;
use crate::node::node4::Node4Children;
use crate::node::NodeRef;

const MAX_PREFIX_LEN: usize = 16;

pub(crate) type InternalNodeRef<C, V> = NonNull<InternalNode<C, V>>;

pub(in crate::node) struct InternalNode<C, V> {
    partial_prefix: [u8; MAX_PREFIX_LEN],
    partial_prefix_len: usize,
    leaf: Option<LeafRef<V>>,
    children_count: u8,
    children: C
}

trait ChildrenContainer {

}

pub(crate) enum SearchResult<R> {
    GoUp,
    FoundSelf,
    Found(R)
}

impl<C, V> InternalNode<C, V> {
    pub(crate) fn find_lower_bound(&self, key: &[u8], depth: usize) -> SearchResult<LeafRef<V>> {
        let left_key = &key[depth..];
        let partial_prefix  = self.partial_prefix();

        if left_key.len() <= partial_prefix.len() {
            match left_key.cmp(partial_prefix) {
                Ordering::Less | Ordering::Equal => FoundSelf,
                Ordering::Greater => GoUp
            }
        } else {

        }
    }

    #[inline(always)]
    fn partial_prefix(&self) -> &[u8] {
        &self.partial_prefix[0..self.partial_prefix_len]
    }
}
