use crate::node::InternalNodeRef;
use crate::node::LeafNodeRef;
use crate::node::{NodeKind, NodeRef, DEFAULT_TREE_DEPTH};

struct SearchStackEntry<V> {
  node: InternalNodeRef<V>,
  key: u8,
}

pub(crate) enum SearchResult<R> {
  GoUp,
  GoDown(usize),
  Found(R),
}

#[derive(Copy, Clone)]
pub(crate) struct SearchArgument<'a> {
  key: &'a [u8],
  depth: usize,
}

impl<V> NodeRef<V> {
  /// Find first leaf node, whose keys is not less than input key.
  pub(crate) fn find_lower_bound_node(self, key: &[u8]) -> Option<LeafNodeRef<V>> {
    let mut stack = Vec::<SearchStackEntry<V>>::with_capacity(DEFAULT_TREE_DEPTH);

    let mut cur = self;
    let mut arg = SearchArgument { key, depth: 0 };

    loop {
      match cur.downcast() {
        NodeKind::Internal(node) => match node.find_lower_bound(arg) {
          SearchResult::GoUp => break,
          SearchResult::Found(ret) => {
            return Some(ret);
          }
          SearchResult::GoDown(new_depth) => {
            let k = key[new_depth];
            if let Some(c) = node.find_child(k) {
              stack.push(SearchStackEntry { node, key: k });
              arg.depth = new_depth + 1;
              cur = c;
            } else {
              break;
            }
          }
        },
        NodeKind::Leaf(node) => {
          if node.is_lower_bound(arg) {
            return Some(node);
          } else {
            break;
          }
        }
      }
    }

    while let Some(entry) = stack.pop() {
      if let Some(c) = entry.node.find_next_child(entry.key) {
        return Some(c.minimum_leaf());
      }
    }

    None
  }
}

impl<'a> SearchArgument<'a> {
  #[inline(always)]
  pub(crate) fn partial_key(&self) -> &[u8] {
    // This is possible when that last node of key matches some internal node,
    // and we need to go down.
    if self.depth >= self.key.len() {
      &[0u8; 0]
    } else {
      &self.key[self.depth..]
    }
  }

  #[inline(always)]
  pub(crate) fn depth(&self) -> usize {
    self.depth
  }
}
