#[macro_use]
extern crate rundo_derive;
extern crate types;
use types::{Rundo, ValueType};
use rundo_derive::*;

#[derive(Rundo)]
struct Point {
  a: i32,
  b: i32,
}

#[test]
fn test_simple() {
  let mut pt = R_Point::from(Point { a: 1, b: 2 });
  assert_eq!(*pt.a, 1);
  assert!(!pt.dirty());

  *pt.a = 4;
  assert!(pt.dirty());
  assert_eq!(*pt.a, 4);

  let ops = pt.change_ops().unwrap();
  assert_eq!(ops.len(), 1);

  pt.reset();
  assert!(!pt.dirty());

  let ops = pt.change_ops();
  assert!(ops.is_none());
}

mod wrap {
  use std;
  use types::{Rundo, ValueType};

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

  #[derive(Rundo)]
  pub struct XXX {
    a: i32,
  }
}

#[test]
fn test_visible() {
  use wrap::*;
  let mut cmplx = R_CmplxStruct::from(CmplxStruct::new(3, 32.0));
  assert_eq!(*cmplx.pub_field, 32.0);
  *cmplx.pub_field = 6.0;
  assert_eq!(*cmplx.pub_field, 6.0);

  assert!(cmplx.dirty());

  cmplx.reset();
  assert!(!cmplx.dirty());
  assert!(!cmplx.pub_field.dirty());
}
