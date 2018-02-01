#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;

use types::{Rundo, ValueType};
use attrs::rundo;

#[rundo]
struct Test {
    x: i32,
    y: f32,
}

#[test]
fn rundo_basic() {
    let _t = Test! {x: 1, y: 2.0};
}
