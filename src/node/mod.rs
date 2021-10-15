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

pub(crate) enum NodeRef<V> {
    Node4(InternalNodeRef<Node4Children<V>, V>),
    Node16(InternalNodeRef<Node16Children<V>, V>),
    Node48(InternalNodeRef<Node48Children<V>, V>),
    Node256(InternalNodeRef<Node256Children<V>, V>),
    Leaf(LeafRef<V>)
}

impl<V> NodeRef<V> {
    pub(crate) fn lower_bound(&self, _key: &[u8]) -> Option<LeafRef<V>> {
        unimplemented!()
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
