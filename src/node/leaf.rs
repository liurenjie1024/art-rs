use std::ptr::NonNull;

pub struct LeafRange<V> {
    start: Option<NonNull<Leaf<V>>>,
    end: Option<NonNull<Leaf<V>>>
}

pub(crate) type LeafRef<V> = NonNull<Leaf<V>>;

pub struct Leaf<V> {
    key: Vec<u8>,
    value: V,
    prev: Option<NonNull<Leaf<V>>>,
    next: Option<NonNull<Leaf<V>>>
}