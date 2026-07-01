extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_prime_0(x: Ptr<i32>) -> bool {
    let i: Value<i32> = Rc::new(RefCell::new(2));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (x.read())
    } {
        if ({
            let _lhs = (x.read());
            _lhs % (*i.borrow())
        } == 0)
        {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn largest_prime_1(n: Ptr<i32>) -> i32 {
    let max: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (n.read())
    } {
        if ({ is_prime_0(i.as_pointer()) }) {
            (*max.borrow_mut()) = (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*max.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(270000));
    let largest: Value<i32> = Rc::new(RefCell::new(({ largest_prime_1(N.as_pointer()) })));
    write!(
        libcc2rs::cout(),
        "The largest prime < {:} is: {:}\n",
        (*N.borrow()),
        (*largest.borrow()),
    );
    return 0;
}
