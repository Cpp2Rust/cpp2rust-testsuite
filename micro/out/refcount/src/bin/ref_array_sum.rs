extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn initialize_0(array: Ptr<Option<Value<Box<[i32]>>>>, N: i32) {
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = (*i.borrow());
        (*array.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*i.borrow()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn sum_1(array: Ptr<Option<Value<Box<[i32]>>>>, N: i32) -> i64 {
    let array: Value<Ptr<Option<Value<Box<[i32]>>>>> = Rc::new(RefCell::new(array));
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let sum: Value<i64> = Rc::new(RefCell::new(0_i64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (*sum.borrow_mut()) += ((*(*array.borrow()).upgrade().deref())
            .as_ref()
            .unwrap()
            .borrow()[((*i.borrow()) as usize) as usize] as i64);
        (*i.borrow_mut()).prefix_inc();
    }
    return (*sum.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(100000000));
    let out: Value<i64> = Rc::new(RefCell::new(0_i64));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < 35) {
        let array: Value<Option<Value<Box<[i32]>>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..((*N.borrow()) as usize))
                    .map(|_| <i32>::default())
                    .collect::<Box<[_]>>(),
            )))));
        ({ initialize_0(array.as_pointer(), (*N.borrow())) });
        (*out.borrow_mut()) += ({ sum_1((array.as_pointer()), (*N.borrow())) });
        (*k.borrow_mut()).prefix_inc();
    }
    write!(libcc2rs::cout(), "Sum: {:}\n", (*out.borrow()),);
    return 0;
}
