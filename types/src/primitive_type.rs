use std::ops::{Deref, DerefMut};
use std::convert::From;
use std::cmp::PartialEq;
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

impl<T> Rundo for ValueType<T>
where
    T: Clone + PartialEq,
{
    fn dirty(&self) -> bool {
        match self.origin {
            Some(ref ori) => *ori == self.value,
            None => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number() {
        let mut leaf = ValueType::from(5);
        *leaf = 6;
        assert_eq!(leaf.value, 6);
        assert_eq!(leaf.origin, Some(5));
    }

    #[test]
    fn string() {
        let mut leaf = ValueType::from("hello world");
        *leaf = "hello adoo!";
        assert_eq!(leaf.value, "hello adoo!");
        assert_eq!(leaf.origin, Some("hello world"));
    }
}
