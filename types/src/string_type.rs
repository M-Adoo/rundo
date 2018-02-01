use std;
use Rundo;
use primitive_type::ValueType;
use difference::{Changeset, Difference};
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

impl Rundo for ValueType<String> {
  type Op = std::vec::Vec<StrOP>;

  fn dirty(&self) -> bool {
    self.origin.is_some()
  }

  fn reset(&mut self) {
    self.origin = None;
  }

  fn change_op(&self) -> Option<Self::Op> {
    self
      .origin
      .as_ref()
      .map(|ref ori| Changeset::new(ori, &self.value, "").diffs)
      .map(|diffs| {
        let mut ops = Vec::with_capacity(diffs.len());
        let mut base = 0;
        let mut rem_diff = false;
        for diff in diffs {
          let is_rem_diff = std::mem::discriminant(&diff)
            == std::mem::discriminant(&Difference::Rem("".to_string()));
          match diff {
            Difference::Same(text) => base += text.len(),
            Difference::Rem(text) => {
              let idx = base;
              base += text.len();
              ops.push(StrOP::Del { idx, value: text });
            }
            Difference::Add(ref text) if rem_diff => {
              if let Some(StrOP::Del { idx, value }) = ops.pop() {
                ops.push(StrOP::Chg {
                  idx,
                  from: value,
                  to: text.to_string(),
                });
              } else {
                panic!("must be a Del op here!");
              }
            }
            Difference::Add(text) => {
              ops.push(StrOP::Ins {
                idx: base,
                value: text,
              });
            }
          };
          rem_diff = is_rem_diff;
        }
        return ops;
      })
  }

  fn back(&mut self, op: &Self::Op) {
    self.reset();
    let op: Vec<StrOP> = op.iter()
      .scan(0isize, |base, uop| {
        let uop = match uop {
          &StrOP::Ins { idx, ref value } => {
            let nidx = (idx as isize) + *base;
            *base += value.len() as isize;
            StrOP::Del {
              idx: nidx as usize,
              value: value.to_string(),
            }
          }
          &StrOP::Del { idx, ref value } => {
            let nidx = (idx as isize) + *base;
            *base -= value.len() as isize;
            StrOP::Ins {
              idx: nidx as usize,
              value: value.to_string(),
            }
          }
          &StrOP::Chg {
            idx,
            ref from,
            ref to,
          } => {
            let nidx = (idx as isize) + *base;
            *base += to.len() as isize - from.len() as isize;
            StrOP::Chg {
              idx: nidx as usize,
              to: from.to_string(),
              from: to.to_string(),
            }
          }
        };
        Some(uop)
      })
      .collect();
    self.forward(&op);
  }

  fn forward(&mut self, op: &Self::Op) {
    self.reset();
    let realloc_size = op.iter().fold(0, |acc, x| {
      acc + match x {
        &StrOP::Ins { ref value, .. } => value.len() as isize,
        &StrOP::Del { ref value, .. } => -(value.len() as isize),
        &StrOP::Chg {
          ref from, ref to, ..
        } => to.len() as isize - from.len() as isize,
      }
    });

    let mut base = 0;
    let size = self.value.len() as isize + realloc_size;
    let mut newstr = String::with_capacity(size as usize);
    for uop in op {
      match uop {
        &StrOP::Ins { idx, ref value } => {
          newstr += &self.value[base..idx];
          newstr += value;
          base = idx;
        }
        &StrOP::Del { idx, ref value } => {
          newstr += &self.value[base..idx];
          base = idx + value.len();
        }
        &StrOP::Chg {
          idx,
          ref from,
          ref to,
        } => {
          newstr += &self.value[base..idx];
          base = idx + from.len();
          newstr += to;
        }
      };
    }

    newstr += &self.value[base..];
    self.value = newstr;
  }
}

#[test]
fn string_ops() {
  let mut hello = ValueType::<String>::from("hello world!".to_string());
  *hello = "hello adoo!".to_string();
  let ops = hello.change_op().unwrap();
  assert_eq!(
    ops,
    vec![
      StrOP::Chg {
        idx: 6,
        from: "w".to_string(),
        to: "ad".to_string(),
      },
      StrOP::Chg {
        idx: 8,
        from: "rld".to_string(),
        to: "o".to_string(),
      },
    ]
  );

  hello.reset();
  *hello = "hello adoo! by Rust.".to_string();
  let ops = hello.change_op().unwrap();
  assert_eq!(
    ops,
    vec![
      StrOP::Ins {
        idx: 11,
        value: " by Rust.".to_string(),
      },
    ]
  );

  hello.reset();
  *hello = "by Rust.".to_string();
  let ops = hello.change_op().unwrap();
  assert_eq!(
    ops,
    vec![
      StrOP::Del {
        idx: 0,
        value: "hello adoo! ".to_string(),
      },
    ]
  )
}

#[test]
fn string_chinese() {
  let mut chinese = ValueType::<String>::from("我白天是程序员".to_string());
  *chinese = "晚上是个学生".to_string();
  let op = chinese.change_op().unwrap();
  assert_eq!(
    op,
    vec![
      StrOP::Chg {
        idx: 0,
        from: "我白天".to_string(),
        to: "晚上".to_string(),
      },
      StrOP::Chg {
        idx: 12,
        from: "程序员".to_string(),
        to: "个学生".to_string(),
      },
    ]
  );

  chinese.back(&op);
  assert_eq!(*chinese, "我白天是程序员");
  chinese.forward(&op);
  assert_eq!(*chinese, "晚上是个学生");
}

#[test]
fn string_poem() {
  let one = "You say that you love rain, but you open your umbrella when it rains...";
  let two = "You say that you love the sun, but you find a shadow spot when the sun shines...";
  let three = "You say that you love the wind, But you close your windows when wind blows...";
  let four = "This is why I am afraid; You say that you love me too...";
  let mut afriad = ValueType::<String>::from(one.to_string());

  *afriad = two.to_string();
  let op1 = afriad.change_op().unwrap();
  afriad.reset();
  assert_eq!(afriad.value, two);

  *afriad = three.to_string();
  let op2 = afriad.change_op().unwrap();
  afriad.reset();
  assert_eq!(afriad.value, three);

  *afriad = four.to_string();
  let op3 = afriad.change_op().unwrap();
  afriad.reset();
  assert_eq!(afriad.value, four);

  afriad.back(&op3);
  assert_eq!(afriad.value, three);

  afriad.back(&op2);
  assert_eq!(afriad.value, two);

  afriad.back(&op1);
  assert_eq!(afriad.value, one);

  afriad.forward(&op1);
  assert_eq!(afriad.value, two);

  afriad.forward(&op2);
  assert_eq!(afriad.value, three);

  afriad.forward(&op3);
  assert_eq!(afriad.value, four);
}
