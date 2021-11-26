use crate::common_len;
use crate::marker::Mut;
use crate::node::{InternalNodeRef, NodeRef};

impl<'a, V> NodeRef<Mut<'a>, V> {
  /// Insert `key`, `value` into this node.
  ///
  /// This method is designed to be used by entry api, which already checked prefix against parents
  /// of this node.
  ///
  /// # Returns
  ///
  /// A new subtree's root node with value inserted.
  ///
  /// # Panics
  ///
  /// If same key already exists.
  pub(crate) fn insert_node(mut self, key: &[u8], value: V) -> Self {}
}

impl<'a, V> InternalNodeRef<Mut<'a>, V> {
  fn insert_node(mut self, key: &[u8], value: V) -> Self {
    let this_partial_key = self.inner().partial_key();
    let input_partial_key = &key[self.inner().prefix_ken()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    if common_key_len < this_partial_key.len() {
      /// First create leaf node using new value
      let new_leaf_node = if common_key_len < input_partial_key.len() {
        
      }
    }
  }
}
