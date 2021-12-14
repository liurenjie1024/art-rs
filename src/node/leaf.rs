use std::mem::swap;
use std::ptr::NonNull;

use crate::node::{NodeBase, NodeType};

// pub(crate) type BoxedLeafNode<V> = NonNull<LeafNode<V>>;

#[repr(C)]
pub(crate) struct LeafNode<K, V> {
    node_base: NodeBase<K, V>,
    key: K,
    value: V,
}

impl<K, V> LeafNode<K, V> {
    pub(crate) fn new(key: K, value: V) -> Box<Self> {
        Box::new(Self {
            node_base: NodeBase::new(NodeType::Leaf),
            key,
            value,
        })
    }

    pub(crate) fn set_value(&mut self, mut value: V) -> V {
        swap(&mut self.value, &mut value);
        value
    }

    pub(crate) fn key_ref(&self) -> &K {
        &self.key
    }

    pub(crate) fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub(crate) fn value_ref(&self) -> &V {
        &self.value
    }

    pub(crate) fn value_ptr(&mut self) -> NonNull<V> {
        NonNull::from(&mut self.value)
    }
}
