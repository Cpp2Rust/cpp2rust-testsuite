extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: u64 = 25000000000_u64;
    let mut sum: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    let mut j: u64 = N;
    'loop_: while ((i) < (j)) {
        sum = (sum).wrapping_add((i).wrapping_add(j));
        {
            i.prefix_inc();
            j.prefix_dec()
        };
    }
    write!((*libcc2rs::cout_unsafe()), "Sum: {:}\n", sum,);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
