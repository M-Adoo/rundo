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

mod wrap {
    #[derive(Rundo)]
    pub struct CmplxStruct {
        private_field: i32,
        pub pub_field: f32,
    }

    impl CmplxStruct {
        pub fn new(private_field: i32, pub_field: f32) -> CmplxStruct {
            CmplxStruct {
                private_field,
                pub_field,
            }
        }
    }
}

#[test]
fn test_pub_life() {
    use wrap::*;
    let cmplx = R_CmplxStruct::from(CmplxStruct::new(3, 32.0));
    assert_eq!(*cmplx.pub_field, 32.0);
}
