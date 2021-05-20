use crate::node::{RawNode, NodeType, NodeInner};
use crate::node::base::NodeBase;

const NODE4_CAPACITY: usize = 4;

pub(in crate::node) struct Node4 {
    base: NodeBase,
    keys: [u8; NODE4_CAPACITY],
    children: [*mut u8; NODE4_CAPACITY],
    children_types: [NodeType; NODE4_CAPACITY],
}

impl RawNode for Node4 {
    fn get_type() -> NodeType where Self: Sized {
        todo!()
    }

    fn search(&self, keys: &[u8]) -> *const u8 {
        (self as &dyn NodeInner).search(keys)
    }
}

impl NodeInner for Node4 {
    fn get_node_base(&self) -> &NodeBase {
        todo!()
    }
    fn search_after_node_base(&self, keys: &[u8]) -> *const u8 {
        todo!()
    }
}