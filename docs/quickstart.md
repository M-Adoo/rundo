# Quick Start

Rundo can help your data struct has undo redo ability painless. Thanks Rust Procedural Macros, you only need add a attr before your struct!

## Just one line support undo/redo

Assume you have a struct `Point`, as below.

```rust
struct Point {
    x: f32,
    y: f32,
}
```

After add `#[rundo]` before `Point`, undo redo was supported. that all!  A full demo:

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

  // x will chgange to 3.0
  *space.get_mut().x = 3.0;
  assert_eq!(*space.data.x, 3.0);

  // x will chgange to 4.0
  *space.get_mut().x = 4.0;
  assert_eq!(*space.data.x, 4.0);

  // x will undo to 3.0
  space.undo();
  assert_eq!(*space.data.x, 3.0);

  // x will undo to 2.0
  space.undo();
  assert_eq!(*space.data.x, 2.0);

  // x will redo to 3.0
  space.redo();
  assert_eq!(*space.data.x, 3.0);
}
```

Note the macro `Point!` at the first line of main function, why not use `Point {...}` ? That because Rundo redefined origin Point type with the same shape, but support undo redo. You can use it as same as before, but to literal construct must use a same name macro replace. And remenber, to use undo redo, must place struct in Rundo Workspace.

## Batch changes

In some case we want merge mulit change to one action, of cause:

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
    let point = &mut space.get_mut();   // <----+
    // x change many times              //      |
    *point.x = 5.0;                     //      |
    *point.x = 6.0;                     //      |
    *point.x = 7.0;                     //      |
    *point.x = 8.0;                     //      |
  }//--------- point life time -----------------+

  // undo will direct back to 2.0
  space.undo();
  assert_eq!(*space.data.x, 2.0);
}
```

Like what your see, all change in `point life time` will merge together.

Multi `get_mut()` life time can also be batched:

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
    let point = &mut space.get_mut();
    *point.x = 3.0;
  }

  {
    let point = &mut space.get_mut();
    *point.x = 4.0;
  }
  space.end_op();

  // undo will direct back to 2.0
  space.undo();
  assert_eq!(*space.data.x, 2.0);
}
```

`begin_op` and `end_op` can be nested pair call.


You can access and modify `Point` across `Workspace.data` directly, like:

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

if you directly modify value, the change isn't captured automatic. You can manaul capture by `begin_op` and `end_op`.

## Skip Specified Field

If some field you don't want use undo redo, `#[rundo(skip)]` can skip it.

```rust
#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    #[rundo(skip)]
    x: f32,
    y: f32,
}

fn main() {
    let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
    space.get_mut().x = 5.0;
    *space.get_mut().y = 6.0;

    space.undo();
    // x change will be not capture, undo will not affect it.
    assert_eq!(space.data.x, 5.0);
    // but y is undo to 2.0
    assert_eq!(*space.data.y, 2.0);
}
```

## Custom Impl Rundo

You have a special struct, and want to implement undo redo by yourself, that easy, just implement the Rundo Trait.

.... Sorry, It's time to sleep, I will write this section in this weekend.