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
    let mut N: i32 = 100000000;
    let mut sum: i64 = 0_i64;
    let mut k: i32 = 0;
    'loop_: while (k < 35) {
        let array: Value<Option<Value<Box<[i32]>>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..(N as usize))
                    .map(|_| <i32>::default())
                    .collect::<Box<[_]>>(),
            )))));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let __rhs = i;
            (*array.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            sum += ((*array.borrow()).as_ref().unwrap().borrow()[(i as usize) as usize] as i64);
            i.prefix_inc();
        }
        k.prefix_inc();
    }
    write!(libcc2rs::cout(), "{:}\n", sum,);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
