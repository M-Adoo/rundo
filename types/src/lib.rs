#![feature(specialization)]

mod primitive_type;
mod op_type;

pub use primitive_type::ValueType;
pub use op_type::OpType;

pub trait Rundo {
    fn dirty(&self) -> bool;
}

struct Op<T> {
    v: T,
}

pub trait Node {
    fn ops();
    fn apply_op();
    fn revert();
}
