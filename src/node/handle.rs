use crate::node::{BoxedInternalNode, NodeRef};
use std::ptr::NonNull;

/// A handle is a pointer to a child node ref in internal node.
///
/// We need this because when we want to do modify a node, we also need to update pointer in parent.
pub(crate) struct Handle<BorrowType, V> {
  /// A reference to pointer slot in parent.
  pub(crate) parent_ref: Option<NonNull<BoxedInternalNode<V>>>,
  pub(crate) node_ref: NodeRef<BorrowType, V>,
}

impl<BorrowType, V> Handle<BorrowType, V> {
  pub(crate) fn new(
    parent_ref: Option<NonNull<BoxedInternalNode<V>>>,
    node_ref: NodeRef<BorrowType, V>,
  ) -> Self {
    Self {
      parent_ref,
      node_ref,
    }
  }
}

// impl<BorrowType, V> Into<NodeRef<BorrowType, V>> for Handle<BorrowType, V> {
//   fn into(self) -> NodeRef<BorrowType, V> {
//     self.parent.child_at(self.idx)
//   }
// }
