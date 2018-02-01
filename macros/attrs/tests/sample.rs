#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;

use types::prelude::*;
use attrs::rundo;

#[rundo]
struct Test {
    x: i32,
    y: f32,
    z: Embed,
}

#[rundo]
pub struct Embed {
    a: f32,
}

#[test]
fn rundo_basic() {
    let _embed = Embed! {a: 1.0};
    let _t = Test! {x: 1, y: 2.0, z: Embed!{a: 1.0} };
}
