use std::mem::transmute;
use std::ptr::null_mut;

use crate::node::node16::Node16;
use crate::node::node256::Node256;
use crate::node::node48::Node48;
use crate::node::node4::Node4;

mod base;
mod node4;
mod node16;
mod node48;
mod node256;

#[repr(u8)]
pub(crate) enum NodeType {
    Node4,
    Node16,
    Node48,
    Node256,
}

pub(crate) enum ChildType {
    Node(NodeType),
    Value,
}

pub(crate) trait RawNode {
    fn get_type() -> NodeType where Self: Sized;
    fn search(&self, keys: &[u8]) -> *const u8;
}

pub(crate) struct ChildPtr {
    child_type: ChildType,
    child_ptr: *mut u8,
}

impl Default for ChildPtr {
    fn default() -> Self {
        ChildPtr {
            child_type: ChildType::Value,
            child_ptr: null_mut(),
        }
    }
}

impl ChildPtr {
    pub(crate) fn is_empty(&self) -> bool {
        self.child_ptr.is_null()
    }

    pub(crate) unsafe fn search(&self, keys: &[u8]) -> Option<*const u8> {
        if self.is_empty() {
            None
        } else {
            match self.child_type {
                ChildType::Node(_) => self.to_node().map(|node| node.search(keys)),
                ChildType::Value => self.to_value()
            }
        }
    }

    unsafe fn to_node(&self) -> Option<&dyn RawNode> {
        if !self.is_empty() {
            match self.child_type {
                ChildType::Node(NodeType::Node4) => Some(&*transmute::<*mut u8, *mut Node4>(self.child_ptr)),
                ChildType::Node(NodeType::Node16) => Some(&*transmute::<*mut u8, *mut Node16>(self.child_ptr)),
                ChildType::Node(NodeType::Node48) => Some(&*transmute::<*mut u8, *mut Node48>(self.child_ptr)),
                ChildType::Node(NodeType::Node256) => Some(&*transmute::<*mut u8, *mut Node256>(self.child_ptr)),
                _ => panic!("Value node should not reach here!")
            }
        } else {
            None
        }
    }

    unsafe fn to_value(&self) -> Option<*const u8> {
        if !self.is_empty() {
            match self.child_type {
                ChildType::Value => Some(transmute::<*mut u8, *const u8>(self.child_ptr)),
                _ => panic!("Inner node should not reach here!")
            }
        } else {
            None
        }
    }
}

