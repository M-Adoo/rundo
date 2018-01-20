#![feature(proc_macro)]
#![feature(decl_macro)]

extern crate attrs;
extern crate types;

use std::cell::RefCell;
pub use types::*;
pub use attrs::*;


pub enum OpType {
    /// user Op means, user manaual called capture_op on workspace,
    /// and this Op record all the changed between the OpGuard lifetime
    UserOp,
    /// Robot Op means, some data change occurs not in any OpGuard lifetime.
    /// The change between after an OpGuard over and before a new OpGuard create compose 
    /// a RobotOP. An RobotOp will not become an individual undo/redo Op, but
    /// will be comsumed by the nearest UserOp, when do undo redo.
    RobotOp,
}

/// OpGuard is an help object to auto record op
pub struct OpGuard<'a, T: 'static + Rundo> {
    ws: &'a Workspace<T>,
}

impl<'a, T> Drop for OpGuard<'a, T> where T: 'static + Rundo  {
    fn drop(&mut self) {
        *self.ws.batch.borrow_mut() -= 1;
        assert!(*self.ws.batch.borrow() >= 0, "stack batch should never less than zero!!!
        some like you not always paired call begin_op and end_op, this always stand for a serious bug.");

        if let Some(op) = self.ws.pick_op() {
            self.ws.stack.borrow_mut().push((OpType::UserOp, op));
            *self.ws.user_ops_len.borrow_mut() += 1;
        }
    }
}

/// Workspace is the data store in rundo.
pub struct Workspace<T: Rundo + 'static> {
    root: RefCell<T>,
    stack: RefCell<Vec<(OpType, T::Op)>>,
    user_ops_len: RefCell<usize>,
    batch: RefCell<i32>,
}

impl<T:Rundo + 'static> AsMut<T> for Workspace<T> {
    fn as_mut(&mut self) -> &mut T {
        self.root.get_mut()
    }
}

const STACK_DEFAULT_SIZE: usize = 128;

impl<T: Rundo> Workspace<T> {
    pub fn new(root: T) -> Self {
        return Workspace {
            root: RefCell::new(root) ,
            stack: RefCell::new(Vec::with_capacity(STACK_DEFAULT_SIZE)) ,
            user_ops_len: RefCell::new(0),
            batch: RefCell::new(0),
        };
    }

    pub fn capture_op(&self) -> OpGuard<T> {
        if let Some(op) = self.pick_op() {
            self.stack.borrow_mut().push((OpType::RobotOp, op));
        }
        *self.batch.borrow_mut() += 1;

        OpGuard {ws: self}
    }

    pub fn redo(&self) {
        unimplemented!();
    }

    pub fn undo(&self) {
        unimplemented!();
    }

    pub fn zip() {
        unimplemented!()
    }

    pub fn ops_len(&self) -> usize{
        *self.user_ops_len.borrow()
    }

    pub fn robot_ops_len(&self) -> usize {
        self.stack.borrow().len() - self.ops_len()
    }

    fn pick_op(&self) -> Option<T::Op>{
         if *self.batch.borrow() == 0 {
            let mut mut_root = self.root.borrow_mut();
            if let Some(op) = mut_root.change_op(){
                mut_root.reset();
                return Some(op);
            }
        }
        None
    }
}

#[cfg(test)]
mod workspace {
    use super::*;
    use attrs::rundo;

    #[rundo]
    struct Point {
        x: f32,
        y: f32,
    }

    type Space = Workspace<Point>;
    fn new_space() -> Space  {
        Workspace::new(Point! { x: 1.0, y: 2.0 })
    }

    fn action_modify(space: & Space, x: f32, y: f32)-> & Space {
        let _guard = space.capture_op();
        *space.root.borrow_mut().x = x;
        *space.root.borrow_mut().y = y;

        space
    }

    #[test]
    fn stack_len() {
        let mut ws = new_space();

        {
            let root = &mut ws.root.borrow_mut();
            let x = &mut root.x;
            **x = 5.0;
        }
        action_modify(&ws, 1.0, 2.0);

        assert_eq!(ws.stack.borrow().len(), 2);
        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.robot_ops_len(), 1);

        action_modify(&ws, 2.0, 3.0);
        assert_eq!(ws.stack.borrow().len(), 3);
        assert_eq!(ws.robot_ops_len(), 1);
        assert_eq!(ws.ops_len(), 2);
    }
}
