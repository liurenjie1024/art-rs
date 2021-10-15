use crate::node::node256::NODE256_CAPACITY;
use crate::node::NodeRef;

const NODE48_CAPACITY: usize = 256;

pub(in crate::node) struct Node48Children<V> {
    keys: [u8; NODE256_CAPACITY],
    children: [NodeRef<V>; NODE48_CAPACITY],
}