#[macro_use]
extern crate rundo_derive;
extern crate types;

pub use types::*;
pub use rundo_derive::*;

pub enum ActionType {
    /// user action means, this action generate by explict called begin_action and end_action
    /// on workspace, and this action record all the changed between begin_action and end_action
    UserAction,
    /// Skip action generate because this is some data changed in workspace before
    /// begin_action called, when begin_action celled, an skip action will be generated.
    /// In mose case, skip action is the change not want be use as a individual undo/redo action.
    SkipAction,
}

/// Workspace is the data store in rundo.
pub struct Workspace<T: Rundo> {
    root: T,
    stack: Vec<(ActionType, T::Op)>,
}

pub struct AutoAction<'a, T: 'static + Rundo> {
    ws: &'a Workspace<T>,
}

impl<T: Rundo> Workspace<T> {
    fn new(root: T) -> Self {
        return Workspace {
            root: root,
            stack: vec![],
        };
    }

    fn auto_action(&self) -> AutoAction<T> {
        return AutoAction { ws: &self };
    }

    fn begin_action() {
        unimplemented!();
    }

    fn end_action() {
        unimplemented!();
    }

    fn redo() {
        unimplemented!();
    }

    fn undo() {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Rundo)]
    struct Point {
        x: f32,
        y: f32,
    }

    fn workspace() {
        let point = R_Point::from(Point { x: 1.0, y: 2.0 });
        let ws = Workspace::new(point);
        let aa = ws.auto_action();
    }

}
