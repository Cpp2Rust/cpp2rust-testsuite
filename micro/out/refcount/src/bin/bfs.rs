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
pub fn BFS_0(graph: Ptr<Graph>, start_vertex: u32) -> Ptr<u32> {
    let start_vertex: Value<u32> = Rc::new(RefCell::new(start_vertex));
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
    let visited: Value<Ptr<bool>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..(graph.with(|__s| __s.V) as usize))
            .map(|_| false)
            .collect::<Box<[bool]>>(),
    )));
    let pred: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..(graph.with(|__s| __s.V) as usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    )));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ({ (*i.borrow()) } < { graph.with(|__s| __s.V) }) {
        elem!((*visited.borrow()), (*i.borrow())).write(false);
        elem!((*pred.borrow()), (*i.borrow())).write({ (*i.borrow()) });
        (*i.borrow_mut()).prefix_inc();
    }
    elem!((*visited.borrow()), (*start_vertex.borrow())).write(true);
    ({ QueueImpl::enqueue(&Q.as_pointer(), ((*start_vertex.borrow()) as i32)) });
    'loop_: while !({ QueueImpl::empty(&Q.as_pointer()) }) {
        let current_vertex: Value<i32> = Rc::new(RefCell::new(
            (({ QueueImpl::dequeue(&Q.as_pointer()) }) as i32),
        ));
        let head: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
            (elem!(
                graph.with(|__s| __s.adj.clone()),
                (*current_vertex.borrow())
            )
            .read())
            .clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let adj_vertex: Value<i32> = Rc::new(RefCell::new(
                ((*head.borrow()).with(|__s| __s.vertex) as i32),
            ));
            if !(elem!((*visited.borrow()), (*adj_vertex.borrow())).read()) {
                elem!((*visited.borrow()), (*adj_vertex.borrow())).write(true);
                ({ QueueImpl::enqueue(&Q.as_pointer(), (*adj_vertex.borrow())) });
                elem!((*pred.borrow()), (*adj_vertex.borrow()))
                    .write({ ((*current_vertex.borrow()) as u32) });
            }
            (*head.borrow_mut()) = { (*head.borrow()).with(|__s| __s.next.clone()) };
        }
    }
    (*visited.borrow()).delete();
    { (*Q.borrow()).elems.clone() }.delete();
    return (*pred.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<usize> = Rc::new(RefCell::new(300_usize));
    let V: Value<usize> = Rc::new(RefCell::new((*N.borrow()).wrapping_mul((*N.borrow()))));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: ((*V.borrow()) as u32),
        adj: Ptr::alloc_array(
            (0..(*V.borrow()))
                .map(|_| Ptr::<GraphNode>::null())
                .collect::<Box<[Ptr<GraphNode>]>>(),
        ),
    }));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*V.borrow())) {
        elem!({ (*graph.borrow()).adj.clone() }, (*i.borrow())).write(Ptr::<GraphNode>::null());
        (*i.borrow_mut()).prefix_inc();
    }
    let r: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*r.borrow()) as usize) < (*N.borrow())) {
        let c: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while (((*c.borrow()) as usize) < (*N.borrow())) {
            let current: Value<u32> = Rc::new(RefCell::new(
                (((((*r.borrow()) as usize).wrapping_mul((*N.borrow())))
                    .wrapping_add(((*c.borrow()) as usize))) as u32),
            ));
            let step: Value<u32> = Rc::new(RefCell::new(1_u32));
            'loop_: while ((*step.borrow()) <= 80_u32) {
                if ((((*c.borrow()).wrapping_add((*step.borrow()))) as usize) < (*N.borrow())) {
                    ({
                        GraphImpl::push(
                            &graph.as_pointer(),
                            (*current.borrow()),
                            (((((*r.borrow()) as usize).wrapping_mul((*N.borrow()))).wrapping_add(
                                (((*c.borrow()).wrapping_add((*step.borrow()))) as usize),
                            )) as u32),
                        )
                    });
                }
                (*step.borrow_mut()).prefix_inc();
            }
            let step: Value<u32> = Rc::new(RefCell::new(1_u32));
            'loop_: while ((*step.borrow()) <= 80_u32) {
                if ((((*r.borrow()).wrapping_add((*step.borrow()))) as usize) < (*N.borrow())) {
                    ({
                        GraphImpl::push(
                            &graph.as_pointer(),
                            (*current.borrow()),
                            ((((((*r.borrow()).wrapping_add((*step.borrow()))) as usize)
                                .wrapping_mul((*N.borrow())))
                            .wrapping_add(((*c.borrow()) as usize)))
                                as u32),
                        )
                    });
                }
                (*step.borrow_mut()).prefix_inc();
            }
            (*c.borrow_mut()).prefix_inc();
        }
        (*r.borrow_mut()).prefix_inc();
    }
    let pred: Value<Ptr<u32>> = Rc::new(RefCell::new(({ BFS_0(graph.as_pointer(), 0_u32) })));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*V.borrow())) {
        let head: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
            (elem!({ (*graph.borrow()).adj.clone() }, (*i.borrow())).read()).clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let next: Value<Ptr<GraphNode>> =
                Rc::new(RefCell::new((*head.borrow()).with(|__s| __s.next.clone())));
            (*head.borrow()).delete();
            (*head.borrow_mut()) = (*next.borrow()).clone();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*V.borrow())) {
        write!(
            libcc2rs::cout(),
            "{:} -> {:}\n",
            (*i.borrow()),
            (elem!((*pred.borrow()), (*i.borrow())).read()),
        );
        (*i.borrow_mut()).prefix_inc();
    }
    { (*graph.borrow()).adj.clone() }.delete();
    (*pred.borrow()).delete();
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
    fn enqueue(&self, elem: i32) {
        let elem: Value<i32> = Rc::new(RefCell::new(elem));
        if ((*self).with(|__s| __s.back) == (*self).with(|__s| __s.capacity)) {
            return;
        }
        let __rhs = ((*elem.borrow()) as u32);
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
