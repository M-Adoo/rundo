#[macro_use]
extern crate rundo_derive;
extern crate types;

pub use types::*;
pub use rundo_derive::*;


#[cfg(test)]
mod test {
    pub use super::*;
    #[derive(Rundo)]
    struct A;

    #[test]
    fn test_derive() {
        let a = A;
        assert_eq!(A::diff(), 4);
    }

}
