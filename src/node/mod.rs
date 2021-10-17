use std::ptr::{NonNull, null};


use crate::node::base::{NodeBase, PrefixMatchResult};

mod node4;
use node4::*;
mod node16;
use node16::*;
mod node48;
use node48::*;
mod node256;
use node256::*;
mod internal;
use internal::*;
mod leaf;
use leaf::*;
use crate::node::NodeRef::{Node16, Node256, Node4, Node48};

const DEFAULT_TREE_DEPTH: usize = 16;

#[derive(Copy, Clone)]
pub(crate) enum NodeRef<V> {
    Node4(InternalNodeRef<Node4Children<V>, V>),
    Node16(InternalNodeRef<Node16Children<V>, V>),
    Node48(InternalNodeRef<Node48Children<V>, V>),
    Node256(InternalNodeRef<Node256Children<V>, V>),
    Leaf(LeafNodeRef<V>)
}

struct SearchStackEntry<V> {
    node: NodeRef<V>,
    key: u8
}

pub(crate) enum SearchResult<R> {
    GoUp,
    GoDown(usize),
    Found(R),
}

pub(crate) struct SearchArgument<'a> {
    key: &'a [u8],
    depth: usize,
}

impl<V> NodeRef<V> {
    /// Find first leaf node, whose keys is not less than input key.
    pub(crate) fn find_lower_bound_node(&self, _key: &[u8]) -> Option<LeafNodeRef<V>> {
        let mut stack = Vec::<SearchStackEntry<V>>::with_capacity(DEFAULT_TREE_DEPTH);

        let cur = self.clone();

        loop {

        }
    }

    fn search_lower_bound(&self, arg: SearchArgument) -> SearchResult<LeafNodeRef<V>> {
        unsafe {
            match self {
                Node4(node) => node.as_ref().find_lower_bound(arg),
                Node16(node) => node.as_ref().find_lower_bound(arg),
                Node48(node) => node.as_ref().find_lower_bound(arg),
                Node256(node) => node.as_ref().find_lower_bound(arg),
                Leaf(node) => node.as_ref().find_lower_bound(arg),
            }
        }
    }
}


impl<'a> SearchArgument<'a> {
    #[inline(always)]
    pub(crate) fn partial_key(&self) -> &[u8] {
        self.key[depth..]
    }
}

// pub(crate) trait RawNode {
//     fn get_type() -> NodeType where Self: Sized;
//     // fn search(&self, keys: &[u8]) -> *const u8 {
//     //     let node_base = self.get_node_base();
//     //     match node_base.search(keys) {
//     //         PrefixMatchResult::Fail  => null(),
//     //         PrefixMatchResult::Exact => node_base.get_empty_value(),
//     //         PrefixMatchResult::Extra => self.search_after_node_base(&keys[node_base.get_prefix_size()..keys.len()])
//     //     }
//     // }
//
//     fn search(&self, keys: &[u8]) -> *const u8;
// }
//
// pub(in crate::node) trait NodeInner {
//     fn search(&self, keys: &[u8]) -> *const u8 {
//         let node_base = self.get_node_base();
//         match node_base.search(keys) {
//             PrefixMatchResult::Fail => null(),
//             PrefixMatchResult::Exact => node_base.get_empty_value(),
//             PrefixMatchResult::Extra => self.search_after_node_base(&keys[node_base.get_prefix_size()..keys.len()])
//         }
//     }
//
//     fn get_node_base(&self) -> &NodeBase;
//     fn search_after_node_base(&self, keys: &[u8]) -> *const u8;
// }
//
// // pub(crate) struct ChildPtr {
// //     child_type: ChildType,
// //     child_ptr: *mut u8,
// // }
// //
// // impl Default for ChildPtr {
// //     fn default() -> Self {
// //         ChildPtr {
// //             child_type: ChildType::Value,
// //             child_ptr: null_mut(),
// //         }
// //     }
// // }
// //
// // impl ChildPtr {
// //     pub(crate) fn is_empty(&self) -> bool {
// //         self.child_ptr.is_null()
// //     }
// //
// //     pub(crate) unsafe fn search(&self, keys: &[u8]) -> Option<*const u8> {
// //         if self.is_empty() {
// //             None
// //         } else {
// //             match self.child_type {
// //                 ChildType::Node(_) => self.to_node().map(|node| node.search(keys)),
// //                 ChildType::Value => self.to_value()
// //             }
// //         }
// //     }
// //
// //     unsafe fn to_node(&self) -> Option<&dyn RawNode> {
// //         if !self.is_empty() {
// //             match self.child_type {
// //                 ChildType::Node(NodeType::Node4) => Some(&*transmute::<*mut u8, *mut Node4>(self.child_ptr)),
// //                 ChildType::Node(NodeType::Node16) => Some(&*transmute::<*mut u8, *mut Node16>(self.child_ptr)),
// //                 ChildType::Node(NodeType::Node48) => Some(&*transmute::<*mut u8, *mut Node48>(self.child_ptr)),
// //                 ChildType::Node(NodeType::Node256) => Some(&*transmute::<*mut u8, *mut Node256>(self.child_ptr)),
// //                 _ => panic!("Value node should not reach here!")
// //             }
// //         } else {
// //             None
// //         }
// //     }
// //
// //     unsafe fn to_value(&self) -> Option<*const u8> {
// //         if !self.is_empty() {
// //             match self.child_type {
// //                 ChildType::Value => Some(transmute::<*mut u8, *const u8>(self.child_ptr)),
// //                 _ => panic!("Inner node should not reach here!")
// //             }
// //         } else {
// //             None
// //         }
// //     }
// // }
//
