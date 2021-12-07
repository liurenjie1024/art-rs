use std::ptr::NonNull;
use std::slice::from_raw_parts;

use either::Either;

use crate::common_len;
use crate::marker::{Internal, Leaf, Mut, Owned};
use crate::node::{Handle, InternalNode4, LeafNode, NodeKind, NodeRef};

impl<'a, V> Handle<Mut<'a>, V> {
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
  pub(crate) fn insert_node(self, key: &[u8], value: V) -> NonNull<V> {
    let mut parent_ref = self.parent_ref;
    let (new_node, leaf_value_ptr) = match self.node_ref.downcast() {
      NodeKind::Internal(internal) => {
        match internal.insert_node(key, value) {
          (Either::Left(origin_internal_node), leaf_value_ptr) => (unsafe { origin_internal_node.inner() }, leaf_value_ptr),
          (Either::Right(new_internal_node), leaf_value_ptr) => (unsafe { new_internal_node.inner() }, leaf_value_ptr)
        }
      }
      NodeKind::Leaf(leaf) => {
        let (new_node, leaf_value_ptr) = leaf.insert_node(key, value);
        (unsafe { new_node.inner() }, leaf_value_ptr)
      }
    };

    if let Some(mut ptr) = parent_ref {
      unsafe {
        *ptr.as_mut() = new_node;
      }
    }

    leaf_value_ptr
  }
}

impl<'a, V> NodeRef<Mut<'a>, V, Internal> {
  /// Insert into current node.
  ///
  /// # Returns
  ///
  /// When current node doesn't need to split to new node, just return itself.
  /// Otherwise, returns a newly created node.
  fn insert_node(mut self, key: &[u8], value: V) -> (Either<Self, NodeRef<Owned, V, Internal>>, NonNull<V>) {
    let this_partial_key = self.as_internal_ref().partial_key();
    let input_partial_key = &key[self.as_internal_ref().prefix_len()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    // Insert self
    if common_key_len < this_partial_key.len() {
      let mut new_parent =
          unsafe { InternalNode4::new(self.as_internal_ref().prefix_len() + common_key_len) };
      new_parent
          .base_mut()
          .set_partial_key(&this_partial_key[0..common_key_len]);


      // First create leaf node using new value
      let leaf_value_ptr = if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        let mut new_leaf_node = LeafNode::new(
          self.as_internal_ref().prefix_len() + common_key_len + 1,
          key,
          value,
        );
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        if let Some(_) = unsafe {
          new_parent.insert_child(
            new_k,
            NodeRef::from_new_leaf_node(new_leaf_node).inner(),
          )
        } {
          unreachable!("This should not happen!");
        }

        leaf_value_ptr
      } else {
        let mut new_leaf_node = LeafNode::new(key.len(), key, value);
        let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
        new_parent.base_mut().set_leaf(new_leaf_node);
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

        self.as_internal_mut().set_partial_key(new_partial_key);

        if let Some(_) = unsafe { new_parent.insert_child(new_k, self.inner()) } {
          unreachable!("This should not happen!");
        }
      }

      (Either::Right(NodeRef::from_new_internal_node(new_parent)), leaf_value_ptr)
    } else {
      let mut new_leaf_node = LeafNode::new(
        self.as_internal_ref().prefix_len() + common_key_len,
        key,
        value,
      );
      let leaf_value_ptr = NonNull::from(new_leaf_node.value_mut());
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        if let Some(_) = unsafe {
          self.as_internal_mut().insert_child(
            new_k,
            NodeRef::from_new_leaf_node(new_leaf_node).inner(),
          )
        } {
          unreachable!("This should not happen!")
        }
      } else {
        self.as_internal_mut().set_leaf(new_leaf_node);
      }

      (Either::Left(self), leaf_value_ptr)
    }
  }
}

impl<'a, V> NodeRef<Mut<'a>, V, Leaf> {
  fn insert_node(mut self, key: &[u8], value: V) -> (NodeRef<Owned, V, Internal>, NonNull<V>) {
    let this_partial_key = self.as_leaf_ref().partial_key();
    let input_partial_key = &key[self.as_leaf_ref().partial_key().len()..];

    let common_key_len = common_len(this_partial_key, input_partial_key);

    let mut new_parent =
        unsafe { InternalNode4::new(self.as_base_ref().prefix_len() + common_key_len) };
    new_parent
        .base_mut()
        .set_partial_key(&this_partial_key[0..common_key_len]);

    // Insert new leaf node
    let leaf_value_ptr = {
      if common_key_len < input_partial_key.len() {
        let new_k = input_partial_key[common_key_len];
        let new_prefix_len = self.as_base_ref().prefix_len() + common_key_len + 1;
        let mut new_leaf = LeafNode::new(new_prefix_len, key, value);
        let leaf_value_ptr = NonNull::from(new_leaf.value_mut());
        unsafe {
          new_parent.insert_child(
            new_k,
            NodeRef::from_new_leaf_node(new_leaf).inner(),
          );
        }
        leaf_value_ptr
      } else {
        let mut new_leaf = LeafNode::new(key.len(), key, value);
        let leaf_value_ptr = NonNull::from(new_leaf.value_mut());
        new_parent.base_mut().set_leaf(new_leaf);
        leaf_value_ptr
      }
    };

    // Insert current node
    {
      if common_key_len < this_partial_key.len() {
        let new_k = this_partial_key[common_key_len];
        self.set_prefix_len(self.as_base_ref().prefix_len() + common_key_len + 1);
        unsafe {
          new_parent.insert_child(new_k, self.inner());
        }
      } else {
        self.set_prefix_len(self.as_base_ref().prefix_len() + common_key_len);
        unsafe {
          new_parent
              .base_mut()
              .set_leaf(Box::from_raw(self.as_leaf_mut()));
        }
      }
    }

    (NodeRef::from_new_internal_node(new_parent), leaf_value_ptr)
  }
}
