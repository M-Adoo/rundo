#![feature(proc_macro)]
#![feature(decl_macro)]

#[cfg(test)]
mod test {
    use rundo_types::prelude::*;
    use rundo_attrs::rundo;
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

    fn action_modify(space: &mut Space, x: f32, y: f32) -> &Space {
        space.begin_op();
        *space.data.x = x;
        *space.data.y = y;
        space.end_op();
        space
    }

    #[test]
    fn stack_len() {
        let mut ws = new_space();

        {
            *ws.data.x = 5.0;
        }

        action_modify(&mut ws, 1.0, 2.0);

        assert_eq!(ws.stack.len(), 1);
        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.robot_ops_len(), 0);

        action_modify(&mut ws, 2.0, 3.0);
        assert_eq!(ws.stack.len(), 2);
        assert_eq!(ws.robot_ops_len(), 0);
        assert_eq!(ws.ops_len(), 2);
    }

    #[test]
    fn nest_batch() {
        let mut ws = new_space();
        {
            ws.begin_op();
            {
                // get_mut will auto capture op
                let mut data = ws.get_mut();
                *data.x = 5.0;
            }
            assert_eq!(ws.ops_len(), 0);

            *ws.data.x = 6.0;
            assert_eq!(ws.ops_len(), 0);
            ws.end_op();
        }

        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.iter.curr, 1);
    }

    #[test]
    fn rollback() {
        let mut ws = new_space();
        {
            ws.begin_op();
            *ws.data.x = 5.0;
            ws.end_op();
        }

        {
            ws.begin_op();
            *ws.data.x = 4.0;
            ws.rollback();
            assert!(!ws.data.dirty());
            ws.begin_op();
        }

        assert_eq!(*ws.data.x, 5.0);
        assert_eq!(ws.ops_len(), 1);
    }

    #[test]
    fn stack_overwrite() {
        let mut ws = new_space();
        action_modify(&mut ws, 1.0, 1.0);
        action_modify(&mut ws, 2.0, 2.0);
        action_modify(&mut ws, 3.0, 3.0);
        assert_eq!(*ws.data.x, 3.0);

        ws.undo();
        ws.undo();
        assert_eq!(*ws.data.x, 1.0);
        action_modify(&mut ws, 4.0, 4.0);
        assert_eq!(ws.ops_len(), 2);

        ws.redo();
        assert_eq!(*ws.data.x, 4.0);

        ws.undo();
        assert_eq!(*ws.data.x, 1.0);
        ws.undo();
        assert_eq!(*ws.data.x, 0.0);
        ws.undo();
        assert_eq!(*ws.data.x, 0.0);
        ws.undo();
        assert_eq!(*ws.data.x, 0.0);

        println!("workspace ops is {:?}", ws.stack);
        ws.redo();
        assert_eq!(*ws.data.x, 1.0);
        ws.redo();
        assert_eq!(*ws.data.x, 4.0);
        ws.redo();
        assert_eq!(*ws.data.x, 4.0);
    }

    #[test]
    fn undo_redo() {
        let mut ws = new_space();

        action_modify(&mut ws, 1.0, 1.0);
        assert_eq!(ws.ops_len(), 1);
        assert_eq!(ws.robot_ops_len(), 0);

        action_modify(&mut ws, 2.0, 2.0);
        assert_eq!(ws.ops_len(), 2);
        assert_eq!(*ws.data.y, 2.0);

        action_modify(&mut ws, 3.0, 3.0);
        assert_eq!(ws.ops_len(), 3);
        assert_eq!(*ws.data.y, 3.0);

        ws.undo();
        assert_eq!(*ws.data.y, 2.0);
        ws.undo();
        assert_eq!(*ws.data.y, 1.0);
        ws.redo();
        assert_eq!(*ws.data.y, 2.0);
        ws.undo();
        assert_eq!(*ws.data.y, 1.0);
        ws.undo();
        assert_eq!(*ws.data.y, 0.0);
        ws.undo();
        assert_eq!(*ws.data.y, 0.0);
        ws.redo();
        assert_eq!(*ws.data.y, 1.0);
        ws.redo();
        assert_eq!(*ws.data.y, 2.0);
        ws.redo();
        assert_eq!(*ws.data.y, 3.0);
        ws.redo();
        assert_eq!(*ws.data.y, 3.0);
    }

    #[test]
    fn next_ver() {
        let mut ws = new_space();
        ws.begin_op();
        *ws.data.x = 5.0;
        assert!(ws.next_ver().is_some());
        ws.end_op();
        assert!(ws.next_ver().is_none());
    }
}
