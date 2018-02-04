//! Rundo is a redo / undo library for rust which can auto generate actions.
//!
//! Thanks for rust Procedural Macros, Rundo will be disign and implementation to zero-cost support undo-redo in Rust.
//! Rundo dedicated to support undo/redo transparent for user code, it's should be used painless.
//! In most case, just use rundo attrs `#[rundo]` for your data struct, that all.
//!
//!## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rundo = "0.1"
//! ```
//!
//!## Examples
//!
//! below code will show how can Rundo maight be used.
//!
//! ```
//!#![feature(proc_macro)]
//!#![feature(decl_macro)]
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
//! // Note here the macro `Point`, Rundo redefine your origin Point type
//! // with the same shape, but support undo redo.
//! // You can use it as same as before, but to literal construct
//! // must use a same name macro replace.
//!
//! fn main(){
//!   let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
//!   {
//!     // access data across get_mut will auto collect change action during its life time.
//!     *space.get_mut().x = 3.0;
//!   }
//!
//!  // x was changed to 3.0
//!  assert_eq!(*space.data.x, 3.0);
//!
//!  // x will undo to 2.0
//!  space.undo();
//!  assert_eq!(*space.data.x, 2.0);
//!
//!  // x will redo to 3.0
//!  space.redo();
//!  assert_eq!(*space.data.x, 3.0);
//! }
//! ```
//!
//! You can also manual control change action generate;
//!
//! ```
//! # #![feature(proc_macro)]
//! # #![feature(decl_macro)]
//! #
//! # extern crate rundo;
//! # use rundo::prelude::*;
//! #
//! # #[rundo]
//! # struct Point {
//! #    x: f32,
//! #    y: f32,
//! # }
//! #
//! # fn main() {
//! let mut space = Workspace::new(Point! {x: 2.0, y: 2.0,});
//! space.begin_op();       // form here all chage will be
//!                           // merge to one op until `end_op` called
//!
//! *space.get_mut().x = 5.0;
//! *space.get_mut().y = 6.0;
//! *space.get_mut().x = 3.0;
//!
//! space.end_op();        // generate op
//!
//! // only a user op will be generate
//! space.undo();
//!
//! assert_eq!(*space.data.x, 2.0);
//! assert_eq!(*space.data.y, 2.0);
//!# }
//! ```
//! [README]: https://github.com/M-Adoo/rundo#rundo

#![feature(external_doc)]
#![feature(proc_macro)]
#![feature(decl_macro)]

#[doc(include = "../readme.md")]
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
