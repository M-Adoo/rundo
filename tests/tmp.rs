#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate rundo;
use rundo::prelude::*;

#[rundo]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn tmp() {
    // Note here macro Point, Rundo redefine your type Point with same shape, but support undo redo.
    // You can use it as same as before, but with literal construct must use a same name macro.
    let space = Workspace::new(Point! {x: 2.0, y: 2.0,});

    {
        // generate a guard to auto collect a action.
        let _guard = space.capture_op();
        // get your point
        let mut pt = space.borrow_data_mut();
        *pt.x = 3.0;
    }

    // x was changed to 3.0
    assert_eq!(*space.borrow_data().x, 3.0);

    // x will undo to 2.0
    space.undo();
    assert_eq!(*space.borrow_data().x, 2.0);

    // x will redo to 3.0
    space.redo();
    assert_eq!(*space.borrow_data().x, 3.0);
}
