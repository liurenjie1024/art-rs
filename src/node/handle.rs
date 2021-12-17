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
  pub(crate) node: NodeRef<BorrowType, K, V, Internal>,
  pub(crate) pos: NodePos,
}

impl<BorrowType, K, V> Handle<BorrowType, K, V> {
  pub(crate) fn new(node_ref: NodeRef<BorrowType, K, V, Internal>, pos: NodePos) -> Self {
    Self {
      node: node_ref,
      pos,
    }
  }

  /// Resolve the actual node reference this handle points to.
  pub(crate) fn resolve_node(&self) -> NodeRef<BorrowType, K, V, InternalOrLeaf> {
    match self.pos {
      NodePos::Child(idx) => self.node.child_at(idx),
      NodePos::Leaf => self.node.get_leaf().unwrap().forget_type(),
    }
  }

  /// Write `new_node_ptr` to handle position.
  ///
  /// # Safety
  ///
  /// This method blindly write ptr to position pointed by handle, and doesn't care about memory
  /// management.
  pub(crate) unsafe fn replace_node(
    &mut self,
    new_node_ptr: BoxedNode<K, V>,
  ) -> Option<BoxedNode<K, V>> {
    match self.pos {
      NodePos::Child(idx) => {
        assert!(new_node_ptr.as_ref().node_type.is_internal());
        self
          .node
          .reborrow()
          .as_internal_impl()
          .set_child_at(idx, new_node_ptr)
      }
      NodePos::Leaf => {
        assert!(new_node_ptr.as_ref().node_type.is_leaf());
        self
          .node
          .reborrow()
          .as_internal_mut()
          .set_leaf(new_node_ptr)
      }
    }
  }
}
