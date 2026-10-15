extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn smooth_0(mut p: Ptr<i32>, mut n: i32) {
    let mut j: i32 = 1;
    'loop_: while (j < n) {
        elem!(p, j).write({ (((elem!(p, j).read()) + (elem!(p, (j - 1)).read())) % 1000) });
        j.prefix_inc();
    }
}
pub fn sum_1(mut p: Ptr<i32>, mut end: Ptr<i32>) -> i64 {
    let mut s: i64 = 0_i64;
    'loop_: while ({ (p).clone() } != { (end).clone() }) {
        s += ((p.postfix_inc().read()) as i64);
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 2000;
    let grid: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let __a1 = (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>();
            (*grid.borrow_mut()).push(__a1)
        };
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            let __rhs = ((i * j) % 7);
            index!(
                elem!((grid.as_pointer() as Ptr<Vec<i32>>), (i as usize)),
                (j as usize)
            )
            .write(__rhs);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut out: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while (r < 10) {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut p: Ptr<i32> = (((grid.as_pointer() as Ptr<Vec<i32>>)
                .offset((i as usize))
                .as_pointer() as Ptr<i32>)
                .offset(0_usize));
            ({
                let _p: Ptr<i32> = (p).clone();
                let _n: i32 = N;
                smooth_0(_p, _n)
            });
            out += ({
                let _p: Ptr<i32> = (p).clone();
                let _end: Ptr<i32> = p.offset((N) as isize);
                sum_1(_p, _end)
            });
            i.prefix_inc();
        }
        r.prefix_inc();
    }
    write!(libcc2rs::cout(), "{:}\n", out,);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
