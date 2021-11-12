use crate::node::{InternalNodeRef, NodeRef};

pub(crate) struct Handle<BorrowType, V> {
  parent: InternalNodeRef<BorrowType, V>,
  /// Index of child in parent.
  ///
  /// We don't use `u8` here to avoid another search.
  idx: usize,
}
