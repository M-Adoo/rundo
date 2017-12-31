#[macro_use]
extern crate rundo;

use rundo::*;

#[derive(Rundo)]
struct A;


#[test]
fn test_derive() {
    assert_eq!(A::diff(), 4);
}
