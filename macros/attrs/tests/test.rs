#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro)]
#![feature(trace_macros)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

extern crate attrs;
extern crate types;

use types::{Rundo, ValueType};
use attrs::rundo;

struct OpPoint {
    a: Option<Vec<<ValueType<i32> as Rundo>::Op>>,
    b: Option<Vec<<ValueType<i32> as Rundo>::Op>>,
    //  impl CmplxStruct {
    //    pub fn new(private_field: i32, pub_field: f32) -> CmplxStruct {
    // shorthand literal struct has break by rust bug #46489
    //  CmplxStruct! {
    //    private_field,
    //    pub_field
    //  }
    //    }
    //  }

    //#[test]
    //fn test_visible() {
    //  use wrap::*;
    //  let mut cmplx = CmplxStruct::new(3, 32.0);
    //  assert_eq!(*cmplx.pub_field, 32.0);
    //  *cmplx.pub_field = 6.0;
    //  assert_eq!(*cmplx.pub_field, 6.0);
    //
    //  assert!(cmplx.dirty());
    //
    //  cmplx.reset();
    //  assert!(!cmplx.dirty());
    //  assert!(!cmplx.pub_field.dirty());
    //}
}
struct _Point {
    a: ValueType<i32>,
    b: ValueType<i32>,
}
struct Point {
    value: _Point,
    dirty: bool,
}
impl std::ops::Deref for Point {
    type Target = _Point;
    fn deref(&self) -> &_Point {
        &self.value
    }
}
impl std::ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut _Point {
        if !self.dirty {
            self.dirty = true;
        }
        &mut self.value
    }
}
impl Rundo for Point {
    type Op = OpPoint;
    fn dirty(&self) -> bool {
        self.dirty
    }
    fn reset(&mut self) {
        self.dirty = false;
        self.value.a.reset();
        self.value.b.reset();
    }
    fn change_ops(&self) -> Option<std::vec::Vec<OpPoint>> {
        unimplemented!();
    }
}
macro_rules! Point(( a ) => ( ValueType :: < i32 > :: from ( a ) ) ; (
                   a : $ e : tt ) => ( ValueType :: < i32 > :: from ( $ e ) )
                   ; ( b ) => ( ValueType :: < i32 > :: from ( b ) ) ; (
                   b : $ e : tt ) => ( ValueType :: < i32 > :: from ( $ e ) )
                   ; ( $ ( $ id : ident , ) * ) => {
                   Point {
                   dirty : false , value : _Point {
                   $ ( $ id : Point ! { $ id } ) , * } } } ; (
                   $ ( $ id : ident ) , * ) => {
                   Point {
                   dirty : false , value : _Point {
                   $ ( $ id : Point ! { $ id } ) , * } } } ; (
                   $ ( $ id : ident : $ e : tt , ) * ) => {
                   Point {
                   dirty : false , value : _Point {
                   $ ( $ id : Point ! { $ id : $ e } ) , * } } } ; (
                   $ ( $ id : ident : $ e : tt ) , * ) => {
                   Point {
                   dirty : false , value : _Point {
                   $ ( $ id : Point ! { $ id : $ e } ) , * } } } ;);

mod wrap {
    use super::*;
    pub struct OpCmplxStruct {
        pub_field: Option<Vec<<ValueType<f32> as Rundo>::Op>>,
    }
    pub struct _CmplxStruct {
        pub pub_field: ValueType<f32>,
    }
    pub struct CmplxStruct {
        value: _CmplxStruct,
        dirty: bool,
    }
    impl std::ops::Deref for CmplxStruct {
        type Target = _CmplxStruct;
        fn deref(&self) -> &_CmplxStruct {
            &self.value
        }
    }
    impl std::ops::DerefMut for CmplxStruct {
        fn deref_mut(&mut self) -> &mut _CmplxStruct {
            if !self.dirty {
                self.dirty = true;
            }
            &mut self.value
        }
    }
    impl Rundo for CmplxStruct {
        type Op = OpCmplxStruct;
        fn dirty(&self) -> bool {
            self.dirty
        }
        fn reset(&mut self) {
            self.dirty = false;
            self.value.pub_field.reset();
        }
        fn change_ops(&self) -> Option<std::vec::Vec<OpCmplxStruct>> {
            unimplemented!();
        }
    }
    macro_rules! CmplxStruct(( pub_field ) => (
                             ValueType :: < f32 > :: from ( pub_field ) ) ; (
                             pub_field : $ e : tt ) => (
                             ValueType :: < f32 > :: from ( $ e ) ) ; (
                             $ ( $ id : ident , ) * ) => {
                             CmplxStruct {
                             dirty : false , value : _CmplxStruct {
                             $ ( $ id : CmplxStruct ! { $ id } ) , * } } } ; (
                             $ ( $ id : ident ) , * ) => {
                             CmplxStruct {
                             dirty : false , value : _CmplxStruct {
                             $ ( $ id : CmplxStruct ! { $ id } ) , * } } } ; (
                             $ ( $ id : ident : $ e : tt , ) * ) => {
                             CmplxStruct {
                             dirty : false , value : _CmplxStruct {
                             $ ( $ id : CmplxStruct ! { $ id : $ e } ) , * } }
                             } ; ( $ ( $ id : ident : $ e : tt ) , * ) => {
                             CmplxStruct {
                             dirty : false , value : _CmplxStruct {
                             $ ( $ id : CmplxStruct ! { $ id : $ e } ) , * } }
                             } ;);
}
