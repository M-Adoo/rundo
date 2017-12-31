use std::ops::{Deref, DerefMut};
use std::convert::From;

/// Value type like a memory rundo/redo type.
/// Rundo will clone its origin value as a backup, so Clone must be implemented.
/// **Be careful use it for struct or other big size type**,
/// OpType is design for this scenario, or you must implment your custrom rundo type.
pub struct ValueType<T: Clone> {
    value: T,
    origin: Option<T>,
}

/// impl Deref let ValueType<T> transparent to user access T value.
impl<T: Clone> Deref for ValueType<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// when user try to get a mut refercence, Rundo it will change the value later.
impl<T: Clone> DerefMut for ValueType<T> {
    fn deref_mut(&mut self) -> &mut T {
        // when try to change the leaf value, ensure recorded origin value.
        if self.origin.is_none() {
            self.origin = Some(self.value.clone());
        }
        &mut self.value
    }
}

impl<T: Clone> From<T> for ValueType<T> {
    fn from(from: T) -> Self {
        ValueType {
            value: from,
            origin: None,
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
