use crate::node::{RawNode, NodeType};
use crate::node::base::NodeBase;

pub(in crate::node) const NODE256_CAPACITY: usize = 256;

pub(in crate::node) struct Node256 {
    base: NodeBase,

    children: [*mut u8; NODE256_CAPACITY],
    children_types: [NodeType; NODE256_CAPACITY],
}

impl RawNode for Node256 {
    fn get_type() -> NodeType {
        NodeType::Node256
    }

    fn search(&self, keys: &[u8]) -> *const u8 {
        todo!()
    }
}