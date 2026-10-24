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
    pub fn new(mut data: Ptr<u8>, mut len: usize) -> Self {
        Self {
            buffer_: (data).clone(),
            length_: len,
            offset_: 0_usize,
        }
    }
}
pub fn Size255UShort_9(mut value: u16) -> usize {
    let mut result: usize = 3_usize;
    if ((value as i32) < 253) {
        result = 1_usize;
    } else if ((value as i32) < 762) {
        result = 2_usize;
    } else {
        result = 3_usize;
    }
    return result;
}
pub fn Write255UShort_10(mut out: Ptr<Vec<u8>>, mut value: i32) {
    if (value < 253) {
        {
            let __a1 = (value as u8);
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else if (value < 506) {
        {
            let __a1 = 255_u8;
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = ((value - 253) as u8);
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else if (value < 762) {
        {
            let __a1 = 254_u8;
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = ((value - 506) as u8);
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    } else {
        {
            let __a1 = 253_u8;
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = ((value >> 8) as u8);
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
        {
            let __a1 = ((value & 255) as u8);
            out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
        };
    }
}
pub fn Store255UShort_11(mut val: i32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let packed: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    ({ Write255UShort_10((packed.as_pointer()), val) });
    'loop_: for packed_byte in IterRange::new(packed.decay().begin(), packed.decay().end()) {
        let mut packed_byte: u8 = (packed_byte.iter_deref().read());
        let __rhs = packed_byte;
        elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    }
}
pub fn Read255UShort_12(mut buf: Ptr<woff2_Buffer>, mut value: Ptr<u32>) -> bool {
    let code: Value<u8> = Rc::new(RefCell::new(0_u8));
    if !({ woff2_BufferImpl::ReadU8(&buf, (code.as_pointer())) }) {
        return false;
    }
    if (((*code.borrow()) as i32) == kWordCode_13.with(|rc| *rc.borrow())) {
        let result: Value<u16> = Rc::new(RefCell::new(0_u16));
        if !({ woff2_BufferImpl::ReadU16(&buf, (result.as_pointer())) }) {
            return false;
        }
        value.write({ ((*result.borrow()) as u32) });
        return true;
    } else if (((*code.borrow()) as i32) == kOneMoreByteCode1_15.with(|rc| *rc.borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&buf, (result.as_pointer())) }) {
            return false;
        }
        value.write({
            ((((*result.borrow()) as i32) + kLowestUCode_16.with(|rc| *rc.borrow())) as u32)
        });
        return true;
    } else if (((*code.borrow()) as i32) == kOneMoreByteCode2_14.with(|rc| *rc.borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&buf, (result.as_pointer())) }) {
            return false;
        }
        value.write({
            ((((*result.borrow()) as i32) + (kLowestUCode_16.with(|rc| *rc.borrow()) * 2)) as u32)
        });
        return true;
    } else {
        value.write({ ((*code.borrow()) as u32) });
        return true;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn ReadBase128_17(mut buf: Ptr<woff2_Buffer>, mut value: Ptr<u32>) -> bool {
    let mut result: u32 = 0_u32;
    let mut i: usize = 0_usize;
    'loop_: while (i < 5_usize) {
        let code: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ woff2_BufferImpl::ReadU8(&buf, (code.as_pointer())) }) {
            return false;
        }
        if (i == 0_usize) && (((*code.borrow()) as i32) == 128) {
            return false;
        }
        if ((result & 4261412864_u32) != 0) {
            return false;
        }
        result = { ((result << 7) | ((((*code.borrow()) as i32) & 127) as u32)) };
        if ((((*code.borrow()) as i32) & 128) == 0) {
            value.write({ result });
            return true;
        }
        i.prefix_inc();
    }
    return false;
}
pub fn Base128Size_18(mut n: usize) -> usize {
    let mut size: usize = 1_usize;
    'loop_: while (n >= 128_usize) {
        size.prefix_inc();
        n >>= 7;
    }
    return size;
}
pub fn StoreBase128_19(mut len: usize, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let mut size: usize = ({ Base128Size_18(len) });
    let mut i: usize = 0_usize;
    'loop_: while (i < size) {
        let mut b: i32 = (((len
            >> ((7_usize).wrapping_mul((((size).wrapping_sub(i)).wrapping_sub(1_usize)))))
            & 127_usize) as i32);
        if (i < (size).wrapping_sub(1_usize)) {
            b |= 128;
        }
        let __rhs = (b as u8);
        elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
        i.prefix_inc();
    }
}
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
impl CppLess for woff2_Table {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        woff2_TableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for woff2_Table {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(woff2_TableImpl::operator_lt(__lhs, __rhs.clone()))
            && !(woff2_TableImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
pub fn Log2Floor_25(mut n: u32) -> i32 {
    return if (n == 0_u32) {
        -1_i32
    } else {
        (31 ^ n.leading_zeros() as i32)
    };
}
pub fn ComputeULongSum_26(mut buf: Ptr<u8>, mut size: usize) -> u32 {
    let mut checksum: u32 = 0_u32;
    let mut aligned_size: usize = (size & (!3 as usize));
    let mut i: usize = 0_usize;
    'loop_: while (i < aligned_size) {
        checksum = {
            (checksum).wrapping_add(
                (((((((elem!(buf, i).read()) as i32) << 24)
                    | (((elem!(buf, (i).wrapping_add(1_usize)).read()) as i32) << 16))
                    | (((elem!(buf, (i).wrapping_add(2_usize)).read()) as i32) << 8))
                    | ((elem!(buf, (i).wrapping_add(3_usize)).read()) as i32))
                    as u32),
            )
        };
        i = { (i).wrapping_add(4_usize) };
    }
    if (size != aligned_size) {
        let mut v: u32 = 0_u32;
        let mut i: usize = aligned_size;
        'loop_: while (i < size) {
            v |= (({ ((elem!(buf, i).read()) as i32) } << {
                ((24_usize).wrapping_sub((8_usize).wrapping_mul((i & 3_usize))))
            }) as u32);
            i.prefix_inc();
        }
        checksum = { (checksum).wrapping_add(v) };
    }
    return checksum;
}
pub fn CollectionHeaderSize_27(mut header_version: u32, mut num_fonts: u32) -> usize {
    let mut size: usize = 0_usize;
    if (header_version == 131072_u32) {
        size = { (size).wrapping_add(12_usize) };
    }
    if (header_version == 65536_u32) || (header_version == 131072_u32) {
        size = {
            (size).wrapping_add((((12_u32).wrapping_add((4_u32).wrapping_mul(num_fonts))) as usize))
        };
    }
    return size;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    pub buffer: Vec<u8>,
    #[offset(48)]
    #[byte_size(8)]
    pub reuse_of: Ptr<woff2_Font_Table>,
    #[offset(56)]
    pub flag_byte: u8,
}
impl woff2_Font_Table {
    pub fn move_from(_a0: Ptr<woff2_Font_Table>) -> Self {
        Self {
            tag: { (*_a0.upgrade().deref()).tag },
            checksum: { (*_a0.upgrade().deref()).checksum },
            offset: { (*_a0.upgrade().deref()).offset },
            length: { (*_a0.upgrade().deref()).length },
            data: { (*_a0.upgrade().deref()).data.clone() },
            buffer: field!(_a0, buffer).with_mut(std::mem::take),
            reuse_of: { (*_a0.upgrade().deref()).reuse_of.clone() },
            flag_byte: { (*_a0.upgrade().deref()).flag_byte },
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(56)]
pub struct woff2_Font {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub num_tables: u16,
    #[offset(8)]
    #[byte_size(48)]
    pub tables: Map<u32, (Value<u32>, Value<woff2_Font_Table>)>,
}
impl woff2_Font {
    pub fn move_from(_a0: Ptr<woff2_Font>) -> Self {
        Self {
            flavor: { (*_a0.upgrade().deref()).flavor },
            num_tables: { (*_a0.upgrade().deref()).num_tables },
            tables: field_ptr!(_a0, tables).with_mut(
                |__m: &mut Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| std::mem::take(__m),
            ),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(80)]
pub struct woff2_FontCollection {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub header_version: u32,
    #[offset(8)]
    #[byte_size(48)]
    pub tables: Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>,
    #[offset(56)]
    #[byte_size(24)]
    pub fonts: Vec<woff2_Font>,
}
impl woff2_FontCollection {
    pub fn move_from(_a0: Ptr<woff2_FontCollection>) -> Self {
        Self {
            flavor: { (*_a0.upgrade().deref()).flavor },
            header_version: { (*_a0.upgrade().deref()).header_version },
            tables: field_ptr!(_a0, tables).with_mut(
                |__m: &mut Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>| {
                    std::mem::take(__m)
                },
            ),
            fonts: field!(_a0, fonts).with_mut(std::mem::take),
        }
    }
}
pub fn StoreU32_28(mut dst: Ptr<u8>, mut offset: usize, mut x: u32) -> usize {
    elem!(dst, offset).write({ ((x >> 24) as u8) });
    elem!(dst, (offset).wrapping_add(1_usize)).write({ ((x >> 16) as u8) });
    elem!(dst, (offset).wrapping_add(2_usize)).write({ ((x >> 8) as u8) });
    elem!(dst, (offset).wrapping_add(3_usize)).write({ (x as u8) });
    return (offset).wrapping_add(4_usize);
}
pub fn Store16_29(mut dst: Ptr<u8>, mut offset: usize, mut x: i32) -> usize {
    elem!(dst, offset).write({ ((x >> 8) as u8) });
    elem!(dst, (offset).wrapping_add(1_usize)).write({ (x as u8) });
    return (offset).wrapping_add(2_usize);
}
pub fn StoreU32_30(mut val: u32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let __rhs = ((val >> 24) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = ((val >> 16) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = ((val >> 8) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = (val as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
}
pub fn Store16_31(mut val: i32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let __rhs = ((val >> 8) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = (val as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
}
pub fn StoreBytes_32(mut data: Ptr<u8>, mut len: usize, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    {
        ((dst.offset((offset.read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(&(data).to_any(), len as usize);
        ((dst.offset((offset.read()) as isize)) as Ptr<u8>).to_any()
    };
    offset.write({ (offset.read()).wrapping_add(len) });
}
pub fn ReadTrueTypeFont_33(
    mut file: Ptr<woff2_Buffer>,
    mut data: Ptr<u8>,
    mut len: usize,
    mut font: Ptr<woff2_Font>,
) -> bool {
    if (!({ woff2_BufferImpl::ReadU16(&file, (field_ptr!(font, num_tables))) }))
        || (!({ woff2_BufferImpl::Skip(&file, 6_usize) }))
    {
        return false;
    }
    let intervals: Value<Map<u32, (Value<u32>, Value<u32>)>> = Rc::new(RefCell::new(Map::new()));
    let mut i: u16 = 0_u16;
    'loop_: while ({ (i as i32) } < { (font.with(|__s| __s.num_tables) as i32) }) {
        let table: Value<woff2_Font_Table> = Rc::new(RefCell::new(<woff2_Font_Table>::default()));
        (*table.borrow_mut()).flag_byte = 0_u8;
        (*table.borrow_mut()).reuse_of = Ptr::<woff2_Font_Table>::null();
        if (((!({ woff2_BufferImpl::ReadU32(&file, (field_ptr!(table.as_pointer(), tag))) }))
            || (!({
                woff2_BufferImpl::ReadU32(&file, (field_ptr!(table.as_pointer(), checksum)))
            })))
            || (!({ woff2_BufferImpl::ReadU32(&file, (field_ptr!(table.as_pointer(), offset))) })))
            || (!({ woff2_BufferImpl::ReadU32(&file, (field_ptr!(table.as_pointer(), length))) }))
        {
            return false;
        }
        if ((({ (*table.borrow()).offset } & 3_u32) != 0_u32)
            || (({ (*table.borrow()).length } as usize) > len))
            || ((len).wrapping_sub(({ (*table.borrow()).length } as usize))
                < ({ (*table.borrow()).offset } as usize))
        {
            return false;
        }
        let __rhs = { (*table.borrow()).length };
        Map::subscript(
            intervals.as_pointer(),
            field_ptr!(table.as_pointer(), offset),
        )
        .write(__rhs);
        (*table.borrow_mut()).data = { data.offset(({ (*table.borrow()).offset }) as isize) };
        if field_ptr!(font, tables).with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| {
            __m.find(&field_ptr!(table.as_pointer(), tag))
        }) != field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end())
        {
            return false;
        }
        let __rhs = (*table.borrow()).clone();
        Map::subscript(
            field_ptr!(font, tables),
            field_ptr!(table.as_pointer(), tag),
        )
        .write(__rhs);
        i.prefix_inc();
    }
    let mut last_offset: u32 = (((12_u64 as u64)
        .wrapping_add((16_u64 as u64).wrapping_mul((font.with(|__s| __s.num_tables) as u64))))
        as u32);
    'loop_: for i in IterRange::new(
        intervals
            .as_pointer()
            .with(|__m: &Map<u32, (Value<u32>, Value<u32>)>| __m.begin()),
        intervals
            .as_pointer()
            .with(|__m: &Map<u32, (Value<u32>, Value<u32>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<u32>)> = i.iter_deref();
        if ({ (*(*i.upgrade().deref()).0.borrow()) } < { last_offset })
            || ({
                (*(*i.upgrade().deref()).0.borrow())
                    .wrapping_add((*(*i.upgrade().deref()).1.borrow()))
            } < { (*(*i.upgrade().deref()).0.borrow()) })
        {
            return false;
        }
        last_offset =
            (*(*i.upgrade().deref()).0.borrow()).wrapping_add((*(*i.upgrade().deref()).1.borrow()));
    }
    let mut head_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHeadTableTag_1.with(|rc| *rc.borrow())) });
    if (!((head_table).is_null())) && (head_table.with(|__s| __s.length) < 52_u32) {
        return false;
    }
    return true;
}
pub fn ReadCollectionFont_34(
    mut file: Ptr<woff2_Buffer>,
    mut data: Ptr<u8>,
    mut len: usize,
    mut font: Ptr<woff2_Font>,
    mut all_tables: Ptr<Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>>,
) -> bool {
    if !({ woff2_BufferImpl::ReadU32(&file, (field_ptr!(font, flavor))) }) {
        return false;
    }
    if !({ ReadTrueTypeFont_33((file).clone(), (data).clone(), len, (font).clone()) }) {
        return false;
    };
    'loop_: for entry in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let entry: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = entry.iter_deref();
        let table: Ptr<woff2_Font_Table> = (*entry.upgrade().deref()).1.as_pointer();
        if all_tables.with(
            |__m: &Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>| {
                __m.find(&field_ptr!(table, offset))
            },
        ) == all_tables
            .with(|__m: &Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>| __m.end())
        {
            let __rhs = ({ woff2_FontImpl::FindTable_2(&font, table.with(|__s| __s.tag)) });
            Map::subscript((all_tables).clone(), field_ptr!(table, offset)).write(__rhs);
        } else {
            let __rhs = (Map::subscript((all_tables).clone(), field_ptr!(table, offset)).read());
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
    mut file: Ptr<woff2_Buffer>,
    mut data: Ptr<u8>,
    mut len: usize,
    mut font_collection: Ptr<woff2_FontCollection>,
) -> bool {
    let num_fonts: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (!({ woff2_BufferImpl::ReadU32(&file, (field_ptr!(font_collection, header_version))) }))
        || (!({ woff2_BufferImpl::ReadU32(&file, (num_fonts.as_pointer())) }))
    {
        return false;
    }
    let offsets: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*num_fonts.borrow()) as usize)) {
        let offset: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !({ woff2_BufferImpl::ReadU32(&file, (offset.as_pointer())) }) {
            return false;
        }
        {
            let a0_clone = (*offset.borrow()).clone();
            (*offsets.borrow_mut()).push(a0_clone)
        };
        i.postfix_inc();
    }
    {
        let __a0 = (*offsets.borrow()).len() as usize;
        field!(font_collection, fonts)
            .with_mut(|__v: &mut Vec<woff2_Font>| __v.resize_with(__a0, || <woff2_Font>::default()))
    };
    let font_it: Value<RandomAccessIterator<woff2_Font>> = Rc::new(RefCell::new(
        field_ref!(font_collection, fonts).decay().begin(),
    ));
    let all_tables: Value<Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>> =
        Rc::new(RefCell::new(Map::new()));
    'loop_: for offset in IterRange::new(offsets.decay().begin(), offsets.decay().end()) {
        let mut offset: u32 = (offset.iter_deref().read());
        if !({ woff2_BufferImpl::set_offset(&file, (offset as usize)) }) {
            return false;
        }
        let font: Ptr<woff2_Font> = (*font_it.borrow_mut()).iter_post_inc().iter_deref();
        if !({
            ReadCollectionFont_34(
                (file).clone(),
                (data).clone(),
                len,
                (font).clone(),
                (all_tables.as_pointer()),
            )
        }) {
            return false;
        }
    }
    return true;
}
pub fn ReadFont_36(mut data: Ptr<u8>, mut len: usize, mut font: Ptr<woff2_Font>) -> bool {
    let file: Value<woff2_Buffer> =
        Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, { len })));
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(font, flavor))) }) {
        return false;
    }
    if ({ font.with(|__s| __s.flavor) } == { kTtcFontFlavor_22.with(|rc| *rc.borrow()) }) {
        return false;
    }
    return ({ ReadTrueTypeFont_33((file.as_pointer()), data, len, font) });
}
pub fn ReadFontCollection_37(
    mut data: Ptr<u8>,
    mut len: usize,
    mut font_collection: Ptr<woff2_FontCollection>,
) -> bool {
    let file: Value<woff2_Buffer> =
        Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, { len })));
    if !({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(font_collection, flavor))) }) {
        return false;
    }
    if ({ font_collection.with(|__s| __s.flavor) } != { kTtcFontFlavor_22.with(|rc| *rc.borrow()) })
    {
        {
            let __a0 = 1_usize as usize;
            field!(font_collection, fonts).with_mut(|__v: &mut Vec<woff2_Font>| {
                __v.resize_with(__a0, || <woff2_Font>::default())
            })
        };
        let font: Ptr<woff2_Font> = field_ref!(font_collection, fonts).decay().offset(0_usize);
        field!(font, flavor).write(font_collection.with(|__s| __s.flavor));
        return ({ ReadTrueTypeFont_33((file.as_pointer()), data, len, (font).clone()) });
    }
    return ({ ReadTrueTypeCollection_35((file.as_pointer()), data, len, font_collection) });
}
pub fn FontFileSize_38(font: Ptr<woff2_Font>) -> usize {
    let max_offset: Value<usize> = Rc::new(RefCell::new(
        (((12_u64 as u64)
            .wrapping_add((16_u64 as u64).wrapping_mul((font.with(|__s| __s.num_tables) as u64))))
            as usize),
    ));
    'loop_: for i in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
        let table: Ptr<woff2_Font_Table> = (*i.upgrade().deref()).1.as_pointer();
        let mut padding_size: usize =
            ((((4_u32).wrapping_sub((table.with(|__s| __s.length) & 3_u32))) & 3_u32) as usize);
        let end_offset: Value<usize> = Rc::new(RefCell::new(
            ((padding_size).wrapping_add((table.with(|__s| __s.offset) as usize)))
                .wrapping_add((table.with(|__s| __s.length) as usize)),
        ));
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*max_offset.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*end_offset.borrow()) as u64)));
            ({
                let __a = __tmp_0.as_pointer();
                let __b = __tmp_1.as_pointer();
                if CppLess::less(&__a, &__b) { __b } else { __a }
            }
            .read())
        } as usize);
        (*max_offset.borrow_mut()) = __rhs;
    }
    return (*max_offset.borrow());
}
pub fn FontCollectionFileSize_39(font_collection: Ptr<woff2_FontCollection>) -> usize {
    let max_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        let __rhs = ({
            let __tmp_2: Value<u64> = Rc::new(RefCell::new(((*max_offset.borrow()) as u64)));
            let __tmp_3: Value<u64> =
                Rc::new(RefCell::new((({ FontFileSize_38((font).clone()) }) as u64)));
            ({
                let __a = __tmp_2.as_pointer();
                let __b = __tmp_3.as_pointer();
                if CppLess::less(&__a, &__b) { __b } else { __a }
            }
            .read())
        } as usize);
        (*max_offset.borrow_mut()) = __rhs;
    }
    return (*max_offset.borrow());
}
pub fn WriteFont_40(font: Ptr<woff2_Font>, mut dst: Ptr<u8>, mut dst_size: usize) -> bool {
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    return ({
        let _font: Ptr<woff2_Font> = (font).clone();
        let _offset: Ptr<usize> = (offset.as_pointer());
        let _dst: Ptr<u8> = dst;
        let _dst_size: usize = dst_size;
        WriteFont_41(_font, _offset, _dst, _dst_size)
    });
}
pub fn WriteTableRecord_42(
    mut table: Ptr<woff2_Font_Table>,
    mut offset: Ptr<usize>,
    mut dst: Ptr<u8>,
    mut dst_size: usize,
) -> bool {
    if ({ dst_size } < { (offset.read()).wrapping_add(kSfntEntrySize_24.with(|rc| *rc.borrow())) })
    {
        return false;
    }
    if ({ woff2_Font_TableImpl::IsReused(&table) }) {
        table = { table.with(|__s| __s.reuse_of.clone()) };
    }
    ({ StoreU32_30(table.with(|__s| __s.tag), (offset).clone(), (dst).clone()) });
    ({
        StoreU32_30(
            table.with(|__s| __s.checksum),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        StoreU32_30(
            table.with(|__s| __s.offset),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        StoreU32_30(
            table.with(|__s| __s.length),
            (offset).clone(),
            (dst).clone(),
        )
    });
    return true;
}
pub fn WriteTable_43(
    table: Ptr<woff2_Font_Table>,
    mut offset: Ptr<usize>,
    mut dst: Ptr<u8>,
    mut dst_size: usize,
) -> bool {
    if !({
        let _offset: Ptr<usize> = (offset).clone();
        let _dst_size: usize = dst_size;
        WriteTableRecord_42((table).clone(), _offset, (dst).clone(), _dst_size)
    }) {
        return false;
    }
    if !({ woff2_Font_TableImpl::IsReused(&table) }) {
        if ({ (table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length)) } < {
            table.with(|__s| __s.offset)
        }) || ({ dst_size } < {
            (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length))) as usize)
        }) {
            return false;
        }
        {
            (dst.offset((table.with(|__s| __s.offset)) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &(table.with(|__s| __s.data.clone()) as Ptr<u8>).to_any(),
                    (table.with(|__s| __s.length) as usize) as usize,
                );
            (dst.offset((table.with(|__s| __s.offset)) as isize) as Ptr<u8>).to_any()
        };
        let mut padding_size: usize =
            ((((4_u32).wrapping_sub((table.with(|__s| __s.length) & 3_u32))) & 3_u32) as usize);
        if ({
            (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length))) as usize)
                .wrapping_add(padding_size)
        } < { padding_size })
            || ({ dst_size } < {
                (((table.with(|__s| __s.offset)).wrapping_add(table.with(|__s| __s.length)))
                    as usize)
                    .wrapping_add(padding_size)
            })
        {
            return false;
        }
        {
            (dst.offset((table.with(|__s| __s.offset)) as isize)
                .offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>)
                .to_any()
                .memset((0) as u8, padding_size as usize);
            (dst.offset((table.with(|__s| __s.offset)) as isize)
                .offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>)
                .to_any()
        };
    }
    return true;
}
pub fn WriteFont_41(
    font: Ptr<woff2_Font>,
    mut offset: Ptr<usize>,
    mut dst: Ptr<u8>,
    mut dst_size: usize,
) -> bool {
    if ({ (dst_size as u64) } < {
        (12_u64 as u64)
            .wrapping_add((16_u64 as u64).wrapping_mul((font.with(|__s| __s.num_tables) as u64)))
    }) {
        return false;
    }
    ({ StoreU32_30(font.with(|__s| __s.flavor), (offset).clone(), (dst).clone()) });
    ({
        Store16_31(
            (font.with(|__s| __s.num_tables) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
    let mut max_pow2: u16 = (if (font.with(|__s| __s.num_tables) != 0) {
        ({ Log2Floor_25((font.with(|__s| __s.num_tables) as u32)) })
    } else {
        0
    } as u16);
    let mut search_range: u16 = (if (max_pow2 != 0) {
        (1 << ((max_pow2 as i32) + 4))
    } else {
        0
    } as u16);
    let mut range_shift: u16 =
        (({ ((font.with(|__s| __s.num_tables) as i32) << 4) } - { (search_range as i32) }) as u16);
    ({ Store16_31((search_range as i32), (offset).clone(), (dst).clone()) });
    ({ Store16_31((max_pow2 as i32), (offset).clone(), (dst).clone()) });
    ({ Store16_31((range_shift as i32), (offset).clone(), (dst).clone()) });
    'loop_: for i in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
        if !({
            let _offset: Ptr<usize> = (offset).clone();
            let _dst_size: usize = dst_size;
            WriteTable_43(
                (*i.upgrade().deref()).1.as_pointer(),
                _offset,
                (dst).clone(),
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
    mut dst: Ptr<u8>,
    mut dst_size: usize,
) -> bool {
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ({ font_collection.with(|__s| __s.flavor) } != { kTtcFontFlavor_22.with(|rc| *rc.borrow()) })
    {
        return ({
            WriteFont_41(
                field_ref!(font_collection, fonts).decay().offset(0_usize),
                (offset.as_pointer()),
                dst,
                dst_size,
            )
        });
    }
    ({
        StoreU32_30(
            kTtcFontFlavor_22.with(|rc| *rc.borrow()),
            (offset.as_pointer()),
            (dst).clone(),
        )
    });
    ({
        StoreU32_30(
            font_collection.with(|__s| __s.header_version),
            (offset.as_pointer()),
            (dst).clone(),
        )
    });
    ({
        StoreU32_30(
            ((*font_collection.upgrade().deref()).fonts.len() as u32),
            (offset.as_pointer()),
            (dst).clone(),
        )
    });
    let offset_table: Value<usize> = Rc::new(RefCell::new((*offset.borrow())));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*font_collection.upgrade().deref()).fonts.len() }) {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (dst).clone()) });
        i.postfix_inc();
    }
    if (font_collection.with(|__s| __s.header_version) == 131072_u32) {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (dst).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (dst).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (dst).clone()) });
    }
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*font_collection.upgrade().deref()).fonts.len() }) {
        let font: Ptr<woff2_Font> = field_ref!(font_collection, fonts).decay().offset(i);
        ({
            StoreU32_30(
                ((*offset.borrow()) as u32),
                (offset_table.as_pointer()),
                (dst).clone(),
            )
        });
        if !({
            let _font: Ptr<woff2_Font> = (font).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            let _dst_size: usize = dst_size;
            WriteFont_41(_font, _offset, _dst, _dst_size)
        }) {
            return false;
        }
        i.postfix_inc();
    }
    return true;
}
pub fn NumGlyphs_45(font: Ptr<woff2_Font>) -> i32 {
    let mut head_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    let mut loca_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kLocaTableTag_2.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    if (((head_table).is_null()) || ((loca_table).is_null()))
        || (head_table.with(|__s| __s.length) < 52_u32)
    {
        return 0;
    }
    let mut index_fmt: i32 = ({ IndexFormat_46((font).clone()) });
    let mut loca_record_size: i32 = (if (index_fmt == 0) { 2 } else { 4 });
    if ({ loca_table.with(|__s| __s.length) } < { (loca_record_size as u32) }) {
        return 0;
    }
    return ((((loca_table.with(|__s| __s.length)).wrapping_div((loca_record_size as u32)))
        .wrapping_sub(1_u32)) as i32);
}
pub fn IndexFormat_46(font: Ptr<woff2_Font>) -> i32 {
    let mut head_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    if (head_table).is_null() {
        return 0;
    }
    return ((elem!(head_table.with(|__s| __s.data.clone()), 51).read()) as i32);
}
pub fn GetGlyphData_47(
    font: Ptr<woff2_Font>,
    mut glyph_index: i32,
    mut glyph_data: Ptr<Ptr<u8>>,
    mut glyph_size: Ptr<usize>,
) -> bool {
    if (glyph_index < 0) {
        return false;
    }
    let mut head_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kHeadTableTag_1.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    let mut loca_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kLocaTableTag_2.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    let mut glyf_table: Ptr<woff2_Font_Table> = ({
        let _tag: u32 = kGlyfTableTag_0.with(|rc| *rc.borrow());
        woff2_FontImpl::FindTable_3(&font, _tag)
    });
    if ((((head_table).is_null()) || ((loca_table).is_null())) || ((glyf_table).is_null()))
        || (head_table.with(|__s| __s.length) < 52_u32)
    {
        return false;
    }
    let mut index_fmt: i32 = ({ IndexFormat_46((font).clone()) });
    let loca_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { loca_table.with(|__s| __s.data.clone()) },
        { (loca_table.with(|__s| __s.length) as usize) },
    )));
    if (index_fmt == 0) {
        let offset1: Value<u16> = Rc::new(RefCell::new(0_u16));
        let offset2: Value<u16> = Rc::new(RefCell::new(0_u16));
        if ((((!({
            woff2_BufferImpl::Skip(&loca_buf.as_pointer(), ((2 * glyph_index) as usize))
        })) || (!({
            woff2_BufferImpl::ReadU16(&loca_buf.as_pointer(), (offset1.as_pointer()))
        }))) || (!({
            woff2_BufferImpl::ReadU16(&loca_buf.as_pointer(), (offset2.as_pointer()))
        }))) || (((*offset2.borrow()) as i32) < ((*offset1.borrow()) as i32)))
            || ({ ((2 * ((*offset2.borrow()) as i32)) as u32) } > {
                glyf_table.with(|__s| __s.length)
            })
        {
            return false;
        }
        glyph_data.write({
            glyf_table
                .with(|__s| __s.data.clone())
                .offset((2 * ((*offset1.borrow()) as i32)) as isize)
        });
        glyph_size.write({
            ((2 * (((*offset2.borrow()) as i32) - ((*offset1.borrow()) as i32))) as usize)
        });
    } else {
        let offset1: Value<u32> = Rc::new(RefCell::new(0_u32));
        let offset2: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((((!({
            woff2_BufferImpl::Skip(&loca_buf.as_pointer(), ((4 * glyph_index) as usize))
        })) || (!({
            woff2_BufferImpl::ReadU32(&loca_buf.as_pointer(), (offset1.as_pointer()))
        }))) || (!({
            woff2_BufferImpl::ReadU32(&loca_buf.as_pointer(), (offset2.as_pointer()))
        }))) || ((*offset2.borrow()) < (*offset1.borrow())))
            || ({ (*offset2.borrow()) } > { glyf_table.with(|__s| __s.length) })
        {
            return false;
        }
        glyph_data.write({
            glyf_table
                .with(|__s| __s.data.clone())
                .offset((*offset1.borrow()) as isize)
        });
        glyph_size.write({ (((*offset2.borrow()).wrapping_sub((*offset1.borrow()))) as usize) });
    }
    return true;
}
pub fn RemoveDigitalSignature_48(mut font: Ptr<woff2_Font>) -> bool {
    let it: Value<BidirectionalIterator<(Value<u32>, Value<woff2_Font_Table>)>> =
        Rc::new(RefCell::new(field_ptr!(font, tables).with(
            |__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| {
                __m.find(&kDsigTableTag_3.with(|v| v.as_pointer()))
            },
        )));
    if (*it.borrow())
        != field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end())
    {
        {
            let __it = (*it.borrow()).clone();
            Map::erase(field_ptr!(font, tables), &__it)
        };
        let __rhs = ((*font.upgrade().deref()).tables.len() as u16);
        field!(font, num_tables).write(__rhs);
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct woff2_Glyph_Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    pub on_curve: bool,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(72)]
pub struct woff2_Glyph {
    #[offset(0)]
    pub x_min: i16,
    #[offset(2)]
    pub x_max: i16,
    #[offset(4)]
    pub y_min: i16,
    #[offset(6)]
    pub y_max: i16,
    #[offset(8)]
    pub instructions_size: u16,
    #[offset(16)]
    #[byte_size(8)]
    pub instructions_data: Ptr<u8>,
    #[offset(24)]
    pub overlap_simple_flag_set: bool,
    #[offset(32)]
    #[byte_size(24)]
    pub contours: Vec<Vec<woff2_Glyph_Point>>,
    #[offset(56)]
    #[byte_size(8)]
    pub composite_data: Ptr<u8>,
    #[offset(64)]
    pub composite_data_size: u32,
    #[offset(68)]
    pub have_instructions: bool,
}
impl woff2_Glyph {
    pub fn new() -> Self {
        Self {
            x_min: 0_i16,
            x_max: 0_i16,
            y_min: 0_i16,
            y_max: 0_i16,
            instructions_size: 0_u16,
            instructions_data: Ptr::<u8>::null(),
            overlap_simple_flag_set: false,
            contours: Vec::new(),
            composite_data: Ptr::<u8>::null(),
            composite_data_size: 0_u32,
            have_instructions: false,
        }
    }
    pub fn move_from(_a0: Ptr<woff2_Glyph>) -> Self {
        Self {
            x_min: { (*_a0.upgrade().deref()).x_min },
            x_max: { (*_a0.upgrade().deref()).x_max },
            y_min: { (*_a0.upgrade().deref()).y_min },
            y_max: { (*_a0.upgrade().deref()).y_max },
            instructions_size: { (*_a0.upgrade().deref()).instructions_size },
            instructions_data: { (*_a0.upgrade().deref()).instructions_data.clone() },
            overlap_simple_flag_set: { (*_a0.upgrade().deref()).overlap_simple_flag_set },
            contours: field!(_a0, contours).with_mut(std::mem::take),
            composite_data: { (*_a0.upgrade().deref()).composite_data.clone() },
            composite_data_size: { (*_a0.upgrade().deref()).composite_data_size },
            have_instructions: { (*_a0.upgrade().deref()).have_instructions },
        }
    }
}
impl Default for woff2_Glyph {
    fn default() -> Self {
        { woff2_Glyph::new() }
    }
}
thread_local!(
    pub static kFLAG_ONCURVE_49: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static kFLAG_XSHORT_50: Value<i32> = Rc::new(RefCell::new((1 << 1)));
);
thread_local!(
    pub static kFLAG_YSHORT_51: Value<i32> = Rc::new(RefCell::new((1 << 2)));
);
thread_local!(
    pub static kFLAG_REPEAT_52: Value<i32> = Rc::new(RefCell::new((1 << 3)));
);
thread_local!(
    pub static kFLAG_XREPEATSIGN_53: Value<i32> = Rc::new(RefCell::new((1 << 4)));
);
thread_local!(
    pub static kFLAG_YREPEATSIGN_54: Value<i32> = Rc::new(RefCell::new((1 << 5)));
);
thread_local!(
    pub static kFLAG_OVERLAP_SIMPLE_55: Value<i32> = Rc::new(RefCell::new((1 << 6)));
);
thread_local!(
    pub static kFLAG_ARG_1_AND_2_ARE_WORDS_56: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static kFLAG_WE_HAVE_A_SCALE_57: Value<i32> = Rc::new(RefCell::new((1 << 3)));
);
thread_local!(
    pub static kFLAG_MORE_COMPONENTS_58: Value<i32> = Rc::new(RefCell::new((1 << 5)));
);
thread_local!(
    pub static kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59: Value<i32> = Rc::new(RefCell::new((1 << 6)));
);
thread_local!(
    pub static kFLAG_WE_HAVE_A_TWO_BY_TWO_60: Value<i32> = Rc::new(RefCell::new((1 << 7)));
);
thread_local!(
    pub static kFLAG_WE_HAVE_INSTRUCTIONS_61: Value<i32> = Rc::new(RefCell::new((1 << 8)));
);
pub fn ReadCompositeGlyphData_62(
    mut buffer: Ptr<woff2_Buffer>,
    mut glyph: Ptr<woff2_Glyph>,
) -> bool {
    field!(glyph, have_instructions).write(false);
    let __rhs = ({ woff2_BufferImpl::buffer(&buffer) })
        .offset(({ woff2_BufferImpl::offset(&buffer) }) as isize);
    field!(glyph, composite_data).write(__rhs);
    let mut start_offset: usize = ({ woff2_BufferImpl::offset(&buffer) });
    let flags: Value<u16> = Rc::new(RefCell::new(
        (kFLAG_MORE_COMPONENTS_58.with(|rc| *rc.borrow()) as u16),
    ));
    'loop_: while ((((*flags.borrow()) as i32) & kFLAG_MORE_COMPONENTS_58.with(|rc| *rc.borrow()))
        != 0)
    {
        if !({ woff2_BufferImpl::ReadU16(&buffer, (flags.as_pointer())) }) {
            return false;
        }
        field!(glyph, have_instructions).write({
            ((glyph.with(|__s| __s.have_instructions) as i32)
                | (((((*flags.borrow()) as i32)
                    & kFLAG_WE_HAVE_INSTRUCTIONS_61.with(|rc| *rc.borrow()))
                    != 0) as i32))
                != 0
        });
        let mut arg_size: usize = 2_usize;
        if ((((*flags.borrow()) as i32) & kFLAG_ARG_1_AND_2_ARE_WORDS_56.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(4_usize) };
        } else {
            arg_size = { (arg_size).wrapping_add(2_usize) };
        }
        if ((((*flags.borrow()) as i32) & kFLAG_WE_HAVE_A_SCALE_57.with(|rc| *rc.borrow())) != 0) {
            arg_size = { (arg_size).wrapping_add(2_usize) };
        } else if ((((*flags.borrow()) as i32)
            & kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(4_usize) };
        } else if ((((*flags.borrow()) as i32)
            & kFLAG_WE_HAVE_A_TWO_BY_TWO_60.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(8_usize) };
        }
        if !({ woff2_BufferImpl::Skip(&buffer, arg_size) }) {
            return false;
        }
    }
    if ({ ({ woff2_BufferImpl::offset(&buffer) }).wrapping_sub(start_offset) } > {
        (<u32>::MAX as usize)
    }) {
        return false;
    }
    let __rhs = ((({ woff2_BufferImpl::offset(&buffer) }).wrapping_sub(start_offset)) as u32);
    field!(glyph, composite_data_size).write(__rhs);
    return true;
}
pub fn ReadGlyph_63(mut data: Ptr<u8>, mut len: usize, mut glyph: Ptr<woff2_Glyph>) -> bool {
    let buffer: Value<woff2_Buffer> =
        Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, { len })));
    let num_contours: Value<i16> = Rc::new(RefCell::new(0_i16));
    if !({ woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (num_contours.as_pointer())) }) {
        return false;
    }
    if (((!({ woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (field_ptr!(glyph, x_min))) }))
        || (!({ woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (field_ptr!(glyph, y_min))) })))
        || (!({ woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (field_ptr!(glyph, x_max))) })))
        || (!({ woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (field_ptr!(glyph, y_max))) }))
    {
        return false;
    }
    if (((*num_contours.borrow()) as i32) == 0) {
        return true;
    }
    if (((*num_contours.borrow()) as i32) > 0) {
        {
            let __a0 = ((*num_contours.borrow()) as usize) as usize;
            field!(glyph, contours).with_mut(|__v: &mut Vec<Vec<woff2_Glyph_Point>>| {
                __v.resize_with(__a0, || <Vec<woff2_Glyph_Point>>::default())
            })
        };
        let mut last_point_index: u16 = 0_u16;
        let mut i: i32 = 0;
        'loop_: while (i < ((*num_contours.borrow()) as i32)) {
            let point_index: Value<u16> = Rc::new(RefCell::new(0_u16));
            if !({ woff2_BufferImpl::ReadU16(&buffer.as_pointer(), (point_index.as_pointer())) }) {
                return false;
            }
            let mut num_points: u16 = (((((*point_index.borrow()) as i32)
                - (last_point_index as i32))
                + (if (i == 0) { 1 } else { 0 })) as u16);
            {
                let __a0 = (num_points as usize) as usize;
                vec_elem!(glyph, contours, (i as usize)).with_mut(
                    |__v: &mut Vec<woff2_Glyph_Point>| {
                        __v.resize_with(__a0, || <woff2_Glyph_Point>::default())
                    },
                )
            };
            last_point_index = (*point_index.borrow());
            i.prefix_inc();
        }
        if !({
            woff2_BufferImpl::ReadU16(&buffer.as_pointer(), (field_ptr!(glyph, instructions_size)))
        }) {
            return false;
        }
        let __rhs = data.offset(({ woff2_BufferImpl::offset(&buffer.as_pointer()) }) as isize);
        field!(glyph, instructions_data).write(__rhs);
        if !({
            woff2_BufferImpl::Skip(
                &buffer.as_pointer(),
                (glyph.with(|__s| __s.instructions_size) as usize),
            )
        }) {
            return false;
        }
        let flags: Value<Vec<Vec<u8>>> = Rc::new(RefCell::new(
            (0..((*num_contours.borrow()) as usize) as usize)
                .map(|_| <Vec<u8>>::default())
                .collect::<Vec<_>>(),
        ));
        {
            let flag: Value<u8> = Rc::new(RefCell::new(0_u8));
            let flag_repeat: Value<u8> = Rc::new(RefCell::new(0_u8));
            let mut i: i32 = 0;
            'loop_: while (i < ((*num_contours.borrow()) as i32)) {
                {
                    let __a0 = (*field_ref!(glyph, contours)
                        .decay()
                        .offset((i as usize))
                        .upgrade()
                        .deref())
                    .len() as usize;
                    elem!(flags.decay(), (i as usize))
                        .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
                };
                let mut j: usize = 0_usize;
                'loop_: while ({ j } < {
                    (*field_ref!(glyph, contours)
                        .decay()
                        .offset((i as usize))
                        .upgrade()
                        .deref())
                    .len()
                }) {
                    if (((*flag_repeat.borrow()) as i32) == 0) {
                        if !({
                            woff2_BufferImpl::ReadU8(&buffer.as_pointer(), (flag.as_pointer()))
                        }) {
                            return false;
                        }
                        if ((((*flag.borrow()) as i32) & kFLAG_REPEAT_52.with(|rc| *rc.borrow()))
                            != 0)
                        {
                            if !({
                                woff2_BufferImpl::ReadU8(
                                    &buffer.as_pointer(),
                                    (flag_repeat.as_pointer()),
                                )
                            }) {
                                return false;
                            }
                        }
                    } else {
                        (*flag_repeat.borrow_mut()).postfix_dec();
                    }
                    index!(elem!(flags.decay(), (i as usize)), j).write((*flag.borrow()));
                    field!(
                        index!(vec_elem!(glyph, contours, (i as usize)), j),
                        on_curve
                    )
                    .write(
                        ((((*flag.borrow()) as i32) & kFLAG_ONCURVE_49.with(|rc| *rc.borrow()))
                            != 0),
                    );
                    j.prefix_inc();
                }
                i.prefix_inc();
            }
        }
        if (!((*flags.borrow()).is_empty()))
            && (!((*flags.decay().offset(0_usize).upgrade().deref()).is_empty()))
        {
            let __rhs = ((((index!(elem!(flags.decay(), 0_usize), 0_usize).read()) as i32)
                & kFLAG_OVERLAP_SIMPLE_55.with(|rc| *rc.borrow()))
                != 0);
            field!(glyph, overlap_simple_flag_set).write(__rhs);
        }
        let mut prev_x: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while (i < ((*num_contours.borrow()) as i32)) {
            let mut j: usize = 0_usize;
            'loop_: while ({ j } < {
                (*field_ref!(glyph, contours)
                    .decay()
                    .offset((i as usize))
                    .upgrade()
                    .deref())
                .len()
            }) {
                let mut flag: u8 = (index!(elem!(flags.decay(), (i as usize)), j).read());
                if (((flag as i32) & kFLAG_XSHORT_50.with(|rc| *rc.borrow())) != 0) {
                    let x_delta: Value<u8> = Rc::new(RefCell::new(0_u8));
                    if !({ woff2_BufferImpl::ReadU8(&buffer.as_pointer(), (x_delta.as_pointer())) })
                    {
                        return false;
                    }
                    let mut sign: i32 =
                        if (((flag as i32) & kFLAG_XREPEATSIGN_53.with(|rc| *rc.borrow())) != 0) {
                            1
                        } else {
                            -1_i32
                        };
                    field!(index!(vec_elem!(glyph, contours, (i as usize)), j), x)
                        .write((prev_x + (sign * ((*x_delta.borrow()) as i32))));
                } else {
                    let x_delta: Value<i16> = Rc::new(RefCell::new(0_i16));
                    if !(((flag as i32) & kFLAG_XREPEATSIGN_53.with(|rc| *rc.borrow())) != 0) {
                        if !({
                            woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (x_delta.as_pointer()))
                        }) {
                            return false;
                        }
                    }
                    field!(index!(vec_elem!(glyph, contours, (i as usize)), j), x)
                        .write((prev_x + ((*x_delta.borrow()) as i32)));
                }
                prev_x = index!(vec_elem!(glyph, contours, (i as usize)), j).with(|__s| __s.x);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut prev_y: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while (i < ((*num_contours.borrow()) as i32)) {
            let mut j: usize = 0_usize;
            'loop_: while ({ j } < {
                (*field_ref!(glyph, contours)
                    .decay()
                    .offset((i as usize))
                    .upgrade()
                    .deref())
                .len()
            }) {
                let mut flag: u8 = (index!(elem!(flags.decay(), (i as usize)), j).read());
                if (((flag as i32) & kFLAG_YSHORT_51.with(|rc| *rc.borrow())) != 0) {
                    let y_delta: Value<u8> = Rc::new(RefCell::new(0_u8));
                    if !({ woff2_BufferImpl::ReadU8(&buffer.as_pointer(), (y_delta.as_pointer())) })
                    {
                        return false;
                    }
                    let mut sign: i32 =
                        if (((flag as i32) & kFLAG_YREPEATSIGN_54.with(|rc| *rc.borrow())) != 0) {
                            1
                        } else {
                            -1_i32
                        };
                    field!(index!(vec_elem!(glyph, contours, (i as usize)), j), y)
                        .write((prev_y + (sign * ((*y_delta.borrow()) as i32))));
                } else {
                    let y_delta: Value<i16> = Rc::new(RefCell::new(0_i16));
                    if !(((flag as i32) & kFLAG_YREPEATSIGN_54.with(|rc| *rc.borrow())) != 0) {
                        if !({
                            woff2_BufferImpl::ReadS16(&buffer.as_pointer(), (y_delta.as_pointer()))
                        }) {
                            return false;
                        }
                    }
                    field!(index!(vec_elem!(glyph, contours, (i as usize)), j), y)
                        .write((prev_y + ((*y_delta.borrow()) as i32)));
                }
                prev_y = index!(vec_elem!(glyph, contours, (i as usize)), j).with(|__s| __s.y);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    } else if (((*num_contours.borrow()) as i32) == -1_i32) {
        if !({ ReadCompositeGlyphData_62((buffer.as_pointer()), (glyph).clone()) }) {
            return false;
        }
        if glyph.with(|__s| __s.have_instructions) {
            if !({
                woff2_BufferImpl::ReadU16(
                    &buffer.as_pointer(),
                    (field_ptr!(glyph, instructions_size)),
                )
            }) {
                return false;
            }
            let __rhs = data.offset(({ woff2_BufferImpl::offset(&buffer.as_pointer()) }) as isize);
            field!(glyph, instructions_data).write(__rhs);
            if !({
                woff2_BufferImpl::Skip(
                    &buffer.as_pointer(),
                    (glyph.with(|__s| __s.instructions_size) as usize),
                )
            }) {
                return false;
            }
        } else {
            field!(glyph, instructions_size).write(0_u16);
        }
    } else {
        return false;
    }
    return true;
}
pub fn StoreBbox_64(glyph: Ptr<woff2_Glyph>, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    ({
        Store16_31(
            (glyph.with(|__s| __s.x_min) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        Store16_31(
            (glyph.with(|__s| __s.y_min) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        Store16_31(
            (glyph.with(|__s| __s.x_max) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        Store16_31(
            (glyph.with(|__s| __s.y_max) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
}
pub fn StoreInstructions_65(glyph: Ptr<woff2_Glyph>, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    ({
        Store16_31(
            (glyph.with(|__s| __s.instructions_size) as i32),
            (offset).clone(),
            (dst).clone(),
        )
    });
    ({
        let _data: Ptr<u8> = glyph.with(|__s| __s.instructions_data.clone());
        let _len: usize = (glyph.with(|__s| __s.instructions_size) as usize);
        let _offset: Ptr<usize> = (offset).clone();
        StoreBytes_32(_data, _len, _offset, (dst).clone())
    });
}
pub fn StoreEndPtsOfContours_66(
    glyph: Ptr<woff2_Glyph>,
    mut offset: Ptr<usize>,
    mut dst: Ptr<u8>,
) -> bool {
    let mut end_point: i32 = -1_i32;
    'loop_: for contour in IterRange::new(
        field_ref!(glyph, contours).decay().begin(),
        field_ref!(glyph, contours).decay().end(),
    ) {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.iter_deref();
        {
            let rhs_0 =
                ((end_point as usize).wrapping_add((*contour.upgrade().deref()).len())) as i32;
            end_point = rhs_0
        };
        if ({ (*contour.upgrade().deref()).len() } > { (<u16>::MAX as usize) })
            || (end_point > (<u16>::MAX as i32))
        {
            return false;
        }
        ({ Store16_31(end_point, (offset).clone(), (dst).clone()) });
    }
    return true;
}
pub fn StorePoints_67(
    glyph: Ptr<woff2_Glyph>,
    mut offset: Ptr<usize>,
    mut dst: Ptr<u8>,
    mut dst_size: usize,
) -> bool {
    let mut previous_flag: i32 = -1_i32;
    let mut repeat_count: i32 = 0;
    let mut last_x: i32 = 0;
    let mut last_y: i32 = 0;
    let mut x_bytes: usize = 0_usize;
    let mut y_bytes: usize = 0_usize;
    'loop_: for contour in IterRange::new(
        field_ref!(glyph, contours).decay().begin(),
        field_ref!(glyph, contours).decay().end(),
    ) {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.iter_deref();
        'loop_: for point in IterRange::new(contour.decay().begin(), contour.decay().end()) {
            let point: Ptr<woff2_Glyph_Point> = point.iter_deref();
            let mut flag: i32 = if point.with(|__s| __s.on_curve) {
                kFLAG_ONCURVE_49.with(|rc| *rc.borrow())
            } else {
                0
            };
            if (previous_flag == -1_i32) && (glyph.with(|__s| __s.overlap_simple_flag_set)) {
                flag = { (flag | kFLAG_OVERLAP_SIMPLE_55.with(|rc| *rc.borrow())) };
            }
            let mut dx: i32 = ({ point.with(|__s| __s.x) } - { last_x });
            let mut dy: i32 = ({ point.with(|__s| __s.y) } - { last_y });
            if (dx == 0) {
                flag |= kFLAG_XREPEATSIGN_53.with(|rc| *rc.borrow());
            } else if (dx > -256_i32) && (dx < 256) {
                flag |= (kFLAG_XSHORT_50.with(|rc| *rc.borrow())
                    | (if (dx > 0) {
                        kFLAG_XREPEATSIGN_53.with(|rc| *rc.borrow())
                    } else {
                        0
                    }));
                x_bytes = { (x_bytes).wrapping_add(1_usize) };
            } else {
                x_bytes = { (x_bytes).wrapping_add(2_usize) };
            }
            if (dy == 0) {
                flag |= kFLAG_YREPEATSIGN_54.with(|rc| *rc.borrow());
            } else if (dy > -256_i32) && (dy < 256) {
                flag |= (kFLAG_YSHORT_51.with(|rc| *rc.borrow())
                    | (if (dy > 0) {
                        kFLAG_YREPEATSIGN_54.with(|rc| *rc.borrow())
                    } else {
                        0
                    }));
                y_bytes = { (y_bytes).wrapping_add(1_usize) };
            } else {
                y_bytes = { (y_bytes).wrapping_add(2_usize) };
            }
            if (flag == previous_flag) && (repeat_count != 255) {
                elem!(dst, (offset.read()).wrapping_sub(1_usize)).write({
                    (((elem!(dst, (offset.read()).wrapping_sub(1_usize)).read()) as i32)
                        | kFLAG_REPEAT_52.with(|rc| *rc.borrow())) as u8
                });
                repeat_count.postfix_inc();
            } else {
                if (repeat_count != 0) {
                    if ({ (offset.read()) } >= { dst_size }) {
                        return false;
                    }
                    let __rhs = (repeat_count as u8);
                    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
                }
                if ({ (offset.read()) } >= { dst_size }) {
                    return false;
                }
                let __rhs = (flag as u8);
                elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
                repeat_count = 0;
            }
            last_x = point.with(|__s| __s.x);
            last_y = point.with(|__s| __s.y);
            previous_flag = flag;
        }
    }
    if (repeat_count != 0) {
        if ({ (offset.read()) } >= { dst_size }) {
            return false;
        }
        let __rhs = (repeat_count as u8);
        elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    }
    if ({ ((offset.read()).wrapping_add(x_bytes)).wrapping_add(y_bytes) } > { dst_size }) {
        return false;
    }
    let x_offset: Value<usize> = Rc::new(RefCell::new((offset.read())));
    let y_offset: Value<usize> = Rc::new(RefCell::new((offset.read()).wrapping_add(x_bytes)));
    last_x = 0;
    last_y = 0;
    'loop_: for contour in IterRange::new(
        field_ref!(glyph, contours).decay().begin(),
        field_ref!(glyph, contours).decay().end(),
    ) {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.iter_deref();
        'loop_: for point in IterRange::new(contour.decay().begin(), contour.decay().end()) {
            let point: Ptr<woff2_Glyph_Point> = point.iter_deref();
            let mut dx: i32 = ({ point.with(|__s| __s.x) } - { last_x });
            let mut dy: i32 = ({ point.with(|__s| __s.y) } - { last_y });
            if (dx == 0) {
            } else if (dx > -256_i32) && (dx < 256) {
                let __rhs = (dx.abs() as u8);
                elem!(dst, (*x_offset.borrow_mut()).postfix_inc()).write(__rhs);
            } else {
                ({ Store16_31(dx, (x_offset.as_pointer()), (dst).clone()) });
            }
            if (dy == 0) {
            } else if (dy > -256_i32) && (dy < 256) {
                let __rhs = (dy.abs() as u8);
                elem!(dst, (*y_offset.borrow_mut()).postfix_inc()).write(__rhs);
            } else {
                ({ Store16_31(dy, (y_offset.as_pointer()), (dst).clone()) });
            }
            last_x += dx;
            last_y += dy;
        }
    }
    offset.write({ (*y_offset.borrow()) });
    return true;
}
pub fn StoreGlyph_68(glyph: Ptr<woff2_Glyph>, mut dst: Ptr<u8>, mut dst_size: Ptr<usize>) -> bool {
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    if (glyph.with(|__s| __s.composite_data_size) > 0_u32) {
        if ({ ((dst_size.read()) as u64) } < {
            (((10_u64 as u64).wrapping_add((glyph.with(|__s| __s.composite_data_size) as u64)))
                .wrapping_add(
                    ((if glyph.with(|__s| __s.have_instructions) {
                        2_u64
                    } else {
                        0_u64
                    })
                    .wrapping_add((glyph.with(|__s| __s.instructions_size) as u64))),
                ))
        }) {
            return false;
        }
        ({ Store16_31(-1_i32, (offset.as_pointer()), (dst).clone()) });
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            StoreBbox_64(_glyph, _offset, _dst)
        });
        ({
            let _data: Ptr<u8> = glyph.with(|__s| __s.composite_data.clone());
            let _len: usize = (glyph.with(|__s| __s.composite_data_size) as usize);
            StoreBytes_32(_data, _len, (offset.as_pointer()), (dst).clone())
        });
        if glyph.with(|__s| __s.have_instructions) {
            ({
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                let _offset: Ptr<usize> = (offset.as_pointer());
                let _dst: Ptr<u8> = (dst).clone();
                StoreInstructions_65(_glyph, _offset, _dst)
            });
        }
    } else if ((*glyph.upgrade().deref()).contours.len() > 0_usize) {
        if ({ (*glyph.upgrade().deref()).contours.len() } > { (<i16>::MAX as usize) }) {
            return false;
        }
        if ({ ((dst_size.read()) as u64) } < {
            (((12_u64 as u64).wrapping_add(
                (((2_usize).wrapping_mul((*glyph.upgrade().deref()).contours.len())) as u64),
            ))
            .wrapping_add((glyph.with(|__s| __s.instructions_size) as u64)))
        }) {
            return false;
        }
        ({
            Store16_31(
                ((*glyph.upgrade().deref()).contours.len() as i32),
                (offset.as_pointer()),
                (dst).clone(),
            )
        });
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            StoreBbox_64(_glyph, _offset, _dst)
        });
        if !({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            StoreEndPtsOfContours_66(_glyph, _offset, _dst)
        }) {
            return false;
        }
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            StoreInstructions_65(_glyph, _offset, _dst)
        });
        if !({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (dst).clone();
            let _dst_size: usize = (dst_size.read());
            StorePoints_67(_glyph, _offset, _dst, _dst_size)
        }) {
            return false;
        }
    }
    dst_size.write({ (*offset.borrow()) });
    return true;
}
pub fn Round4_69(mut value: i32) -> i32 {
    if ((<i32>::MAX - value) < 3) {
        return value;
    }
    return ((value + 3) & !3);
}
pub fn Round4_70(mut value: u64) -> u64 {
    if ((<u64>::MAX as u64).wrapping_sub(value) < 3_u64) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub fn Round4_71(mut value: u32) -> u32 {
    if ((<u32>::MAX as u32).wrapping_sub(value) < 3_u32) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub fn StoreLoca_72(mut index_fmt: i32, mut value: u32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    if (index_fmt == 0) {
        ({ Store16_31(((value >> 1) as i32), (offset).clone(), (dst).clone()) });
    } else {
        ({ StoreU32_30(value, (offset).clone(), (dst).clone()) });
    }
}
pub fn WriteNormalizedLoca_73(
    mut index_fmt: i32,
    mut num_glyphs: i32,
    mut font: Ptr<woff2_Font>,
) -> bool {
    let mut glyf_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kGlyfTableTag_0.with(|rc| *rc.borrow())) });
    let mut loca_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kLocaTableTag_2.with(|rc| *rc.borrow())) });
    let mut glyph_sz: i32 = if (index_fmt == 0) { 2 } else { 4 };
    {
        let __a0 = ((({ Round4_69((num_glyphs + 1)) }) * glyph_sz) as usize) as usize;
        field!(loca_table, buffer)
            .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    field!(loca_table, length).write((((num_glyphs + 1) * glyph_sz) as u32));
    let mut glyf_dst: Ptr<u8> = if (num_glyphs != 0) {
        (field_ref!(glyf_table, buffer).decay().offset(0_usize))
    } else {
        Ptr::<u8>::null()
    };
    let mut loca_dst: Ptr<u8> = (field_ref!(loca_table, buffer).decay().offset(0_usize));
    let mut glyf_offset: u32 = 0_u32;
    let loca_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut i: i32 = 0;
    'loop_: while (i < num_glyphs) {
        ({
            StoreLoca_72(
                index_fmt,
                glyf_offset,
                (loca_offset.as_pointer()),
                (loca_dst).clone(),
            )
        });
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::new()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (font).clone();
            let _glyph_index: i32 = i;
            let _glyph_data: Ptr<Ptr<u8>> = (glyph_data.as_pointer());
            let _glyph_size: Ptr<usize> = (glyph_size.as_pointer());
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((*glyph_size.borrow()) > 0_usize)
            && (!({
                ReadGlyph_63(
                    (*glyph_data.borrow()).clone(),
                    (*glyph_size.borrow()),
                    (glyph.as_pointer()),
                )
            })))
        {
            return false;
        }
        let glyf_dst_size: Value<usize> = Rc::new(RefCell::new(
            ((*glyf_table.upgrade().deref()).buffer.len()).wrapping_sub((glyf_offset as usize)),
        ));
        if !({
            StoreGlyph_68(
                glyph.as_pointer(),
                glyf_dst.offset((glyf_offset) as isize),
                (glyf_dst_size.as_pointer()),
            )
        }) {
            return false;
        }
        let __rhs = (({ Round4_70(((*glyf_dst_size.borrow()) as u64)) }) as usize);
        (*glyf_dst_size.borrow_mut()) = __rhs;
        if (((*glyf_dst_size.borrow()) > (<u32>::MAX as usize))
            || ((glyf_offset).wrapping_add(((*glyf_dst_size.borrow()) as u32)) < glyf_offset))
            || ((index_fmt == 0)
                && ((glyf_offset as usize).wrapping_add((*glyf_dst_size.borrow()))
                    >= ((1_u64 << 17) as usize)))
        {
            return false;
        }
        glyf_offset = { ((glyf_offset as usize).wrapping_add((*glyf_dst_size.borrow()))) as u32 };
        i.prefix_inc();
    }
    ({
        StoreLoca_72(
            index_fmt,
            glyf_offset,
            (loca_offset.as_pointer()),
            (loca_dst).clone(),
        )
    });
    {
        let __a0 = (glyf_offset as usize) as usize;
        field!(glyf_table, buffer)
            .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    let __rhs = if (glyf_offset != 0) {
        (field_ref!(glyf_table, buffer).decay().offset(0_usize))
    } else {
        Ptr::<u8>::null()
    };
    field!(glyf_table, data).write(__rhs);
    field!(glyf_table, length).write(glyf_offset);
    let __rhs = if ((*loca_offset.borrow()) != 0) {
        (field_ref!(loca_table, buffer).decay().offset(0_usize))
    } else {
        Ptr::<u8>::null()
    };
    field!(loca_table, data).write(__rhs);
    return true;
}
pub fn MakeEditableBuffer_74(mut font: Ptr<woff2_Font>, mut tableTag: i32) -> bool {
    let mut table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, (tableTag as u32)) });
    if (table).is_null() {
        return false;
    }
    if ({ woff2_Font_TableImpl::IsReused(&table) }) {
        return true;
    }
    let mut sz: i32 = (({ Round4_71(table.with(|__s| __s.length)) }) as i32);
    {
        let __a0 = (sz as usize) as usize;
        field!(table, buffer)
            .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    let mut buf: Ptr<u8> = (field_ref!(table, buffer).decay().offset(0_usize));
    {
        (buf).to_any().memcpy(
            &(table.with(|__s| __s.data.clone()) as Ptr<u8>).to_any(),
            (table.with(|__s| __s.length) as usize) as usize,
        );
        (buf).to_any()
    };
    if ((({ (sz as u32) } > { table.with(|__s| __s.length) }) as i64) != 0) {
        {
            (buf.offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>)
                .to_any()
                .memset(
                    (0) as u8,
                    (((sz as u32).wrapping_sub(table.with(|__s| __s.length))) as usize) as usize,
                );
            (buf.offset((table.with(|__s| __s.length)) as isize) as Ptr<u8>).to_any()
        };
    }
    field!(table, data).write((buf).clone());
    return true;
}
pub fn NormalizeGlyphs_75(mut font: Ptr<woff2_Font>) -> bool {
    let mut head_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHeadTableTag_1.with(|rc| *rc.borrow())) });
    let mut glyf_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kGlyfTableTag_0.with(|rc| *rc.borrow())) });
    let mut loca_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kLocaTableTag_2.with(|rc| *rc.borrow())) });
    if (head_table).is_null() {
        return false;
    }
    if ((loca_table).is_null()) && ((glyf_table).is_null()) {
        return true;
    }
    if ({ (((glyf_table).is_null()) as i32) } != { (((loca_table).is_null()) as i32) }) {
        return false;
    }
    if ({ (({ woff2_Font_TableImpl::IsReused(&loca_table) }) as i32) } != {
        (({ woff2_Font_TableImpl::IsReused(&glyf_table) }) as i32)
    }) {
        return false;
    }
    if ({ woff2_Font_TableImpl::IsReused(&loca_table) }) {
        return true;
    }
    let mut index_fmt: i32 = ((elem!(head_table.with(|__s| __s.data.clone()), 51).read()) as i32);
    let mut num_glyphs: i32 = ({ NumGlyphs_45((font).clone()) });
    let mut max_normalized_glyf_size: usize = (({
        (1.1_f64 * (glyf_table.with(|__s| __s.length) as f64))
    } + { ((2 * num_glyphs) as f64) }) as usize);
    {
        let __a0 = max_normalized_glyf_size as usize;
        field!(glyf_table, buffer)
            .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    if !({ WriteNormalizedLoca_73(index_fmt, num_glyphs, (font).clone()) }) {
        if (index_fmt != 0) {
            return false;
        }
        index_fmt = 1;
        if !({ WriteNormalizedLoca_73(index_fmt, num_glyphs, (font).clone()) }) {
            return false;
        }
        vec_elem!(head_table, buffer, 51_usize).write(1_u8);
    }
    return true;
}
pub fn NormalizeOffsets_76(mut font: Ptr<woff2_Font>) -> bool {
    let mut offset: u32 = ((12 + (16 * (font.with(|__s| __s.num_tables) as i32))) as u32);
    let __tmp_0: Value<Vec<u32>> =
        Rc::new(RefCell::new(({ woff2_FontImpl::OutputOrderedTags(&font) })));
    let __range1: Ptr<Vec<u32>> = __tmp_0.as_pointer();
    'loop_: for tag in IterRange::new(__range1.decay().begin(), __range1.decay().end()) {
        let tag: Value<u32> = Rc::new(RefCell::new((tag.iter_deref().read())));
        let table: Ptr<woff2_Font_Table> =
            Map::subscript(field_ptr!(font, tables), tag.as_pointer());
        field!(table, offset).write(offset);
        {
            let rhs_0 = (offset).wrapping_add(({ Round4_71(table.with(|__s| __s.length)) }));
            offset = rhs_0
        };
    }
    return true;
}
pub fn ComputeHeaderChecksum_77(font: Ptr<woff2_Font>) -> u32 {
    let mut checksum: u32 = font.with(|__s| __s.flavor);
    let mut max_pow2: u16 = (if (font.with(|__s| __s.num_tables) != 0) {
        ({ Log2Floor_25((font.with(|__s| __s.num_tables) as u32)) })
    } else {
        0
    } as u16);
    let mut search_range: u16 = (if (max_pow2 != 0) {
        (1 << ((max_pow2 as i32) + 4))
    } else {
        0
    } as u16);
    let mut range_shift: u16 =
        (({ ((font.with(|__s| __s.num_tables) as i32) << 4) } - { (search_range as i32) }) as u16);
    checksum = {
        (checksum).wrapping_add(
            (({ ((font.with(|__s| __s.num_tables) as i32) << 16) } | { (search_range as i32) })
                as u32),
        )
    };
    checksum =
        { (checksum).wrapping_add(((((max_pow2 as i32) << 16) | (range_shift as i32)) as u32)) };
    'loop_: for i in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
        let mut table: Ptr<woff2_Font_Table> = ((*i.upgrade().deref()).1.as_pointer());
        if ({ woff2_Font_TableImpl::IsReused(&table) }) {
            table = { table.with(|__s| __s.reuse_of.clone()) };
        }
        checksum = { (checksum).wrapping_add(table.with(|__s| __s.tag)) };
        checksum = { (checksum).wrapping_add(table.with(|__s| __s.checksum)) };
        checksum = { (checksum).wrapping_add(table.with(|__s| __s.offset)) };
        checksum = { (checksum).wrapping_add(table.with(|__s| __s.length)) };
    }
    return checksum;
}
pub fn FixChecksums_78(mut font: Ptr<woff2_Font>) -> bool {
    let mut head_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHeadTableTag_1.with(|rc| *rc.borrow())) });
    if (head_table).is_null() {
        return false;
    }
    if !((head_table.with(|__s| __s.reuse_of.clone())).is_null()) {
        head_table = { head_table.with(|__s| __s.reuse_of.clone()) };
    }
    if (head_table.with(|__s| __s.length) < 12_u32) {
        return false;
    }
    let mut head_buf: Ptr<u8> = (field_ref!(head_table, buffer).decay().offset(0_usize));
    let offset: Value<usize> = Rc::new(RefCell::new(8_usize));
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (head_buf).clone()) });
    let mut file_checksum: u32 = 0_u32;
    let mut head_checksum: u32 = 0_u32;
    'loop_: for i in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
        let mut table: Ptr<woff2_Font_Table> = ((*i.upgrade().deref()).1.as_pointer());
        if ({ woff2_Font_TableImpl::IsReused(&table) }) {
            table = { table.with(|__s| __s.reuse_of.clone()) };
        }
        let __rhs = ({
            let _buf: Ptr<u8> = table.with(|__s| __s.data.clone());
            let _size: usize = (table.with(|__s| __s.length) as usize);
            ComputeULongSum_26(_buf, _size)
        });
        field!(table, checksum).write(__rhs);
        file_checksum = { (file_checksum).wrapping_add(table.with(|__s| __s.checksum)) };
        if ({ table.with(|__s| __s.tag) } == { kHeadTableTag_1.with(|rc| *rc.borrow()) }) {
            head_checksum = table.with(|__s| __s.checksum);
        }
    }
    {
        let rhs_0 = (file_checksum).wrapping_add(({ ComputeHeaderChecksum_77((font).clone()) }));
        file_checksum = rhs_0
    };
    (*offset.borrow_mut()) = 8_usize;
    ({
        StoreU32_30(
            (2981146554_u32 as u32).wrapping_sub(file_checksum),
            (offset.as_pointer()),
            (head_buf).clone(),
        )
    });
    return true;
}
pub fn MarkTransformed_79(mut font: Ptr<woff2_Font>) -> bool {
    let mut head_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHeadTableTag_1.with(|rc| *rc.borrow())) });
    if (head_table).is_null() {
        return false;
    }
    if !((head_table.with(|__s| __s.reuse_of.clone())).is_null()) {
        head_table = { head_table.with(|__s| __s.reuse_of.clone()) };
    }
    if (head_table.with(|__s| __s.length) < 17_u32) {
        return false;
    }
    let mut head_flags: i32 = ((elem!(head_table.with(|__s| __s.data.clone()), 16).read()) as i32);
    vec_elem!(head_table, buffer, 16_usize).write(((head_flags | 8) as u8));
    return true;
}
pub fn NormalizeWithoutFixingChecksums_80(mut font: Ptr<woff2_Font>) -> bool {
    return ((((({
        MakeEditableBuffer_74(
            (font).clone(),
            (kHeadTableTag_1.with(|rc| *rc.borrow()) as i32),
        )
    }) && ({ RemoveDigitalSignature_48((font).clone()) }))
        && ({ MarkTransformed_79((font).clone()) }))
        && ({ NormalizeGlyphs_75((font).clone()) }))
        && ({ NormalizeOffsets_76((font).clone()) }));
}
pub fn NormalizeFont_81(mut font: Ptr<woff2_Font>) -> bool {
    return (({ NormalizeWithoutFixingChecksums_80((font).clone()) })
        && ({ FixChecksums_78((font).clone()) }));
}
pub fn NormalizeFontCollection_82(mut font_collection: Ptr<woff2_FontCollection>) -> bool {
    if ((*font_collection.upgrade().deref()).fonts.len() == 1_usize) {
        return ({
            NormalizeFont_81((field_ref!(font_collection, fonts).decay().offset(0_usize)))
        });
    }
    let mut offset: u32 = (({
        let _header_version: u32 = font_collection.with(|__s| __s.header_version);
        let _num_fonts: u32 = ((*font_collection.upgrade().deref()).fonts.len() as u32);
        CollectionHeaderSize_27(_header_version, _num_fonts)
    }) as u32);
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        if !({ NormalizeWithoutFixingChecksums_80((font).clone()) }) {
            eprintln!("Font normalization failed.");
            return false;
        }
        offset = {
            ((offset as usize).wrapping_add(
                (kSfntHeaderSize_23.with(|rc| *rc.borrow())).wrapping_add(
                    (kSfntEntrySize_24.with(|rc| *rc.borrow()))
                        .wrapping_mul((font.with(|__s| __s.num_tables) as usize)),
                ),
            )) as u32
        };
    }
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        let __tmp_1: Value<Vec<u32>> =
            Rc::new(RefCell::new(({ woff2_FontImpl::OutputOrderedTags(&font) })));
        let __range2: Ptr<Vec<u32>> = __tmp_1.as_pointer();
        'loop_: for tag in IterRange::new(__range2.decay().begin(), __range2.decay().end()) {
            let tag: Value<u32> = Rc::new(RefCell::new((tag.iter_deref().read())));
            let table: Ptr<woff2_Font_Table> =
                Map::subscript(field_ptr!(font, tables), tag.as_pointer());
            if ({ woff2_Font_TableImpl::IsReused(&table) }) {
                field!(table, offset).write({
                    table
                        .with(|__s| __s.reuse_of.clone())
                        .with(|__s| __s.offset)
                });
            } else {
                field!(table, offset).write(offset);
                {
                    let rhs_0 =
                        (offset).wrapping_add(({ Round4_71(table.with(|__s| __s.length)) }));
                    offset = rhs_0
                };
            }
        }
    }
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        if !({ FixChecksums_78((font).clone()) }) {
            eprintln!("Failed to fix checksums");
            return false;
        }
    }
    return true;
}
thread_local!(
    pub static FLAG_ARG_1_AND_2_ARE_WORDS_83: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static FLAG_WE_HAVE_INSTRUCTIONS_84: Value<i32> = Rc::new(RefCell::new((1 << 8)));
);
thread_local!(
    pub static FLAG_OVERLAP_SIMPLE_BITMAP_85: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
pub fn WriteBytes_86(mut out: Ptr<Vec<u8>>, mut data: Ptr<u8>, mut len: usize) {
    if (len == 0_usize) {
        return;
    }
    let mut offset: usize = (*out.upgrade().deref()).len();
    {
        let __a0 = (offset).wrapping_add(len) as usize;
        out.with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    {
        (((out).decay().offset(offset)) as Ptr<u8>)
            .to_any()
            .memcpy(&(data).to_any(), len as usize);
        (((out).decay().offset(offset)) as Ptr<u8>).to_any()
    };
}
pub fn WriteBytes_87(mut out: Ptr<Vec<u8>>, in_: Ptr<Vec<u8>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*in_.upgrade().deref()).len() }) {
        {
            let a0_clone = (index!(in_, i).read()).clone();
            out.with_mut(|__v: &mut Vec<u8>| __v.push(a0_clone))
        };
        i.prefix_inc();
    }
}
pub fn WriteUShort_88(mut out: Ptr<Vec<u8>>, mut value: i32) {
    {
        let __a1 = ((value >> 8) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
    {
        let __a1 = ((value & 255) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
}
pub fn WriteLong_89(mut out: Ptr<Vec<u8>>, mut value: i32) {
    {
        let __a1 = (((value >> 24) & 255) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
    {
        let __a1 = (((value >> 16) & 255) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
    {
        let __a1 = (((value >> 8) & 255) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
    {
        let __a1 = ((value & 255) as u8);
        out.with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(224)]
pub struct woff2_GlyfEncoder {
    #[offset(0)]
    #[byte_size(24)]
    n_contour_stream_: Vec<u8>,
    #[offset(24)]
    #[byte_size(24)]
    n_points_stream_: Vec<u8>,
    #[offset(48)]
    #[byte_size(24)]
    flag_byte_stream_: Vec<u8>,
    #[offset(72)]
    #[byte_size(24)]
    composite_stream_: Vec<u8>,
    #[offset(96)]
    #[byte_size(24)]
    bbox_bitmap_: Vec<u8>,
    #[offset(120)]
    #[byte_size(24)]
    bbox_stream_: Vec<u8>,
    #[offset(144)]
    #[byte_size(24)]
    glyph_stream_: Vec<u8>,
    #[offset(168)]
    #[byte_size(24)]
    instruction_stream_: Vec<u8>,
    #[offset(192)]
    #[byte_size(24)]
    overlap_bitmap_: Vec<u8>,
    #[offset(216)]
    n_glyphs_: i32,
}
impl woff2_GlyfEncoder {
    pub fn new(mut num_glyphs: i32) -> Self {
        let __this: Value<woff2_GlyfEncoder> = Rc::new(RefCell::new(Self {
            n_contour_stream_: Vec::new(),
            n_points_stream_: Vec::new(),
            flag_byte_stream_: Vec::new(),
            composite_stream_: Vec::new(),
            bbox_bitmap_: Vec::new(),
            bbox_stream_: Vec::new(),
            glyph_stream_: Vec::new(),
            instruction_stream_: Vec::new(),
            overlap_bitmap_: Vec::new(),
            n_glyphs_: num_glyphs,
        }));
        let this: Ptr<woff2_GlyfEncoder> = __this.as_pointer();
        {
            let __a0 = ((((num_glyphs + 31) >> 5) << 2) as usize) as usize;
            field!(this, bbox_bitmap_)
                .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<woff2_GlyfEncoder>) -> Self {
        Self {
            n_contour_stream_: field!(_a0, n_contour_stream_).with_mut(std::mem::take),
            n_points_stream_: field!(_a0, n_points_stream_).with_mut(std::mem::take),
            flag_byte_stream_: field!(_a0, flag_byte_stream_).with_mut(std::mem::take),
            composite_stream_: field!(_a0, composite_stream_).with_mut(std::mem::take),
            bbox_bitmap_: field!(_a0, bbox_bitmap_).with_mut(std::mem::take),
            bbox_stream_: field!(_a0, bbox_stream_).with_mut(std::mem::take),
            glyph_stream_: field!(_a0, glyph_stream_).with_mut(std::mem::take),
            instruction_stream_: field!(_a0, instruction_stream_).with_mut(std::mem::take),
            overlap_bitmap_: field!(_a0, overlap_bitmap_).with_mut(std::mem::take),
            n_glyphs_: { (*_a0.upgrade().deref()).n_glyphs_ },
        }
    }
}
pub fn TransformGlyfAndLocaTables_90(mut font: Ptr<woff2_Font>) -> bool {
    let mut glyf_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kGlyfTableTag_0.with(|rc| *rc.borrow())) });
    let mut loca_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kLocaTableTag_2.with(|rc| *rc.borrow())) });
    if ((loca_table).is_null()) && ((glyf_table).is_null()) {
        return true;
    }
    if ({ (((glyf_table).is_null()) as i32) } != { (((loca_table).is_null()) as i32) }) {
        return false;
    }
    if ({ (({ woff2_Font_TableImpl::IsReused(&loca_table) }) as i32) } != {
        (({ woff2_Font_TableImpl::IsReused(&glyf_table) }) as i32)
    }) {
        return false;
    }
    if ({ woff2_Font_TableImpl::IsReused(&loca_table) }) {
        return true;
    }
    let mut transformed_glyf: Ptr<woff2_Font_Table> = (Map::subscript_move(
        field_ptr!(font, tables),
        Rc::new(RefCell::new(
            (kGlyfTableTag_0.with(|rc| *rc.borrow()) ^ 2155905152_u32),
        ))
        .as_pointer(),
        |__k| cpp_move!(__k),
    ));
    let mut transformed_loca: Ptr<woff2_Font_Table> = (Map::subscript_move(
        field_ptr!(font, tables),
        Rc::new(RefCell::new(
            (kLocaTableTag_2.with(|rc| *rc.borrow()) ^ 2155905152_u32),
        ))
        .as_pointer(),
        |__k| cpp_move!(__k),
    ));
    let mut num_glyphs: i32 = ({ NumGlyphs_45((font).clone()) });
    let encoder: Value<woff2_GlyfEncoder> =
        Rc::new(RefCell::new(woff2_GlyfEncoder::new({ num_glyphs })));
    let mut i: i32 = 0;
    'loop_: while (i < num_glyphs) {
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::new()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (font).clone();
            let _glyph_index: i32 = i;
            let _glyph_data: Ptr<Ptr<u8>> = (glyph_data.as_pointer());
            let _glyph_size: Ptr<usize> = (glyph_size.as_pointer());
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((*glyph_size.borrow()) > 0_usize)
            && (!({
                ReadGlyph_63(
                    (*glyph_data.borrow()).clone(),
                    (*glyph_size.borrow()),
                    (glyph.as_pointer()),
                )
            })))
        {
            return false;
        }
        ({ woff2_GlyfEncoderImpl::Encode(&encoder.as_pointer(), i, glyph.as_pointer()) });
        i.prefix_inc();
    }
    ({
        woff2_GlyfEncoderImpl::GetTransformedGlyfBytes(
            &encoder.as_pointer(),
            (field_ref!(transformed_glyf, buffer).as_pointer()),
        )
    });
    let mut head_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHeadTableTag_1.with(|rc| *rc.borrow())) });
    if ((head_table).is_null()) || (head_table.with(|__s| __s.length) < 52_u32) {
        return false;
    }
    let __rhs = (elem!(head_table.with(|__s| __s.data.clone()), 51).read());
    vec_elem!(transformed_glyf, buffer, 7_usize).write(__rhs);
    field!(transformed_glyf, tag).write((kGlyfTableTag_0.with(|rc| *rc.borrow()) ^ 2155905152_u32));
    let __rhs = ((*transformed_glyf.upgrade().deref()).buffer.len() as u32);
    field!(transformed_glyf, length).write(__rhs);
    let __rhs = field_ref!(transformed_glyf, buffer).decay();
    field!(transformed_glyf, data).write(__rhs);
    field!(transformed_loca, tag).write((kLocaTableTag_2.with(|rc| *rc.borrow()) ^ 2155905152_u32));
    field!(transformed_loca, length).write(0_u32);
    field!(transformed_loca, data).write(Ptr::<u8>::null());
    return true;
}
pub fn TransformHmtxTable_91(mut font: Ptr<woff2_Font>) -> bool {
    let mut glyf_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kGlyfTableTag_0.with(|rc| *rc.borrow())) });
    let mut hmtx_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHmtxTableTag_5.with(|rc| *rc.borrow())) });
    let mut hhea_table: Ptr<woff2_Font_Table> =
        ({ woff2_FontImpl::FindTable_2(&font, kHheaTableTag_6.with(|rc| *rc.borrow())) });
    if ((hmtx_table).is_null()) || ((glyf_table).is_null()) {
        return true;
    }
    if (hhea_table).is_null() {
        return false;
    }
    let hhea_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { hhea_table.with(|__s| __s.data.clone()) },
        { (hhea_table.with(|__s| __s.length) as usize) },
    )));
    let num_hmetrics: Value<u16> = Rc::new(RefCell::new(0_u16));
    if (!({ woff2_BufferImpl::Skip(&hhea_buf.as_pointer(), 34_usize) }))
        || (!({ woff2_BufferImpl::ReadU16(&hhea_buf.as_pointer(), (num_hmetrics.as_pointer())) }))
    {
        return false;
    }
    if (((*num_hmetrics.borrow()) as i32) < 1) {
        return false;
    }
    let mut num_glyphs: i32 = ({ NumGlyphs_45((font).clone()) });
    let advance_widths: Value<Vec<u16>> = Rc::new(RefCell::new(Vec::new()));
    let proportional_lsbs: Value<Vec<i16>> = Rc::new(RefCell::new(Vec::new()));
    let monospace_lsbs: Value<Vec<i16>> = Rc::new(RefCell::new(Vec::new()));
    let mut remove_proportional_lsb: bool = true;
    let mut remove_monospace_lsb: bool = ((num_glyphs - ((*num_hmetrics.borrow()) as i32)) > 0);
    let hmtx_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { hmtx_table.with(|__s| __s.data.clone()) },
        { (hmtx_table.with(|__s| __s.length) as usize) },
    )));
    let mut i: i32 = 0;
    'loop_: while (i < num_glyphs) {
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::new()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (font).clone();
            let _glyph_index: i32 = i;
            let _glyph_data: Ptr<Ptr<u8>> = (glyph_data.as_pointer());
            let _glyph_size: Ptr<usize> = (glyph_size.as_pointer());
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((*glyph_size.borrow()) > 0_usize)
            && (!({
                ReadGlyph_63(
                    (*glyph_data.borrow()).clone(),
                    (*glyph_size.borrow()),
                    (glyph.as_pointer()),
                )
            })))
        {
            return false;
        }
        let advance_width: Value<u16> = Rc::new(RefCell::new(0_u16));
        let lsb: Value<i16> = Rc::new(RefCell::new(0_i16));
        if (i < ((*num_hmetrics.borrow()) as i32)) {
            if !({
                woff2_BufferImpl::ReadU16(&hmtx_buf.as_pointer(), (advance_width.as_pointer()))
            }) {
                return false;
            }
            if !({ woff2_BufferImpl::ReadS16(&hmtx_buf.as_pointer(), (lsb.as_pointer())) }) {
                return false;
            }
            if ((*glyph_size.borrow()) > 0_usize)
                && (({ (*glyph.borrow()).x_min } as i32) != ((*lsb.borrow()) as i32))
            {
                remove_proportional_lsb = false;
            }
            {
                let a0_clone = (*advance_width.borrow()).clone();
                (*advance_widths.borrow_mut()).push(a0_clone)
            };
            {
                let a0_clone = (*lsb.borrow()).clone();
                (*proportional_lsbs.borrow_mut()).push(a0_clone)
            };
        } else {
            if !({ woff2_BufferImpl::ReadS16(&hmtx_buf.as_pointer(), (lsb.as_pointer())) }) {
                return false;
            }
            if ((*glyph_size.borrow()) > 0_usize)
                && (({ (*glyph.borrow()).x_min } as i32) != ((*lsb.borrow()) as i32))
            {
                remove_monospace_lsb = false;
            }
            {
                let a0_clone = (*lsb.borrow()).clone();
                (*monospace_lsbs.borrow_mut()).push(a0_clone)
            };
        }
        if (!(remove_proportional_lsb)) && (!(remove_monospace_lsb)) {
            return true;
        }
        i.postfix_inc();
    }
    let mut transformed_hmtx: Ptr<woff2_Font_Table> = (Map::subscript_move(
        field_ptr!(font, tables),
        Rc::new(RefCell::new(
            (kHmtxTableTag_5.with(|rc| *rc.borrow()) ^ 2155905152_u32),
        ))
        .as_pointer(),
        |__k| cpp_move!(__k),
    ));
    let flags: Value<u8> = Rc::new(RefCell::new(0_u8));
    let mut transformed_size: usize =
        (1_usize).wrapping_add((2_usize).wrapping_mul((*advance_widths.borrow()).len()));
    if remove_proportional_lsb {
        (*flags.borrow_mut()) = { (((*flags.borrow()) as i32) | 1) as u8 };
    } else {
        {
            let rhs_0 = ((transformed_size as u64)
                .wrapping_add(((2_usize).wrapping_mul((*proportional_lsbs.borrow()).len()) as u64)))
                as usize;
            transformed_size = rhs_0
        };
    }
    if remove_monospace_lsb {
        (*flags.borrow_mut()) = { (((*flags.borrow()) as i32) | (1 << 1)) as u8 };
    } else {
        {
            let rhs_0 = ((transformed_size as u64)
                .wrapping_add(((2_usize).wrapping_mul((*monospace_lsbs.borrow()).len()) as u64)))
                as usize;
            transformed_size = rhs_0
        };
    }
    if transformed_size as usize > (*transformed_hmtx.upgrade().deref()).buffer.capacity() as usize
    {
        let len_0 = (*transformed_hmtx.upgrade().deref()).buffer.len();
        field!(transformed_hmtx, buffer).with_mut(|__v: &mut Vec<u8>| {
            __v.reserve_exact(transformed_size as usize - len_0 as usize)
        });
    };
    let mut out: Ptr<Vec<u8>> = (field_ref!(transformed_hmtx, buffer).as_pointer());
    ({ WriteBytes_86((out).clone(), (flags.as_pointer()), 1_usize) });
    'loop_: for advance_width in
        IterRange::new(advance_widths.decay().begin(), advance_widths.decay().end())
    {
        let mut advance_width: u16 = (advance_width.iter_deref().read());
        ({ WriteUShort_88((out).clone(), (advance_width as i32)) });
    }
    if !(remove_proportional_lsb) {
        'loop_: for lsb in IterRange::new(
            proportional_lsbs.decay().begin(),
            proportional_lsbs.decay().end(),
        ) {
            let mut lsb: i16 = (lsb.iter_deref().read());
            ({ WriteUShort_88((out).clone(), (lsb as i32)) });
        }
    }
    if !(remove_monospace_lsb) {
        'loop_: for lsb in
            IterRange::new(monospace_lsbs.decay().begin(), monospace_lsbs.decay().end())
        {
            let mut lsb: i16 = (lsb.iter_deref().read());
            ({ WriteUShort_88((out).clone(), (lsb as i32)) });
        }
    }
    field!(transformed_hmtx, tag).write((kHmtxTableTag_5.with(|rc| *rc.borrow()) ^ 2155905152_u32));
    field!(transformed_hmtx, flag_byte).write(((1 << 6) as u8));
    let __rhs = ((*transformed_hmtx.upgrade().deref()).buffer.len() as u32);
    field!(transformed_hmtx, length).write(__rhs);
    let __rhs = field_ref!(transformed_hmtx, buffer).decay();
    field!(transformed_hmtx, data).write(__rhs);
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(40)]
pub struct woff2_WOFF2Params {
    #[offset(0)]
    #[byte_size(32)]
    pub extended_metadata: Vec<i8>,
    #[offset(32)]
    pub brotli_quality: i32,
    #[offset(36)]
    pub allow_transforms: bool,
}
impl woff2_WOFF2Params {
    pub fn new() -> Self {
        Self {
            extended_metadata: {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
                __bytes.push(0);
                __bytes
            },
            brotli_quality: 11,
            allow_transforms: true,
        }
    }
    pub fn move_from(_a0: Ptr<woff2_WOFF2Params>) -> Self {
        Self {
            extended_metadata: field!(_a0, extended_metadata).with_mut(std::mem::take),
            brotli_quality: { (*_a0.upgrade().deref()).brotli_quality },
            allow_transforms: { (*_a0.upgrade().deref()).allow_transforms },
        }
    }
}
impl Default for woff2_WOFF2Params {
    fn default() -> Self {
        { woff2_WOFF2Params::new() }
    }
}
thread_local!(
    pub static kWoff2HeaderSize_92: Value<usize> = Rc::new(RefCell::new(48_usize));
);
thread_local!(
    pub static kWoff2EntrySize_93: Value<usize> = Rc::new(RefCell::new(20_usize));
);
pub fn Compress_94(
    mut data: Ptr<u8>,
    mut len: usize,
    mut result: Ptr<u8>,
    mut result_len: Ptr<u32>,
    mut mode: ::brotli_sys::BrotliEncoderMode,
    mut quality: i32,
) -> bool {
    let compressed_len: Value<usize> = Rc::new(RefCell::new(((result_len.read()) as usize)));
    if ({
        // Compress into a buffer bounded by the input size, as the output buffer
        // may be much larger and is expensive to borrow if reinterpreted.
        let mut __out_len = (compressed_len.as_pointer()).read();
        let __max = unsafe { ::brotli_sys::BrotliEncoderMaxCompressedSize(len) };
        let mut __out = vec![
            0u8;
            if __max == 0 {
                __out_len
            } else {
                __out_len.min(__max)
            }
        ];
        __out_len = __out.len();
        let __ok = data.with_slice(len, |__in| unsafe {
            ::brotli_sys::BrotliEncoderCompress(
                quality,
                22,
                mode,
                len,
                __in.as_ptr(),
                &mut __out_len,
                __out.as_mut_ptr(),
            )
        });
        if __ok != 0 {
            result.with_slice_mut(__out_len, |__s| __s.copy_from_slice(&__out[..__out_len]));
            (compressed_len.as_pointer()).write(__out_len);
        }
        __ok
    } == 0)
    {
        return false;
    }
    result_len.write({ ((*compressed_len.borrow()) as u32) });
    return true;
}
pub fn Woff2Compress_95(
    mut data: Ptr<u8>,
    mut len: usize,
    mut result: Ptr<u8>,
    mut result_len: Ptr<u32>,
    mut quality: i32,
) -> bool {
    return ({
        Compress_94(
            data,
            len,
            result,
            result_len,
            (::brotli_sys::BROTLI_MODE_FONT as ::brotli_sys::BrotliEncoderMode),
            quality,
        )
    });
}
pub fn TextCompress_96(
    mut data: Ptr<u8>,
    mut len: usize,
    mut result: Ptr<u8>,
    mut result_len: Ptr<u32>,
    mut quality: i32,
) -> bool {
    return ({
        Compress_94(
            data,
            len,
            result,
            result_len,
            (::brotli_sys::BROTLI_MODE_TEXT as ::brotli_sys::BrotliEncoderMode),
            quality,
        )
    });
}
pub fn KnownTableIndex_97(mut tag: u32) -> i32 {
    let mut i: i32 = 0;
    'loop_: while (i < 63) {
        if (tag
            == ({
                let __idx0 = (i) as usize;
                kKnownTags_8.with(|rc| rc.borrow()[__idx0])
            }))
        {
            return i;
        }
        i.prefix_inc();
    }
    return 63;
}
pub fn StoreTableEntry_98(table: Ptr<woff2_Table>, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let mut flag_byte: u8 = (({ (table.with(|__s| __s.flags) & 192_u32) } | {
        (({ KnownTableIndex_97(table.with(|__s| __s.tag)) }) as u32)
    }) as u8);
    let __rhs = flag_byte;
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    if (((flag_byte as i32) & 63) == 63) {
        ({ StoreU32_30(table.with(|__s| __s.tag), (offset).clone(), (dst).clone()) });
    }
    ({
        let _len: usize = (table.with(|__s| __s.src_length) as usize);
        let _offset: Ptr<usize> = (offset).clone();
        StoreBase128_19(_len, _offset, (dst).clone())
    });
    if (({ table.with(|__s| __s.flags) } & { kWoff2FlagsTransform_21.with(|rc| *rc.borrow()) })
        != 0_u32)
    {
        ({
            let _len: usize = (table.with(|__s| __s.transform_length) as usize);
            let _offset: Ptr<usize> = (offset).clone();
            StoreBase128_19(_len, _offset, (dst).clone())
        });
    }
}
pub fn TableEntrySize_99(table: Ptr<woff2_Table>) -> usize {
    let mut flag_byte: u8 = (({ KnownTableIndex_97(table.with(|__s| __s.tag)) }) as u8);
    let mut size: usize = (if (((flag_byte as i32) & 63) != 63) {
        1
    } else {
        5
    } as usize);
    {
        let rhs_0 =
            (size).wrapping_add(({ Base128Size_18((table.with(|__s| __s.src_length) as usize)) }));
        size = rhs_0
    };
    if (({ table.with(|__s| __s.flags) } & { kWoff2FlagsTransform_21.with(|rc| *rc.borrow()) })
        != 0_u32)
    {
        {
            let rhs_0 = (size).wrapping_add(
                ({ Base128Size_18((table.with(|__s| __s.transform_length) as usize)) }),
            );
            size = rhs_0
        };
    }
    return size;
}
pub fn ComputeWoff2Length_100(
    font_collection: Ptr<woff2_FontCollection>,
    tables: Ptr<Vec<woff2_Table>>,
    index_by_tag_offset: Map<
        (Value<u32>, Value<u32>),
        (Value<(Value<u32>, Value<u32>)>, Value<u16>),
    >,
    mut compressed_data_length: usize,
    mut extended_metadata_length: usize,
) -> usize {
    let index_by_tag_offset: Value<
        Map<(Value<u32>, Value<u32>), (Value<(Value<u32>, Value<u32>)>, Value<u16>)>,
    > = Rc::new(RefCell::new(index_by_tag_offset));
    let mut size: usize = kWoff2HeaderSize_92.with(|rc| *rc.borrow());
    'loop_: for table in IterRange::new(tables.decay().begin(), tables.decay().end()) {
        let table: Ptr<woff2_Table> = table.iter_deref();
        {
            let rhs_0 = (size).wrapping_add(({ TableEntrySize_99((table).clone()) }));
            size = rhs_0
        };
    }
    if ({ font_collection.with(|__s| __s.flavor) } == { kTtcFontFlavor_22.with(|rc| *rc.borrow()) })
    {
        size = { (size).wrapping_add(4_usize) };
        {
            let rhs_0 = (size).wrapping_add(
                ({ Size255UShort_9(((*font_collection.upgrade().deref()).fonts.len() as u16)) }),
            );
            size = rhs_0
        };
        {
            let rhs_0 = ((size as u64).wrapping_add(
                ((4_usize).wrapping_mul((*font_collection.upgrade().deref()).fonts.len()) as u64),
            )) as usize;
            size = rhs_0
        };
        'loop_: for font in IterRange::new(
            field_ref!(font_collection, fonts).decay().begin(),
            field_ref!(font_collection, fonts).decay().end(),
        ) {
            let font: Ptr<woff2_Font> = font.iter_deref();
            {
                let rhs_0 = (size).wrapping_add(
                    ({ Size255UShort_9(((*font.upgrade().deref()).tables.len() as u16)) }),
                );
                size = rhs_0
            };
            'loop_: for entry in IterRange::new(
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
            ) {
                let entry: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = entry.iter_deref();
                let table: Ptr<woff2_Font_Table> = (*entry.upgrade().deref()).1.as_pointer();
                if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(
                        table
                            .with(|__s| __s.tag)
                            .try_into()
                            .expect("failed conversion"),
                    )),
                    Rc::new(RefCell::new(
                        table
                            .with(|__s| __s.offset)
                            .try_into()
                            .expect("failed conversion"),
                    )),
                )));
                let mut table_index: u16 =
                    (Map::subscript(index_by_tag_offset.as_pointer(), tag_offset.as_pointer())
                        .read());
                {
                    let rhs_0 = (size).wrapping_add(({ Size255UShort_9(table_index) }));
                    size = rhs_0
                };
            }
        }
    }
    size = { (size).wrapping_add(compressed_data_length) };
    let __rhs = (({ Round4_70((size as u64)) }) as usize);
    size = __rhs;
    size = { (size).wrapping_add(extended_metadata_length) };
    return size;
}
pub fn ComputeUncompressedLength_101(font: Ptr<woff2_Font>) -> usize {
    let mut size: usize = ((12 + (16 * (font.with(|__s| __s.num_tables) as i32))) as usize);
    'loop_: for entry in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let entry: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = entry.iter_deref();
        let table: Ptr<woff2_Font_Table> = (*entry.upgrade().deref()).1.as_pointer();
        if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
            continue 'loop_;
        }
        if ({ woff2_Font_TableImpl::IsReused(&table) }) {
            continue 'loop_;
        }
        {
            let rhs_0 =
                (size).wrapping_add((({ Round4_71(table.with(|__s| __s.length)) }) as usize));
            size = rhs_0
        };
    }
    return size;
}
pub fn ComputeUncompressedLength_102(font_collection: Ptr<woff2_FontCollection>) -> usize {
    if ({ font_collection.with(|__s| __s.flavor) } != { kTtcFontFlavor_22.with(|rc| *rc.borrow()) })
    {
        return ({
            ComputeUncompressedLength_101(
                field_ref!(font_collection, fonts).decay().offset(0_usize),
            )
        });
    }
    let mut size: usize = ({
        let _header_version: u32 = font_collection.with(|__s| __s.header_version);
        let _num_fonts: u32 = ((*font_collection.upgrade().deref()).fonts.len() as u32);
        CollectionHeaderSize_27(_header_version, _num_fonts)
    });
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        {
            let rhs_0 = (size).wrapping_add(({ ComputeUncompressedLength_101((font).clone()) }));
            size = rhs_0
        };
    }
    return size;
}
pub fn ComputeTotalTransformLength_103(font: Ptr<woff2_Font>) -> usize {
    let mut total: usize = 0_usize;
    'loop_: for i in IterRange::new(
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
        field_ptr!(font, tables)
            .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
    ) {
        let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
        let table: Ptr<woff2_Font_Table> = (*i.upgrade().deref()).1.as_pointer();
        if ({ woff2_Font_TableImpl::IsReused(&table) }) {
            continue 'loop_;
        }
        if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0)
            || (!(!({
                let _tag: u32 = (table.with(|__s| __s.tag) ^ 2155905152_u32);
                woff2_FontImpl::FindTable_3(&font, _tag)
            })
            .is_null()))
        {
            total = { (total).wrapping_add((table.with(|__s| __s.length) as usize)) };
        }
    }
    return total;
}
pub fn MaxWOFF2CompressedSize_104(mut data: Ptr<u8>, mut length: usize) -> usize {
    return ({
        let _extended_metadata: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
            __bytes.push(0);
            __bytes
        }));
        MaxWOFF2CompressedSize_105(data, length, _extended_metadata.as_pointer())
    });
}
pub fn MaxWOFF2CompressedSize_105(
    mut data: Ptr<u8>,
    mut length: usize,
    extended_metadata: Ptr<Vec<i8>>,
) -> usize {
    return (((length).wrapping_add(1024_usize) as u64)
        .wrapping_add((((*extended_metadata.upgrade().deref()).len() - 1) as u64))
        as usize);
}
pub fn CompressedBufferSize_106(mut original_size: u32) -> u32 {
    return (((1.2_f64 * (original_size as f64)) + 10240_f64) as u32);
}
pub fn TransformFontCollection_107(mut font_collection: Ptr<woff2_FontCollection>) -> bool {
    'loop_: for font in IterRange::new(
        field_ref!(font_collection, fonts).decay().begin(),
        field_ref!(font_collection, fonts).decay().end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        if !({ TransformGlyfAndLocaTables_90((font).clone()) }) {
            eprintln!("glyf/loca transformation failed.");
            return false;
        }
    }
    return true;
}
pub fn ConvertTTFToWOFF2_108(
    mut data: Ptr<u8>,
    mut length: usize,
    mut result: Ptr<u8>,
    mut result_length: Ptr<usize>,
) -> bool {
    let params: Value<woff2_WOFF2Params> = Rc::new(RefCell::new(woff2_WOFF2Params::new()));
    return ({
        let _length: usize = length;
        let _result_length: Ptr<usize> = result_length;
        ConvertTTFToWOFF2_109(data, _length, result, _result_length, params.as_pointer())
    });
}
pub fn ConvertTTFToWOFF2_109(
    mut data: Ptr<u8>,
    mut length: usize,
    mut result: Ptr<u8>,
    mut result_length: Ptr<usize>,
    params: Ptr<woff2_WOFF2Params>,
) -> bool {
    let font_collection: Value<woff2_FontCollection> =
        Rc::new(RefCell::new(<woff2_FontCollection>::default()));
    if !({ ReadFontCollection_37((data).clone(), length, (font_collection.as_pointer())) }) {
        eprintln!("Parsing of the input font failed.");
        return false;
    }
    if !({ NormalizeFontCollection_82((font_collection.as_pointer())) }) {
        return false;
    }
    if (params.with(|__s| __s.allow_transforms))
        && (!({ TransformFontCollection_107((font_collection.as_pointer())) }))
    {
        return false;
    } else {
        'loop_: for font in IterRange::new(
            field_ref!(font_collection.as_pointer(), fonts)
                .decay()
                .begin(),
            field_ref!(font_collection.as_pointer(), fonts)
                .decay()
                .end(),
        ) {
            let font: Ptr<woff2_Font> = font.iter_deref();
            let mut glyf_table: Ptr<woff2_Font_Table> = ({
                let _tag: u32 = kGlyfTableTag_0.with(|rc| *rc.borrow());
                woff2_FontImpl::FindTable_2(&font, _tag)
            });
            let mut loca_table: Ptr<woff2_Font_Table> = ({
                let _tag: u32 = kLocaTableTag_2.with(|rc| *rc.borrow());
                woff2_FontImpl::FindTable_2(&font, _tag)
            });
            if !(glyf_table).is_null() {
                field!(glyf_table, flag_byte)
                    .write({ ((glyf_table.with(|__s| __s.flag_byte) as i32) | 192) as u8 });
            }
            if !(loca_table).is_null() {
                field!(loca_table, flag_byte)
                    .write({ ((loca_table.with(|__s| __s.flag_byte) as i32) | 192) as u8 });
            }
        }
    }
    let mut total_transform_length: usize = 0_usize;
    'loop_: for font in IterRange::new(
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .begin(),
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        {
            let rhs_0 = (total_transform_length)
                .wrapping_add(({ ComputeTotalTransformLength_103((font).clone()) }));
            total_transform_length = rhs_0
        };
    }
    let mut compression_buffer_size: usize =
        (({ CompressedBufferSize_106((total_transform_length as u32)) }) as usize);
    let compression_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(compression_buffer_size) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let total_compressed_length: Value<u32> =
        Rc::new(RefCell::new((compression_buffer_size as u32)));
    let transform_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(total_transform_length) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let transform_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for font in IterRange::new(
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .begin(),
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        let __tmp_0: Value<Vec<u32>> =
            Rc::new(RefCell::new(({ woff2_FontImpl::OutputOrderedTags(&font) })));
        let __range2: Ptr<Vec<u32>> = __tmp_0.as_pointer();
        'loop_: for tag in IterRange::new(__range2.decay().begin(), __range2.decay().end()) {
            let tag: Value<u32> = Rc::new(RefCell::new((tag.iter_deref().read())));
            let original: Ptr<woff2_Font_Table> = (*font.upgrade().deref())
                .tables
                .get(&tag.as_pointer())
                .expect("out of range!")
                .borrow()
                .1
                .as_pointer();
            if ({ woff2_Font_TableImpl::IsReused(&original) }) {
                continue 'loop_;
            }
            if (((*tag.borrow()) & 2155905152_u32) != 0) {
                continue 'loop_;
            }
            let mut table_to_store: Ptr<woff2_Font_Table> = ({
                let _tag: u32 = ((*tag.borrow()) ^ 2155905152_u32);
                woff2_FontImpl::FindTable_3(&font, _tag)
            });
            if (table_to_store).is_null() {
                table_to_store = (original).clone();
            }
            ({
                let _data: Ptr<u8> = table_to_store.with(|__s| __s.data.clone());
                let _len: usize = (table_to_store.with(|__s| __s.length) as usize);
                StoreBytes_32(
                    _data,
                    _len,
                    (transform_offset.as_pointer()),
                    (transform_buf.decay().offset(0_usize)),
                )
            });
        }
    }
    if !({
        Woff2Compress_95(
            transform_buf.decay(),
            total_transform_length,
            (compression_buf.decay().offset(0_usize)),
            (total_compressed_length.as_pointer()),
            params.with(|__s| __s.brotli_quality),
        )
    }) {
        eprintln!("Compression of combined table failed.");
        return false;
    }
    eprintln!(
        "Compressed {} to {}.",
        total_transform_length,
        (*total_compressed_length.borrow())
    );
    let compressed_metadata_buf_length: Value<u32> = Rc::new(RefCell::new(
        ({
            CompressedBufferSize_106(
                (((*params.upgrade().deref()).extended_metadata.len() - 1) as u32),
            )
        }),
    ));
    let compressed_metadata_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((*compressed_metadata_buf_length.borrow()) as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    if (((*params.upgrade().deref()).extended_metadata.len() - 1) > 0_usize) {
        if !({
            let _data: Ptr<u8> = field_ref!(params, extended_metadata)
                .decay()
                .reinterpret_cast::<u8>();
            let _len: usize = ((*params.upgrade().deref()).extended_metadata.len() - 1);
            let _quality: i32 = params.with(|__s| __s.brotli_quality);
            TextCompress_96(
                _data,
                _len,
                compressed_metadata_buf.decay(),
                (compressed_metadata_buf_length.as_pointer()),
                _quality,
            )
        }) {
            eprintln!("Compression of extended metadata failed.");
            return false;
        }
    } else {
        (*compressed_metadata_buf_length.borrow_mut()) = 0_u32;
    }
    let tables: Value<Vec<woff2_Table>> = Rc::new(RefCell::new(Vec::new()));
    let index_by_tag_offset: Value<
        Map<(Value<u32>, Value<u32>), (Value<(Value<u32>, Value<u32>)>, Value<u16>)>,
    > = Rc::new(RefCell::new(Map::new()));
    'loop_: for font in IterRange::new(
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .begin(),
        field_ref!(font_collection.as_pointer(), fonts)
            .decay()
            .end(),
    ) {
        let font: Ptr<woff2_Font> = font.iter_deref();
        let __tmp_1: Value<Vec<u32>> =
            Rc::new(RefCell::new(({ woff2_FontImpl::OutputOrderedTags(&font) })));
        let __range2: Ptr<Vec<u32>> = __tmp_1.as_pointer();
        'loop_: for tag in IterRange::new(__range2.decay().begin(), __range2.decay().end()) {
            let tag: Value<u32> = Rc::new(RefCell::new((tag.iter_deref().read())));
            let src_table: Ptr<woff2_Font_Table> = (*font.upgrade().deref())
                .tables
                .get(&tag.as_pointer())
                .expect("out of range!")
                .borrow()
                .1
                .as_pointer();
            if ({ woff2_Font_TableImpl::IsReused(&src_table) }) {
                continue 'loop_;
            }
            let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new(
                    src_table
                        .with(|__s| __s.tag)
                        .try_into()
                        .expect("failed conversion"),
                )),
                Rc::new(RefCell::new(
                    src_table
                        .with(|__s| __s.offset)
                        .try_into()
                        .expect("failed conversion"),
                )),
            )));
            if index_by_tag_offset.as_pointer().with(
                |__m: &Map<
                    (Value<u32>, Value<u32>),
                    (Value<(Value<u32>, Value<u32>)>, Value<u16>),
                >| __m.find(&tag_offset.as_pointer()),
            ) == index_by_tag_offset.as_pointer().with(
                |__m: &Map<
                    (Value<u32>, Value<u32>),
                    (Value<(Value<u32>, Value<u32>)>, Value<u16>),
                >| __m.end(),
            ) {
                Map::subscript(index_by_tag_offset.as_pointer(), tag_offset.as_pointer())
                    .write(((*tables.borrow()).len() as u16));
            } else {
                return false;
            }
            let mut table: woff2_Table = <woff2_Table>::default();
            table.tag = src_table.with(|__s| __s.tag);
            table.flags = (src_table.with(|__s| __s.flag_byte) as u32);
            table.src_length = src_table.with(|__s| __s.length);
            table.transform_length = src_table.with(|__s| __s.length);
            let mut transformed_data: Ptr<u8> = src_table.with(|__s| __s.data.clone());
            let mut transformed_table: Ptr<woff2_Font_Table> = ({
                let _tag: u32 = (src_table.with(|__s| __s.tag) ^ 2155905152_u32);
                woff2_FontImpl::FindTable_3(&font, _tag)
            });
            if !((transformed_table).is_null()) {
                table.flags = (transformed_table.with(|__s| __s.flag_byte) as u32);
                table.flags |= kWoff2FlagsTransform_21.with(|rc| *rc.borrow());
                table.transform_length = transformed_table.with(|__s| __s.length);
                transformed_data = transformed_table.with(|__s| __s.data.clone());
            }
            {
                let a0_clone = table.clone();
                (*tables.borrow_mut()).push(a0_clone)
            };
        }
    }
    let mut woff2_length: usize = ({
        ComputeWoff2Length_100(
            font_collection.as_pointer(),
            tables.as_pointer(),
            (*index_by_tag_offset.borrow()).clone(),
            ((*total_compressed_length.borrow()) as usize),
            ((*compressed_metadata_buf_length.borrow()) as usize),
        )
    });
    if ({ woff2_length } > { (result_length.read()) }) {
        eprintln!(
            "Result allocation was too small ({} vs {} bytes).",
            (result_length.read()),
            woff2_length
        );
        return false;
    }
    result_length.write({ woff2_length });
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        StoreU32_30(
            kWoff2Signature_20.with(|rc| *rc.borrow()),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    if ({ (*font_collection.borrow()).flavor } != kTtcFontFlavor_22.with(|rc| *rc.borrow())) {
        ({
            StoreU32_30(
                vec_elem!(font_collection.as_pointer(), fonts, 0_usize).with(|__s| __s.flavor),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
    } else {
        ({
            StoreU32_30(
                kTtcFontFlavor_22.with(|rc| *rc.borrow()),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
    }
    ({
        StoreU32_30(
            (woff2_length as u32),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    ({
        Store16_31(
            ((*tables.borrow()).len() as i32),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    ({ Store16_31(0, (offset.as_pointer()), (result).clone()) });
    ({
        StoreU32_30(
            (({ ComputeUncompressedLength_102(font_collection.as_pointer()) }) as u32),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    ({
        StoreU32_30(
            (*total_compressed_length.borrow()),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    ({ Store16_31(1, (offset.as_pointer()), (result).clone()) });
    ({ Store16_31(0, (offset.as_pointer()), (result).clone()) });
    if ((*compressed_metadata_buf_length.borrow()) > 0_u32) {
        ({
            StoreU32_30(
                (((woff2_length)
                    .wrapping_sub(((*compressed_metadata_buf_length.borrow()) as usize)))
                    as u32),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
        ({
            StoreU32_30(
                (*compressed_metadata_buf_length.borrow()),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
        ({
            StoreU32_30(
                (((*params.upgrade().deref()).extended_metadata.len() - 1) as u32),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
    } else {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (result).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (result).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (result).clone()) });
    }
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (result).clone()) });
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (result).clone()) });
    'loop_: for table in IterRange::new(tables.decay().begin(), tables.decay().end()) {
        let table: Ptr<woff2_Table> = table.iter_deref();
        ({
            let _table: Ptr<woff2_Table> = (table).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (result).clone();
            StoreTableEntry_98(_table, _offset, _dst)
        });
    }
    if ({ (*font_collection.borrow()).flavor } == kTtcFontFlavor_22.with(|rc| *rc.borrow())) {
        ({
            StoreU32_30(
                { (*font_collection.borrow()).header_version },
                (offset.as_pointer()),
                (result).clone(),
            )
        });
        ({
            Store255UShort_11(
                ((*font_collection.borrow()).fonts.len() as i32),
                (offset.as_pointer()),
                (result).clone(),
            )
        });
        'loop_: for font in IterRange::new(
            field_ref!(font_collection.as_pointer(), fonts)
                .decay()
                .begin(),
            field_ref!(font_collection.as_pointer(), fonts)
                .decay()
                .end(),
        ) {
            let font: Ptr<woff2_Font> = font.iter_deref();
            let mut num_tables: u16 = 0_u16;
            'loop_: for entry in IterRange::new(
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
            ) {
                let entry: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = entry.iter_deref();
                let table: Ptr<woff2_Font_Table> = (*entry.upgrade().deref()).1.as_pointer();
                if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                num_tables.postfix_inc();
            }
            ({ Store255UShort_11((num_tables as i32), (offset.as_pointer()), (result).clone()) });
            ({
                StoreU32_30(
                    font.with(|__s| __s.flavor),
                    (offset.as_pointer()),
                    (result).clone(),
                )
            });
            'loop_: for entry in IterRange::new(
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
                field_ptr!(font, tables)
                    .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
            ) {
                let entry: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = entry.iter_deref();
                let table: Ptr<woff2_Font_Table> = (*entry.upgrade().deref()).1.as_pointer();
                if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                let mut table_offset: u32 = if ({ woff2_Font_TableImpl::IsReused(&table) }) {
                    table
                        .with(|__s| __s.reuse_of.clone())
                        .with(|__s| __s.offset)
                } else {
                    table.with(|__s| __s.offset)
                };
                let mut table_length: u32 = if ({ woff2_Font_TableImpl::IsReused(&table) }) {
                    table
                        .with(|__s| __s.reuse_of.clone())
                        .with(|__s| __s.length)
                } else {
                    table.with(|__s| __s.length)
                };
                let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(
                        table
                            .with(|__s| __s.tag)
                            .try_into()
                            .expect("failed conversion"),
                    )),
                    Rc::new(RefCell::new(
                        table_offset.try_into().expect("failed conversion"),
                    )),
                )));
                if index_by_tag_offset.as_pointer().with(
                    |__m: &Map<
                        (Value<u32>, Value<u32>),
                        (Value<(Value<u32>, Value<u32>)>, Value<u16>),
                    >| __m.find(&tag_offset.as_pointer()),
                ) == index_by_tag_offset.as_pointer().with(
                    |__m: &Map<
                        (Value<u32>, Value<u32>),
                        (Value<(Value<u32>, Value<u32>)>, Value<u16>),
                    >| __m.end(),
                ) {
                    eprintln!("Missing table index for offset 0x{:08x}", table_offset);
                    return false;
                }
                let mut index: u16 =
                    (Map::subscript(index_by_tag_offset.as_pointer(), tag_offset.as_pointer())
                        .read());
                ({ Store255UShort_11((index as i32), (offset.as_pointer()), (result).clone()) });
            }
        }
    }
    ({
        StoreBytes_32(
            (compression_buf.decay().offset(0_usize)),
            ((*total_compressed_length.borrow()) as usize),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    let __rhs = (({ Round4_70(((*offset.borrow()) as u64)) }) as usize);
    (*offset.borrow_mut()) = __rhs;
    ({
        StoreBytes_32(
            compressed_metadata_buf.decay(),
            ((*compressed_metadata_buf_length.borrow()) as usize),
            (offset.as_pointer()),
            (result).clone(),
        )
    });
    if ({ (result_length.read()) } != { (*offset.borrow()) }) {
        eprintln!(
            "Mismatch between computed and actual length ({} vs {})",
            (result_length.read()),
            (*offset.borrow())
        );
        return false;
    }
    return true;
}
pub fn GetFileContent_110(filename: Vec<i8>) -> Vec<i8> {
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new(filename));
    let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open(filename.decay().to_string()).expect("Failed to open file"),
    ));
    return {
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &(*ifs.borrow());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    };
}
pub fn SetFileContents_111(
    filename: Vec<i8>,
    start: RandomAccessIterator<i8>,
    end: RandomAccessIterator<i8>,
) {
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new(filename));
    let start: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(start));
    let end: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(end));
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create(filename.decay().to_string()).expect("Failed to open file"),
    ));
    {
        let mut __it = (*start.borrow()).clone();
        let __last = (*end.borrow()).clone();
        let __out = &mut (*ofs.borrow_mut());
        let mut __bytes: Vec<u8> = Vec::new();
        while __it != __last {
            __bytes.push(__it.iter_deref().read() as u8);
            __it.iter_inc();
        }
        __out.write_all(&__bytes);
        __out.try_clone().unwrap()
    };
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<i8>>) -> i32 {
    if (argc != 2) {
        eprintln!("One argument, the input filename, must be provided.");
        return 1;
    }
    let filename: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = (elem!(argv, 1).read()).to_c_bytes();
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
    println!("Processing {} => {}", filename.decay(), outfilename.decay());
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({ GetFileContent_110((*filename.borrow()).clone()) }),
    ));
    let mut input_data: Ptr<u8> = input.decay().reinterpret_cast::<u8>();
    let output_size: Value<usize> = Rc::new(RefCell::new(
        ({ MaxWOFF2CompressedSize_104((input_data).clone(), ((*input.borrow()).len() - 1)) }),
    ));
    let output: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![0_i8; (*output_size.borrow()) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let mut output_data: Ptr<u8> = (output.decay().offset(0_usize)).reinterpret_cast::<u8>();
    let params: Value<woff2_WOFF2Params> = Rc::new(RefCell::new(woff2_WOFF2Params::new()));
    if !({
        ConvertTTFToWOFF2_109(
            (input_data).clone(),
            ((*input.borrow()).len() - 1),
            (output_data).clone(),
            (output_size.as_pointer()),
            params.as_pointer(),
        )
    }) {
        eprintln!("Compression failed.");
        return 1;
    }
    {
        (*output.borrow_mut()).pop();
        (*output.borrow_mut()).resize((*output_size.borrow()) as usize, 0);
        (*output.borrow_mut()).push(0)
    };
    ({
        let _start: RandomAccessIterator<i8> = RandomAccessIterator::Vec(output.decay());
        let _end: RandomAccessIterator<i8> = RandomAccessIterator::Vec(output.decay().to_last());
        SetFileContents_111((*outfilename.borrow()).clone(), _start, _end)
    });
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
    fn Skip(&self, mut n_bytes: usize) -> bool {
        return ({ woff2_BufferImpl::Read(self, Ptr::<u8>::null(), n_bytes) });
    }
    fn Read(&self, mut data: Ptr<u8>, mut n_bytes: usize) -> bool {
        if (n_bytes > (((1024 * 1024) * 1024) as usize)) {
            return false;
        }
        if (((*self).with(|__s| __s.offset_)).wrapping_add(n_bytes)
            > (*self).with(|__s| __s.length_))
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(n_bytes))
        {
            return false;
        }
        if !(data).is_null() {
            {
                (data).to_any().memcpy(
                    &((*self)
                        .with(|__s| __s.buffer_.clone())
                        .offset(((*self).with(|__s| __s.offset_)) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    n_bytes as usize,
                );
                (data).to_any()
            };
        }
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(n_bytes) });
        return true;
    }
    fn ReadU8(&self, mut value: Ptr<u8>) -> bool {
        if ((*self).with(|__s| __s.length_) < 1_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(1_usize))
        {
            return false;
        }
        value.write({
            (elem!(
                (*self).with(|__s| __s.buffer_.clone()),
                (*self).with(|__s| __s.offset_)
            )
            .read())
        });
        field!((*self), offset_).with_mut(|__v| __v.prefix_inc());
        return true;
    }
    fn ReadU16(&self, mut value: Ptr<u16>) -> bool {
        if ((*self).with(|__s| __s.length_) < 2_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(2_usize))
        {
            return false;
        }
        {
            (value).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u16>() as usize,
            );
            (value).to_any()
        };
        let __rhs = u16::from_be((value.read()));
        value.write(__rhs);
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(2_usize) });
        return true;
    }
    fn ReadS16(&self, mut value: Ptr<i16>) -> bool {
        return ({ woff2_BufferImpl::ReadU16(self, value.reinterpret_cast::<u16>()) });
    }
    fn ReadU24(&self, mut value: Ptr<u32>) -> bool {
        if ((*self).with(|__s| __s.length_) < 3_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(3_usize))
        {
            return false;
        }
        value.write({
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
    fn ReadU32(&self, mut value: Ptr<u32>) -> bool {
        if ((*self).with(|__s| __s.length_) < 4_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            (value).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            (value).to_any()
        };
        let __rhs = u32::from_be((value.read()));
        value.write(__rhs);
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(4_usize) });
        return true;
    }
    fn ReadS32(&self, mut value: Ptr<i32>) -> bool {
        return ({ woff2_BufferImpl::ReadU32(self, value.reinterpret_cast::<u32>()) });
    }
    fn ReadTag(&self, mut value: Ptr<u32>) -> bool {
        if ((*self).with(|__s| __s.length_) < 4_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            (value).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            (value).to_any()
        };
        field!((*self), offset_).write({ ((*self).with(|__s| __s.offset_)).wrapping_add(4_usize) });
        return true;
    }
    fn ReadR64(&self, mut value: Ptr<u64>) -> bool {
        if ((*self).with(|__s| __s.length_) < 8_usize)
            || ((*self).with(|__s| __s.offset_)
                > ((*self).with(|__s| __s.length_)).wrapping_sub(8_usize))
        {
            return false;
        }
        {
            (value).to_any().memcpy(
                &((*self)
                    .with(|__s| __s.buffer_.clone())
                    .offset(((*self).with(|__s| __s.offset_)) as isize)
                    as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u64>() as usize,
            );
            (value).to_any()
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
    fn set_offset(&self, mut newoffset: usize) -> bool {
        if (newoffset > (*self).with(|__s| __s.length_)) {
            return false;
        }
        field!((*self), offset_).write(newoffset);
        return true;
    }
}
pub trait woff2_FontImpl {
    fn OutputOrderedTags(&self) -> Vec<u32>;
    fn FindTable_2(&self, tag: u32) -> Ptr<woff2_Font_Table>;
    fn FindTable_3(&self, tag: u32) -> Ptr<woff2_Font_Table>;
    fn move_assign(&self, _a0: Ptr<woff2_Font>) -> Ptr<woff2_Font>;
}
impl woff2_FontImpl for Ptr<woff2_Font> {
    fn move_assign(&self, _a0: Ptr<woff2_Font>) -> Ptr<woff2_Font> {
        field!((*self), flavor).write({ { (*_a0.upgrade().deref()).flavor } });
        field!((*self), num_tables).write({ { (*_a0.upgrade().deref()).num_tables } });
        {
            let __src = field_ptr!(_a0, tables).with_mut(
                |__m: &mut Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| std::mem::take(__m),
            );
            field_ptr!((*self), tables).write(__src)
        };
        return (*self).clone();
    }
    fn FindTable_2(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<BidirectionalIterator<(Value<u32>, Value<woff2_Font_Table>)>> =
            Rc::new(RefCell::new(field_ptr!((*self), tables).with(
                |__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.find(&tag.as_pointer()),
            )));
        return if (*it.borrow())
            == field_ptr!((*self), tables)
                .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end())
        {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*(*it.borrow()).iter_deref().upgrade().deref())
                .1
                .as_pointer())
        };
    }
    fn FindTable_3(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<BidirectionalIterator<(Value<u32>, Value<woff2_Font_Table>)>> =
            Rc::new(RefCell::new(field_ptr!((*self), tables).with(
                |__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.find(&tag.as_pointer()),
            )));
        return if (*it.borrow())
            == field_ptr!((*self), tables)
                .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end())
        {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*(*it.borrow()).iter_deref().upgrade().deref())
                .1
                .as_pointer())
        };
    }
    fn OutputOrderedTags(&self) -> Vec<u32> {
        let output_order: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        'loop_: for i in IterRange::new(
            field_ptr!((*self), tables)
                .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.begin()),
            field_ptr!((*self), tables)
                .with(|__m: &Map<u32, (Value<u32>, Value<woff2_Font_Table>)>| __m.end()),
        ) {
            let i: Ptr<(Value<u32>, Value<woff2_Font_Table>)> = i.iter_deref();
            let table: Ptr<woff2_Font_Table> = (*i.upgrade().deref()).1.as_pointer();
            if ((table.with(|__s| __s.tag) & 2155905152_u32) != 0) {
                continue 'loop_;
            }
            {
                let a0_clone = table.with(|__s| __s.tag).clone();
                (*output_order.borrow_mut()).push(a0_clone)
            };
        }
        let glyf_loc: Value<RandomAccessIterator<u32>> = Rc::new(RefCell::new({
            let mut __it = output_order.decay().begin();
            let __last = output_order.decay().end();
            let __value = kGlyfTableTag_0.with(|v| v.as_pointer());
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        }));
        let loca_loc: Value<RandomAccessIterator<u32>> = Rc::new(RefCell::new({
            let mut __it = output_order.decay().begin();
            let __last = output_order.decay().end();
            let __value = kLocaTableTag_2.with(|v| v.as_pointer());
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        }));
        if ((*glyf_loc.borrow()) != output_order.decay().end())
            && ((*loca_loc.borrow()) != output_order.decay().end())
        {
            {
                let __it = <RandomAccessIterator<u32>>::from((*loca_loc.borrow()).clone());
                let __off = __it.index();
                output_order
                    .as_pointer()
                    .with_mut(|__v: &mut Vec<u32>| __v.remove(__off));
                __it
            };
            {
                let __it = <RandomAccessIterator<u32>>::from(
                    {
                        let mut __it = output_order.decay().begin();
                        let __last = output_order.decay().end();
                        let __value = kGlyfTableTag_0.with(|v| v.as_pointer());
                        while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                            __it.iter_inc();
                        }
                        __it
                    }
                    .iter_add(1_isize),
                );
                let __off = __it.index();
                (*output_order.borrow_mut()).insert(__off, kLocaTableTag_2.with(|rc| *rc.borrow()));
                __it
            };
        }
        return std::mem::take(&mut (*output_order.borrow_mut()));
    }
}
pub trait woff2_FontCollectionImpl {
    fn move_assign(&self, _a0: Ptr<woff2_FontCollection>) -> Ptr<woff2_FontCollection>;
}
impl woff2_FontCollectionImpl for Ptr<woff2_FontCollection> {
    fn move_assign(&self, _a0: Ptr<woff2_FontCollection>) -> Ptr<woff2_FontCollection> {
        field!((*self), flavor).write({ { (*_a0.upgrade().deref()).flavor } });
        field!((*self), header_version).write({ { (*_a0.upgrade().deref()).header_version } });
        {
            let __src = field_ptr!(_a0, tables).with_mut(
                |__m: &mut Map<u32, (Value<u32>, Value<Ptr<woff2_Font_Table>>)>| {
                    std::mem::take(__m)
                },
            );
            field_ptr!((*self), tables).write(__src)
        };
        field_ref!((*self), fonts)
            .as_pointer()
            .write(field!(_a0, fonts).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait woff2_Font_TableImpl {
    fn IsReused(&self) -> bool;
    fn move_assign(&self, _a0: Ptr<woff2_Font_Table>) -> Ptr<woff2_Font_Table>;
}
impl woff2_Font_TableImpl for Ptr<woff2_Font_Table> {
    fn move_assign(&self, _a0: Ptr<woff2_Font_Table>) -> Ptr<woff2_Font_Table> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        field!((*self), checksum).write({ { (*_a0.upgrade().deref()).checksum } });
        field!((*self), offset).write({ { (*_a0.upgrade().deref()).offset } });
        field!((*self), length).write({ { (*_a0.upgrade().deref()).length } });
        field!((*self), data).write({ { (*_a0.upgrade().deref()).data.clone() } });
        field_ref!((*self), buffer)
            .as_pointer()
            .write(field!(_a0, buffer).with_mut(std::mem::take));
        field!((*self), reuse_of).write({ { (*_a0.upgrade().deref()).reuse_of.clone() } });
        field!((*self), flag_byte).write({ { (*_a0.upgrade().deref()).flag_byte } });
        return (*self).clone();
    }
    fn IsReused(&self) -> bool {
        return !(((*self).with(|__s| __s.reuse_of.clone())).is_null());
    }
}
pub trait woff2_GlyfEncoderImpl {
    fn Encode(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>) -> bool;
    fn GetTransformedGlyfBytes(&self, result: Ptr<Vec<u8>>);
    fn WriteInstructions(&self, glyph: Ptr<woff2_Glyph>);
    fn ShouldWriteSimpleGlyphBbox(&self, glyph: Ptr<woff2_Glyph>) -> bool;
    fn WriteSimpleGlyph(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>);
    fn WriteCompositeGlyph(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>);
    fn WriteBbox(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>);
    fn WriteTriplet(&self, on_curve: bool, x: i32, y: i32);
    fn EnsureOverlapBitmap(&self);
    fn move_assign(&self, _a0: Ptr<woff2_GlyfEncoder>) -> Ptr<woff2_GlyfEncoder>;
}
impl woff2_GlyfEncoderImpl for Ptr<woff2_GlyfEncoder> {
    fn Encode(&self, mut glyph_id: i32, glyph: Ptr<woff2_Glyph>) -> bool {
        if (glyph.with(|__s| __s.composite_data_size) > 0_u32) {
            ({
                let _glyph_id: i32 = glyph_id;
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                woff2_GlyfEncoderImpl::WriteCompositeGlyph(self, _glyph_id, _glyph)
            });
        } else if ((*glyph.upgrade().deref()).contours.len() > 0_usize) {
            ({
                let _glyph_id: i32 = glyph_id;
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                woff2_GlyfEncoderImpl::WriteSimpleGlyph(self, _glyph_id, _glyph)
            });
        } else {
            ({ WriteUShort_88((field_ref!((*self), n_contour_stream_).as_pointer()), 0) });
        }
        return true;
    }
    fn GetTransformedGlyfBytes(&self, mut result: Ptr<Vec<u8>>) {
        ({ WriteUShort_88((result).clone(), 0) });
        ({
            WriteUShort_88(
                (result).clone(),
                if (*(*self).upgrade().deref()).overlap_bitmap_.is_empty() {
                    0
                } else {
                    FLAG_OVERLAP_SIMPLE_BITMAP_85.with(|rc| *rc.borrow())
                },
            )
        });
        ({ WriteUShort_88((result).clone(), (*self).with(|__s| __s.n_glyphs_)) });
        ({ WriteUShort_88((result).clone(), 0) });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).n_contour_stream_.len() as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).n_points_stream_.len() as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).flag_byte_stream_.len() as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).glyph_stream_.len() as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).composite_stream_.len() as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((((*(*self).upgrade().deref()).bbox_bitmap_.len())
                    .wrapping_add((*(*self).upgrade().deref()).bbox_stream_.len()))
                    as i32),
            )
        });
        ({
            WriteLong_89(
                (result).clone(),
                ((*(*self).upgrade().deref()).instruction_stream_.len() as i32),
            )
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), n_contour_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), n_points_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), flag_byte_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), glyph_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), composite_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), bbox_bitmap_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), bbox_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        ({
            let _out: Ptr<Vec<u8>> = (result).clone();
            let _in_: Ptr<Vec<u8>> = field_ref!((*self), instruction_stream_).as_pointer();
            WriteBytes_87(_out, _in_)
        });
        if !((*(*self).upgrade().deref()).overlap_bitmap_.is_empty()) {
            ({
                let _out: Ptr<Vec<u8>> = (result).clone();
                let _in_: Ptr<Vec<u8>> = field_ref!((*self), overlap_bitmap_).as_pointer();
                WriteBytes_87(_out, _in_)
            });
        }
    }
    fn WriteInstructions(&self, glyph: Ptr<woff2_Glyph>) {
        ({
            Write255UShort_10(
                (field_ref!((*self), glyph_stream_).as_pointer()),
                (glyph.with(|__s| __s.instructions_size) as i32),
            )
        });
        ({
            let _data: Ptr<u8> = glyph.with(|__s| __s.instructions_data.clone());
            let _len: usize = (glyph.with(|__s| __s.instructions_size) as usize);
            WriteBytes_86(
                (field_ref!((*self), instruction_stream_).as_pointer()),
                _data,
                _len,
            )
        });
    }
    fn ShouldWriteSimpleGlyphBbox(&self, glyph: Ptr<woff2_Glyph>) -> bool {
        if ((*glyph.upgrade().deref()).contours.is_empty())
            || ((*field_ref!(glyph, contours)
                .decay()
                .offset(0_usize)
                .upgrade()
                .deref())
            .is_empty())
        {
            return (((glyph.with(|__s| __s.x_min) != 0) || (glyph.with(|__s| __s.y_min) != 0))
                || (glyph.with(|__s| __s.x_max) != 0))
                || (glyph.with(|__s| __s.y_max) != 0);
        }
        let mut x_min: i16 =
            (index!(vec_elem!(glyph, contours, 0_usize), 0_usize).with(|__s| __s.x) as i16);
        let mut y_min: i16 =
            (index!(vec_elem!(glyph, contours, 0_usize), 0_usize).with(|__s| __s.y) as i16);
        let mut x_max: i16 = x_min;
        let mut y_max: i16 = y_min;
        'loop_: for contour in IterRange::new(
            field_ref!(glyph, contours).decay().begin(),
            field_ref!(glyph, contours).decay().end(),
        ) {
            let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.iter_deref();
            'loop_: for point in IterRange::new(contour.decay().begin(), contour.decay().end()) {
                let point: Ptr<woff2_Glyph_Point> = point.iter_deref();
                if ({ point.with(|__s| __s.x) } < { (x_min as i32) }) {
                    x_min = (point.with(|__s| __s.x) as i16);
                }
                if ({ point.with(|__s| __s.x) } > { (x_max as i32) }) {
                    x_max = (point.with(|__s| __s.x) as i16);
                }
                if ({ point.with(|__s| __s.y) } < { (y_min as i32) }) {
                    y_min = (point.with(|__s| __s.y) as i16);
                }
                if ({ point.with(|__s| __s.y) } > { (y_max as i32) }) {
                    y_max = (point.with(|__s| __s.y) as i16);
                }
            }
        }
        if ({ (glyph.with(|__s| __s.x_min) as i32) } != { (x_min as i32) }) {
            return true;
        }
        if ({ (glyph.with(|__s| __s.y_min) as i32) } != { (y_min as i32) }) {
            return true;
        }
        if ({ (glyph.with(|__s| __s.x_max) as i32) } != { (x_max as i32) }) {
            return true;
        }
        if ({ (glyph.with(|__s| __s.y_max) as i32) } != { (y_max as i32) }) {
            return true;
        }
        return false;
    }
    fn WriteSimpleGlyph(&self, mut glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        if glyph.with(|__s| __s.overlap_simple_flag_set) {
            ({ woff2_GlyfEncoderImpl::EnsureOverlapBitmap(self) });
            {
                let rhs_0 = (((vec_elem!((*self), overlap_bitmap_, ((glyph_id >> 3) as usize))
                    .read()) as i32)
                    | (128 >> (glyph_id & 7))) as u8;
                vec_elem!((*self), overlap_bitmap_, ((glyph_id >> 3) as usize)).write(rhs_0)
            };
        }
        let mut num_contours: i32 = ((*glyph.upgrade().deref()).contours.len() as i32);
        ({
            WriteUShort_88(
                (field_ref!((*self), n_contour_stream_).as_pointer()),
                num_contours,
            )
        });
        if ({ woff2_GlyfEncoderImpl::ShouldWriteSimpleGlyphBbox(self, (glyph).clone()) }) {
            ({
                let _glyph_id: i32 = glyph_id;
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                woff2_GlyfEncoderImpl::WriteBbox(self, _glyph_id, _glyph)
            });
        }
        let mut i: i32 = 0;
        'loop_: while (i < num_contours) {
            ({
                Write255UShort_10(
                    (field_ref!((*self), n_points_stream_).as_pointer()),
                    ((*field_ref!(glyph, contours)
                        .decay()
                        .offset((i as usize))
                        .upgrade()
                        .deref())
                    .len() as i32),
                )
            });
            i.postfix_inc();
        }
        let mut lastX: i32 = 0;
        let mut lastY: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while (i < num_contours) {
            let mut num_points: i32 = ((*field_ref!(glyph, contours)
                .decay()
                .offset((i as usize))
                .upgrade()
                .deref())
            .len() as i32);
            let mut j: i32 = 0;
            'loop_: while (j < num_points) {
                let mut x: i32 = index!(vec_elem!(glyph, contours, (i as usize)), (j as usize))
                    .with(|__s| __s.x);
                let mut y: i32 = index!(vec_elem!(glyph, contours, (i as usize)), (j as usize))
                    .with(|__s| __s.y);
                let mut dx: i32 = (x - lastX);
                let mut dy: i32 = (y - lastY);
                ({
                    woff2_GlyfEncoderImpl::WriteTriplet(
                        self,
                        index!(vec_elem!(glyph, contours, (i as usize)), (j as usize))
                            .with(|__s| __s.on_curve),
                        dx,
                        dy,
                    )
                });
                lastX = x;
                lastY = y;
                j.postfix_inc();
            }
            i.postfix_inc();
        }
        if (num_contours > 0) {
            ({ woff2_GlyfEncoderImpl::WriteInstructions(self, (glyph).clone()) });
        }
    }
    fn WriteCompositeGlyph(&self, mut glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        ({
            WriteUShort_88(
                (field_ref!((*self), n_contour_stream_).as_pointer()),
                -1_i32,
            )
        });
        ({
            let _glyph_id: i32 = glyph_id;
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            woff2_GlyfEncoderImpl::WriteBbox(self, _glyph_id, _glyph)
        });
        ({
            let _data: Ptr<u8> = glyph.with(|__s| __s.composite_data.clone());
            let _len: usize = (glyph.with(|__s| __s.composite_data_size) as usize);
            WriteBytes_86(
                (field_ref!((*self), composite_stream_).as_pointer()),
                _data,
                _len,
            )
        });
        if glyph.with(|__s| __s.have_instructions) {
            ({ woff2_GlyfEncoderImpl::WriteInstructions(self, (glyph).clone()) });
        }
    }
    fn WriteBbox(&self, mut glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        {
            let rhs_0 = (((vec_elem!((*self), bbox_bitmap_, ((glyph_id >> 3) as usize)).read())
                as i32)
                | (128 >> (glyph_id & 7))) as u8;
            vec_elem!((*self), bbox_bitmap_, ((glyph_id >> 3) as usize)).write(rhs_0)
        };
        ({
            WriteUShort_88(
                (field_ref!((*self), bbox_stream_).as_pointer()),
                (glyph.with(|__s| __s.x_min) as i32),
            )
        });
        ({
            WriteUShort_88(
                (field_ref!((*self), bbox_stream_).as_pointer()),
                (glyph.with(|__s| __s.y_min) as i32),
            )
        });
        ({
            WriteUShort_88(
                (field_ref!((*self), bbox_stream_).as_pointer()),
                (glyph.with(|__s| __s.x_max) as i32),
            )
        });
        ({
            WriteUShort_88(
                (field_ref!((*self), bbox_stream_).as_pointer()),
                (glyph.with(|__s| __s.y_max) as i32),
            )
        });
    }
    fn WriteTriplet(&self, mut on_curve: bool, mut x: i32, mut y: i32) {
        let mut abs_x: i32 = x.abs();
        let mut abs_y: i32 = y.abs();
        let mut on_curve_bit: i32 = if on_curve { 0 } else { 128 };
        let mut x_sign_bit: i32 = if (x < 0) { 0 } else { 1 };
        let mut y_sign_bit: i32 = if (y < 0) { 0 } else { 1 };
        let mut xy_sign_bits: i32 = (x_sign_bit + (2 * y_sign_bit));
        if (x == 0) && (abs_y < 1280) {
            {
                let __a1 = (((on_curve_bit + ((abs_y & 3840) >> 7)) + y_sign_bit) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_y & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        } else if (y == 0) && (abs_x < 1280) {
            {
                let __a1 = ((((on_curve_bit + 10) + ((abs_x & 3840) >> 7)) + x_sign_bit) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_x & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        } else if (abs_x < 65) && (abs_y < 65) {
            {
                let __a1 = (((((on_curve_bit + 20) + ((abs_x - 1) & 48))
                    + (((abs_y - 1) & 48) >> 2))
                    + xy_sign_bits) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = (((((abs_x - 1) & 15) << 4) | ((abs_y - 1) & 15)) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        } else if (abs_x < 769) && (abs_y < 769) {
            {
                let __a1 = (((((on_curve_bit + 84) + (12 * (((abs_x - 1) & 768) >> 8)))
                    + (((abs_y - 1) & 768) >> 6))
                    + xy_sign_bits) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = (((abs_x - 1) & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = (((abs_y - 1) & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        } else if (abs_x < 4096) && (abs_y < 4096) {
            {
                let __a1 = (((on_curve_bit + 120) + xy_sign_bits) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_x >> 4) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((((abs_x & 15) << 4) | (abs_y >> 8)) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_y & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        } else {
            {
                let __a1 = (((on_curve_bit + 124) + xy_sign_bits) as u8);
                field!((*self), flag_byte_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_x >> 8) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_x & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_y >> 8) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
            {
                let __a1 = ((abs_y & 255) as u8);
                field!((*self), glyph_stream_).with_mut(|__v: &mut Vec<u8>| __v.push(__a1))
            };
        }
    }
    fn EnsureOverlapBitmap(&self) {
        if (*(*self).upgrade().deref()).overlap_bitmap_.is_empty() {
            {
                let __a0 = ((((*self).with(|__s| __s.n_glyphs_) + 7) >> 3) as usize) as usize;
                field!((*self), overlap_bitmap_)
                    .with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
            };
        }
    }
    fn move_assign(&self, _a0: Ptr<woff2_GlyfEncoder>) -> Ptr<woff2_GlyfEncoder> {
        field_ref!((*self), n_contour_stream_)
            .as_pointer()
            .write(field!(_a0, n_contour_stream_).with_mut(std::mem::take));
        field_ref!((*self), n_points_stream_)
            .as_pointer()
            .write(field!(_a0, n_points_stream_).with_mut(std::mem::take));
        field_ref!((*self), flag_byte_stream_)
            .as_pointer()
            .write(field!(_a0, flag_byte_stream_).with_mut(std::mem::take));
        field_ref!((*self), composite_stream_)
            .as_pointer()
            .write(field!(_a0, composite_stream_).with_mut(std::mem::take));
        field_ref!((*self), bbox_bitmap_)
            .as_pointer()
            .write(field!(_a0, bbox_bitmap_).with_mut(std::mem::take));
        field_ref!((*self), bbox_stream_)
            .as_pointer()
            .write(field!(_a0, bbox_stream_).with_mut(std::mem::take));
        field_ref!((*self), glyph_stream_)
            .as_pointer()
            .write(field!(_a0, glyph_stream_).with_mut(std::mem::take));
        field_ref!((*self), instruction_stream_)
            .as_pointer()
            .write(field!(_a0, instruction_stream_).with_mut(std::mem::take));
        field_ref!((*self), overlap_bitmap_)
            .as_pointer()
            .write(field!(_a0, overlap_bitmap_).with_mut(std::mem::take));
        field!((*self), n_glyphs_).write({ { (*_a0.upgrade().deref()).n_glyphs_ } });
        return (*self).clone();
    }
}
pub trait woff2_GlyphImpl {
    fn move_assign(&self, _a0: Ptr<woff2_Glyph>) -> Ptr<woff2_Glyph>;
}
impl woff2_GlyphImpl for Ptr<woff2_Glyph> {
    fn move_assign(&self, _a0: Ptr<woff2_Glyph>) -> Ptr<woff2_Glyph> {
        field!((*self), x_min).write({ { (*_a0.upgrade().deref()).x_min } });
        field!((*self), x_max).write({ { (*_a0.upgrade().deref()).x_max } });
        field!((*self), y_min).write({ { (*_a0.upgrade().deref()).y_min } });
        field!((*self), y_max).write({ { (*_a0.upgrade().deref()).y_max } });
        field!((*self), instructions_size)
            .write({ { (*_a0.upgrade().deref()).instructions_size } });
        field!((*self), instructions_data)
            .write({ { (*_a0.upgrade().deref()).instructions_data.clone() } });
        field!((*self), overlap_simple_flag_set)
            .write({ { (*_a0.upgrade().deref()).overlap_simple_flag_set } });
        field_ref!((*self), contours)
            .as_pointer()
            .write(field!(_a0, contours).with_mut(std::mem::take));
        field!((*self), composite_data)
            .write({ { (*_a0.upgrade().deref()).composite_data.clone() } });
        field!((*self), composite_data_size)
            .write({ { (*_a0.upgrade().deref()).composite_data_size } });
        field!((*self), have_instructions)
            .write({ { (*_a0.upgrade().deref()).have_instructions } });
        return (*self).clone();
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
pub trait woff2_WOFF2ParamsImpl {
    fn move_assign(&self, _a0: Ptr<woff2_WOFF2Params>) -> Ptr<woff2_WOFF2Params>;
}
impl woff2_WOFF2ParamsImpl for Ptr<woff2_WOFF2Params> {
    fn move_assign(&self, _a0: Ptr<woff2_WOFF2Params>) -> Ptr<woff2_WOFF2Params> {
        field!((*self), extended_metadata)
            .write({ { (*_a0.upgrade().deref()).extended_metadata.clone() } });
        field!((*self), brotli_quality).write({ { (*_a0.upgrade().deref()).brotli_quality } });
        field!((*self), allow_transforms).write({ { (*_a0.upgrade().deref()).allow_transforms } });
        return (*self).clone();
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
    let _ = kFLAG_ONCURVE_49.with(|_| ());
    let _ = kFLAG_XSHORT_50.with(|_| ());
    let _ = kFLAG_YSHORT_51.with(|_| ());
    let _ = kFLAG_REPEAT_52.with(|_| ());
    let _ = kFLAG_XREPEATSIGN_53.with(|_| ());
    let _ = kFLAG_YREPEATSIGN_54.with(|_| ());
    let _ = kFLAG_OVERLAP_SIMPLE_55.with(|_| ());
    let _ = kFLAG_ARG_1_AND_2_ARE_WORDS_56.with(|_| ());
    let _ = kFLAG_WE_HAVE_A_SCALE_57.with(|_| ());
    let _ = kFLAG_MORE_COMPONENTS_58.with(|_| ());
    let _ = kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59.with(|_| ());
    let _ = kFLAG_WE_HAVE_A_TWO_BY_TWO_60.with(|_| ());
    let _ = kFLAG_WE_HAVE_INSTRUCTIONS_61.with(|_| ());
    let _ = FLAG_ARG_1_AND_2_ARE_WORDS_83.with(|_| ());
    let _ = FLAG_WE_HAVE_INSTRUCTIONS_84.with(|_| ());
    let _ = FLAG_OVERLAP_SIMPLE_BITMAP_85.with(|_| ());
    let _ = kWoff2HeaderSize_92.with(|_| ());
    let _ = kWoff2EntrySize_93.with(|_| ());
}
