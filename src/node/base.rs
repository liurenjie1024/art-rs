pub(in crate::node) struct NodeBase {
    prefix: Vec<u8>,
    empty_value: *mut u8,
    children_count: u8,
}

impl NodeBase {
    pub(in crate::node) unsafe fn get_empty_value(&self) -> Option<*const u8> {
        if self.empty_value.is_null() {
            None
        } else {
            Some(&*self.empty_value)
        }
    }

    pub(in crate::node) unsafe fn search(&self, keys: &[u8]) -> Option<*const u8> {
        match self.match_prefix(keys) {
            PrefixMatchResult::Fail => None,
            PrefixMatchResult::Exact => self.get_empty_value(),
            PrefixMatchResult::Extra => todo!()
        }
    }

    fn match_prefix(&self, keys: &[u8]) -> PrefixMatchResult {
        let prefix_size = self.prefix_size();
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

    #[inline]
    fn prefix_size(&self) -> usize {
        self.prefix.len()
    }
}

enum PrefixMatchResult {
    Fail,
    Exact,
    Extra
}
