use std::ops::{Deref, DerefMut};
use std::convert::From;
use std::cmp::PartialEq;
use std::fmt::Debug;
use super::Rundo;

/// Value type like a memory rundo/redo type.
/// Rundo will clone its origin value as a backup, so Clone must be implemented.
/// **Be careful use it for struct or other big size type**,
/// OpType is design for this scenario, or you must implment your custrom rundo type.
pub struct ValueType<T>
where
    T: Clone + PartialEq,
{
    value: T,
    origin: Option<T>,
}

/// impl Deref let ValueType<T> transparent to user access T value.
impl<T> Deref for ValueType<T>
where
    T: Clone + PartialEq,
{
    type Target = T;
    fn deref(&self) -> &T {
        println!("derefed vaule");
        &self.value
    }
}

/// when user try to get a mut refercence, Rundo it will change the value later.
impl<T> DerefMut for ValueType<T>
where
    T: Clone + PartialEq,
{
    fn deref_mut(&mut self) -> &mut T {
        // when try to change the leaf value, ensure recorded origin value.
        if self.origin.is_none() {
            self.origin = Some(self.value.clone());
        }
        &mut self.value
    }
}

impl<T> From<T> for ValueType<T>
where
    T: Clone + PartialEq,
{
    fn from(from: T) -> Self {
        ValueType {
            value: from,
            origin: None,
        }
    }
}

#[derive(Debug)]
pub struct VtOp<T> {
    prev: T,
    curr: T,
}

impl<T> Rundo for ValueType<T>
where
    T: Clone + PartialEq,
{
    type Op = VtOp<T>;

    fn dirty(&self) -> bool {
        match self.origin {
            Some(ref ori) => *ori != self.value,
            None => false,
        }
    }

    fn reset(&mut self) {
        self.origin = None;
    }

    fn change_ops(&self) -> Option<Vec<Self::Op>> {
        match self.origin {
            Some(ref ori) => Some(vec![
                VtOp {
                    prev: ori.clone(),
                    curr: self.value.clone(),
                },
            ]),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_test<T>(init: T, new: T)
    where
        T: Clone + PartialEq + Debug,
    {
        let mut leaf = ValueType::<T>::from(init.clone());
        assert!(!leaf.dirty());

        *leaf = new.clone();
        assert_eq!(leaf.value, new.clone());
        assert_eq!(leaf.origin, Some(init.clone()));
        assert!(leaf.dirty());

        let mut ops = leaf.change_ops().unwrap();
        assert_eq!(ops.len(), 1);
        let op = ops.pop().unwrap();
        assert_eq!(op.prev, init.clone());
        assert_eq!(op.curr, new.clone());

        leaf.reset();
        assert!(!leaf.dirty());
        //assert_eq!(leaf.change_ops(), None);
    }

    #[test]
    fn test_i32() {
        type_test(5, 6);
    }

    #[test]
    fn test_f32() {
        type_test(3.0, 2.0)
    }

    #[test]
    fn test_string() {
        type_test("hello world".to_owned(), "hello adoo".to_owned());
    }
}
