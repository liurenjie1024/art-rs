use crate::node::NodeRef;

pub(in crate::node) const NODE256_CAPACITY: usize = 256;

pub(in crate::node) struct Node256Children<V> {
  children: [NodeRef<V>; NODE256_CAPACITY],
}
