#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;

use types::{Rundo, ValueType};
use attrs::rundo;

#[rundo]
struct test {
    x: i32,
    y: f32,
}
