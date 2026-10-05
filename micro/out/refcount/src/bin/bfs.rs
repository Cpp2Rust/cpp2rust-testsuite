extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct Queue {
    #[offset(0)]
    #[byte_size(8)]
    pub elems: Ptr<u32>,
    #[offset(8)]
    pub front: usize,
    #[offset(16)]
    pub back: usize,
    #[offset(24)]
    pub capacity: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct GraphNode {
    #[offset(0)]
    pub vertex: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<GraphNode>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    pub V: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub adj: Ptr<Ptr<GraphNode>>,
}
pub fn BFS_0(graph: Ptr<Graph>, mut start_vertex: u32) -> Ptr<u32> {
    let Q: Value<Queue> = Rc::new(RefCell::new(Queue {
        elems: Ptr::alloc_array(
            (0..(graph.with(|__s| __s.V) as usize))
                .map(|_| 0_u32)
                .collect::<Box<[u32]>>(),
        ),
        front: 0_usize,
        back: 0_usize,
        capacity: (graph.with(|__s| __s.V) as usize),
    }));
    let mut visited: Ptr<bool> = Ptr::alloc_array(
        (0..(graph.with(|__s| __s.V) as usize))
            .map(|_| false)
            .collect::<Box<[bool]>>(),
    );
    let mut pred: Ptr<u32> = Ptr::alloc_array(
        (0..(graph.with(|__s| __s.V) as usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    );
    let mut i: u32 = 0_u32;
    'loop_: while ({ i } < { graph.with(|__s| __s.V) }) {
        elem!(visited, i).write(false);
        elem!(pred, i).write({ i });
        i.prefix_inc();
    }
    elem!(visited, start_vertex).write(true);
    ({ QueueImpl::enqueue(&Q.as_pointer(), (start_vertex as i32)) });
    'loop_: while !({ QueueImpl::empty(&Q.as_pointer()) }) {
        let mut current_vertex: i32 = (({ QueueImpl::dequeue(&Q.as_pointer()) }) as i32);
        let mut head: Ptr<GraphNode> =
            (elem!(graph.with(|__s| __s.adj.clone()), current_vertex).read()).clone();
        'loop_: while !((head).is_null()) {
            let mut adj_vertex: i32 = (head.with(|__s| __s.vertex) as i32);
            if !(elem!(visited, adj_vertex).read()) {
                elem!(visited, adj_vertex).write(true);
                ({ QueueImpl::enqueue(&Q.as_pointer(), adj_vertex) });
                elem!(pred, adj_vertex).write({ (current_vertex as u32) });
            }
            head = { head.with(|__s| __s.next.clone()) };
        }
    }
    visited.delete();
    { (*Q.borrow()).elems.clone() }.delete();
    return (pred).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: usize = 300_usize;
    let V: Value<usize> = Rc::new(RefCell::new((N).wrapping_mul(N)));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: ((*V.borrow()) as u32),
        adj: Ptr::alloc_array(
            (0..(*V.borrow()))
                .map(|_| Ptr::<GraphNode>::null())
                .collect::<Box<[Ptr<GraphNode>]>>(),
        ),
    }));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*V.borrow())) {
        elem!({ (*graph.borrow()).adj.clone() }, i).write(Ptr::<GraphNode>::null());
        i.prefix_inc();
    }
    let mut r: u32 = 0_u32;
    'loop_: while ((r as usize) < N) {
        let mut c: u32 = 0_u32;
        'loop_: while ((c as usize) < N) {
            let mut current: u32 =
                ((((r as usize).wrapping_mul(N)).wrapping_add((c as usize))) as u32);
            let mut step: u32 = 1_u32;
            'loop_: while (step <= 80_u32) {
                if ((((c).wrapping_add(step)) as usize) < N) {
                    ({
                        GraphImpl::push(
                            &graph.as_pointer(),
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
            'loop_: while (step <= 80_u32) {
                if ((((r).wrapping_add(step)) as usize) < N) {
                    ({
                        GraphImpl::push(
                            &graph.as_pointer(),
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
    let mut pred: Ptr<u32> = ({ BFS_0(graph.as_pointer(), 0_u32) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*V.borrow())) {
        let mut head: Ptr<GraphNode> = (elem!({ (*graph.borrow()).adj.clone() }, i).read()).clone();
        'loop_: while !((head).is_null()) {
            let mut next: Ptr<GraphNode> = head.with(|__s| __s.next.clone());
            head.delete();
            head = (next).clone();
        }
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*V.borrow())) {
        write!(libcc2rs::cout(), "{:} -> {:}\n", i, (elem!(pred, i).read()),);
        i.prefix_inc();
    }
    { (*graph.borrow()).adj.clone() }.delete();
    pred.delete();
    return 0;
}
pub trait GraphImpl {
    fn push(&self, src: u32, dst: u32);
}
impl GraphImpl for Ptr<Graph> {
    fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        let __rhs = Ptr::alloc(GraphNode {
            vertex: (*dst.borrow()),
            next: (elem!((*self).with(|__s| __s.adj.clone()), (*src.borrow())).read()).clone(),
        });
        elem!((*self).with(|__s| __s.adj.clone()), (*src.borrow())).write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            vertex: (*src.borrow()),
            next: (elem!((*self).with(|__s| __s.adj.clone()), (*dst.borrow())).read()).clone(),
        });
        elem!((*self).with(|__s| __s.adj.clone()), (*dst.borrow())).write(__rhs);
    }
}
pub trait QueueImpl {
    fn enqueue(&self, elem: i32);
    fn dequeue(&self) -> u32;
    fn empty(&self) -> bool;
}
impl QueueImpl for Ptr<Queue> {
    fn enqueue(&self, mut elem: i32) {
        if ((*self).with(|__s| __s.back) == (*self).with(|__s| __s.capacity)) {
            return;
        }
        let __rhs = (elem as u32);
        elem!(
            (*self).with(|__s| __s.elems.clone()),
            field!((*self), back).with_mut(|__v| __v.postfix_inc())
        )
        .write(__rhs);
    }
    fn dequeue(&self) -> u32 {
        if ({ QueueImpl::empty(self) }) {
            return (-1_i32 as u32);
        }
        return (elem!(
            (*self).with(|__s| __s.elems.clone()),
            field!((*self), front).with_mut(|__v| __v.postfix_inc())
        )
        .read());
    }
    fn empty(&self) -> bool {
        return ((*self).with(|__s| __s.front) == (*self).with(|__s| __s.back));
    }
}
pub fn __cpp2rust_init_globals() {}
