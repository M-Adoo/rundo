#[cfg(test)]
mod test {
    #![feature(proc_macro)]
    #![feature(decl_macro)]

    use std;
    use types::*;
    use attrs::rundo;
    use workspace::Workspace;

    #[rundo]
    struct Point {
        x: f32,
        y: f32,
    }

    type Space = Workspace<Point>;
    fn new_space() -> Space {
        Workspace::new(Point! { x: 0.0, y: 0.0 })
    }

    fn action_modify(space: &Space, x: f32, y: f32) -> &Space {
        let _guard = space.capture_op();
        *space.root.borrow_mut().x = x;
        *space.root.borrow_mut().y = y;

        space
    }

    #[test]
    fn stack_len() {
        let ws = new_space();

        {
            let mut root = ws.borrow_data_mut();
            let x = root.x.as_mut();
            *x = 5.0;
        }

        action_modify(&ws, 1.0, 2.0);

        assert_eq!(ws.stack.borrow().len(), 1);
        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.robot_ops_len(), 0);

        action_modify(&ws, 2.0, 3.0);
        assert_eq!(ws.stack.borrow().len(), 2);
        assert_eq!(ws.robot_ops_len(), 0);
        assert_eq!(ws.ops_len(), 2);
    }

    #[test]
    fn nest_batch() {
        let ws = new_space();
        {
            let _guard = ws.capture_op();
            {
                let _guard = ws.capture_op();
                *ws.borrow_data_mut().x = 5.0;
                assert_eq!(ws.ops_len(), 0);
            }

            *ws.borrow_data_mut().x = 6.0;
            assert_eq!(ws.ops_len(), 0);
        }

        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.iter.borrow().curr, 1);
    }

    #[test]
    fn rollback() {
        let ws = new_space();
        {
            let _guard = ws.capture_op();
            *ws.borrow_data_mut().x = 5.0;
        }

        {
            let _guard = ws.capture_op();
            *ws.borrow_data_mut().x = 4.0;
            ws.rollback();
            assert!(!ws.borrow_data().dirty());
        }

        assert_eq!(*ws.borrow_data().x, 5.0);
        assert_eq!(ws.ops_len(), 1);
    }

    #[test]
    fn stack_overwrite() {
        let ws = new_space();
        action_modify(&ws, 1.0, 1.0);
        action_modify(&ws, 2.0, 2.0);
        action_modify(&ws, 3.0, 3.0);
        assert_eq!(*ws.borrow_data().x, 3.0);

        ws.undo();
        ws.undo();
        assert_eq!(*ws.borrow_data().x, 1.0);
        action_modify(&ws, 4.0, 4.0);
        assert_eq!(ws.ops_len(), 2);

        ws.redo();
        assert_eq!(*ws.borrow_data().x, 4.0);

        ws.undo();
        assert_eq!(*ws.borrow_data().x, 1.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().x, 0.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().x, 0.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().x, 0.0);

        ws.redo();
        assert_eq!(*ws.borrow_data().x, 1.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().x, 4.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().x, 4.0);
    }

    #[test]
    fn undo_redo() {
        let ws = new_space();

        action_modify(&ws, 1.0, 1.0);
        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.robot_ops_len(), 0);

        action_modify(&ws, 2.0, 2.0);
        assert_eq!(ws.ops_len(), 2);
        assert_eq!(*ws.borrow_data().y, 2.0);

        action_modify(&ws, 3.0, 3.0);
        assert_eq!(ws.ops_len(), 3);
        assert_eq!(*ws.borrow_data().y, 3.0);

        ws.undo();
        assert_eq!(*ws.borrow_data().y, 2.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().y, 1.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().y, 2.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().y, 1.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().y, 0.0);
        ws.undo();
        assert_eq!(*ws.borrow_data().y, 0.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().y, 1.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().y, 2.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().y, 3.0);
        ws.redo();
        assert_eq!(*ws.borrow_data().y, 3.0);
    }

}
