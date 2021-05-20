use std::ptr::NonNull;

pub(in crate::node) struct Leaf {
    key: Vec<u8>,
    value: NonNull<u8>,
    prev: *mut Leaf,
    next: *mut Leaf,
}