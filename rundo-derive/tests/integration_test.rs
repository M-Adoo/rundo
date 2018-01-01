#[macro_use]
extern crate rundo_derive;
extern crate types;

use rundo_derive::*;
use types::*;

#[derive(Rundo)]
struct Point {
    a: i32,
    b: i32,
}

#[test]
fn test_dirty() {
    let mut pt = R_Point::from(Point { a: 1, b: 2 });
    assert_eq!(*pt.a, 1);
    assert!(!pt.dirty());
    *pt.a = 4;
    assert!(pt.dirty());
    assert_eq!(*pt.a, 4);
}
