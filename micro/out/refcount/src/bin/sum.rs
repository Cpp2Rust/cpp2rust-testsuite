extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<u64> = Rc::new(RefCell::new(25000000000_u64));
    let sum: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    let j: Value<u64> = Rc::new(RefCell::new((*N.borrow())));
    'loop_: while ((*i.borrow()) < (*j.borrow())) {
        let rhs_0 = (*sum.borrow()).wrapping_add((*i.borrow()).wrapping_add((*j.borrow())));
        (*sum.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
        (*j.borrow_mut()).prefix_dec();
    }
    write!(libcc2rs::cout(), "Sum: {:}\n", (*sum.borrow()),);
    return 0;
}
