use std::mem::MaybeUninit;

const MAX_PREFIX_LEN: usize = 16;

pub(in crate::node) struct NodeBase {
    partial_prefix: MaybeUninit<[u8;MAX_PREFIX_LEN]>,
    partial_prefix_len: usize,
    empty_value: *mut u8,
    children_count: u8,
}

impl NodeBase {
    pub(in crate::node) fn search(&self, keys: &[u8]) -> PrefixMatchResult {
        self.match_prefix(keys)
    }

    #[inline]
    pub(in crate::node) fn get_empty_value(&self) -> *const u8 {
        self.empty_value
    }

    #[inline]
    pub(in crate::node) fn get_prefix_size(&self) -> usize {
        self.prefix.len()
    }


    fn match_prefix(&self, keys: &[u8]) -> PrefixMatchResult {
        let prefix_size = self.get_prefix_size();
        if keys.len() < prefix_size {
            PrefixMatchResult::Fail
        } else if keys.len() == prefix_size {
            if self.prefix == keys {
                PrefixMatchResult::Exact
            } else {
                PrefixMatchResult::Fail
            }
        } else {
            if self.prefix == keys[0..prefix_size] {
                PrefixMatchResult::Extra
            } else {
                PrefixMatchResult::Fail
            }
        }
    }
}

pub(in crate::node) enum PrefixMatchResult {
    Fail,
    Exact,
    Extra,
}
