extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static kGlyfTableTag_0: Value<u32> = Rc::new(RefCell::new(1735162214_u32));
);
thread_local!(
    pub static kHeadTableTag_1: Value<u32> = Rc::new(RefCell::new(1751474532_u32));
);
thread_local!(
    pub static kLocaTableTag_2: Value<u32> = Rc::new(RefCell::new(1819239265_u32));
);
thread_local!(
    pub static kDsigTableTag_3: Value<u32> = Rc::new(RefCell::new(1146308935_u32));
);
thread_local!(
    pub static kCffTableTag_4: Value<u32> = Rc::new(RefCell::new(1128678944_u32));
);
thread_local!(
    pub static kHmtxTableTag_5: Value<u32> = Rc::new(RefCell::new(1752003704_u32));
);
thread_local!(
    pub static kHheaTableTag_6: Value<u32> = Rc::new(RefCell::new(1751672161_u32));
);
thread_local!(
    pub static kMaxpTableTag_7: Value<u32> = Rc::new(RefCell::new(1835104368_u32));
);
thread_local!(
    pub static kKnownTags_8: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        ((((((('c' as i8) as i32) << 24) | ((('m' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('p' as i8) as i32)) as u32),
        ((((((('h' as i8) as i32) << 24) | ((('e' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('d' as i8) as i32)) as u32),
        ((((((('h' as i8) as i32) << 24) | ((('h' as i8) as i32) << 16))
            | ((('e' as i8) as i32) << 8))
            | (('a' as i8) as i32)) as u32),
        ((((((('h' as i8) as i32) << 24) | ((('m' as i8) as i32) << 16))
            | ((('t' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('m' as i8) as i32) << 24) | ((('a' as i8) as i32) << 16))
            | ((('x' as i8) as i32) << 8))
            | (('p' as i8) as i32)) as u32),
        ((((((('n' as i8) as i32) << 24) | ((('a' as i8) as i32) << 16))
            | ((('m' as i8) as i32) << 8))
            | (('e' as i8) as i32)) as u32),
        ((((((('O' as i8) as i32) << 24) | ((('S' as i8) as i32) << 16))
            | ((('/' as i8) as i32) << 8))
            | (('2' as i8) as i32)) as u32),
        ((((((('p' as i8) as i32) << 24) | ((('o' as i8) as i32) << 16))
            | ((('s' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('c' as i8) as i32) << 24) | ((('v' as i8) as i32) << 16))
            | ((('t' as i8) as i32) << 8))
            | ((' ' as i8) as i32)) as u32),
        ((((((('f' as i8) as i32) << 24) | ((('p' as i8) as i32) << 16))
            | ((('g' as i8) as i32) << 8))
            | (('m' as i8) as i32)) as u32),
        ((((((('g' as i8) as i32) << 24) | ((('l' as i8) as i32) << 16))
            | ((('y' as i8) as i32) << 8))
            | (('f' as i8) as i32)) as u32),
        ((((((('l' as i8) as i32) << 24) | ((('o' as i8) as i32) << 16))
            | ((('c' as i8) as i32) << 8))
            | (('a' as i8) as i32)) as u32),
        ((((((('p' as i8) as i32) << 24) | ((('r' as i8) as i32) << 16))
            | ((('e' as i8) as i32) << 8))
            | (('p' as i8) as i32)) as u32),
        ((((((('C' as i8) as i32) << 24) | ((('F' as i8) as i32) << 16))
            | ((('F' as i8) as i32) << 8))
            | ((' ' as i8) as i32)) as u32),
        ((((((('V' as i8) as i32) << 24) | ((('O' as i8) as i32) << 16))
            | ((('R' as i8) as i32) << 8))
            | (('G' as i8) as i32)) as u32),
        ((((((('E' as i8) as i32) << 24) | ((('B' as i8) as i32) << 16))
            | ((('D' as i8) as i32) << 8))
            | (('T' as i8) as i32)) as u32),
        ((((((('E' as i8) as i32) << 24) | ((('B' as i8) as i32) << 16))
            | ((('L' as i8) as i32) << 8))
            | (('C' as i8) as i32)) as u32),
        ((((((('g' as i8) as i32) << 24) | ((('a' as i8) as i32) << 16))
            | ((('s' as i8) as i32) << 8))
            | (('p' as i8) as i32)) as u32),
        ((((((('h' as i8) as i32) << 24) | ((('d' as i8) as i32) << 16))
            | ((('m' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('k' as i8) as i32) << 24) | ((('e' as i8) as i32) << 16))
            | ((('r' as i8) as i32) << 8))
            | (('n' as i8) as i32)) as u32),
        ((((((('L' as i8) as i32) << 24) | ((('T' as i8) as i32) << 16))
            | ((('S' as i8) as i32) << 8))
            | (('H' as i8) as i32)) as u32),
        ((((((('P' as i8) as i32) << 24) | ((('C' as i8) as i32) << 16))
            | ((('L' as i8) as i32) << 8))
            | (('T' as i8) as i32)) as u32),
        ((((((('V' as i8) as i32) << 24) | ((('D' as i8) as i32) << 16))
            | ((('M' as i8) as i32) << 8))
            | (('X' as i8) as i32)) as u32),
        ((((((('v' as i8) as i32) << 24) | ((('h' as i8) as i32) << 16))
            | ((('e' as i8) as i32) << 8))
            | (('a' as i8) as i32)) as u32),
        ((((((('v' as i8) as i32) << 24) | ((('m' as i8) as i32) << 16))
            | ((('t' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('B' as i8) as i32) << 24) | ((('A' as i8) as i32) << 16))
            | ((('S' as i8) as i32) << 8))
            | (('E' as i8) as i32)) as u32),
        ((((((('G' as i8) as i32) << 24) | ((('D' as i8) as i32) << 16))
            | ((('E' as i8) as i32) << 8))
            | (('F' as i8) as i32)) as u32),
        ((((((('G' as i8) as i32) << 24) | ((('P' as i8) as i32) << 16))
            | ((('O' as i8) as i32) << 8))
            | (('S' as i8) as i32)) as u32),
        ((((((('G' as i8) as i32) << 24) | ((('S' as i8) as i32) << 16))
            | ((('U' as i8) as i32) << 8))
            | (('B' as i8) as i32)) as u32),
        ((((((('E' as i8) as i32) << 24) | ((('B' as i8) as i32) << 16))
            | ((('S' as i8) as i32) << 8))
            | (('C' as i8) as i32)) as u32),
        ((((((('J' as i8) as i32) << 24) | ((('S' as i8) as i32) << 16))
            | ((('T' as i8) as i32) << 8))
            | (('F' as i8) as i32)) as u32),
        ((((((('M' as i8) as i32) << 24) | ((('A' as i8) as i32) << 16))
            | ((('T' as i8) as i32) << 8))
            | (('H' as i8) as i32)) as u32),
        ((((((('C' as i8) as i32) << 24) | ((('B' as i8) as i32) << 16))
            | ((('D' as i8) as i32) << 8))
            | (('T' as i8) as i32)) as u32),
        ((((((('C' as i8) as i32) << 24) | ((('B' as i8) as i32) << 16))
            | ((('L' as i8) as i32) << 8))
            | (('C' as i8) as i32)) as u32),
        ((((((('C' as i8) as i32) << 24) | ((('O' as i8) as i32) << 16))
            | ((('L' as i8) as i32) << 8))
            | (('R' as i8) as i32)) as u32),
        ((((((('C' as i8) as i32) << 24) | ((('P' as i8) as i32) << 16))
            | ((('A' as i8) as i32) << 8))
            | (('L' as i8) as i32)) as u32),
        ((((((('S' as i8) as i32) << 24) | ((('V' as i8) as i32) << 16))
            | ((('G' as i8) as i32) << 8))
            | ((' ' as i8) as i32)) as u32),
        ((((((('s' as i8) as i32) << 24) | ((('b' as i8) as i32) << 16))
            | ((('i' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('a' as i8) as i32) << 24) | ((('c' as i8) as i32) << 16))
            | ((('n' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('a' as i8) as i32) << 24) | ((('v' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('r' as i8) as i32)) as u32),
        ((((((('b' as i8) as i32) << 24) | ((('d' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('b' as i8) as i32) << 24) | ((('l' as i8) as i32) << 16))
            | ((('o' as i8) as i32) << 8))
            | (('c' as i8) as i32)) as u32),
        ((((((('b' as i8) as i32) << 24) | ((('s' as i8) as i32) << 16))
            | ((('l' as i8) as i32) << 8))
            | (('n' as i8) as i32)) as u32),
        ((((((('c' as i8) as i32) << 24) | ((('v' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('r' as i8) as i32)) as u32),
        ((((((('f' as i8) as i32) << 24) | ((('d' as i8) as i32) << 16))
            | ((('s' as i8) as i32) << 8))
            | (('c' as i8) as i32)) as u32),
        ((((((('f' as i8) as i32) << 24) | ((('e' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('f' as i8) as i32) << 24) | ((('m' as i8) as i32) << 16))
            | ((('t' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('f' as i8) as i32) << 24) | ((('v' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('r' as i8) as i32)) as u32),
        ((((((('g' as i8) as i32) << 24) | ((('v' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('r' as i8) as i32)) as u32),
        ((((((('h' as i8) as i32) << 24) | ((('s' as i8) as i32) << 16))
            | ((('t' as i8) as i32) << 8))
            | (('y' as i8) as i32)) as u32),
        ((((((('j' as i8) as i32) << 24) | ((('u' as i8) as i32) << 16))
            | ((('s' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('l' as i8) as i32) << 24) | ((('c' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('r' as i8) as i32)) as u32),
        ((((((('m' as i8) as i32) << 24) | ((('o' as i8) as i32) << 16))
            | ((('r' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('m' as i8) as i32) << 24) | ((('o' as i8) as i32) << 16))
            | ((('r' as i8) as i32) << 8))
            | (('x' as i8) as i32)) as u32),
        ((((((('o' as i8) as i32) << 24) | ((('p' as i8) as i32) << 16))
            | ((('b' as i8) as i32) << 8))
            | (('d' as i8) as i32)) as u32),
        ((((((('p' as i8) as i32) << 24) | ((('r' as i8) as i32) << 16))
            | ((('o' as i8) as i32) << 8))
            | (('p' as i8) as i32)) as u32),
        ((((((('t' as i8) as i32) << 24) | ((('r' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('k' as i8) as i32)) as u32),
        ((((((('Z' as i8) as i32) << 24) | ((('a' as i8) as i32) << 16))
            | ((('p' as i8) as i32) << 8))
            | (('f' as i8) as i32)) as u32),
        ((((((('S' as i8) as i32) << 24) | ((('i' as i8) as i32) << 16))
            | ((('l' as i8) as i32) << 8))
            | (('f' as i8) as i32)) as u32),
        ((((((('G' as i8) as i32) << 24) | ((('l' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('G' as i8) as i32) << 24) | ((('l' as i8) as i32) << 16))
            | ((('o' as i8) as i32) << 8))
            | (('c' as i8) as i32)) as u32),
        ((((((('F' as i8) as i32) << 24) | ((('e' as i8) as i32) << 16))
            | ((('a' as i8) as i32) << 8))
            | (('t' as i8) as i32)) as u32),
        ((((((('S' as i8) as i32) << 24) | ((('i' as i8) as i32) << 16))
            | ((('l' as i8) as i32) << 8))
            | (('l' as i8) as i32)) as u32),
    ])));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct woff2_Buffer {
    #[offset(0)]
    #[byte_size(8)]
    buffer_: Ptr<u8>,
    #[offset(8)]
    length_: usize,
    #[offset(16)]
    offset_: usize,
}
impl woff2_Buffer {
    pub fn new(data: Ptr<u8>, len: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        let __this: Value<woff2_Buffer> = Rc::new(RefCell::new(Self {
            buffer_: (*data.borrow()).clone(),
            length_: (*len.borrow()),
            offset_: 0_usize,
        }));
        let this: Ptr<woff2_Buffer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn Size255UShort_9(value: u16) -> usize {
    let value: Value<u16> = Rc::new(RefCell::new(value));
    let result: Value<usize> = Rc::new(RefCell::new(3_usize));
    if (((*value.borrow()) as i32) < 253) {
        (*result.borrow_mut()) = 1_usize;
    } else if (((*value.borrow()) as i32) < 762) {
        (*result.borrow_mut()) = 2_usize;
    } else {
        (*result.borrow_mut()) = 3_usize;
    }
    return (*result.borrow());
}
pub fn Write255UShort_10(out: Ptr<Vec<u8>>, value: i32) {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ((*value.borrow()) < 253) {
        {
            let __a1 = ((*value.borrow()) as u8);
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else if ((*value.borrow()) < 506) {
        {
            let __a1 = 255_u8;
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = (((*value.borrow()) - 253) as u8);
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else if ((*value.borrow()) < 762) {
        {
            let __a1 = 254_u8;
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = (((*value.borrow()) - 506) as u8);
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else {
        {
            let __a1 = 253_u8;
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = (((*value.borrow()) >> 8) as u8);
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = (((*value.borrow()) & 255) as u8);
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    }
}
pub fn Store255UShort_11(val: i32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let packed: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    ({ Write255UShort_10((packed.as_pointer()), (*val.borrow())) });
    'loop_: for mut packed_byte in packed.as_pointer() as Ptr<u8> {
        let packed_byte: Value<u8> = Rc::new(RefCell::new(packed_byte.read()));
        let __rhs = (*packed_byte.borrow());
        elem!(
            (*dst.borrow()),
            (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
        )
        .write(__rhs);
    }
}
pub fn Read255UShort_12(buf: Ptr<woff2_Buffer>, value: Ptr<u32>) -> bool {
    let buf: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(buf));
    let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
    thread_local!(
        static kWordCode_13: Value<i32> = Rc::new(RefCell::new(253));
    );
    thread_local!(
        static kOneMoreByteCode2_14: Value<i32> = Rc::new(RefCell::new(254));
    );
    thread_local!(
        static kOneMoreByteCode1_15: Value<i32> = Rc::new(RefCell::new(255));
    );
    thread_local!(
        static kLowestUCode_16: Value<i32> = Rc::new(RefCell::new(253));
    );
    let code: Value<u8> = Rc::new(RefCell::new(0_u8));
    if !({ woff2_BufferImpl::ReadU8(&(*buf.borrow()), (code.as_pointer())) }) {
        return false;
    }
    if (((*code.borrow()) as i32) == kWordCode_13.with(|rc| *rc.borrow())) {
        let result: Value<u16> = Rc::new(RefCell::new(0_u16));
        if !({ woff2_BufferImpl::ReadU16(&(*buf.borrow()), (result.as_pointer())) }) {
            return false;
        }
        (*value.borrow()).write({ ((*result.borrow()) as u32) });
        return true;
    } else if (((*code.borrow()) as i32) == kOneMoreByteCode1_15.with(|rc| *rc.borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&(*buf.borrow()), (result.as_pointer())) }) {
            return false;
        }
        (*value.borrow()).write({
            ((((*result.borrow()) as i32) + kLowestUCode_16.with(|rc| *rc.borrow())) as u32)
        });
        return true;
    } else if (((*code.borrow()) as i32) == kOneMoreByteCode2_14.with(|rc| *rc.borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&(*buf.borrow()), (result.as_pointer())) }) {
            return false;
        }
        (*value.borrow()).write({
            ((((*result.borrow()) as i32) + (kLowestUCode_16.with(|rc| *rc.borrow()) * 2)) as u32)
        });
        return true;
    } else {
        (*value.borrow()).write({ ((*code.borrow()) as u32) });
        return true;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ReadBase128_17(buf: Ptr<woff2_Buffer>, value: Ptr<u32>) -> bool {
    let buf: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(buf));
    let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
    let result: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < 5_usize) {
        let code: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&(*buf.borrow()), (code.as_pointer())) }) {
            return false;
        }
        if ((*i.borrow()) == 0_usize) && (((*code.borrow()) as i32) == 128) {
            return false;
        }
        if (((*result.borrow()) & 4261412864_u32) != 0) {
            return false;
        }
        (*result.borrow_mut()) =
            { (((*result.borrow()) << 7) | ((((*code.borrow()) as i32) & 127) as u32)) };
        if ((((*code.borrow()) as i32) & 128) == 0) {
            (*value.borrow()).write({ (*result.borrow()) });
            return true;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return false;
}
pub fn Base128Size_18(n: usize) -> usize {
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let size: Value<usize> = Rc::new(RefCell::new(1_usize));
    'loop_: while ((*n.borrow()) >= 128_usize) {
        (*size.borrow_mut()).prefix_inc();
        (*n.borrow_mut()) >>= 7;
    }
    return (*size.borrow());
}
pub fn StoreBase128_19(len: usize, offset: Ptr<usize>, dst: Ptr<u8>) {
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let size: Value<usize> = Rc::new(RefCell::new(({ Base128Size_18((*len.borrow())) })));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*size.borrow())) {
        let b: Value<i32> = Rc::new(RefCell::new(
            ((((*len.borrow())
                >> ((7_usize).wrapping_mul(
                    (((*size.borrow()).wrapping_sub((*i.borrow()))).wrapping_sub(1_usize)),
                )))
                & 127_usize) as i32),
        ));
        if ((*i.borrow()) < (*size.borrow()).wrapping_sub(1_usize)) {
            (*b.borrow_mut()) |= 128;
        }
        let __rhs = ((*b.borrow()) as u8);
        elem!(
            (*dst.borrow()),
            (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
        )
        .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
}
thread_local!(
    pub static kWoff2Signature_20: Value<u32> = Rc::new(RefCell::new(2001684018_u32));
);
thread_local!(
    pub static kWoff2FlagsTransform_21: Value<u32> = Rc::new(RefCell::new(((1 << 8) as u32)));
);
thread_local!(
    pub static kTtcFontFlavor_22: Value<u32> = Rc::new(RefCell::new(1953784678_u32));
);
thread_local!(
    pub static kSfntHeaderSize_23: Value<usize> = Rc::new(RefCell::new(12_usize));
);
thread_local!(
    pub static kSfntEntrySize_24: Value<usize> = Rc::new(RefCell::new(16_usize));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct woff2_Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    pub on_curve: bool,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct woff2_Table {
    #[offset(0)]
    pub tag: u32,
    #[offset(4)]
    pub flags: u32,
    #[offset(8)]
    pub src_offset: u32,
    #[offset(12)]
    pub src_length: u32,
    #[offset(16)]
    pub transform_length: u32,
    #[offset(20)]
    pub dst_offset: u32,
    #[offset(24)]
    pub dst_length: u32,
    #[offset(32)]
    #[byte_size(8)]
    pub dst_data: Ptr<u8>,
}
impl std::cmp::Ord for woff2_Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if woff2_TableImpl::operator_lt(
                &Rc::new(RefCell::new(woff2_Table {
                    tag: self.tag.clone(),
                    flags: self.flags.clone(),
                    src_offset: self.src_offset.clone(),
                    src_length: self.src_length.clone(),
                    transform_length: self.transform_length.clone(),
                    dst_offset: self.dst_offset.clone(),
                    dst_length: self.dst_length.clone(),
                    dst_data: self.dst_data.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(woff2_Table {
                    tag: other.tag.clone(),
                    flags: other.flags.clone(),
                    src_offset: other.src_offset.clone(),
                    src_length: other.src_length.clone(),
                    transform_length: other.transform_length.clone(),
                    dst_offset: other.dst_offset.clone(),
                    dst_length: other.dst_length.clone(),
                    dst_data: other.dst_data.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if woff2_TableImpl::operator_lt(
                &Rc::new(RefCell::new(woff2_Table {
                    tag: other.tag.clone(),
                    flags: other.flags.clone(),
                    src_offset: other.src_offset.clone(),
                    src_length: other.src_length.clone(),
                    transform_length: other.transform_length.clone(),
                    dst_offset: other.dst_offset.clone(),
                    dst_length: other.dst_length.clone(),
                    dst_data: other.dst_data.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(woff2_Table {
                    tag: self.tag.clone(),
                    flags: self.flags.clone(),
                    src_offset: self.src_offset.clone(),
                    src_length: self.src_length.clone(),
                    transform_length: self.transform_length.clone(),
                    dst_offset: self.dst_offset.clone(),
                    dst_length: self.dst_length.clone(),
                    dst_data: self.dst_data.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for woff2_Table {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for woff2_Table {
    fn eq(&self, other: &Self) -> bool {
        {
            !(woff2_TableImpl::operator_lt(
                &Rc::new(RefCell::new(woff2_Table {
                    tag: self.tag.clone(),
                    flags: self.flags.clone(),
                    src_offset: self.src_offset.clone(),
                    src_length: self.src_length.clone(),
                    transform_length: self.transform_length.clone(),
                    dst_offset: self.dst_offset.clone(),
                    dst_length: self.dst_length.clone(),
                    dst_data: self.dst_data.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(woff2_Table {
                    tag: other.tag.clone(),
                    flags: other.flags.clone(),
                    src_offset: other.src_offset.clone(),
                    src_length: other.src_length.clone(),
                    transform_length: other.transform_length.clone(),
                    dst_offset: other.dst_offset.clone(),
                    dst_length: other.dst_length.clone(),
                    dst_data: other.dst_data.clone(),
                }))
                .as_pointer(),
            )) && !(woff2_TableImpl::operator_lt(
                &Rc::new(RefCell::new(woff2_Table {
                    tag: other.tag.clone(),
                    flags: other.flags.clone(),
                    src_offset: other.src_offset.clone(),
                    src_length: other.src_length.clone(),
                    transform_length: other.transform_length.clone(),
                    dst_offset: other.dst_offset.clone(),
                    dst_length: other.dst_length.clone(),
                    dst_data: other.dst_data.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(woff2_Table {
                    tag: self.tag.clone(),
                    flags: self.flags.clone(),
                    src_offset: self.src_offset.clone(),
                    src_length: self.src_length.clone(),
                    transform_length: self.transform_length.clone(),
                    dst_offset: self.dst_offset.clone(),
                    dst_length: self.dst_length.clone(),
                    dst_data: self.dst_data.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for woff2_Table {}
pub fn Log2Floor_25(n: u32) -> i32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) == 0_u32) {
        -1_i32
    } else {
        (31 ^ (*n.borrow()).leading_zeros() as i32)
    };
}
pub fn ComputeULongSum_26(buf: Ptr<u8>, size: usize) -> u32 {
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let size: Value<usize> = Rc::new(RefCell::new(size));
    let checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    let aligned_size: Value<usize> = Rc::new(RefCell::new(((*size.borrow()) & (!3 as usize))));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*aligned_size.borrow())) {
        (*checksum.borrow_mut()) = {
            (*checksum.borrow()).wrapping_add(
                (((((((elem!((*buf.borrow()), (*i.borrow())).read()) as i32) << 24)
                    | (((elem!((*buf.borrow()), (*i.borrow()).wrapping_add(1_usize)).read())
                        as i32)
                        << 16))
                    | (((elem!((*buf.borrow()), (*i.borrow()).wrapping_add(2_usize)).read())
                        as i32)
                        << 8))
                    | ((elem!((*buf.borrow()), (*i.borrow()).wrapping_add(3_usize)).read()) as i32))
                    as u32),
            )
        };
        (*i.borrow_mut()) = { (*i.borrow()).wrapping_add(4_usize) };
    }
    if ((*size.borrow()) != (*aligned_size.borrow())) {
        let v: Value<u32> = Rc::new(RefCell::new(0_u32));
        let i: Value<usize> = Rc::new(RefCell::new((*aligned_size.borrow())));
        'loop_: while ((*i.borrow()) < (*size.borrow())) {
            (*v.borrow_mut()) |= (({ ((elem!((*buf.borrow()), (*i.borrow())).read()) as i32) } << {
                ((24_usize).wrapping_sub((8_usize).wrapping_mul(((*i.borrow()) & 3_usize))))
            }) as u32);
            (*i.borrow_mut()).prefix_inc();
        }
        (*checksum.borrow_mut()) = { (*checksum.borrow()).wrapping_add((*v.borrow())) };
    }
    return (*checksum.borrow());
}
pub fn CollectionHeaderSize_27(header_version: u32, num_fonts: u32) -> usize {
    let header_version: Value<u32> = Rc::new(RefCell::new(header_version));
    let num_fonts: Value<u32> = Rc::new(RefCell::new(num_fonts));
    let size: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*header_version.borrow()) == 131072_u32) {
        (*size.borrow_mut()) = { (*size.borrow()).wrapping_add(12_usize) };
    }
    if ((*header_version.borrow()) == 65536_u32) || ((*header_version.borrow()) == 131072_u32) {
        (*size.borrow_mut()) = {
            (*size.borrow()).wrapping_add(
                (((12_u32).wrapping_add((4_u32).wrapping_mul((*num_fonts.borrow())))) as usize),
            )
        };
    }
    return (*size.borrow());
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
pub struct woff2_Font_Table {
    #[offset(0)]
    pub tag: u32,
    #[offset(4)]
    pub checksum: u32,
    #[offset(8)]
    pub offset: u32,
    #[offset(12)]
    pub length: u32,
    #[offset(16)]
    #[byte_size(8)]
    pub data: Ptr<u8>,
    #[offset(24)]
    #[byte_size(24)]
    pub buffer: Value<Vec<u8>>,
    #[offset(48)]
    #[byte_size(8)]
    pub reuse_of: Ptr<woff2_Font_Table>,
    #[offset(56)]
    pub flag_byte: u8,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(56)]
pub struct woff2_Font {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub num_tables: u16,
    #[offset(8)]
    #[byte_size(48)]
    pub tables: BTreeMap<u32, Value<woff2_Font_Table>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct woff2_FontCollection {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub header_version: u32,
    #[offset(8)]
    #[byte_size(48)]
    pub tables: BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>,
    #[offset(56)]
    #[byte_size(24)]
    pub fonts: Value<Vec<woff2_Font>>,
}
pub fn StoreU32_28(dst: Ptr<u8>, offset: usize, x: u32) -> usize {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    let x: Value<u32> = Rc::new(RefCell::new(x));
    elem!((*dst.borrow()), (*offset.borrow())).write({ (((*x.borrow()) >> 24) as u8) });
    elem!((*dst.borrow()), (*offset.borrow()).wrapping_add(1_usize))
        .write({ (((*x.borrow()) >> 16) as u8) });
    elem!((*dst.borrow()), (*offset.borrow()).wrapping_add(2_usize))
        .write({ (((*x.borrow()) >> 8) as u8) });
    elem!((*dst.borrow()), (*offset.borrow()).wrapping_add(3_usize))
        .write({ ((*x.borrow()) as u8) });
    return (*offset.borrow()).wrapping_add(4_usize);
}
pub fn Store16_29(dst: Ptr<u8>, offset: usize, x: i32) -> usize {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    elem!((*dst.borrow()), (*offset.borrow())).write({ (((*x.borrow()) >> 8) as u8) });
    elem!((*dst.borrow()), (*offset.borrow()).wrapping_add(1_usize))
        .write({ ((*x.borrow()) as u8) });
    return (*offset.borrow()).wrapping_add(2_usize);
}
pub fn StoreU32_30(val: u32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<u32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let __rhs = (((*val.borrow()) >> 24) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    let __rhs = (((*val.borrow()) >> 16) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    let __rhs = (((*val.borrow()) >> 8) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    let __rhs = ((*val.borrow()) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
}
pub fn Store16_31(val: i32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let __rhs = (((*val.borrow()) >> 8) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
    let __rhs = ((*val.borrow()) as u8);
    elem!(
        (*dst.borrow()),
        (*offset.borrow()).with_mut(|__v| __v.postfix_inc())
    )
    .write(__rhs);
}
pub fn StoreBytes_32(data: Ptr<u8>, len: usize, offset: Ptr<usize>, dst: Ptr<u8>) {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    {
        (((*dst.borrow()).offset(((*offset.borrow()).read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(&(*data.borrow()).to_any(), (*len.borrow()) as usize);
        (((*dst.borrow()).offset(((*offset.borrow()).read()) as isize)) as Ptr<u8>).to_any()
    };
    (*offset.borrow()).write({ ((*offset.borrow()).read()).wrapping_add((*len.borrow())) });
}
pub fn ReadTrueTypeFont_33(
    file: Ptr<woff2_Buffer>,
    data: Ptr<u8>,
    len: usize,
    font: Ptr<woff2_Font>,
) -> bool {
    let file: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(file));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    if (!({
        woff2_BufferImpl::ReadU16(
            &(*file.borrow()),
            (field_ptr!((*font.borrow()), num_tables)),
        )
    })) || (!({ woff2_BufferImpl::Skip(&(*file.borrow()), 6_usize) }))
    {
        return false;
    }
    let intervals: Value<BTreeMap<u32, Value<u32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let i: Value<u16> = Rc::new(RefCell::new(0_u16));
    'loop_: while ({ ((*i.borrow()) as i32) } < {
        ((*font.borrow()).with(|__s| __s.num_tables) as i32)
    }) {
        let table: Value<woff2_Font_Table> = Rc::new(RefCell::new(<woff2_Font_Table>::default()));
        (*table.borrow_mut()).flag_byte = 0_u8;
        (*table.borrow_mut()).reuse_of = Ptr::<woff2_Font_Table>::null();
        if (((!({
            woff2_BufferImpl::ReadU32(&(*file.borrow()), (field_ptr!(table.as_pointer(), tag)))
        })) || (!({
            woff2_BufferImpl::ReadU32(
                &(*file.borrow()),
                (field_ptr!(table.as_pointer(), checksum)),
            )
        }))) || (!({
            woff2_BufferImpl::ReadU32(&(*file.borrow()), (field_ptr!(table.as_pointer(), offset)))
        }))) || (!({
            woff2_BufferImpl::ReadU32(&(*file.borrow()), (field_ptr!(table.as_pointer(), length)))
        })) {
            return false;
        }
        if ((({ (*table.borrow()).offset } & 3_u32) != 0_u32)
            || (({ (*table.borrow()).length } as usize) > (*len.borrow())))
            || ((*len.borrow()).wrapping_sub(({ (*table.borrow()).length } as usize))
                < ({ (*table.borrow()).offset } as usize))
        {
            return false;
        }
        let __rhs = { (*table.borrow()).length };
        (intervals.as_pointer() as Ptr<BTreeMap<u32, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                __v.entry({ (*table.borrow()).offset })
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*table.borrow_mut()).data =
            { (*data.borrow()).offset(({ (*table.borrow()).offset }) as isize) };
        if RefcountMapIter::find_key(
            (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &{ (*table.borrow()).tag },
        ) != RefcountMapIter::end(
            (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
        ) {
            return false;
        }
        let __rhs = (*table.borrow()).clone();
        (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                __v.entry({ (*table.borrow()).tag })
                    .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let last_offset: Value<u32> = Rc::new(RefCell::new(
        (((12_u64 as u64).wrapping_add(
            (16_u64 as u64).wrapping_mul(((*font.borrow()).with(|__s| __s.num_tables) as u64)),
        )) as u32),
    ));
    'loop_: for i in RefcountMapIter::begin(intervals.as_pointer()) {
        if ({ (*i.first().borrow()) } < { (*last_offset.borrow()) })
            || ({ (*i.first().borrow()).wrapping_add((*i.second().borrow())) } < {
                (*i.first().borrow())
            })
        {
            return false;
        }
        (*last_offset.borrow_mut()) = (*i.first().borrow()).wrapping_add((*i.second().borrow()));
    }
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            woff2_FontImpl::FindTable_2(&(*font.borrow()), kHeadTableTag_1.with(|rc| *rc.borrow()))
        }),
    ));
    if (!((*head_table.borrow()).is_null()))
        && ((*head_table.borrow()).with(|__s| __s.length) < 52_u32)
    {
        return false;
    }
    return true;
}
pub fn ReadCollectionFont_34(
    file: Ptr<woff2_Buffer>,
    data: Ptr<u8>,
    len: usize,
    font: Ptr<woff2_Font>,
    all_tables: Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>,
) -> bool {
    let file: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(file));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let all_tables: Value<Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>> =
        Rc::new(RefCell::new(all_tables));
    if !({ woff2_BufferImpl::ReadU32(&(*file.borrow()), (field_ptr!((*font.borrow()), flavor))) }) {
        return false;
    }
    if !({
        ReadTrueTypeFont_33(
            (*file.borrow()).clone(),
            (*data.borrow()).clone(),
            (*len.borrow()),
            (*font.borrow()).clone(),
        )
    }) {
        return false;
    }
    'loop_: for entry in RefcountMapIter::begin(field_ptr!((*font.borrow()), tables)) {
        let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
        if RefcountMapIter::find_key(
            ((*all_tables.borrow()).clone() as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>),
            &table.with(|__s| __s.offset),
        ) == RefcountMapIter::end(
            ((*all_tables.borrow()).clone() as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>),
        ) {
            let __rhs =
                ({ woff2_FontImpl::FindTable_2(&(*font.borrow()), table.with(|__s| __s.tag)) });
            ((*all_tables.borrow()).clone() as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>| {
                    __v.entry(table.with(|__s| __s.offset))
                        .or_insert_with(|| {
                            Rc::new(RefCell::new(<Ptr<woff2_Font_Table>>::default()))
                        })
                        .as_pointer()
                })
                .write(__rhs);
        } else {
            let __rhs = (((*all_tables.borrow()).clone()
                as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>| {
                    __v.entry(table.with(|__s| __s.offset))
                        .or_insert_with(|| {
                            Rc::new(RefCell::new(<Ptr<woff2_Font_Table>>::default()))
                        })
                        .as_pointer()
                })
                .read())
            .clone();
            field!(table, reuse_of).write(__rhs);
            if ({ table.with(|__s| __s.tag) } != {
                table.with(|__s| __s.reuse_of.clone()).with(|__s| __s.tag)
            }) {
                return false;
            }
        }
    }
    return true;
}
pub fn ReadTrueTypeCollection_35(
    file: Ptr<woff2_Buffer>,
    data: Ptr<u8>,
    len: usize,
    font_collection: Ptr<woff2_FontCollection>,
) -> bool {
    let file: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(file));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let font_collection: Value<Ptr<woff2_FontCollection>> = Rc::new(RefCell::new(font_collection));
    let num_fonts: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (!({
        woff2_BufferImpl::ReadU32(
            &(*file.borrow()),
            (field_ptr!((*font_collection.borrow()), header_version)),
        )
    })) || (!({ woff2_BufferImpl::ReadU32(&(*file.borrow()), (num_fonts.as_pointer())) }))
    {
        return false;
    }
    let offsets: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ((*num_fonts.borrow()) as usize)) {
        let offset: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !({ woff2_BufferImpl::ReadU32(&(*file.borrow()), (offset.as_pointer())) }) {
            return false;
        }
        {
            let a0_clone = (*offset.borrow()).clone();
            (*offsets.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    {
        let __a0 = (*offsets.borrow()).len() as usize;
        (*(*font_collection.borrow())
            .with(|__s| __s.fonts.clone())
            .borrow_mut())
        .resize_with(__a0, || <woff2_Font>::default())
    };
    let font_it: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(
        ((*font_collection.borrow()).with(|__s| __s.fonts.as_pointer()) as Ptr<woff2_Font>),
    ));
    let all_tables: Value<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    'loop_: for offset in offsets.as_pointer() as Ptr<u32> {
        let offset: Value<u32> = Rc::new(RefCell::new(offset.read()));
        if !({ woff2_BufferImpl::set_offset(&(*file.borrow()), ((*offset.borrow()) as usize)) }) {
            return false;
        }
        let font: Ptr<woff2_Font> = (*font_it.borrow_mut()).postfix_inc();
        if !({
            ReadCollectionFont_34(
                (*file.borrow()).clone(),
                (*data.borrow()).clone(),
                (*len.borrow()),
                (font).clone(),
                (all_tables.as_pointer()),
            )
        }) {
            return false;
        }
    }
    return true;
}
pub fn ReadFont_36(data: Ptr<u8>, len: usize, font: Ptr<woff2_Font>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*data.borrow()).clone() },
        { (*len.borrow()) },
    )));
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!((*font.borrow()), flavor))) })
    {
        return false;
    }
    if ({ (*font.borrow()).with(|__s| __s.flavor) } == {
        kTtcFontFlavor_22.with(|rc| *rc.borrow())
    }) {
        return false;
    }
    return ({
        ReadTrueTypeFont_33(
            (file.as_pointer()),
            (*data.borrow()).clone(),
            (*len.borrow()),
            (*font.borrow()).clone(),
        )
    });
}
pub fn ReadFontCollection_37(
    data: Ptr<u8>,
    len: usize,
    font_collection: Ptr<woff2_FontCollection>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let font_collection: Value<Ptr<woff2_FontCollection>> = Rc::new(RefCell::new(font_collection));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*data.borrow()).clone() },
        { (*len.borrow()) },
    )));
    if !({
        woff2_BufferImpl::ReadU32(
            &file.as_pointer(),
            (field_ptr!((*font_collection.borrow()), flavor)),
        )
    }) {
        return false;
    }
    if ({ (*font_collection.borrow()).with(|__s| __s.flavor) } != {
        kTtcFontFlavor_22.with(|rc| *rc.borrow())
    }) {
        {
            let __a0 = 1_usize as usize;
            (*(*font_collection.borrow())
                .with(|__s| __s.fonts.clone())
                .borrow_mut())
            .resize_with(__a0, || <woff2_Font>::default())
        };
        let font: Ptr<woff2_Font> = ((*font_collection.borrow()).with(|__s| __s.fonts.as_pointer())
            as Ptr<woff2_Font>)
            .offset(0_usize);
        field!(font, flavor).write((*font_collection.borrow()).with(|__s| __s.flavor));
        return ({
            ReadTrueTypeFont_33(
                (file.as_pointer()),
                (*data.borrow()).clone(),
                (*len.borrow()),
                (font).clone(),
            )
        });
    }
    return ({
        ReadTrueTypeCollection_35(
            (file.as_pointer()),
            (*data.borrow()).clone(),
            (*len.borrow()),
            (*font_collection.borrow()).clone(),
        )
    });
}
pub fn FontFileSize_38(font: Ptr<woff2_Font>) -> usize {
    let max_offset: Value<usize> = Rc::new(RefCell::new(
        (((12_u64 as u64)
            .wrapping_add((16_u64 as u64).wrapping_mul((font.with(|__s| __s.num_tables) as u64))))
            as usize),
    ));
    'loop_: for i in RefcountMapIter::begin(field_ptr!(font, tables)) {
        let table: Ptr<woff2_Font_Table> = i.second().as_pointer();
        let padding_size: Value<usize> = Rc::new(RefCell::new(
            ((((4_u32).wrapping_sub((table.with(|__s| __s.length) & 3_u32))) & 3_u32) as usize),
        ));
        let end_offset: Value<usize> = Rc::new(RefCell::new(
            ((*padding_size.borrow()).wrapping_add((table.with(|__s| __s.offset) as usize)))
                .wrapping_add((table.with(|__s| __s.length) as usize)),
        ));
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*max_offset.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*end_offset.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        (*max_offset.borrow_mut()) = __rhs;
    }
    return (*max_offset.borrow());
}
pub fn FontCollectionFileSize_39(font_collection: Ptr<woff2_FontCollection>) -> usize {
    let max_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for mut font in font_collection.with(|__s| __s.fonts.as_pointer()) as Ptr<woff2_Font> {
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*max_offset.borrow()) as u64)));
            let __tmp_1: Value<u64> =
                Rc::new(RefCell::new((({ FontFileSize_38((font).clone()) }) as u64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        (*max_offset.borrow_mut()) = __rhs;
    }
    return (*max_offset.borrow());
}
pub fn WriteFont_40(font: Ptr<woff2_Font>, dst: Ptr<u8>, dst_size: usize) -> bool {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    return ({
        let _font: Ptr<woff2_Font> = (font).clone();
        let _offset: Ptr<usize> = (offset.as_pointer());
        let _dst: Ptr<u8> = (*dst.borrow()).clone();
        let _dst_size: usize = (*dst_size.borrow());
        WriteFont_41(_font, _offset, _dst, _dst_size)
    });
}
pub fn WriteTableRecord_42(
    table: Ptr<woff2_Font_Table>,
    offset: Ptr<usize>,
    dst: Ptr<u8>,
    dst_size: usize,
) -> bool {
    let table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(table));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    if ({ (*dst_size.borrow()) } < {
        ((*offset.borrow()).read()).wrapping_add(kSfntEntrySize_24.with(|rc| *rc.borrow()))
    }) {
        return false;
    }
    if ({ woff2_Font_TableImpl::IsReused(&(*table.borrow())) }) {
        (*table.borrow_mut()) = { (*table.borrow()).with(|__s| __s.reuse_of.clone()) };
    }
    ({
        StoreU32_30(
            (*table.borrow()).with(|__s| __s.tag),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*table.borrow()).with(|__s| __s.checksum),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*table.borrow()).with(|__s| __s.offset),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*table.borrow()).with(|__s| __s.length),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    return true;
}
pub fn WriteTable_43(
    table: Ptr<woff2_Font_Table>,
    offset: Ptr<usize>,
    dst: Ptr<u8>,
    dst_size: usize,
) -> bool {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    if !({
        let _offset: Ptr<usize> = (*offset.borrow()).clone();
        let _dst_size: usize = (*dst_size.borrow());
        WriteTableRecord_42((table).clone(), _offset, (*dst.borrow()).clone(), _dst_size)
    }) {
        return false;
    }
    if !({ woff2_Font_TableImpl::IsReused(&table) }) {
        if ({ (table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length)) } < {
            table.with(|__s| __s.offset)
        }) || ({ (*dst_size.borrow()) } < {
            (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length))) as usize)
        }) {
            return false;
        }
        {
            ((*dst.borrow()).offset((table.with(|__s| __s.offset)) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &(table.with(|__s| __s.data.clone()) as Ptr<u8>).to_any(),
                    (table.with(|__s| __s.length) as usize) as usize,
                );
            ((*dst.borrow()).offset((table.with(|__s| __s.offset)) as isize) as Ptr<u8>).to_any()
        };
        let padding_size: Value<usize> = Rc::new(RefCell::new(
            ((((4_u32).wrapping_sub((table.with(|__s| __s.length) & 3_u32))) & 3_u32) as usize),
        ));
        if ({
            (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length))) as usize)
                .wrapping_add((*padding_size.borrow()))
        } < { (*padding_size.borrow()) })
            || ({ (*dst_size.borrow()) } < {
                (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length)))
                    as usize)
                    .wrapping_add((*padding_size.borrow()))
            })
        {
            return false;
        }
        {
            ((*dst.borrow())
                .offset((table.with(|__s| __s.offset)) as isize)
                .offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>)
                .to_any()
                .memset((0) as u8, (*padding_size.borrow()) as usize);
            ((*dst.borrow())
                .offset((table.with(|__s| __s.offset)) as isize)
                .offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>)
                .to_any()
        };
    }
    return true;
}
pub fn WriteFont_41(
    font: Ptr<woff2_Font>,
    offset: Ptr<usize>,
    dst: Ptr<u8>,
    dst_size: usize,
) -> bool {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    if ({ ((*dst_size.borrow()) as u64) } < {
        (12_u64 as u64)
            .wrapping_add((16_u64 as u64).wrapping_mul((font.with(|__s| __s.num_tables) as u64)))
    }) {
        return false;
    }
    ({
        StoreU32_30(
            font.with(|__s| __s.flavor),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            (font.with(|__s| __s.num_tables) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    let max_pow2: Value<u16> = Rc::new(RefCell::new(
        (if (font.with(|__s| __s.num_tables) != 0) {
            ({ Log2Floor_25((font.with(|__s| __s.num_tables) as u32)) })
        } else {
            0
        } as u16),
    ));
    let search_range: Value<u16> = Rc::new(RefCell::new(
        (if ((*max_pow2.borrow()) != 0) {
            (1 << (((*max_pow2.borrow()) as i32) + 4))
        } else {
            0
        } as u16),
    ));
    let range_shift: Value<u16> = Rc::new(RefCell::new(
        (({ ((font.with(|__s| __s.num_tables) as i32) << 4) } - {
            ((*search_range.borrow()) as i32)
        }) as u16),
    ));
    ({
        Store16_31(
            ((*search_range.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*max_pow2.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*range_shift.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    'loop_: for i in RefcountMapIter::begin(field_ptr!(font, tables)) {
        if !({
            let _offset: Ptr<usize> = (*offset.borrow()).clone();
            let _dst_size: usize = (*dst_size.borrow());
            WriteTable_43(
                i.second().as_pointer(),
                _offset,
                (*dst.borrow()).clone(),
                _dst_size,
            )
        }) {
            return false;
        }
    }
    return true;
}
pub fn WriteFontCollection_44(
    font_collection: Ptr<woff2_FontCollection>,
    dst: Ptr<u8>,
    dst_size: usize,
) -> bool {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ({ font_collection.with(|__s| __s.flavor) } != { kTtcFontFlavor_22.with(|rc| *rc.borrow()) })
    {
        return ({
            WriteFont_41(
                (font_collection.with(|__s| __s.fonts.as_pointer()) as Ptr<woff2_Font>)
                    .offset(0_usize),
                (offset.as_pointer()),
                (*dst.borrow()).clone(),
                (*dst_size.borrow()),
            )
        });
    }
    ({
        StoreU32_30(
            kTtcFontFlavor_22.with(|rc| *rc.borrow()),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            font_collection.with(|__s| __s.header_version),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            ((*font_collection.with(|__s| __s.fonts.clone()).borrow()).len() as u32),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    let offset_table: Value<usize> = Rc::new(RefCell::new((*offset.borrow())));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*font_collection.with(|__s| __s.fonts.clone()).borrow()).len()
    }) {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        (*i.borrow_mut()).postfix_inc();
    }
    if (font_collection.with(|__s| __s.header_version) == 131072_u32) {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ({ (*i.borrow()) } < {
        (*font_collection.with(|__s| __s.fonts.clone()).borrow()).len()
    }) {
        let font: Ptr<woff2_Font> = (font_collection.with(|__s| __s.fonts.as_pointer())
            as Ptr<woff2_Font>)
            .offset((*i.borrow()));
        ({
            StoreU32_30(
                ((*offset.borrow()) as u32),
                (offset_table.as_pointer()),
                (*dst.borrow()).clone(),
            )
        });
        if !({
            let _font: Ptr<woff2_Font> = (font).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            let _dst_size: usize = (*dst_size.borrow());
            WriteFont_41(_font, _offset, _dst, _dst_size)
        }) {
            return false;
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return true;
}
pub fn NumGlyphs_45(font: Ptr<woff2_Font>) -> i32 {
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kLocaTableTag_2.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    if (((*head_table.borrow()).is_null()) || ((*loca_table.borrow()).is_null()))
        || ((*head_table.borrow()).with(|__s| __s.length) < 52_u32)
    {
        return 0;
    }
    let index_fmt: Value<i32> = Rc::new(RefCell::new(({ IndexFormat_46((font).clone()) })));
    let loca_record_size: Value<i32> = Rc::new(RefCell::new(
        (if ((*index_fmt.borrow()) == 0) { 2 } else { 4 }),
    ));
    if ({ (*loca_table.borrow()).with(|__s| __s.length) } < {
        ((*loca_record_size.borrow()) as u32)
    }) {
        return 0;
    }
    return (((((*loca_table.borrow()).with(|__s| __s.length))
        .wrapping_div(((*loca_record_size.borrow()) as u32)))
    .wrapping_sub(1_u32)) as i32);
}
pub fn IndexFormat_46(font: Ptr<woff2_Font>) -> i32 {
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    if (*head_table.borrow()).is_null() {
        return 0;
    }
    return ((elem!((*head_table.borrow()).with(|__s| __s.data.clone()), 51).read()) as i32);
}
pub fn GetGlyphData_47(
    font: Ptr<woff2_Font>,
    glyph_index: i32,
    glyph_data: Ptr<Ptr<u8>>,
    glyph_size: Ptr<usize>,
) -> bool {
    let glyph_index: Value<i32> = Rc::new(RefCell::new(glyph_index));
    let glyph_data: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(glyph_data));
    let glyph_size: Value<Ptr<usize>> = Rc::new(RefCell::new(glyph_size));
    if ((*glyph_index.borrow()) < 0) {
        return false;
    }
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kLocaTableTag_2.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = kGlyfTableTag_0.with(|rc| *rc.borrow());
            woff2_FontImpl::FindTable_3(&font, _tag)
        }),
    ));
    if ((((*head_table.borrow()).is_null()) || ((*loca_table.borrow()).is_null()))
        || ((*glyf_table.borrow()).is_null()))
        || ((*head_table.borrow()).with(|__s| __s.length) < 52_u32)
    {
        return false;
    }
    let index_fmt: Value<i32> = Rc::new(RefCell::new(({ IndexFormat_46((font).clone()) })));
    let loca_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*loca_table.borrow()).with(|__s| __s.data.clone()) },
        { ((*loca_table.borrow()).with(|__s| __s.length) as usize) },
    )));
    if ((*index_fmt.borrow()) == 0) {
        let offset1: Value<u16> = Rc::new(RefCell::new(0_u16));
        let offset2: Value<u16> = Rc::new(RefCell::new(0_u16));
        if ((((!({
            woff2_BufferImpl::Skip(
                &loca_buf.as_pointer(),
                ((2 * (*glyph_index.borrow())) as usize),
            )
        })) || (!({
            woff2_BufferImpl::ReadU16(&loca_buf.as_pointer(), (offset1.as_pointer()))
        }))) || (!({
            woff2_BufferImpl::ReadU16(&loca_buf.as_pointer(), (offset2.as_pointer()))
        }))) || (((*offset2.borrow()) as i32) < ((*offset1.borrow()) as i32)))
            || ({ ((2 * ((*offset2.borrow()) as i32)) as u32) } > {
                (*glyf_table.borrow()).with(|__s| __s.length)
            })
        {
            return false;
        }
        (*glyph_data.borrow()).write({
            (*glyf_table.borrow())
                .with(|__s| __s.data.clone())
                .offset((2 * ((*offset1.borrow()) as i32)) as isize)
        });
        (*glyph_size.borrow()).write({
            ((2 * (((*offset2.borrow()) as i32) - ((*offset1.borrow()) as i32))) as usize)
        });
    } else {
        let offset1: Value<u32> = Rc::new(RefCell::new(0_u32));
        let offset2: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((((!({
            woff2_BufferImpl::Skip(
                &loca_buf.as_pointer(),
                ((4 * (*glyph_index.borrow())) as usize),
            )
        })) || (!({
            woff2_BufferImpl::ReadU32(&loca_buf.as_pointer(), (offset1.as_pointer()))
        }))) || (!({
            woff2_BufferImpl::ReadU32(&loca_buf.as_pointer(), (offset2.as_pointer()))
        }))) || ((*offset2.borrow()) < (*offset1.borrow())))
            || ({ (*offset2.borrow()) } > { (*glyf_table.borrow()).with(|__s| __s.length) })
        {
            return false;
        }
        (*glyph_data.borrow()).write({
            (*glyf_table.borrow())
                .with(|__s| __s.data.clone())
                .offset((*offset1.borrow()) as isize)
        });
        (*glyph_size.borrow())
            .write({ (((*offset2.borrow()).wrapping_sub((*offset1.borrow()))) as usize) });
    }
    return true;
}
pub fn RemoveDigitalSignature_48(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
        Rc::new(RefCell::new(RefcountMapIter::find_key(
            (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &kDsigTableTag_3.with(|rc| *rc.borrow()),
        )));
    if (*it.borrow())
        != RefcountMapIter::end(
            (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
        )
    {
        RefcountMapIter::erase(
            (field_ptr!((*font.borrow()), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &(*it.borrow()).clone(),
        );
        let __rhs = ((*(*font.borrow()).upgrade().deref()).tables.len() as u16);
        field!((*font.borrow()), num_tables).write(__rhs);
    }
    return true;
}
pub fn GetFileContent_49(filename: Vec<i8>) -> Vec<i8> {
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new(filename));
    let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open((filename.as_pointer() as Ptr<i8>).to_string())
            .expect("Failed to open file"),
    ));
    return {
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &(*ifs.borrow()).try_clone().unwrap();
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    };
}
pub fn SetFileContents_50(filename: Vec<i8>, start: Ptr<i8>, end: Ptr<i8>) {
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new(filename));
    let start: Value<Ptr<i8>> = Rc::new(RefCell::new(start));
    let end: Value<Ptr<i8>> = Rc::new(RefCell::new(end));
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create((filename.as_pointer() as Ptr<i8>).to_string())
            .expect("Failed to open file"),
    ));
    {
        (*start.borrow()).clone().with_slice(
            (*end.borrow()).clone().get_offset() - (*start.borrow()).clone().get_offset(),
            |__s| {
                CChar::with_u8_slice(__s, |__b| {
                    (*ofs.borrow_mut()).try_clone().unwrap().write_all(__b)
                })
            },
        );
        (*ofs.borrow_mut())
            .try_clone()
            .unwrap()
            .try_clone()
            .unwrap()
    };
}
pub fn PrintTag_51(tag: i32) -> Vec<i8> {
    let tag: Value<i32> = Rc::new(RefCell::new(tag));
    if ((((*tag.borrow()) as u32) & 2155905152_u32) != 0) {
        return {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"_xfm").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
    }
    let printable: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ((((*tag.borrow()) >> 24) & 255) as i8),
        ((((*tag.borrow()) >> 16) & 255) as i8),
        ((((*tag.borrow()) >> 8) & 255) as i8),
        (((*tag.borrow()) & 255) as i8),
    ])));
    return {
        let mut __v = Vec::with_capacity(4_usize as usize + 1);
        (printable.as_pointer() as Ptr<i8>)
            .with_slice(4_usize as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0);
        __v
    };
}
pub fn main() {
    let argv: Vec<Value<Vec<i8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.bytes().map(|c| c as i8).collect())))
        .collect();
    let mut argv: Value<Vec<Ptr<i8>>> = Rc::new(RefCell::new(
        argv.iter()
            .map(|x| {
                x.borrow_mut().push(0);
                x.as_pointer()
            })
            .collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<i8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<i8>>> = Rc::new(RefCell::new(argv));
    if ((*argc.borrow()) != 2) {
        eprintln!("One argument, the input filename, must be provided.");
        return 1;
    }
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = (elem!((*argv.borrow()), 1).read()).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let outfilename: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __tmp2 = {
            let mut __tmp1 = (*filename.borrow())[(0_usize) as usize
                ..::std::cmp::min(
                    (0_usize
                        + Ptr::<i8>::from_string_literal(b".").with_c_str(|__lookup| {
                            (*filename.borrow())
                                .iter()
                                .take((*filename.borrow()).len().saturating_sub(1))
                                .rposition(|&x| __lookup.contains(&x))
                                .unwrap_or(usize::MAX)
                        })) as usize,
                    (*filename.borrow()).len().saturating_sub(1),
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        };
        __tmp2.pop();
        Ptr::<i8>::from_string_literal(b".woff2").with_c_str(|__s| __tmp2.extend_from_slice(__s));
        __tmp2.push(0);
        __tmp2
    }));
    println!(
        "Processing {} => {}",
        (filename.as_pointer() as Ptr<i8>),
        (outfilename.as_pointer() as Ptr<i8>)
    );
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({ GetFileContent_49((*filename.borrow()).clone()) }),
    ));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (input.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>() },
        { ((*input.borrow()).len() - 1) },
    )));
    println!("WOFF2Header");
    let signature: Value<u32> = Rc::new(RefCell::new(0_u32));
    let flavor: Value<u32> = Rc::new(RefCell::new(0_u32));
    let length: Value<u32> = Rc::new(RefCell::new(0_u32));
    let totalSfntSize: Value<u32> = Rc::new(RefCell::new(0_u32));
    let totalCompressedSize: Value<u32> = Rc::new(RefCell::new(0_u32));
    let metaOffset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let metaLength: Value<u32> = Rc::new(RefCell::new(0_u32));
    let metaOrigLength: Value<u32> = Rc::new(RefCell::new(0_u32));
    let privOffset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let privLength: Value<u32> = Rc::new(RefCell::new(0_u32));
    let num_tables: Value<u16> = Rc::new(RefCell::new(0_u16));
    let reserved: Value<u16> = Rc::new(RefCell::new(0_u16));
    let major: Value<u16> = Rc::new(RefCell::new(0_u16));
    let minor: Value<u16> = Rc::new(RefCell::new(0_u16));
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (signature.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (flavor.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (length.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (num_tables.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (reserved.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (totalSfntSize.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (totalCompressedSize.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (major.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (minor.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (metaOffset.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (metaLength.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (metaOrigLength.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (privOffset.as_pointer())) }) {
        return 1;
    }
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (privLength.as_pointer())) }) {
        return 1;
    }
    if ((*signature.borrow()) != 2001684018_u32) {
        println!("Invalid signature: {:08x}", (*signature.borrow()));
        return 1;
    }
    println!("signature           0x{:08x}", (*signature.borrow()));
    println!("flavor              0x{:08x}", (*flavor.borrow()));
    println!("length              {}", (*length.borrow()));
    println!("numTables           {}", ((*num_tables.borrow()) as i32));
    println!("reserved            {}", ((*reserved.borrow()) as i32));
    println!("totalSfntSize       {}", (*totalSfntSize.borrow()));
    println!("totalCompressedSize {}", (*totalCompressedSize.borrow()));
    println!("majorVersion        {}", ((*major.borrow()) as i32));
    println!("minorVersion        {}", ((*minor.borrow()) as i32));
    println!("metaOffset          {}", (*metaOffset.borrow()));
    println!("metaLength          {}", (*metaLength.borrow()));
    println!("metaOrigLength      {}", (*metaOrigLength.borrow()));
    println!("privOffset          {}", (*privOffset.borrow()));
    println!("privLength          {}", (*privLength.borrow()));
    let table_tags: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    println!(
        "TableDirectory starts at +{}",
        ({ woff2_BufferImpl::offset(&file.as_pointer(),) })
    );
    println!("Entry offset flags tag  origLength txLength");
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < ((*num_tables.borrow()) as i32)) {
        let offset: Value<usize> = Rc::new(RefCell::new(
            ({ woff2_BufferImpl::offset(&file.as_pointer()) }),
        ));
        let flags: Value<u8> = Rc::new(RefCell::new(0_u8));
        let tag: Value<u32> = Rc::new(RefCell::new(0_u32));
        let origLength: Value<u32> = Rc::new(RefCell::new(0_u32));
        let transformLength: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !({ woff2_BufferImpl::ReadU8(&file.as_pointer(), (flags.as_pointer())) }) {
            return 1;
        }
        if ((((*flags.borrow()) as i32) & 63) == 63) {
            if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (tag.as_pointer())) }) {
                return 1;
            }
        } else {
            (*tag.borrow_mut()) = ({
                let __idx = (((*flags.borrow()) as i32) & 63) as usize;
                kKnownTags_8.with(|rc| rc.borrow()[__idx])
            });
        }
        {
            let a0_clone = (*tag.borrow()).clone();
            (*table_tags.borrow_mut()).push(a0_clone)
        };
        if !({ ReadBase128_17((file.as_pointer()), (origLength.as_pointer())) }) {
            return 1;
        }
        print!(
            "{:5} {:6}  0x{:02x} {} {:10}",
            (*i.borrow()),
            (*offset.borrow()),
            ((*flags.borrow()) as i32),
            (Rc::new(RefCell::new(({ PrintTag_51(((*tag.borrow()) as i32),) }))).as_pointer()
                as Ptr<i8>),
            (*origLength.borrow())
        );
        let xform_version: Value<u8> = Rc::new(RefCell::new(
            (((((*flags.borrow()) as i32) >> 6) & 3) as u8),
        ));
        if ((*tag.borrow()) == kGlyfTableTag_0.with(|rc| *rc.borrow()))
            || ((*tag.borrow()) == kLocaTableTag_2.with(|rc| *rc.borrow()))
        {
            if (((*xform_version.borrow()) as i32) == 0) {
                if !({ ReadBase128_17((file.as_pointer()), (transformLength.as_pointer())) }) {
                    return 1;
                }
                print!(" {:8}", (*transformLength.borrow()));
            }
        } else if (((*xform_version.borrow()) as i32) > 0) {
            if !({ ReadBase128_17((file.as_pointer()), (transformLength.as_pointer())) }) {
                return 1;
            }
            print!(" {:8}", (*transformLength.borrow()));
        }
        println!("");
        (*i.borrow_mut()).postfix_inc();
    }
    if ((*flavor.borrow()) == kTtcFontFlavor_22.with(|rc| *rc.borrow())) {
        let version: Value<u32> = Rc::new(RefCell::new(0_u32));
        let numFonts: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (version.as_pointer())) }) {
            return 1;
        }
        if !({ Read255UShort_12((file.as_pointer()), (numFonts.as_pointer())) }) {
            return 1;
        }
        println!(
            "CollectionHeader 0x{:08x} {} fonts",
            (*version.borrow()),
            (*numFonts.borrow())
        );
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (((*i.borrow()) as u32) < (*numFonts.borrow())) {
            let numTables: Value<u32> = Rc::new(RefCell::new(0_u32));
            let flavor: Value<u32> = Rc::new(RefCell::new(0_u32));
            if !({ Read255UShort_12((file.as_pointer()), (numTables.as_pointer())) }) {
                return 1;
            }
            if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (flavor.as_pointer())) }) {
                return 1;
            }
            println!(
                "CollectionFontEntry {} flavor 0x{:08x} {} tables",
                (*i.borrow()),
                (*flavor.borrow()),
                (*numTables.borrow())
            );
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while (((*j.borrow()) as u32) < (*numTables.borrow())) {
                let table_idx: Value<u32> = Rc::new(RefCell::new(0_u32));
                if !({ Read255UShort_12((file.as_pointer()), (table_idx.as_pointer())) }) {
                    return 1;
                }
                if (((*table_idx.borrow()) as usize) >= (*table_tags.borrow()).len()) {
                    return 1;
                }
                println!(
                    "  {} {} (idx {})",
                    (*j.borrow()),
                    (Rc::new(RefCell::new(
                        ({
                            PrintTag_51(
                                ((elem!(
                                    (table_tags.as_pointer() as Ptr<u32>),
                                    ((*table_idx.borrow()) as usize)
                                )
                                .read()) as i32),
                            )
                        })
                    ))
                    .as_pointer() as Ptr<i8>),
                    (*table_idx.borrow())
                );
                (*j.borrow_mut()).postfix_inc();
            }
            (*i.borrow_mut()).postfix_inc();
        }
    }
    println!(
        "TableDirectory ends at +{}",
        ({ woff2_BufferImpl::offset(&file.as_pointer(),) })
    );
    return 0;
}
pub trait woff2_BufferImpl {
    fn Skip(&self, n_bytes: usize) -> bool;
    fn Read(&self, data: Ptr<u8>, n_bytes: usize) -> bool;
    fn ReadU8(&self, value: Ptr<u8>) -> bool;
    fn ReadU16(&self, value: Ptr<u16>) -> bool;
    fn ReadS16(&self, value: Ptr<i16>) -> bool;
    fn ReadU24(&self, value: Ptr<u32>) -> bool;
    fn ReadU32(&self, value: Ptr<u32>) -> bool;
    fn ReadS32(&self, value: Ptr<i32>) -> bool;
    fn ReadTag(&self, value: Ptr<u32>) -> bool;
    fn ReadR64(&self, value: Ptr<u64>) -> bool;
    fn buffer(&self) -> Ptr<u8>;
    fn offset(&self) -> usize;
    fn length(&self) -> usize;
    fn set_offset(&self, newoffset: usize) -> bool;
}
impl woff2_BufferImpl for Ptr<woff2_Buffer> {
    fn Skip(&self, n_bytes: usize) -> bool {
        let n_bytes: Value<usize> = Rc::new(RefCell::new(n_bytes));
        return ({ woff2_BufferImpl::Read(self, Ptr::<u8>::null(), (*n_bytes.borrow())) });
    }
    fn Read(&self, data: Ptr<u8>, n_bytes: usize) -> bool {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let n_bytes: Value<usize> = Rc::new(RefCell::new(n_bytes));
        if ((*n_bytes.borrow()) > (((1024 * 1024) * 1024) as usize)) {
            return false;
        }
        if (((*self).with(|__s| __s.offset_)).wrapping_add((*n_bytes.borrow()))
            > (*self).with(|__s| __s.length_))
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub((*n_bytes.borrow())))
        {
            return false;
        }
        if !(*data.borrow()).is_null() {
            {
                (*data.borrow()).to_any().memcpy(
                    &((*self)
                        .with(|__s| __s.buffer_.clone())
                        .offset(((*self).with(|__s| __s.offset_)) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    (*n_bytes.borrow()) as usize,
                );
                (*data.borrow()).to_any()
            };
        }
        field!((*self), offset_)
            .write({ ((*self).with(|__s| __s.offset_)).wrapping_add((*n_bytes.borrow())) });
        return true;
    }
    fn ReadU8(&self, value: Ptr<u8>) -> bool {
        let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 1_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(1_usize))
        {
            return false;
        }
        (*value.borrow()).write({
            (elem!(
                (*self).with(|__s| __s.buffer_.clone()),
                (*self).with(|__s| __s.offset_)
            )
            .read())
        });
        field!((*self), offset_).with_mut(|__v| __v.prefix_inc());
        return true;
    }
    fn ReadU16(&self, value: Ptr<u16>) -> bool {
        let value: Value<Ptr<u16>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 2_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(2_usize))
        {
            return false;
        }
        {
            (*value.borrow()).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u16>() as usize,
            );
            (*value.borrow()).to_any()
        };
        let __rhs = u16::from_be(((*value.borrow()).read()));
        (*value.borrow()).write(__rhs);
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(2_usize) });
        return true;
    }
    fn ReadS16(&self, value: Ptr<i16>) -> bool {
        let value: Value<Ptr<i16>> = Rc::new(RefCell::new(value));
        return ({ woff2_BufferImpl::ReadU16(self, (*value.borrow()).reinterpret_cast::<u16>()) });
    }
    fn ReadU24(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 3_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(3_usize))
        {
            return false;
        }
        (*value.borrow()).write({
            (((((elem!(
                (*self).with(|__s| __s.buffer_.clone()),
                (*self).with(|__s| __s.offset_)
            )
            .read()) as u32)
                << 16)
                | (((elem!(
                    (*self).with(|__s| __s.buffer_.clone()),
                    ((*self).with(|__s| __s.offset_)).wrapping_add(1_usize)
                )
                .read()) as u32)
                    << 8))
                | ((elem!(
                    (*self).with(|__s| __s.buffer_.clone()),
                    ((*self).with(|__s| __s.offset_)).wrapping_add(2_usize)
                )
                .read()) as u32))
        });
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(3_usize) });
        return true;
    }
    fn ReadU32(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 4_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            (*value.borrow()).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            (*value.borrow()).to_any()
        };
        let __rhs = u32::from_be(((*value.borrow()).read()));
        (*value.borrow()).write(__rhs);
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(4_usize) });
        return true;
    }
    fn ReadS32(&self, value: Ptr<i32>) -> bool {
        let value: Value<Ptr<i32>> = Rc::new(RefCell::new(value));
        return ({ woff2_BufferImpl::ReadU32(self, (*value.borrow()).reinterpret_cast::<u32>()) });
    }
    fn ReadTag(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 4_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            (*value.borrow()).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            (*value.borrow()).to_any()
        };
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(4_usize) });
        return true;
    }
    fn ReadR64(&self, value: Ptr<u64>) -> bool {
        let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
        if ((*self).with(|__s| __s.length_) < 8_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(8_usize))
        {
            return false;
        }
        {
            (*value.borrow()).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u64>() as usize,
            );
            (*value.borrow()).to_any()
        };
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(8_usize) });
        return true;
    }
    fn buffer(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.buffer_.clone());
    }
    fn offset(&self) -> usize {
        return (*self).with(|__s| __s.offset_);
    }
    fn length(&self) -> usize {
        return (*self).with(|__s| __s.length_);
    }
    fn set_offset(&self, newoffset: usize) -> bool {
        let newoffset: Value<usize> = Rc::new(RefCell::new(newoffset));
        if ((*newoffset.borrow()) > (*self).with(|__s| __s.length_)) {
            return false;
        }
        field!((*self), offset_).write((*newoffset.borrow()));
        return true;
    }
}
pub trait woff2_FontImpl {
    fn OutputOrderedTags(&self) -> Vec<u32>;
    fn FindTable_2(&self, tag: u32) -> Ptr<woff2_Font_Table>;
    fn FindTable_3(&self, tag: u32) -> Ptr<woff2_Font_Table>;
}
impl woff2_FontImpl for Ptr<woff2_Font> {
    fn FindTable_2(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
            Rc::new(RefCell::new(RefcountMapIter::find_key(
                (field_ptr!((*self), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
                &(*tag.borrow()),
            )));
        return if (*it.borrow())
            == RefcountMapIter::end(
                (field_ptr!((*self), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            ) {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*it.borrow()).second().as_pointer())
        };
    }
    fn FindTable_3(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
            Rc::new(RefCell::new(RefcountMapIter::find_key(
                (field_ptr!((*self), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
                &(*tag.borrow()),
            )));
        return if (*it.borrow())
            == RefcountMapIter::end(
                (field_ptr!((*self), tables) as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            ) {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*it.borrow()).second().as_pointer())
        };
    }
    fn OutputOrderedTags(&self) -> Vec<u32> {
        let output_order: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        'loop_: for i in RefcountMapIter::begin(field_ptr!((*self), tables)) {
            let table: Ptr<woff2_Font_Table> = i.second().as_pointer();
            if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
                continue 'loop_;
            }
            {
                let a0_clone = table.with(|__s| __s.tag).clone();
                (*output_order.borrow_mut()).push(a0_clone)
            };
        }
        let glyf_loc: Value<Ptr<u32>> = Rc::new(RefCell::new(
            (output_order.as_pointer() as Ptr<u32>).offset(
                (output_order.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0
                            < (output_order.as_pointer() as Ptr<u32>)
                                .to_end()
                                .get_offset() as usize
                            && value_0.read() == kGlyfTableTag_0.with(|rc| *rc.borrow())
                    })
                    .unwrap_or(
                        (output_order.as_pointer() as Ptr<u32>)
                            .to_end()
                            .get_offset() as usize,
                    ) as isize,
            ),
        ));
        let loca_loc: Value<Ptr<u32>> = Rc::new(RefCell::new(
            (output_order.as_pointer() as Ptr<u32>).offset(
                (output_order.as_pointer() as Ptr<u32>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0
                            < (output_order.as_pointer() as Ptr<u32>)
                                .to_end()
                                .get_offset() as usize
                            && value_0.read() == kLocaTableTag_2.with(|rc| *rc.borrow())
                    })
                    .unwrap_or(
                        (output_order.as_pointer() as Ptr<u32>)
                            .to_end()
                            .get_offset() as usize,
                    ) as isize,
            ),
        ));
        if ((*glyf_loc.borrow()) != (output_order.as_pointer() as Ptr<u32>).to_end())
            && ((*loca_loc.borrow()) != (output_order.as_pointer() as Ptr<u32>).to_end())
        {
            {
                let idx = (*loca_loc.borrow()).get_offset();
                (output_order.as_pointer() as Ptr<Vec<u32>>)
                    .with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
                (output_order.as_pointer() as Ptr<Vec<u32>>).decay()
            };
            {
                let __off = (output_order.as_pointer() as Ptr<u32>)
                    .offset(
                        (output_order.as_pointer() as Ptr<u32>)
                            .clone()
                            .into_iter()
                            .enumerate()
                            .position(|(index_0, value_0)| {
                                index_0
                                    < (output_order.as_pointer() as Ptr<u32>)
                                        .to_end()
                                        .get_offset() as usize
                                    && value_0.read() == kGlyfTableTag_0.with(|rc| *rc.borrow())
                            })
                            .unwrap_or(
                                (output_order.as_pointer() as Ptr<u32>)
                                    .to_end()
                                    .get_offset() as usize,
                            ) as isize,
                    )
                    .offset(1_i64 as isize)
                    .get_offset();
                (*output_order.borrow_mut()).insert(__off, kLocaTableTag_2.with(|rc| *rc.borrow()));
                (output_order.as_pointer() as Ptr<u32>)
                    .offset(
                        (output_order.as_pointer() as Ptr<u32>)
                            .clone()
                            .into_iter()
                            .enumerate()
                            .position(|(index_0, value_0)| {
                                index_0
                                    < (output_order.as_pointer() as Ptr<u32>)
                                        .to_end()
                                        .get_offset() as usize
                                    && value_0.read() == kGlyfTableTag_0.with(|rc| *rc.borrow())
                            })
                            .unwrap_or(
                                (output_order.as_pointer() as Ptr<u32>)
                                    .to_end()
                                    .get_offset() as usize,
                            ) as isize,
                    )
                    .offset(1_i64 as isize)
            };
        }
        return std::mem::take(&mut (*output_order.borrow_mut()));
    }
}
pub trait woff2_Font_TableImpl {
    fn IsReused(&self) -> bool;
}
impl woff2_Font_TableImpl for Ptr<woff2_Font_Table> {
    fn IsReused(&self) -> bool {
        return !(((*self).with(|__s| __s.reuse_of.clone())).is_null());
    }
}
pub trait woff2_TableImpl {
    fn operator_lt(&self, other: Ptr<woff2_Table>) -> bool;
}
impl woff2_TableImpl for Ptr<woff2_Table> {
    fn operator_lt(&self, other: Ptr<woff2_Table>) -> bool {
        return ({ (*self).with(|__s| __s.tag) } < { other.with(|__s| __s.tag) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = kGlyfTableTag_0.with(|_| ());
    let _ = kHeadTableTag_1.with(|_| ());
    let _ = kLocaTableTag_2.with(|_| ());
    let _ = kDsigTableTag_3.with(|_| ());
    let _ = kCffTableTag_4.with(|_| ());
    let _ = kHmtxTableTag_5.with(|_| ());
    let _ = kHheaTableTag_6.with(|_| ());
    let _ = kMaxpTableTag_7.with(|_| ());
    let _ = kKnownTags_8.with(|_| ());
    let _ = kWoff2Signature_20.with(|_| ());
    let _ = kWoff2FlagsTransform_21.with(|_| ());
    let _ = kTtcFontFlavor_22.with(|_| ());
    let _ = kSfntHeaderSize_23.with(|_| ());
    let _ = kSfntEntrySize_24.with(|_| ());
}
