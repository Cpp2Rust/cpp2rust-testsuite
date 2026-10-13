extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static N_0: Value<i32> = Rc::new(RefCell::new(1000));
);
pub fn smooth_1(mut p: Ptr<i32>, mut n: i32) {
    let mut j: i32 = 1;
    'loop_: while (j < n) {
        elem!(p, j).write({ (((elem!(p, j).read()) + (elem!(p, (j - 1)).read())) % 1000) });
        j.prefix_inc();
    }
}
pub fn sum_2(mut p: Ptr<i32>, mut end: Ptr<i32>) -> i64 {
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
    let grid: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(
        (0..1000)
            .map(|_| (0..1000).map(|_| 0_i32).collect::<Box<[i32]>>())
            .collect::<Box<[Box<[i32]>]>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
        let mut j: i32 = 0;
        'loop_: while (j < N_0.with(|rc| *rc.borrow())) {
            (*grid.borrow_mut())[(i) as usize][(j) as usize] = { ((i * j) % 7) };
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut out: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while (r < 40) {
        let mut row: Ptr<Box<[i32]>> = (grid.as_pointer() as Ptr<Box<[i32]>>);
        let mut i: i32 = 0;
        'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
            let mut p: Ptr<i32> = ((row).as_pointer() as Ptr<i32>).clone();
            ({
                let _p: Ptr<i32> = (p).clone();
                let _n: i32 = N_0.with(|rc| *rc.borrow());
                smooth_1(_p, _n)
            });
            out += ({
                let _p: Ptr<i32> = (p).clone();
                let _end: Ptr<i32> = p.offset((N_0.with(|rc| *rc.borrow())) as isize);
                sum_2(_p, _end)
            });
            {
                i.prefix_inc();
                row.prefix_inc()
            };
        }
        r.prefix_inc();
    }
    write!(libcc2rs::cout(), "{:}\n", out,);
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = N_0.with(|_| ());
}
