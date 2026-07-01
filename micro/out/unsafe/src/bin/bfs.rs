extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Queue {
    pub elems: *mut u32,
    pub front: usize,
    pub back: usize,
    pub capacity: usize,
}
impl Queue {
    pub unsafe fn enqueue(&mut self, mut elem: i32) {
        if ((self.back) == (self.capacity)) {
            return;
        }
        (*self.elems.offset((self.back.postfix_inc()) as isize)) = (elem as u32);
    }
    pub unsafe fn dequeue(&mut self) -> u32 {
        if (unsafe { self.empty() }) {
            return (-1_i32 as u32);
        }
        return (*self.elems.offset((self.front.postfix_inc()) as isize));
    }
    pub unsafe fn empty(&self) -> bool {
        return ((self.front) == (self.back));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct GraphNode {
    pub vertex: u32,
    pub next: *mut GraphNode,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Graph {
    pub V: u32,
    pub adj: *mut *mut GraphNode,
}
impl Graph {
    pub unsafe fn push(&mut self, mut src: u32, mut dst: u32) {
        (*self.adj.offset((src) as isize)) = (Box::leak(Box::new(GraphNode {
            vertex: dst,
            next: (*self.adj.offset((src) as isize)),
        })) as *mut GraphNode);
        (*self.adj.offset((dst) as isize)) = (Box::leak(Box::new(GraphNode {
            vertex: src,
            next: (*self.adj.offset((dst) as isize)),
        })) as *mut GraphNode);
    }
}
pub unsafe fn BFS_0(graph: *const Graph, mut start_vertex: u32) -> *mut u32 {
    let mut Q: Queue = Queue {
        elems: Box::leak(
            (0..((*graph).V as usize))
                .map(|_| 0_u32)
                .collect::<Box<[u32]>>(),
        )
        .as_mut_ptr(),
        front: 0_usize,
        back: 0_usize,
        capacity: ((*graph).V as usize),
    };
    let mut visited: *mut bool = Box::leak(
        (0..((*graph).V as usize))
            .map(|_| false)
            .collect::<Box<[bool]>>(),
    )
    .as_mut_ptr();
    let mut pred: *mut u32 = Box::leak(
        (0..((*graph).V as usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    )
    .as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < ((*graph).V)) {
        (*visited.offset((i) as isize)) = false;
        (*pred.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    (*visited.offset((start_vertex) as isize)) = true;
    (unsafe { Q.enqueue((start_vertex as i32)) });
    'loop_: while !(unsafe { Q.empty() }) {
        let mut current_vertex: i32 = ((unsafe { Q.dequeue() }) as i32);
        let mut head: *mut GraphNode = (*(*graph).adj.offset((current_vertex) as isize));
        'loop_: while !((head).is_null()) {
            let mut adj_vertex: i32 = ((*head).vertex as i32);
            if !(*visited.offset((adj_vertex) as isize)) {
                (*visited.offset((adj_vertex) as isize)) = true;
                (unsafe { Q.enqueue(adj_vertex) });
                (*pred.offset((adj_vertex) as isize)) = (current_vertex as u32);
            }
            head = (*head).next;
        }
    }
    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        visited,
        libcc2rs::malloc_usable_size(visited as *mut ::libc::c_void)
            / ::std::mem::size_of::<bool>(),
    )));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        Q.elems,
        libcc2rs::malloc_usable_size(Q.elems as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return pred;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: usize = 300_usize;
    let V: usize = (N).wrapping_mul(N);
    let mut graph: Graph = Graph {
        V: (V as u32),
        adj: Box::leak(
            (0..V)
                .map(|_| std::ptr::null_mut())
                .collect::<Box<[*mut GraphNode]>>(),
        )
        .as_mut_ptr(),
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (V)) {
        (*graph.adj.offset((i) as isize)) = std::ptr::null_mut();
        i.prefix_inc();
    }
    let mut r: u32 = 0_u32;
    'loop_: while ((r as usize) < (N)) {
        let mut c: u32 = 0_u32;
        'loop_: while ((c as usize) < (N)) {
            let mut current: u32 =
                ((((r as usize).wrapping_mul(N)).wrapping_add((c as usize))) as u32);
            let mut step: u32 = 1_u32;
            'loop_: while ((step) <= (80_u32)) {
                if ((((c).wrapping_add(step)) as usize) < (N)) {
                    (unsafe {
                        graph.push(
                            current,
                            ((((r as usize).wrapping_mul(N))
                                .wrapping_add((((c).wrapping_add(step)) as usize)))
                                as u32),
                        )
                    });
                }
                step.prefix_inc();
            }
            let mut step: u32 = 1_u32;
            'loop_: while ((step) <= (80_u32)) {
                if ((((r).wrapping_add(step)) as usize) < (N)) {
                    (unsafe {
                        graph.push(
                            current,
                            ((((((r).wrapping_add(step)) as usize).wrapping_mul(N))
                                .wrapping_add((c as usize))) as u32),
                        )
                    });
                }
                step.prefix_inc();
            }
            c.prefix_inc();
        }
        r.prefix_inc();
    }
    let mut pred: *mut u32 = (unsafe { BFS_0(&graph as *const Graph, 0_u32) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (V)) {
        let mut head: *mut GraphNode = (*graph.adj.offset((i) as isize));
        'loop_: while !((head).is_null()) {
            let mut next: *mut GraphNode = (*head).next;
            ::std::mem::drop(Box::from_raw(head));
            head = next;
        }
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (V)) {
        write!(
            std::fs::File::from_raw_fd(
                std::io::stdout()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
            ),
            "{:} -> {:}\n",
            i,
            (*pred.offset((i) as isize)),
        );
        i.prefix_inc();
    }
    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        graph.adj,
        libcc2rs::malloc_usable_size(graph.adj as *mut ::libc::c_void)
            / ::std::mem::size_of::<*mut GraphNode>(),
    )));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        pred,
        libcc2rs::malloc_usable_size(pred as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return 0;
}
