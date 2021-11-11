use crate::node::InternalNodeRef;

pub(crate) struct Edge<V> {
  parent: InternalNodeRef<V>,
  /// Index of child in parent.
  ///
  /// We don't use `u8` here to avoid another search.
  idx: usize,
}
