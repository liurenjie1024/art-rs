use crate::node::{RawNode, NodeType, NodeInner, NodeRef};
use crate::node::base::NodeBase;

const NODE4_CAPACITY: usize = 4;

pub(in crate::node) struct Node4Children<V> {
    keys: [u8; NODE4_CAPACITY],
    children: [NodeRef<V>; NODE4_CAPACITY]
}