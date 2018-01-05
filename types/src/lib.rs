#![feature(specialization)]

mod primitive_type;

pub use primitive_type::ValueType;

pub trait Op {}

/// Every rundo node must implement Rundo trait to support undo/redo.
/// In most of case, you can derive Rundo,
/// of course, you can implement it by yourself.
pub trait Rundo {
    /// if this node has been changed between from the last step to current.
    fn dirty(&self) -> bool;
    /// Use Op to describe the change infos.
    fn change_ops(&self) -> Vec<Box<Op>>;
    /// Reset the node change state which mean changes has been record by workspace,
    /// or changes will be ignore.
    fn reset(&mut self);
}
