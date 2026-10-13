extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static N_0: Value<i32> = Rc::new(RefCell::new(400));
);
pub fn matmul_1(mut a: Ptr<Box<[i32]>>, mut b: Ptr<Box<[i32]>>, mut c: Ptr<Box<[i32]>>) {
    let mut i: i32 = 0;
    'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
        let mut k: i32 = 0;
        'loop_: while (k < N_0.with(|rc| *rc.borrow())) {
            let mut j: i32 = 0;
            'loop_: while (j < N_0.with(|rc| *rc.borrow())) {
                {
                    let __rhs = {
                        ({ (elem!((a.offset((i) as isize).as_pointer() as Ptr::<i32>), k).read()) }
                            * {
                                (elem!((b.offset((k) as isize).as_pointer() as Ptr::<i32>), j)
                                    .read())
                            })
                    };
                    elem!((c.offset((i) as isize).as_pointer() as Ptr::<i32>), j)
                        .with_mut(|__v| *__v = *__v + __rhs)
                };
                j.prefix_inc();
            }
            k.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(
        (0..400)
            .map(|_| (0..400).map(|_| 0_i32).collect::<Box<[i32]>>())
            .collect::<Box<[Box<[i32]>]>>(),
    ));
    let b: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(
        (0..400)
            .map(|_| (0..400).map(|_| 0_i32).collect::<Box<[i32]>>())
            .collect::<Box<[Box<[i32]>]>>(),
    ));
    let c: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(
        (0..400)
            .map(|_| (0..400).map(|_| 0_i32).collect::<Box<[i32]>>())
            .collect::<Box<[Box<[i32]>]>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
        let mut j: i32 = 0;
        'loop_: while (j < N_0.with(|rc| *rc.borrow())) {
            (*a.borrow_mut())[(i) as usize][(j) as usize] = { ((i + j) % 7) };
            (*b.borrow_mut())[(i) as usize][(j) as usize] = { ((i * j) % 5) };
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut sum: i64 = 0_i64;
    let mut r: i32 = 0;
    'loop_: while (r < 5) {
        let mut i: i32 = 0;
        'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
            let mut j: i32 = 0;
            'loop_: while (j < N_0.with(|rc| *rc.borrow())) {
                (*c.borrow_mut())[(i) as usize][(j) as usize] = 0;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        ({
            matmul_1(
                (a.as_pointer() as Ptr<Box<[i32]>>),
                (b.as_pointer() as Ptr<Box<[i32]>>),
                (c.as_pointer() as Ptr<Box<[i32]>>),
            )
        });
        let mut i: i32 = 0;
        'loop_: while (i < N_0.with(|rc| *rc.borrow())) {
            let mut j: i32 = 0;
            'loop_: while (j < N_0.with(|rc| *rc.borrow())) {
                sum += ((*c.borrow())[(i) as usize][(j) as usize] as i64);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        r.prefix_inc();
    }
    write!(libcc2rs::cout(), "{:}\n", sum,);
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = N_0.with(|_| ());
}
