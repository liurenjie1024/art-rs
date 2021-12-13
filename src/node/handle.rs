use std::ptr::NonNull;
use crate::marker::{Internal, InternalOrLeaf};
use crate::node::{BoxedNode, NodeRef};

/// Position of a child node in internal node.
pub(crate) enum NodePos {
    /// Index in [`Children`] container.
    Child(usize),
    /// Leaf node in internal node.
    Leaf,
}

/// A handle is a pointer to a child node ref in internal node.
///
/// We need this because when we want to do modify a node, we also need to update pointer in parent.
pub(crate) struct Handle<BorrowType, K, V> {
    pub(crate) node_ref: NodeRef<BorrowType, K, V, Internal>,
    pub(crate) pos: NodePos,
}

impl<BorrowType, K, V> Handle<BorrowType, K, V> {
    pub(crate) fn new(
      node_ref: NodeRef<BorrowType, K, V, Internal>,
      pos: NodePos,
    ) -> Self {
        Self {
            node_ref,
            pos,
        }
    }

    pub(crate) fn node_ref(&self) -> &NodeRef<BorrowType, K, V, Internal> {
        &self.node_ref
    }
}
