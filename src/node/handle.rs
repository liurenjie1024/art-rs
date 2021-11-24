use crate::marker::Immut;
use crate::node::{InternalNodeRef, NodeRef};

pub(crate) struct Handle<BorrowType, V> {
  parent: InternalNodeRef<BorrowType, V>,
  /// Index of child in parent.
  ///
  /// We don't use `u8` here to avoid another search.
  idx: usize,
}

impl<BorrowType, V> Handle<BorrowType, V> {
  pub(crate) fn reborrow(&self) -> Handle<Immut<'_>, V> {
    Handle {
      parent: self.parent.reborrow(),
      idx: self.idx,
    }
  }
}

// impl<BorrowType, V> Into<NodeRef<BorrowType, V>> for Handle<BorrowType, V> {
//   fn into(self) -> NodeRef<BorrowType, V> {
//     self.parent.child_at(self.idx)
//   }
// }
