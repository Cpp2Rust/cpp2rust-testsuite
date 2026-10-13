extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn smooth_0(mut p: *mut i32, mut n: i32) {
    let mut j: i32 = 1;
    'loop_: while ((j) < (n)) {
        (*p.offset((j) as isize)) =
            (((*p.offset((j) as isize)) + (*p.offset(((j) - (1)) as isize))) % (1000));
        j.prefix_inc();
    }
}
pub unsafe fn sum_1(mut p: *const i32, mut end: *const i32) -> i64 {
    let mut s: i64 = 0_i64;
    'loop_: while ((p) != (end)) {
        s += ((*p.postfix_inc()) as i64);
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: i32 = 2000;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let __a1 = (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>();
            grid.push(__a1)
        };
        let mut j: i32 = 0;
        'loop_: while ((j) < (N)) {
            grid[(i as usize)][(j as usize)] = (((i) * (j)) % (7));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut out: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while ((r) < (10)) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut p: *mut i32 = (&mut grid[(i as usize)][(0_usize)] as *mut i32);
            (unsafe {
                let _p: *mut i32 = p;
                let _n: i32 = N;
                smooth_0(_p, _n)
            });
            out += (unsafe {
                let _p: *const i32 = (p).cast_const();
                let _end: *const i32 = (p.offset((N) as isize)).cast_const();
                sum_1(_p, _end)
            });
            i.prefix_inc();
        }
        r.prefix_inc();
    }
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        out,
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
