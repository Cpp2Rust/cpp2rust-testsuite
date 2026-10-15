extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matmul_0(a: Ptr<Vec<Vec<i32>>>, b: Ptr<Vec<Vec<i32>>>, c: Ptr<Vec<Vec<i32>>>, mut n: i32) {
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        let mut k: i32 = 0;
        'loop_: while (k < n) {
            let mut j: i32 = 0;
            'loop_: while (j < n) {
                let __rhs = ({ (index!(index!(a, (i as usize)), (k as usize)).read()) } * {
                    (index!(index!(b, (k as usize)), (j as usize)).read())
                });
                {
                    index!(index!(c, (i as usize)), (j as usize))
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
    let a: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let b: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let __a1 = (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>();
            (*a.borrow_mut()).push(__a1)
        };
        {
            let __a1 = (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>();
            (*b.borrow_mut()).push(__a1)
        };
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            let __rhs = ((i + j) % 7);
            index!(
                elem!((a.as_pointer() as Ptr<Vec<i32>>), (i as usize)),
                (j as usize)
            )
            .write(__rhs);
            let __rhs = ((i * j) % 5);
            index!(
                elem!((b.as_pointer() as Ptr<Vec<i32>>), (i as usize)),
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
        let c: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            {
                let __a1 = (0..(N as usize) as usize)
                    .map(|_| <i32>::default())
                    .collect::<Vec<_>>();
                (*c.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        ({ matmul_0(a.as_pointer(), b.as_pointer(), c.as_pointer(), N) });
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                sum += ((index!(
                    elem!((c.as_pointer() as Ptr<Vec<i32>>), (i as usize)),
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
