use crate::node::NodeRef;

const NODE16_CAPACITY: usize = 16;

pub(in crate::node) struct Node16Children<V> {
    keys: [u8; NODE16_CAPACITY],
    children: [NodeRef<V>; NODE16_CAPACITY],
}