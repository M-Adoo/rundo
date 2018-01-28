use std;
use Rundo;
use primitive_type::ValueType;

use difference::{Difference, Changeset};

#[derive(PartialEq, Debug)]
pub enum StrOP {
    Ins {
        idx: usize,
        value: String,
    },
    Del {
        idx: usize,
        value: String,
    },
    Chg {
        idx: usize,
        from: String,
        to: String,
    },
}

impl Rundo for ValueType<String>
{
    type Op = std::vec::Vec<StrOP>;

    fn dirty(&self) -> bool {
        self.origin.is_some()
    }

    fn reset(&mut self) {
        self.origin = None;
    }

    fn change_op(&self) -> Option<Self::Op> {
        if let Some(ref ori) = self.origin {
            let diffs = Changeset::new(ori, &self.value, "").diffs;
            if diffs.is_empty() {
                None
            } else {
                let mut ops = Vec::with_capacity(diffs.len());
                let mut base = 0;
                for diff in diffs {
                    match diff {
                        Difference::Same(text) => base += text.chars().count(),
                        Difference::Rem(text) => {
                            let len = text.chars().count();
                            ops.push(StrOP::Del{ idx: base, value: text });
                            base += len;
                        },
                        Difference::Add(text) => {
                            let len = text.chars().count();
                            let last_op = ops.pop();
                            if let Some(StrOP::Del {idx, value}) = last_op {
                                ops.push(StrOP::Chg{idx, from: value.to_string(), to: text});
                            } else {
                                if let Some(last_op) = last_op {
                                    ops.push(last_op);
                                }
                                ops.push(StrOP::Ins{ idx: base, value: text });
                                base += len;
                            }
                        }
                    };
                }
                return Some(ops);
            }
        } else {
            return None;
        }
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


#[test]
fn string_ops() {
    let mut hello = ValueType::<String>::from("hello world!".to_string());
    *hello = "hello adoo!".to_string();
    let ops = hello.change_op().unwrap();
    assert_eq!(ops, vec![
        StrOP::Chg {idx: 6, from: "w".to_string(), to: "ad".to_string()},
        StrOP::Chg {idx: 8, from: "rld".to_string(), to: "o".to_string()},
    ]);

    hello.reset();
    *hello = "hello adoo! by Rust.".to_string();
    let ops = hello.change_op().unwrap();
    assert_eq!(ops, vec![
        StrOP::Ins {idx: 11, value: " by Rust.".to_string()},
    ]);

    hello.reset();
    *hello = "by Rust.".to_string();
    let ops = hello.change_op().unwrap();
    assert_eq!(ops, vec![
        StrOP::Del {idx: 0, value: "hello adoo! ".to_string()},
    ])
}

#[test]
fn string_chinese() {
    let mut chinese = ValueType::<String>::from("我白天是程序员".to_string());
    *chinese = "晚上是个学生".to_string();
    let ops = chinese.change_op().unwrap();
    assert_eq!(ops, vec![
        StrOP::Chg {idx: 0, from: "我白天".to_string(), to: "晚上".to_string()},
        StrOP::Chg {idx: 4, from: "程序员".to_string(), to: "个学生".to_string()},
    ]);
}