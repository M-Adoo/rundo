mod primitive_type;
mod op_type;

pub trait Rundo {
    fn diff() -> i32;
}

struct Op<T> {
    v: T,
}

pub trait Node {
    fn ops();
    fn apply_op();
    fn revert();
}
