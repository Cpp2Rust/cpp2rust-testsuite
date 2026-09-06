extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Queue {
    pub elems: Value<Ptr<u32>>,
    pub front: Value<usize>,
    pub back: Value<usize>,
    pub capacity: Value<usize>,
}
pub trait QueueImpl {
    fn enqueue(&self, elem: i32);
    fn dequeue(&self) -> u32;
    fn empty(&self) -> bool;
}
impl Clone for Queue {
    fn clone(&self) -> Self {
        let __this: Value<Queue> = Rc::new(RefCell::new(Self {
            elems: Rc::new(RefCell::new((*self.elems.borrow()).clone())),
            front: Rc::new(RefCell::new((*self.front.borrow()))),
            back: Rc::new(RefCell::new((*self.back.borrow()))),
            capacity: Rc::new(RefCell::new((*self.capacity.borrow()))),
        }));
        let this: Ptr<Queue> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Queue {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.elems.borrow()).to_bytes(&mut buf[0..8]);
        (*self.front.borrow()).to_bytes(&mut buf[8..16]);
        (*self.back.borrow()).to_bytes(&mut buf[16..24]);
        (*self.capacity.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            elems: Rc::new(RefCell::new(<Ptr<u32>>::from_bytes(&buf[0..8]))),
            front: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            back: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
            capacity: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive(Default)]
pub struct GraphNode {
    pub vertex: Value<u32>,
    pub next: Value<Ptr<GraphNode>>,
}
impl Clone for GraphNode {
    fn clone(&self) -> Self {
        let __this: Value<GraphNode> = Rc::new(RefCell::new(Self {
            vertex: Rc::new(RefCell::new((*self.vertex.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }));
        let this: Ptr<GraphNode> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for GraphNode {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vertex.borrow()).to_bytes(&mut buf[0..4]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vertex: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            next: Rc::new(RefCell::new(<Ptr<GraphNode>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct Graph {
    pub V: Value<u32>,
    pub adj: Value<Ptr<Ptr<GraphNode>>>,
}
pub trait GraphImpl {
    fn push(&self, src: u32, dst: u32);
}
impl Clone for Graph {
    fn clone(&self) -> Self {
        let __this: Value<Graph> = Rc::new(RefCell::new(Self {
            V: Rc::new(RefCell::new((*self.V.borrow()))),
            adj: Rc::new(RefCell::new((*self.adj.borrow()).clone())),
        }));
        let this: Ptr<Graph> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Graph {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.V.borrow()).to_bytes(&mut buf[0..4]);
        (*self.adj.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            V: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            adj: Rc::new(RefCell::new(<Ptr<Ptr<GraphNode>>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn BFS_0(graph: Ptr<Graph>, start_vertex: u32) -> Ptr<u32> {
    let start_vertex: Value<u32> = Rc::new(RefCell::new(start_vertex));
    let Q: Value<Queue> = Rc::new(RefCell::new(Queue {
        elems: Rc::new(RefCell::new(Ptr::alloc_array(
            (0..((*(*graph.upgrade().deref()).V.borrow()) as usize))
                .map(|_| <u32>::default())
                .collect::<Box<[u32]>>(),
        ))),
        front: Rc::new(RefCell::new(0_usize)),
        back: Rc::new(RefCell::new(0_usize)),
        capacity: Rc::new(RefCell::new(
            ((*(*graph.upgrade().deref()).V.borrow()) as usize),
        )),
    }));
    let visited: Value<Ptr<bool>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*(*graph.upgrade().deref()).V.borrow()) as usize))
            .map(|_| <bool>::default())
            .collect::<Box<[bool]>>(),
    )));
    let pred: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*(*graph.upgrade().deref()).V.borrow()) as usize))
            .map(|_| <u32>::default())
            .collect::<Box<[u32]>>(),
    )));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*graph.upgrade().deref()).V.borrow())
    } {
        (*visited.borrow())
            .offset((*i.borrow()) as isize)
            .write(false);
        let __rhs = (*i.borrow());
        (*pred.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    (*visited.borrow())
        .offset((*start_vertex.borrow()) as isize)
        .write(true);
    ({ QueueImpl::enqueue(&Q.as_pointer(), ((*start_vertex.borrow()) as i32)) });
    'loop_: while !({ QueueImpl::empty(&Q.as_pointer()) }) {
        let current_vertex: Value<i32> = Rc::new(RefCell::new(
            (({ QueueImpl::dequeue(&Q.as_pointer()) }) as i32),
        ));
        let head: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
            ((*(*graph.upgrade().deref()).adj.borrow())
                .offset((*current_vertex.borrow()) as isize)
                .read())
            .clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let adj_vertex: Value<i32> = Rc::new(RefCell::new(
                ((*(*(*head.borrow()).upgrade().deref()).vertex.borrow()) as i32),
            ));
            if !((*visited.borrow())
                .offset((*adj_vertex.borrow()) as isize)
                .read())
            {
                (*visited.borrow())
                    .offset((*adj_vertex.borrow()) as isize)
                    .write(true);
                ({ QueueImpl::enqueue(&Q.as_pointer(), (*adj_vertex.borrow())) });
                let __rhs = ((*current_vertex.borrow()) as u32);
                (*pred.borrow())
                    .offset((*adj_vertex.borrow()) as isize)
                    .write(__rhs);
            }
            let __rhs = (*(*(*head.borrow()).upgrade().deref()).next.borrow()).clone();
            (*head.borrow_mut()) = __rhs;
        }
    }
    (*visited.borrow()).delete_array();
    (*(*Q.borrow()).elems.borrow()).delete_array();
    return (*pred.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<usize> = Rc::new(RefCell::new(300_usize));
    let V: Value<usize> = Rc::new(RefCell::new((*N.borrow()).wrapping_mul((*N.borrow()))));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: Rc::new(RefCell::new(((*V.borrow()) as u32))),
        adj: Rc::new(RefCell::new(Ptr::alloc_array(
            (0..(*V.borrow()))
                .map(|_| Ptr::<GraphNode>::null())
                .collect::<Box<[Ptr<GraphNode>]>>(),
        ))),
    }));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*V.borrow())) {
        (*(*graph.borrow()).adj.borrow())
            .offset((*i.borrow()) as isize)
            .write(Ptr::<GraphNode>::null());
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
            ((*(*graph.borrow()).adj.borrow())
                .offset((*i.borrow()) as isize)
                .read())
            .clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let next: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
                (*(*(*head.borrow()).upgrade().deref()).next.borrow()).clone(),
            ));
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
            ((*pred.borrow()).offset((*i.borrow()) as isize).read()),
        );
        (*i.borrow_mut()).prefix_inc();
    }
    (*(*graph.borrow()).adj.borrow()).delete_array();
    (*pred.borrow()).delete_array();
    return 0;
}
impl GraphImpl for Ptr<Graph> {
    fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        let __rhs = Ptr::alloc(GraphNode {
            vertex: Rc::new(RefCell::new((*dst.borrow()))),
            next: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).adj.borrow())
                    .offset((*src.borrow()) as isize)
                    .read())
                .clone(),
            )),
        });
        (*(*(*self).upgrade().deref()).adj.borrow())
            .offset((*src.borrow()) as isize)
            .write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            vertex: Rc::new(RefCell::new((*src.borrow()))),
            next: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).adj.borrow())
                    .offset((*dst.borrow()) as isize)
                    .read())
                .clone(),
            )),
        });
        (*(*(*self).upgrade().deref()).adj.borrow())
            .offset((*dst.borrow()) as isize)
            .write(__rhs);
    }
}
impl QueueImpl for Ptr<Queue> {
    fn enqueue(&self, elem: i32) {
        let elem: Value<i32> = Rc::new(RefCell::new(elem));
        if ((*(*(*self).upgrade().deref()).back.borrow())
            == (*(*(*self).upgrade().deref()).capacity.borrow()))
        {
            return;
        }
        let __rhs = ((*elem.borrow()) as u32);
        (*(*(*self).upgrade().deref()).elems.borrow())
            .offset(((*(*(*self).upgrade().deref()).back.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
    }
    fn dequeue(&self) -> u32 {
        if ({ QueueImpl::empty(self) }) {
            return (-1_i32 as u32);
        }
        return ((*(*(*self).upgrade().deref()).elems.borrow())
            .offset(((*(*(*self).upgrade().deref()).front.borrow_mut()).postfix_inc()) as isize)
            .read());
    }
    fn empty(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).front.borrow())
            == (*(*(*self).upgrade().deref()).back.borrow()));
    }
}
