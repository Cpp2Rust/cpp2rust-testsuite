extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fib_0(mut n: u64) -> u64 {
    return if ((n) == (0_u64)) || ((n) == (1_u64)) {
        n
    } else {
        (unsafe {
            let _n: u64 = (n).wrapping_sub(1_u64);
            fib_0(_n)
        })
        .wrapping_add(
            (unsafe {
                let _n: u64 = (n).wrapping_sub(2_u64);
                fib_0(_n)
            }),
        )
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        (unsafe { fib_0(46_u64,) }),
    );
    return 0;
}
