extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fib_0(mut n: u64) -> u64 {
    return if (n == 0_u64) || (n == 1_u64) {
        n
    } else {
        ({ fib_0((n).wrapping_sub(1_u64)) }).wrapping_add(({ fib_0((n).wrapping_sub(2_u64)) }))
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    write!(libcc2rs::cout(), "{:}\n", ({ fib_0(46_u64,) }),);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
