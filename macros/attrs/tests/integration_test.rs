#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;

use types::{Rundo, ValueType};
use attrs::rundo;

#[rundo]
struct Point {
  a: i32,
  b: i32,
}

#[test]
fn test_simple() {
  let mut pt = Point! { a: 1, b: 2 };
  assert_eq!(*pt.a, 1);
  assert!(!pt.dirty());

  {
    let mut a = &mut pt.a;
  }
  assert!(pt.dirty());
  *pt.a = 4;
  assert_eq!(*pt.a, 4);


  let ops = pt.change_op().expect("should have op here");

  pt.reset();
  assert!(!pt.dirty());

  let ops = pt.change_op();
  assert!(ops.is_none());
}

mod wrap {
  use super::*;

  #[rundo]
  pub struct CmplxStruct {
    private_field: i32,
    pub pub_field: f32,
  }

  impl CmplxStruct {
    pub fn new(private_field: i32, pub_field: f32) -> CmplxStruct {
      // shorthand literal struct has break by rust bug #46489
      CmplxStruct! {
        private_field: private_field,
        pub_field: pub_field
      }
    }
  }
}

#[test]
fn test_visible() {
  use wrap::*;
  let mut cmplx = CmplxStruct::new(3, 32.0);
  assert_eq!(*cmplx.pub_field, 32.0);
  *cmplx.pub_field = 6.0;
  assert_eq!(*cmplx.pub_field, 6.0);

  assert!(cmplx.dirty());

  cmplx.reset();
  assert!(!cmplx.dirty());
  assert!(!cmplx.pub_field.dirty());
}
