use std::ops::{Deref, DerefMut};
use bson::oid::ObjectId;

pub use rundo_types::*;
pub use rundo_attrs::*;

#[derive(PartialEq, Debug)]
pub enum WarkSpaceOp<T> {
    /// user Op means, user manaual called capture_op on workspace,
    /// and this Op record all the changed between the RefGuard lifetime
    UserOp((ObjectId, T)),
    /// Robot Op means, some data change occurs not in any RefGuard lifetime.
    /// In most case Robot Op come from server, or sync from other client change.
    /// An Robot Op will not become an individual undo/redo Op, but
    /// will be comsumed by the nearest UserOp, when do undo redo.
    RobotOp((ObjectId, T)),
}

/// RefGuard is an help object to auto record op
pub struct RefGuard<'a, T: 'static + Rundo> {
    ws: &'a mut Workspace<T>,
}

impl<'a, T> Drop for RefGuard<'a, T>
where
    T: 'static + Rundo,
{
    fn drop(&mut self) {
        self.ws.end_op();
    }
}

impl<'a, T> Deref for RefGuard<'a, T>
where
    T: 'static + Rundo,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.ws.data
    }
}

impl<'a, T> DerefMut for RefGuard<'a, T>
where
    T: 'static + Rundo,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ws.data
    }
}

#[doc(hidden)]
pub struct SpaceIter {
    pub(crate) base: usize,
    pub(crate) curr: usize,
}

/// Workspace is the data store in rundo.
pub struct Workspace<T: Rundo + 'static> {
    pub data: T,
    pub(crate) stack: Vec<WarkSpaceOp<T::Op>>,
    pub(crate) user_ops_len: usize,
    pub(crate) batch: i32,
    pub(crate) version: Option<ObjectId>,
    pub(crate) iter: SpaceIter,
}

const STACK_DEFAULT_SIZE: usize = 128;

impl<T: Rundo> Workspace<T> {
    pub fn new(data: T) -> Self {
        return Workspace {
            data,
            stack: Vec::with_capacity(STACK_DEFAULT_SIZE),
            user_ops_len: 0,
            batch: 0,
            version: None,
            iter: SpaceIter { base: 0, curr: 0 },
        };
    }

    pub fn begin_op(&mut self) {
        if self.batch == 0 {
            let oid = ObjectId::new().expect("rundo generate version objectid failed");
            self.version = Some(oid);
            self.data.reset();
        }
        self.batch += 1;
    }

    pub fn end_op(&mut self) {
        self.batch -= 1;
        assert!(self.batch >= 0, "stack batch should never less than zero!!!
        some like you not always paired call begin_op and end_op, this always stand for a serious bug.");

        if self.batch == 0 {
            if let Some(op) = self.data.change_op() {
                self.data.reset();
                let curr = self.iter.curr;
                self.stack.drain(curr..);
                {
                    let oid = self.version.as_ref().unwrap();
                    self.stack.push(WarkSpaceOp::UserOp((oid.clone(), op)));
                }
                self.version = None;
                self.user_ops_len += 1;
                self.iter.curr += 1;
            }
        }
    }

    pub fn get_mut(&mut self) -> RefGuard<T> {
        self.begin_op();
        RefGuard { ws: self }
    }

    /// halfway cancel the operator which not filished
    pub fn rollback(&mut self) {
        if let Some(op) = self.data.change_op() {
            self.data.back(&op);
        }
    }

    pub fn redo(&mut self) {
        let curr_pos = self.iter.curr;
        let stack = &self.stack[curr_pos..];
        let idx = stack.iter().position(|e| {
            if let &WarkSpaceOp::UserOp(ref _op) = e {
                true
            } else {
                false
            }
        });
        let data = &mut self.data;
        let iter = &mut self.iter;
        let user_ops_len = &mut self.user_ops_len;
        if let Some(i) = idx {
            (0..i + 1).for_each(|i| {
                let op = match stack[i] {
                    WarkSpaceOp::RobotOp(ref op) => op,
                    WarkSpaceOp::UserOp(ref op) => op,
                };
                data.forward(&op.1);
                iter.curr += 1;
                *user_ops_len += 1;
            })
        }
    }

    pub fn undo(&mut self) {
        let curr_pos = self.iter.curr;

        // stack top must be a user op, and use it as undo start.
        // find the last second user op as the undo end.
        let stack = &self.stack[..curr_pos];
        let idx = stack.iter().rposition(|e| {
            if let &WarkSpaceOp::UserOp(ref _op) = e {
                true
            } else {
                false
            }
        });

        let data = &mut self.data;
        let iter = &mut self.iter;
        let user_ops_len = &mut self.user_ops_len;
        if let Some(idx) = idx {
            (idx..stack.len()).rev().for_each(|i| {
                let op = match stack[i] {
                    WarkSpaceOp::RobotOp(ref op) => op,
                    WarkSpaceOp::UserOp(ref op) => op,
                };
                data.back(&op.1);
                iter.curr -= 1;
                *user_ops_len -= 1;
            });
        };
    }

    pub fn zip() {
        unimplemented!()
    }

    pub fn ops_len(&self) -> usize {
        self.user_ops_len
    }

    pub fn robot_ops_len(&self) -> usize {
        let stack_len = self.stack.len() - self.iter.base;
        stack_len - self.ops_len()
    }

    pub fn next_ver(&self) -> Option<ObjectId> {
        return self.version.clone();
    }
}
