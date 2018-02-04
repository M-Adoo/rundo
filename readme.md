# Rundo

[![Build Status](https://travis-ci.org/M-Adoo/rundo.svg?branch=master)](https://travis-ci.org/M-Adoo/rundo)

Rundo is a redo / undo library for rust which can auto generate undo op. Below is an example to use Rundo.

```rust
#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    x: f32,
    y: f32,
}

fn main(){

  let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
  *space.get_mut().x = 3.0;

  // x was changed to 3.0
  assert_eq!(*space.data.x, 3.0);

  // x will undo to 2.0
  space.undo();
  assert_eq!(*space.data.x, 2.0);

  // x will redo to 3.0
  space.redo();
  assert_eq!(*space.data.x, 3.0);
}
```
Note the macro `Point!` at the first line of main function, why not use `Point {...}` ? That because Rundo redefined origin Point type with the same shape, but support undo redo. You can use it as same as before, but to literal construct must use a same name macro replace.

You can access data or modify with `Workspace.data` directly, like:

```rust
#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    x: f32,
    y: f32,
}

fn main(){
  let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
  {
    let point = &mut space.data;
    // modify also allowed
    *point.x = 3.0;
  }

  // but undo not work, x not rollback to 2.0
  space.undo();
  assert_eq!(*space.data.x, 3.0);
}

```

The change wasn't captured automatic. Generally, change only between in `begin_op` and `end_op` will be captured. Like this example:

```rust
#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    x: f32,
    y: f32,
}

fn main(){

  let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
  space.begin_op();
  {
    let point = &mut space.data;
    *point.x = 5.0;
    *point.y = 5.0;
  }
  // change will be captured, and change op will be generated.
  space.end_op();

  // undo work correctly
  space.undo();
  assert_eq!(*space.data.x, 2.0);
}
```

Use `get_mut` to access data, Rundo will auto help you capture the the changes. 

```rust

#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    x: f32,
    y: f32,
}

fn main(){

  let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
  {
    let point = &mut space.get_mut(); // <+--
    *point.x = 4.0;                   //  +
    *point.y = 4.0;                   //  +  
                                      //  +  change in this scope 
  }                                   // <+- generate a single op. 
                                      
  

  // undo also work.
  space.undo();
  assert_eq!(*space.data.x, 2.0);
}
```
After point lifetime over, all change will be captured and generate an undo operator.

`begin_op` and `end_op` support nested use. This will work:

```compile_fail
space.begin_op();
  // do some change here
space.begin_op();
  // do some change here
space.begin_op();
  // do some change here
space.end_op();
space.end_op();
space.end_op();

```
All change will be collected by the outer most pair `begin_op/end_op` capture.
