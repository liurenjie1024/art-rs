use crate::node::{RawNode, NodeType};
use crate::node::base::NodeBase;

const NODE16_CAPACITY: usize = 16;

pub(in crate::node) struct Node16 {
    base: NodeBase,
    keys: [u8; NODE16_CAPACITY],
    children: [*mut u8; NODE16_CAPACITY],
    children_types: [NodeType; NODE16_CAPACITY],
}

impl RawNode for Node16 {
    fn get_type() -> NodeType {
        NodeType::Node16
    }

    fn search(&self, keys: &[u8]) -> *const u8 {
        todo!()
    }
}