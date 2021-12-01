use std::marker::PhantomData;

/// Borrow type of a node.
pub trait BorrowType {}

pub enum Owned {}
pub struct Immut<'a>(PhantomData<&'a ()>);
pub struct Mut<'a>(PhantomData<&'a mut ()>);

impl BorrowType for Owned {}
impl<'a> BorrowType for Immut<'a> {}
impl<'a> BorrowType for Mut<'a> {}

/// Node Types
pub enum InternalOrLeaf {}
pub enum Internal {}
pub enum Leaf {}
