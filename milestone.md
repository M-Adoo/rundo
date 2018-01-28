# milestone

## 0.1 todos

- [x] auto record modify primitive type
- [x] auto record modify struct type
- [x] custom derive
- [x] travis
- [ ] more effective string diff method.
    - [x] diff ops
    - [ ] undo/redo
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

## 0.2

- [ ] support skip special struct field
- [ ] support generic
- [ ] support struct attrs lifetime ...
- [ ] support struct in struct
    - [ ] if user directly replace the nested struct, dirty chain will break, and how to generate current change op?
    - [ ] RefCell will break dirty chain which depend on DerefMut
- [ ] support rebase
- [ ] support ops zip
- [ ] docs
- [ ] serde