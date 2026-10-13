extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn matmul_0(
    a: *const Vec<Vec<i32>>,
    b: *const Vec<Vec<i32>>,
    c: *mut Vec<Vec<i32>>,
    mut n: i32,
) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        let mut k: i32 = 0;
        'loop_: while ((k) < (n)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (n)) {
                (&mut (*c))[(i as usize)][(j as usize)] +=
                    (((&(*a))[(i as usize)][(k as usize)]) * ((&(*b))[(k as usize)][(j as usize)]));
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
    let mut N: i32 = 400;
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut b: Vec<Vec<i32>> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        a.push(
            (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>(),
        );
        b.push(
            (0..(N as usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>(),
        );
        let mut j: i32 = 0;
        'loop_: while ((j) < (N)) {
            a[(i as usize)][(j as usize)] = (((i) + (j)) % (7));
            b[(i as usize)][(j as usize)] = (((i) * (j)) % (5));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut sum: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while ((r) < (5)) {
        let mut c: Vec<Vec<i32>> = Vec::new();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            c.push(
                (0..(N as usize) as usize)
                    .map(|_| <i32>::default())
                    .collect::<Vec<_>>(),
            );
            i.prefix_inc();
        }
        (unsafe { matmul_0(&a, &b, &mut c, N) });
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                sum += (c[(i as usize)][(j as usize)] as i64);
                j.prefix_inc();
            }
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
        sum,
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
