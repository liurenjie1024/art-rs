use crate::node::{ChildType, Node, NodeType};
use crate::node::base::NodeBase;

const NODE16_CAPACITY: usize = 16;

pub(in crate::node) struct Node16 {
    base: NodeBase,
    keys: [u8; NODE16_CAPACITY],
    children: [*mut u8; NODE16_CAPACITY],
    children_types: [ChildType; NODE16_CAPACITY],
}

impl Node for Node16 {
    fn get_type() -> NodeType {
        NodeType::Node16
    }

    fn search<V>(&self, _keys: &[u8]) -> Option<&V> {
        todo!()
    }
}