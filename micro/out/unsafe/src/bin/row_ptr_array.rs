extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut N_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1000 });
pub unsafe fn smooth_1(mut p: *mut i32, mut n: i32) {
    let mut j: i32 = 1;
    'loop_: while ((j) < (n)) {
        (*p.offset((j) as isize)) =
            (((*p.offset((j) as isize)) + (*p.offset(((j) - (1)) as isize))) % (1000));
        j.prefix_inc();
    }
}
pub unsafe fn sum_2(mut p: *const i32, mut end: *const i32) -> i64 {
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
    let mut grid: [[i32; 1000]; 1000] = [[0_i32; 1000]; 1000];
    let mut i: i32 = 0;
    'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            grid[(i) as usize][(j) as usize] = (((i) * (j)) % (7));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut out: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while ((r) < (40)) {
        let mut row: *mut [i32; 1000] = grid.as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            let mut p: *mut i32 = (*row).as_mut_ptr();
            (unsafe {
                let _p: *mut i32 = p;
                let _n: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0));
                smooth_1(_p, _n)
            });
            out += (unsafe {
                let _p: *const i32 = (p).cast_const();
                let _end: *const i32 = (p
                    .offset((*std::cell::LazyCell::force_mut(&mut *&raw mut N_0)) as isize))
                .cast_const();
                sum_2(_p, _end)
            });
            {
                i.prefix_inc();
                row.prefix_inc()
            };
        }
        r.prefix_inc();
    }
    write!((*libcc2rs::cout_unsafe()), "{:}\n", out,);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const N_0);
}
