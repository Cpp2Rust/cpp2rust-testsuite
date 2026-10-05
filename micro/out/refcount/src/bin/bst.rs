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
pub fn find_0(mut node: Ptr<node_t>, mut value: i32) -> Ptr<node_t> {
    if ({ value } < { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.left.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.left.clone()), value) });
    } else if ({ value } > { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.right.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.right.clone()), value) });
    } else if ({ value } == { node.with(|__s| __s.value) }) {
        return (node).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(mut node: Ptr<node_t>, mut new_node: Ptr<node_t>) -> Ptr<node_t> {
    if (node).is_null() {
        return (new_node).clone();
    }
    if ({ new_node.with(|__s| __s.value) } < { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.left.clone()), (new_node).clone()) });
        field!(node, left).write(__rhs);
    } else if ({ new_node.with(|__s| __s.value) } > { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.right.clone()), (new_node).clone()) });
        field!(node, right).write(__rhs);
    }
    return (node).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 25000;
    let mut tree: Ptr<node_t> = Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        ({
            insert_1(
                (tree).clone(),
                Ptr::alloc(node_t {
                    left: Ptr::<node_t>::null(),
                    right: Ptr::<node_t>::null(),
                    value: (*i.borrow()),
                }),
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        write!(
            libcc2rs::cout(),
            "Value: {:}, Found: {:}\n",
            i,
            ({ find_0((tree).clone(), i,) }).with(|__s| __s.value),
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
