#[cfg(test)]
mod test {
    #![feature(proc_macro)]
    #![feature(decl_macro)]
    
    use std;
    use types::*;
    use attrs::*;
    use attrs::rundo;
    use Workspace;

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
        let ws = new_space();

        {
            let mut root = ws.borrow_mut();
            let x = root.x.as_mut();
            *x = 5.0;
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

    #[test]
    fn nest_batch() {
        let ws = new_space();
        {
            let _guard = ws.capture_op();
            {
                let _guard = ws.capture_op();
                *ws.borrow_mut().x = 5.0;
                assert_eq!(ws.ops_len(), 0);
            }

            *ws.borrow_mut().x = 6.0;
            assert_eq!(ws.ops_len(), 0);
        }
            assert_eq!(ws.ops_len(), 1);

    }

    #[test]
    fn rollback() {
        let ws = new_space();
        {
            let _guard = ws.capture_op();
            *ws.borrow_mut().x = 5.0;
        }

        {
            let _guard = ws.capture_op();
            *ws.borrow_mut().x = 4.0;
            ws.rollback();
            assert!(!ws.borrow().dirty());
        }
        assert_eq!(*ws.borrow().x, 5.0);
        assert_eq!(ws.ops_len(), 1);
    }
}