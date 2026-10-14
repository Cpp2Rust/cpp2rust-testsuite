extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut N_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 400 });
pub unsafe fn matmul_1(mut a: *const [i32; 400], mut b: *const [i32; 400], mut c: *mut [i32; 400]) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
        let mut k: i32 = 0;
        'loop_: while ((k) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
                (*c.offset((i) as isize))[(j) as usize] += (((*a.offset((i) as isize))
                    [(k) as usize])
                    * ((*b.offset((k) as isize))[(j) as usize]));
                j.prefix_inc();
            }
            k.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [[i32; 400]; 400] = [[0_i32; 400]; 400];
    let mut b: [[i32; 400]; 400] = [[0_i32; 400]; 400];
    let mut c: [[i32; 400]; 400] = [[0_i32; 400]; 400];
    let mut i: i32 = 0;
    'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            a[(i) as usize][(j) as usize] = (((i) + (j)) % (7));
            b[(i) as usize][(j) as usize] = (((i) * (j)) % (5));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut sum: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while ((r) < (5)) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
                c[(i) as usize][(j) as usize] = 0;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        (unsafe {
            matmul_1(
                (a.as_mut_ptr()).cast_const(),
                (b.as_mut_ptr()).cast_const(),
                c.as_mut_ptr(),
            )
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (*std::cell::LazyCell::force_mut(&mut *&raw mut N_0))) {
                sum += (c[(i) as usize][(j) as usize] as i64);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        r.prefix_inc();
    }
    write!((*libcc2rs::cout_unsafe()), "{:}\n", sum,);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const N_0);
}
