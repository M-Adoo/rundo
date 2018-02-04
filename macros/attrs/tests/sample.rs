#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo_attrs;
extern crate rundo_types;

use rundo_types::prelude::*;
use rundo_attrs::rundo;

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
