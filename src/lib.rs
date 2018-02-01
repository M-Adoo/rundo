#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;
pub mod workspace;
#[cfg(test)]
mod test;

pub mod prelude {
    pub use attrs::*;
    pub use workspace;
    pub use types::prelude;
}

pub use types::Rundo;
