use crate::node::{ChildType, Node, NodeType};
use crate::node::base::NodeBase;

const NODE4_CAPACITY: usize = 4;

pub(in crate::node) struct Node4 {
    base: NodeBase,
    keys: [u8; NODE4_CAPACITY],
    children: [*mut u8; NODE4_CAPACITY],
    children_types: [ChildType; NODE4_CAPACITY],
}

impl Node for Node4 {
    fn get_type() -> NodeType {
        NodeType::Node4
    }

    fn search<V>(&self, _keys: &[u8]) -> Option<&V> {
        todo!()
    }
}