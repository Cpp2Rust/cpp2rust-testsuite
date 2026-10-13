extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matmul_0(
    a: Ptr<Vec<Value<Vec<i32>>>>,
    b: Ptr<Vec<Value<Vec<i32>>>>,
    c: Ptr<Vec<Value<Vec<i32>>>>,
    mut n: i32,
) {
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        let mut k: i32 = 0;
        'loop_: while (k < n) {
            let mut j: i32 = 0;
            'loop_: while (j < n) {
                let __rhs = ({
                    (elem!(
                        ((Ptr::<Vec<Value<Vec<i32>>>>::decay(&(a)) as Ptr<Value<Vec<i32>>>)
                            .offset((i as usize))
                            .upgrade()
                            .deref()
                            .as_pointer() as Ptr<i32>),
                        (k as usize)
                    )
                    .read())
                } * {
                    (elem!(
                        ((Ptr::<Vec<Value<Vec<i32>>>>::decay(&(b)) as Ptr<Value<Vec<i32>>>)
                            .offset((k as usize))
                            .upgrade()
                            .deref()
                            .as_pointer() as Ptr<i32>),
                        (j as usize)
                    )
                    .read())
                });
                {
                    elem!(
                        ((Ptr::<Vec<Value<Vec<i32>>>>::decay(&(c)) as Ptr<Value<Vec<i32>>>)
                            .offset((i as usize))
                            .upgrade()
                            .deref()
                            .as_pointer() as Ptr<i32>),
                        (j as usize)
                    )
                    .with_mut(|__v| *__v = *__v + __rhs)
                };
                j.prefix_inc();
            }
            k.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 400;
    let a: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let b: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (a.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
            __v.push(Rc::new(RefCell::new(
                (0..(N as usize) as usize)
                    .map(|_| <i32>::default())
                    .collect::<Vec<_>>(),
            )))
        });
        (b.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
            __v.push(Rc::new(RefCell::new(
                (0..(N as usize) as usize)
                    .map(|_| <i32>::default())
                    .collect::<Vec<_>>(),
            )))
        });
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            let __rhs = ((i + j) % 7);
            elem!(
                ((a.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset((i as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>),
                (j as usize)
            )
            .write(__rhs);
            let __rhs = ((i * j) % 5);
            elem!(
                ((b.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset((i as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>),
                (j as usize)
            )
            .write(__rhs);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut sum: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while (r < 5) {
        let c: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            (c.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(
                |__v: &mut Vec<Value<Vec<i32>>>| {
                    __v.push(Rc::new(RefCell::new(
                        (0..(N as usize) as usize)
                            .map(|_| <i32>::default())
                            .collect::<Vec<_>>(),
                    )))
                },
            );
            i.prefix_inc();
        }
        ({ matmul_0(a.as_pointer(), b.as_pointer(), c.as_pointer(), N) });
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                sum += ((elem!(
                    ((c.as_pointer() as Ptr<Value<Vec<i32>>>)
                        .offset((i as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<i32>),
                    (j as usize)
                )
                .read()) as i64);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        r.prefix_inc();
    }
    write!(libcc2rs::cout(), "{:}\n", sum,);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
