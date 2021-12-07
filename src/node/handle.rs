use std::ptr::NonNull;
use crate::marker::InternalOrLeaf;
use crate::node::{BoxedNode, NodeRef};

/// A handle is a pointer to a child node ref in internal node.
///
/// We need this because when we want to do modify a node, we also need to update pointer in parent.
pub(crate) struct Handle<BorrowType, V> {
  /// A reference to pointer slot in parent.
  pub(crate) parent_ref: Option<NonNull<BoxedNode<V>> >,
  pub(crate) node_ref: NodeRef<BorrowType, V, InternalOrLeaf>,
}

impl<BorrowType, V> Handle<BorrowType, V> {
  pub(crate) fn new(
    parent_ref: Option<NonNull<BoxedNode<V>>>,
    node_ref: NodeRef<BorrowType, V, InternalOrLeaf>,
  ) -> Self {
    Self {
      parent_ref,
      node_ref,
    }
  }
}
