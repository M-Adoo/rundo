# milestone

## 0.1 todos

- [x] auto record modify primitive type
- [x] auto record modify struct type
- [x] custom derive
- [x] travis
- [x] more effective string diff method.
    - [x] diff ops
    - [x] undo/redo
- [x] track modify path，
- [x] implement rundo trait
- [x] std::convert::AsRef
- [x] visible key word.
- [x] use custom macro attrs replace derive
- [x] literal macro crate
- [x] workspace
    - [x] support base undo redo
    - [x] auto batch op
    - [x] halfway rollback

- [x] support struct in struct
- [x] docs
## 0.2

- [x] support skip special struct field
- [ ] support generic
- [ ] support struct attrs lifetime ...
- [x] if user directly replace the nested struct, dirty chain will break, and how to generate current change op?
- [x] RefCell will break dirty chain which depend on DerefMut
- [ ] support ops zip