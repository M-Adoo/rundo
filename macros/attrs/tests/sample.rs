#![feature(proc_macro)]
#![feature(trace_macros)]

extern crate attrs;
extern crate types;

use types::{Rundo, ValueType};
use attrs::rundo;

#[rundo]
struct test {
    x: i32,
}
