use std::ops::{Deref, DerefMut};
use std::convert::From;
use std::cmp::PartialEq;
use std::convert::{ AsRef, AsMut };
use std::fmt::Debug;

use super::Rundo;

/// Value type like a memory undo/redo type.
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

impl<T> AsMut<T> for ValueType<T> 
where T: 'static + Clone + PartialEq {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> AsRef<T> for ValueType<T>
 where T: 'static + Clone + PartialEq {
     fn as_ref(&self) -> &T {
         &self.value
     }
}

pub trait Primitive {}

impl Primitive  for bool {}
impl Primitive  for char {}
impl Primitive  for i8 {}
impl Primitive  for u8 {}
impl Primitive  for i16 {}
impl Primitive  for u16 {}
impl Primitive  for i32 {}
impl Primitive  for u32 {}
impl Primitive  for i64 {}
impl Primitive  for u64 {}
impl Primitive  for f32 {}
impl Primitive  for f64 {}
impl Primitive  for isize {}
impl Primitive  for usize {}

impl<T> Rundo for ValueType<T>
where
    T: Clone + PartialEq + Debug + Primitive,
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

    fn change_op(&self) -> Option<Self::Op> {
        match self.origin {
            Some(ref ori) => Some(
                VtOp {
                    prev: ori.clone(),
                    curr: self.value.clone(),
                },
            ),
            None => None,
        }
    }

    fn back(&mut self, op: &Self::Op) {
        debug_assert_eq!(self.value, op.curr);
        self.value = op.prev.clone();
        self.reset();
    }

    fn forward(&mut self, op: &Self::Op) {
        debug_assert_eq!(op.prev, self.value);
        self.value = op.curr.clone();
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! type_test {
        ($init: expr, $new:expr) => {
            {
                let mut leaf = ValueType::from($init.clone());
                assert!(!leaf.dirty());

                *leaf = $new.clone();
                assert_eq!(leaf.value, $new.clone());
                assert_eq!(leaf.origin, Some($init.clone()));
                assert!(leaf.dirty());

                let op = leaf.change_op().expect("should have op here");
                assert_eq!(op.prev, $init.clone());
                assert_eq!(op.curr, $new.clone());

                leaf.reset();
                assert!(!leaf.dirty());
                assert!(leaf.change_op().is_none());
                
                // test back forward
                *leaf = $new.clone();
                leaf.back(&op);
                assert_eq!(leaf.value, $init.clone());
                assert!(!leaf.dirty());

                assert_eq!(leaf.value, $init.clone());
                leaf.forward(&op);
                assert_eq!(leaf.value, $new.clone());
                assert!(!leaf.dirty());
            }
        }
    }

    #[test]
    fn i32() {
        type_test!(5, 6);
    }

    #[test]
    fn f32() {
        type_test!(3.0, 2.0);
    }

    #[test]
    fn i8() {
        type_test!(1i8, 2i8)
    }
}
