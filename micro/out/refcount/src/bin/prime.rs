extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_prime_0(mut x: i32) -> bool {
    let mut i: i32 = 2;
    'loop_: while (i < x) {
        if ((x % i) == 0) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
pub fn largest_prime_1(mut n: i32) -> i32 {
    let mut max: i32 = -1_i32;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        if ({ is_prime_0(i) }) {
            max = i;
        }
        i.prefix_inc();
    }
    return max;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 270000;
    let mut largest: i32 = ({ largest_prime_1(N) });
    write!(
        libcc2rs::cout(),
        "The largest prime < {:} is: {:}\n",
        N,
        largest,
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
