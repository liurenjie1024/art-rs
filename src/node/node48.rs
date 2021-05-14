use crate::node::{ChildType, Node, NodeType};
use crate::node::base::NodeBase;
use crate::node::node256::NODE256_CAPACITY;

const NODE48_CAPACITY: usize = 256;

pub(in crate::node) struct Node48 {
    base: NodeBase,

    keys: [u8; NODE256_CAPACITY],
    children: [*mut u8; NODE48_CAPACITY],
    children_types: [ChildType; NODE48_CAPACITY],
}

impl Node for Node48 {
    fn get_type() -> NodeType {
        NodeType::Node48
    }

    fn search<V>(&self, _keys: &[u8]) -> Option<&V> {
        todo!()
    }
}