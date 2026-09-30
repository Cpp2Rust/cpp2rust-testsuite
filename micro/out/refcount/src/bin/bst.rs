extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node_t {
    #[offset(0)]
    #[byte_size(8)]
    pub left: Ptr<node_t>,
    #[offset(8)]
    #[byte_size(8)]
    pub right: Ptr<node_t>,
    #[offset(16)]
    pub value: i32,
}
pub fn find_0(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ({
        let _lhs = (*value.borrow());
        _lhs < (*node.borrow()).with(|__s| __s.value)
    }) && (!(((*node.borrow()).with(|__s| __s.left.clone())).is_null()))
    {
        return ({
            find_0(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*value.borrow()),
            )
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*node.borrow()).with(|__s| __s.value)
    }) && (!(((*node.borrow()).with(|__s| __s.right.clone())).is_null()))
    {
        return ({
            find_0(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*value.borrow()),
            )
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*node.borrow()).with(|__s| __s.value)
    } {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if {
        let _lhs = (*new_node.borrow()).with(|__s| __s.value);
        _lhs < (*node.borrow()).with(|__s| __s.value)
    } {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), left).write(__rhs);
    } else if {
        let _lhs = (*new_node.borrow()).with(|__s| __s.value);
        _lhs > (*node.borrow()).with(|__s| __s.value)
    } {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), right).write(__rhs);
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(25000));
    let tree: Value<Ptr<node_t>> = Rc::new(RefCell::new(Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    })));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        ({
            insert_1(
                (*tree.borrow()).clone(),
                Ptr::alloc(node_t {
                    left: Ptr::<node_t>::null(),
                    right: Ptr::<node_t>::null(),
                    value: (*i.borrow()),
                }),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        write!(
            libcc2rs::cout(),
            "Value: {:}, Found: {:}\n",
            (*i.borrow()),
            ({ find_0((*tree.borrow()).clone(), (*i.borrow()),) }).with(|__s| __s.value),
        );
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
