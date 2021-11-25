use crate::marker::Immut;
use crate::node::{InternalNodeRef, NodeRef};

/// A handle is a pointer to a child node ref in internal node.
///
/// We need this because when we want to do modify a node, we also need to update pointer in parent.
pub(crate) struct Handle<BorrowType, V> {
  parent: InternalNodeRef<BorrowType, V>,
  /// Index of child in parent.
  ///
  /// We don't use `u8` here to avoid another search.
  idx: usize,
}

impl<BorrowType, V> Handle<BorrowType, V> {
  pub(crate) fn into_node(self) -> NodeRef<BorrowType, V> {
    self.parent.child_at(self.idx)
  }
}

// impl<BorrowType, V> Into<NodeRef<BorrowType, V>> for Handle<BorrowType, V> {
//   fn into(self) -> NodeRef<BorrowType, V> {
//     self.parent.child_at(self.idx)
//   }
// }
