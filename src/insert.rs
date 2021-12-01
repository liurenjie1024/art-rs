use crate::common_len;
use crate::marker::{Mut, Owned};
use crate::node::{InternalNode4, InternalNodeRef, LeafNode, LeafNodeRef, NodeRef};
use std::slice::from_raw_parts;

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
  pub(crate) fn insert_node(mut self, key: &[u8], value: V) -> Self {
    todo!()
  }
}

impl<'a, V> InternalNodeRef<Mut<'a>, V> {
  fn insert_node(mut self, key: &[u8], value: V) -> Self {
    let this_partial_key = self.inner().partial_key();
    let input_partial_key = &key[self.inner().prefix_ken()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    if common_key_len < this_partial_key.len() {
      let mut new_parent = unsafe { InternalNode4::new(self.inner().prefix_ken()) };
      new_parent
        .base_mut()
        .set_partial_key(&this_partial_key[0..common_key_len]);

      /// First create leaf node using new value
      let new_leaf_node = LeafNode::new(self.inner().prefix_ken() + common_key_len, key, value);
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        if let Some(_) = unsafe {
          new_parent.insert_child(
            new_k,
            LeafNodeRef::from_owned(new_leaf_node).to_ptr().cast(),
          )
        } {
          unreachable!("This should not happen!")
        }
      } else {
        new_parent.base_mut().set_leaf(new_leaf_node);
      }

      // Insert self as child to new parent
      {
        let new_k = this_partial_key[common_key_len];
        let new_partial_key: &[u8] = if (common_key_len + 1) >= this_partial_key.len() {
          &[]
        } else {
          let slice = &this_partial_key[(common_key_len + 1)..];
          // SAFETY: The copy will not override.
          unsafe { from_raw_parts(slice.as_ptr(), slice.len()) }
        };

        self.inner_mut().set_partial_key(new_partial_key);

        if let Some(_) = unsafe { new_parent.insert_child(new_k, self.to_ptr().cast()) } {
          unreachable!("This should not happen!");
        }
      }

      InternalNodeRef::from_new_node(new_parent)
    } else {
      let new_leaf_node = LeafNode::new(self.inner().prefix_ken() + common_key_len, key, value);
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        if let Some(_) = unsafe {
          self.inner_mut().insert_child(
            new_k,
            LeafNodeRef::from_owned(new_leaf_node).to_ptr().cast(),
          )
        } {
          unreachable!("This should not happen!")
        }
      } else {
        self.inner_mut().set_leaf(new_leaf_node);
      }

      self
    }
  }
}
