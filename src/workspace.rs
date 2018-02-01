use std::cell::{Ref, RefCell, RefMut};
pub use types::*;
pub use attrs::*;

#[derive(PartialEq, Debug)]
pub enum WarkSpaceOp<T> {
    /// user Op means, user manaual called capture_op on workspace,
    /// and this Op record all the changed between the OpGuard lifetime
    UserOp(T),
    /// Robot Op means, some data change occurs not in any OpGuard lifetime.
    /// In mose case Robot Op come from server, or sync from other client change.
    /// An Robot Op will not become an individual undo/redo Op, but
    /// will be comsumed by the nearest UserOp, when do undo redo.
    RobotOp(T),
}

/// OpGuard is an help object to auto record op
pub struct OpGuard<'a, T: 'static + Rundo> {
    ws: &'a Workspace<T>,
}

impl<'a, T> Drop for OpGuard<'a, T>
where
    T: 'static + Rundo,
{
    fn drop(&mut self) {
        *self.ws.batch.borrow_mut() -= 1;
        assert!(*self.ws.batch.borrow() >= 0, "stack batch should never less than zero!!!
        some like you not always paired call begin_op and end_op, this always stand for a serious bug.");

        if *self.ws.batch.borrow() == 0 {
            let mut mut_root = self.ws.root.borrow_mut();
            if let Some(op) = mut_root.change_op() {
                mut_root.reset();
                let mut stack = self.ws.stack.borrow_mut();
                let curr = self.ws.iter.borrow().curr;
                stack.drain(curr..);
                stack.push((WarkSpaceOp::UserOp(op)));
                *self.ws.user_ops_len.borrow_mut() += 1;
                self.ws.iter.borrow_mut().curr += 1;
            }
        }
    }
}

pub struct SpaceIter {
    pub(crate) base: usize,
    pub(crate) curr: usize,
}

/// Workspace is the data store in rundo.
pub struct Workspace<T: Rundo + 'static> {
    pub(crate) root: RefCell<T>,
    pub(crate) stack: RefCell<Vec<WarkSpaceOp<T::Op>>>,
    pub(crate) user_ops_len: RefCell<usize>,
    pub(crate) batch: RefCell<i32>,
    pub(crate) iter: RefCell<SpaceIter>,
}

const STACK_DEFAULT_SIZE: usize = 128;

impl<T: Rundo> Workspace<T> {
    pub fn new(root: T) -> Self {
        return Workspace {
            root: RefCell::new(root),
            stack: RefCell::new(Vec::with_capacity(STACK_DEFAULT_SIZE)),
            user_ops_len: RefCell::new(0),
            batch: RefCell::new(0),
            iter: RefCell::new(SpaceIter { base: 0, curr: 0 }),
        };
    }

    pub fn borrow_data_mut(&self) -> RefMut<T> {
        self.root.borrow_mut()
    }

    pub fn borrow_data(&self) -> Ref<T> {
        self.root.borrow()
    }

    pub fn capture_op(&self) -> OpGuard<T> {
        self.borrow_data_mut().reset();
        *self.batch.borrow_mut() += 1;

        OpGuard { ws: self }
    }

    /// halfway cancel the operator which not filished
    pub fn rollback(&self) {
        let mut data = self.borrow_data_mut();
        if let Some(op) = data.change_op() {
            data.back(&op);
        }
    }

    pub fn redo(&self) {
        let curr_pos = self.iter.borrow().curr;
        let stack = &self.stack.borrow()[curr_pos..];
        let idx = stack.iter().position(|e| {
            if let &WarkSpaceOp::UserOp(ref _op) = e {
                true
            } else {
                false
            }
        });

        if let Some(i) = idx {
            (0..i + 1).for_each(|i| {
                let op = match stack[i] {
                    WarkSpaceOp::RobotOp(ref op) => op,
                    WarkSpaceOp::UserOp(ref op) => op,
                };
                self.root.borrow_mut().forward(&op);
                self.iter.borrow_mut().curr += 1;
                *self.user_ops_len.borrow_mut() += 1;
            })
        }
    }

    pub fn undo(&self) {
        let curr_pos = self.iter.borrow().curr;
        let stack = &self.stack.borrow()[..curr_pos];

        // stack top must be a user op, and use it as undo start.
        // find the last second user op as the undo end.
        let idx = stack.iter().rposition(|e| {
            if let &WarkSpaceOp::UserOp(ref _op) = e {
                true
            } else {
                false
            }
        });

        if let Some(idx) = idx {
            (idx..stack.len()).rev().for_each(|i| {
                let op = match stack[i] {
                    WarkSpaceOp::RobotOp(ref op) => op,
                    WarkSpaceOp::UserOp(ref op) => op,
                };
                self.root.borrow_mut().back(&op);
                self.iter.borrow_mut().curr -= 1;
                *self.user_ops_len.borrow_mut() -= 1;
            });
        };
    }

    pub fn zip() {
        unimplemented!()
    }

    pub fn ops_len(&self) -> usize {
        *self.user_ops_len.borrow()
    }

    pub fn robot_ops_len(&self) -> usize {
        let stack_len = self.stack.borrow().len() - self.iter.borrow().base;
        stack_len - self.ops_len()
    }
}
