extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: u64 = 25000000000_u64;
    let mut sum: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    let mut j: u64 = N;
    'loop_: while ((i) < (j)) {
        sum = (sum).wrapping_add((i).wrapping_add(j));
        i.prefix_inc();
        j.prefix_dec();
    }
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "Sum: {:}\n",
        sum,
    );
    return 0;
}
