extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: u64 = 25000000000_u64;
    let mut sum: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    let mut j: u64 = N;
    'loop_: while (i < j) {
        sum = { (sum).wrapping_add((i).wrapping_add(j)) };
        {
            i.prefix_inc();
            j.prefix_dec()
        };
    }
    write!(libcc2rs::cout(), "Sum: {:}\n", sum,);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
