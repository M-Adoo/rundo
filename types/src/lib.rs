#![feature(specialization)]

mod primitive_type;

pub use primitive_type::ValueType;

pub trait Rundo {
    fn dirty(&self) -> bool;
}

pub struct Op {}

pub trait Node {
    fn ops();
    fn apply_op();
    fn revert();
}
