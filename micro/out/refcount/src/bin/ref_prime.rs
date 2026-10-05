extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_prime_0(x: Ptr<i32>) -> bool {
    let mut i: i32 = 2;
    'loop_: while ({ i } < { (x.read()) }) {
        if (({ (x.read()) } % { i }) == 0) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
pub fn largest_prime_1(n: Ptr<i32>) -> i32 {
    let mut max: i32 = -1_i32;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ({ (*i.borrow()) } < { (n.read()) }) {
        if ({ is_prime_0(i.as_pointer()) }) {
            max = (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return max;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(270000));
    let mut largest: i32 = ({ largest_prime_1(N.as_pointer()) });
    write!(
        libcc2rs::cout(),
        "The largest prime < {:} is: {:}\n",
        (*N.borrow()),
        largest,
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
