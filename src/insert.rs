use std::ptr::NonNull;
use std::slice::from_raw_parts;



use crate::common_len;
use crate::marker::{Internal, InternalOrLeaf, Leaf, Mut};
use crate::node::{InternalNode4, LeafNode, NodeImpl, NodeRef};

impl<'a, K: AsRef<[u8]>, V> NodeRef<Mut<'a>, K, V, InternalOrLeaf> {
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
  pub(crate) fn insert_node(self, key: K, value: V) -> NonNull<V> {
    match self.downcast() {
      NodeImpl::Internal(internal) => internal.insert_node(key, value),
      NodeImpl::Leaf(leaf) => leaf.insert_node(key, value)
    }
  }
}

impl<'a, K: 'a + AsRef<[u8]>, V: 'a> NodeRef<Mut<'a>, K, V, Internal> {
  /// Insert into current node.
  ///
  /// # Returns
  ///
  /// When current node doesn't need to split to new node, just return itself.
  /// Otherwise, returns a newly created node.
  fn insert_node(
    mut self,
    key: K,
    value: V,
  ) -> NonNull<V> {
    let this_partial_key = self.partial_key();
    let input_partial_key = &key.as_ref()[self.prefix_len()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    // Insert self
    if common_key_len < this_partial_key.len() {
      let mut new_parent = unsafe { InternalNode4::new() };
      new_parent
          .base_mut()
          .set_partial_key(&this_partial_key[0..common_key_len]);
      let _new_parent_prefix_len = self.prefix_len();

      // First create leaf node using new value
      let leaf_value_ptr = if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        let mut new_leaf_node = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        if let Some(_) =
        unsafe { new_parent.set_child(new_k, NonNull::from(Box::leak(new_leaf_node)).cast()) }
        {
          unreachable!("This should not happen!");
        }

        leaf_value_ptr
      } else {
        let mut new_leaf_node = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        unsafe {
          new_parent
              .base_mut()
              .set_leaf(NonNull::from(Box::leak(new_leaf_node)));
        }
        leaf_value_ptr
      };

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

        self.set_partial_key(new_partial_key);

        if let Some(_) = unsafe { new_parent.set_child(new_k, self.get_inner()) } {
          unreachable!("This should not happen!");
        }

        unsafe {
          self.replace_self_in_parent(Some(NonNull::from(Box::leak(new_parent)).cast()));
        }
      }

      leaf_value_ptr
    } else {
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        let mut new_leaf_node = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        if let Some(_) =
        unsafe { self.insert_child(new_k, NonNull::from(Box::leak(new_leaf_node)).cast()) }
        {
          unreachable!()
        }
        leaf_value_ptr
      } else {
        let mut new_leaf_node = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        if let Some(_) = unsafe { self.set_leaf(NonNull::from(Box::leak(new_leaf_node))) } {
          unreachable!()
        }
        leaf_value_ptr
      }
    }
  }
}

impl<'a, K: 'a + AsRef<[u8]>, V: 'a> NodeRef<Mut<'a>, K, V, Leaf> {
  fn insert_node(mut self, key: K, value: V) -> NonNull<V> {
    let this_partial_key = self.partial_key();
    let input_partial_key = &key.as_ref()[self.partial_key().len()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    let mut new_parent = unsafe { InternalNode4::new() };
    new_parent
        .base_mut()
        .set_partial_key(&this_partial_key[0..common_key_len]);

    // Insert new leaf node
    let leaf_value_ptr = {
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        let _new_prefix_len = self.prefix_len() + common_key_len + 1;
        let mut new_leaf = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf.value_mut());
        unsafe {
          new_parent.set_child(new_k, NonNull::from(Box::leak(new_leaf)).cast());
        }
        leaf_value_ptr
      } else {
        let mut new_leaf = LeafNode::new(key, value);
        let leaf_value_ptr = NonNull::from(new_leaf.value_mut());
        unsafe {
          new_parent
              .base_mut()
              .set_leaf(NonNull::from(Box::leak(new_leaf)));
        }
        leaf_value_ptr
      }
    };

    // Insert current node
    {
      if common_key_len < this_partial_key.len() {
        let new_k = this_partial_key[common_key_len];
        self.set_prefix_len(self.prefix_len() + common_key_len + 1);
        unsafe {
          new_parent.set_child(new_k, self.get_inner());
        }
      } else {
        self.set_prefix_len(self.prefix_len() + common_key_len);
        unsafe {
          new_parent
              .base_mut()
              .set_leaf(self.get_inner().cast());
          self.replace_self_in_parent(Some(NonNull::from(Box::leak(new_parent)).cast()));
        }
      }
    }

    leaf_value_ptr
  }
}
