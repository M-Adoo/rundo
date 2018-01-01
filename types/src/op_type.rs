use std::vec;
use std::ops::{Deref, DerefMut};
use std::convert::From;

use super::{Op, Rundo};

pub struct OpType<T> {
    dirty: bool,
    value: T,
    ops: Option<vec::Vec<Op<T>>>,
}


impl<T> Deref for OpType<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// when user try to get a mut refercence, Rundo it will change the value later.
impl<T> DerefMut for OpType<T> {
    fn deref_mut(&mut self) -> &mut T {
        if !self.dirty {
            self.dirty = true;
        }
        &mut self.value
    }
}

impl<T> From<T> for OpType<T> {
    fn from(from: T) -> Self {
        return OpType {
            dirty: false,
            value: from,
            ops: None,
        };
    }
}

default impl<T> Rundo for OpType<T> {
     default fn dirty(&self) -> bool{
        self.dirty
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use types::primitive_type::ValueType;
    struct Point {
        x: ValueType<f32>,
        y: ValueType<f32>,
    }

    #[test]
    fn dirty() {
        let p = Point {
            x: ValueType::from(2.0),
            y: ValueType::from(3.0),
        };
        let mut p = OpType::from(p);
        assert!(!p.dirty);
        *p.x = 5.0;
        assert!(p.dirty);
    }
}
