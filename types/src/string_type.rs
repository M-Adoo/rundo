use std;
use Rundo;
use ValueType;

#[derive(Debug)]
pub enum StrOP {
    Insert {
        idx: usize,
        value: String,
    },
    Del {
        idx: usize,
        len: usize,
    },
    Update {
        idx: usize,
        len: usize,
        value: String,
    },
}

impl Rundo for ValueType<String>
{
    type Op = std::vec::Vec<StrOP>;

    fn dirty(&self) -> bool {
        unimplemented!();
    }

    fn reset(&mut self) {
        unimplemented!();
    }

    fn change_op(&self) -> Option<Self::Op> {
        unimplemented!();
    }

    fn back(&mut self, op: &Self::Op) {
        self.reset();
        unimplemented!();
    }

    fn forward(&mut self, op: &Self::Op) {
        self.reset();
        unimplemented!();
    }
}
