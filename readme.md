# Rundo

[![Build Status](https://travis-ci.org/M-Adoo/rundo.svg?branch=master)](https://travis-ci.org/M-Adoo/rundo)
[![Coverage Status](https://coveralls.io/repos/github/M-Adoo/rundo/badge.svg)](https://coveralls.io/github/M-Adoo/rundo)

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

## Documents

[Library API](https://docs.rs/rundo)

[Quick Start](./docs/quickstart.md) 2 min learn how to use Rundo.