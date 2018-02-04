//! # Rundo
//!
//! Rundo is a redo / undo library for rust which can atuo generate actions.
//! Thanks for rust Procedural Macros, Rundo will be disign and implementation to zero-cost support undo-redo in Rust.
//! Rundo dedicated to support undo/redo transparent for user code, it's should be used painless.
//! In most case, just use rundo attrs `#[rundo]` for your custom data struct, that all.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rundo = "0.1"
//! ```
//!
//! # Examples
//!
//! blowe code will show can rundo maight be used.
//!
//! ```
//! #![feature(proc_macro)]
//! #![feature(decl_macro)]
//!
//! extern crate rundo;
//! use rundo::prelude::*;
//!
//! #[rundo]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//！ // Note here the macro `Point`, Rundo redefine your origin Point type
//！ // with the same shape, but support undo redo.
//！ // You can use it as same as before, but to literal construct
//！ // must use a same name macro replace.
//!
//! fn main(){
//!   let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
//!   {
//!     // access data across get_mut will auto collect change action.
//!     *space.get_mut().x = 3.0;
//!   }
//！  // x was changed to 3.0
//！  assert_eq!(*space.borrow_data().x, 3.0);
//！  // x will undo to 2.0
//！  space.undo();
//！  assert_eq!(*space.borrow_data().x, 2.0);
//！  // x will redo to 3.0
//！  space.redo();
//！  assert_eq!(*space.borrow_data().x, 3.0);
//! }
//! ```

#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;
#[cfg(test)]
mod test;

pub mod workspace;

pub mod prelude {
    pub use attrs::*;
    pub use workspace::Workspace;
    pub use types::prelude::*;
}
