pub mod rundo {
    use std::vec;
    use std::ops::{Deref, DerefMut};
    use std::convert::From;

    struct Op<T> {
        v: T,
    }

    pub trait Node {
        fn ops();
        fn applyOp();
        fn revert();
    }

    struct NodeWrap<T: Clone> {
        dirty: bool,
        value: T,
        ops: Option<vec::Vec<Op<T>>>,
    }

    struct LeafWrap<T: Clone> {
        value: T,
        origin: Option<T>,
    }

    // impl Deref let leafWrap<T> transparent to user access T value.
    impl<T: Clone> Deref for LeafWrap<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.value
        }
    }

    impl<T: Clone> DerefMut for LeafWrap<T> {
        fn deref_mut(&mut self) -> &mut T {
            // when try to change the leaf value, ensure recorded origin value.
            if self.origin.is_none() {
                self.origin = Some(self.value.clone());
            }
            &mut self.value
        }
    }

    impl<T: Clone> From<T> for LeafWrap<T> {
        fn from(from: T) -> Self {
            LeafWrap {
                value: from,
                origin: None,
            }
        }
    }

    // todo implemant RundoNode for Node<T>
    // todo serder

    #[test]
    fn test() {
        let mut leaf = LeafWrap::from(5);
        *leaf = 6;
        assert_eq!(leaf.value, 6);
        assert_eq!(leaf.origin, Some(5));
    }
}
