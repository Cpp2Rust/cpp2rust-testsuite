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
    'loop_: for mut packed_byte in packed.as_pointer() as Ptr<u8> {
        let mut packed_byte: u8 = packed_byte.read();
        let __rhs = packed_byte;
        elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    }
}
pub fn Read255UShort_12(mut buf: Ptr<woff2_Buffer>, mut value: Ptr<u32>) -> bool {
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
thread_local!(
    pub static kDefaultMaxSize_28: Value<usize> =
        Rc::new(RefCell::new((((128 * 1024) * 1024) as usize)));
);
pub trait woff2_WOFF2Out {
    fn Write_2(&mut self, buf: AnyPtr, n: usize) -> bool;
    fn Write_3(&mut self, buf: AnyPtr, offset: usize, n: usize) -> bool;
    fn Size(&mut self) -> usize;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct woff2_WOFF2StringOut {
    #[offset(8)]
    #[byte_size(8)]
    buf_: Ptr<Vec<i8>>,
    #[offset(16)]
    max_size_: usize,
    #[offset(24)]
    offset_: usize,
}
impl woff2_WOFF2StringOut {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct woff2_WOFF2MemoryOut {
    #[offset(8)]
    #[byte_size(8)]
    buf_: Ptr<u8>,
    #[offset(16)]
    buf_size_: usize,
    #[offset(24)]
    offset_: usize,
}
impl woff2_WOFF2MemoryOut {}
pub fn Round4_29(mut value: u64) -> u64 {
    if ((<u64>::MAX as u64).wrapping_sub(value) < 3_u64) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub fn Round4_30(mut value: u32) -> u32 {
    if ((<u32>::MAX as u32).wrapping_sub(value) < 3_u32) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub fn StoreU32_31(mut dst: Ptr<u8>, mut offset: usize, mut x: u32) -> usize {
    elem!(dst, offset).write({ ((x >> 24) as u8) });
    elem!(dst, (offset).wrapping_add(1_usize)).write({ ((x >> 16) as u8) });
    elem!(dst, (offset).wrapping_add(2_usize)).write({ ((x >> 8) as u8) });
    elem!(dst, (offset).wrapping_add(3_usize)).write({ (x as u8) });
    return (offset).wrapping_add(4_usize);
}
pub fn Store16_32(mut dst: Ptr<u8>, mut offset: usize, mut x: i32) -> usize {
    elem!(dst, offset).write({ ((x >> 8) as u8) });
    elem!(dst, (offset).wrapping_add(1_usize)).write({ (x as u8) });
    return (offset).wrapping_add(2_usize);
}
pub fn StoreU32_33(mut val: u32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let __rhs = ((val >> 24) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = ((val >> 16) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = ((val >> 8) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = (val as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
}
pub fn Store16_34(mut val: i32, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    let __rhs = ((val >> 8) as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
    let __rhs = (val as u8);
    elem!(dst, offset.with_mut(|__v| __v.postfix_inc())).write(__rhs);
}
pub fn StoreBytes_35(mut data: Ptr<u8>, mut len: usize, mut offset: Ptr<usize>, mut dst: Ptr<u8>) {
    {
        ((dst.offset((offset.read()) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(&(data).to_any(), len as usize);
        ((dst.offset((offset.read()) as isize)) as Ptr<u8>).to_any()
    };
    offset.write({ (offset.read()).wrapping_add(len) });
}
thread_local!(
    pub static kGlyfOnCurve_36: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static kGlyfXShort_37: Value<i32> = Rc::new(RefCell::new((1 << 1)));
);
thread_local!(
    pub static kGlyfYShort_38: Value<i32> = Rc::new(RefCell::new((1 << 2)));
);
thread_local!(
    pub static kGlyfRepeat_39: Value<i32> = Rc::new(RefCell::new((1 << 3)));
);
thread_local!(
    pub static kGlyfThisXIsSame_40: Value<i32> = Rc::new(RefCell::new((1 << 4)));
);
thread_local!(
    pub static kGlyfThisYIsSame_41: Value<i32> = Rc::new(RefCell::new((1 << 5)));
);
thread_local!(
    pub static kOverlapSimple_42: Value<i32> = Rc::new(RefCell::new((1 << 6)));
);
thread_local!(
    pub static FLAG_ARG_1_AND_2_ARE_WORDS_43: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static FLAG_WE_HAVE_A_SCALE_44: Value<i32> = Rc::new(RefCell::new((1 << 3)));
);
thread_local!(
    pub static FLAG_MORE_COMPONENTS_45: Value<i32> = Rc::new(RefCell::new((1 << 5)));
);
thread_local!(
    pub static FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46: Value<i32> = Rc::new(RefCell::new((1 << 6)));
);
thread_local!(
    pub static FLAG_WE_HAVE_A_TWO_BY_TWO_47: Value<i32> = Rc::new(RefCell::new((1 << 7)));
);
thread_local!(
    pub static FLAG_WE_HAVE_INSTRUCTIONS_48: Value<i32> = Rc::new(RefCell::new((1 << 8)));
);
thread_local!(
    pub static FLAG_OVERLAP_SIMPLE_BITMAP_49: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static kCheckSumAdjustmentOffset_50: Value<usize> = Rc::new(RefCell::new(8_usize));
);
thread_local!(
    pub static kEndPtsOfContoursOffset_51: Value<usize> = Rc::new(RefCell::new(10_usize));
);
thread_local!(
    pub static kCompositeGlyphBegin_52: Value<usize> = Rc::new(RefCell::new(10_usize));
);
thread_local!(
    pub static kDefaultGlyphBuf_53: Value<usize> = Rc::new(RefCell::new(5120_usize));
);
thread_local!(
    pub static kMaxPlausibleCompressionRatio_54: Value<f32> =
        Rc::new(RefCell::new((1.0E+2 as f32)));
);
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct woff2_TtcFont {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub dst_offset: u32,
    #[offset(8)]
    pub header_checksum: u32,
    #[offset(16)]
    #[byte_size(24)]
    pub table_indices: Value<Vec<u16>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct woff2_WOFF2Header {
    #[offset(0)]
    pub flavor: u32,
    #[offset(4)]
    pub header_version: u32,
    #[offset(8)]
    pub num_tables: u16,
    #[offset(16)]
    pub compressed_offset: u64,
    #[offset(24)]
    pub compressed_length: u32,
    #[offset(28)]
    pub uncompressed_size: u32,
    #[offset(32)]
    #[byte_size(24)]
    pub tables: Value<Vec<woff2_Table>>,
    #[offset(56)]
    #[byte_size(24)]
    pub ttc_fonts: Value<Vec<woff2_TtcFont>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct woff2_WOFF2FontInfo {
    #[offset(0)]
    pub num_glyphs: u16,
    #[offset(2)]
    pub index_format: u16,
    #[offset(4)]
    pub num_hmetrics: u16,
    #[offset(8)]
    #[byte_size(24)]
    pub x_mins: Value<Vec<i16>>,
    #[offset(32)]
    #[byte_size(48)]
    pub table_entry_by_tag: BTreeMap<u32, Value<u32>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct woff2_RebuildMetadata {
    #[offset(0)]
    pub header_checksum: u32,
    #[offset(8)]
    #[byte_size(24)]
    pub font_infos: Value<Vec<woff2_WOFF2FontInfo>>,
    #[offset(32)]
    #[byte_size(48)]
    pub checksums: BTreeMap<(Value<u32>, Value<u32>), Value<u32>>,
}
pub fn WithSign_55(mut flag: i32, mut baseval: i32) -> i32 {
    return if ((flag & 1) != 0) { baseval } else { -baseval };
}
pub fn _SafeIntAddition_56(mut a: i32, mut b: i32, mut result: Ptr<i32>) -> bool {
    if (((((a > 0) && (b > (<i32>::MAX - a))) || ((a < 0) && (b < (<i32>::MIN - a)))) as i64) != 0)
    {
        return false;
    }
    result.write({ (a + b) });
    return true;
}
pub fn TripletDecode_57(
    mut flags_in: Ptr<u8>,
    mut in_: Ptr<u8>,
    mut in_size: usize,
    mut n_points: u32,
    mut result: Ptr<woff2_Point>,
    mut in_bytes_consumed: Ptr<usize>,
) -> bool {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    if ((((n_points as usize) > in_size) as i64) != 0) {
        return false;
    }
    let mut triplet_index: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    'loop_: while (i < n_points) {
        let mut flag: u8 = (elem!(flags_in, i).read());
        let mut on_curve: bool = !(((flag as i32) >> 7) != 0);
        flag = { ((flag as i32) & 127) as u8 };
        let mut n_data_bytes: u32 = 0_u32;
        if ((flag as i32) < 84) {
            n_data_bytes = 1_u32;
        } else if ((flag as i32) < 120) {
            n_data_bytes = 2_u32;
        } else if ((flag as i32) < 124) {
            n_data_bytes = 3_u32;
        } else {
            n_data_bytes = 4_u32;
        }
        if (((((((triplet_index).wrapping_add(n_data_bytes)) as usize) > in_size)
            || ((triplet_index).wrapping_add(n_data_bytes) < triplet_index)) as i64)
            != 0)
        {
            return false;
        }
        let mut dx: i32 = 0_i32;
        let mut dy: i32 = 0_i32;
        if ((flag as i32) < 10) {
            dx = 0;
            dy = ({
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = ({ (((flag as i32) & 14) << 7) } + {
                    ((elem!(in_, triplet_index).read()) as i32)
                });
                WithSign_55(_flag, _baseval)
            });
        } else if ((flag as i32) < 20) {
            dx = ({
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = ({ ((((flag as i32) - 10) & 14) << 7) } + {
                    ((elem!(in_, triplet_index).read()) as i32)
                });
                WithSign_55(_flag, _baseval)
            });
            dy = 0;
        } else if ((flag as i32) < 84) {
            let mut b0: i32 = ((flag as i32) - 20);
            let mut b1: i32 = ((elem!(in_, triplet_index).read()) as i32);
            dx = ({ WithSign_55((flag as i32), ((1 + (b0 & 48)) + (b1 >> 4))) });
            dy = ({ WithSign_55(((flag as i32) >> 1), ((1 + ((b0 & 12) << 2)) + (b1 & 15))) });
        } else if ((flag as i32) < 120) {
            let mut b0: i32 = ((flag as i32) - 84);
            dx = ({
                WithSign_55(
                    (flag as i32),
                    ({ (1 + ((b0 / 12) << 8)) } + { ((elem!(in_, triplet_index).read()) as i32) }),
                )
            });
            dy = ({
                WithSign_55(
                    ((flag as i32) >> 1),
                    ({ (1 + (((b0 % 12) >> 2) << 8)) } + {
                        ((elem!(in_, (triplet_index).wrapping_add(1_u32)).read()) as i32)
                    }),
                )
            });
        } else if ((flag as i32) < 124) {
            let mut b2: i32 = ((elem!(in_, (triplet_index).wrapping_add(1_u32)).read()) as i32);
            dx = ({
                WithSign_55(
                    (flag as i32),
                    ({ (((elem!(in_, triplet_index).read()) as i32) << 4) } + { (b2 >> 4) }),
                )
            });
            dy = ({
                WithSign_55(
                    ((flag as i32) >> 1),
                    ({ ((b2 & 15) << 8) } + {
                        ((elem!(in_, (triplet_index).wrapping_add(2_u32)).read()) as i32)
                    }),
                )
            });
        } else {
            dx = ({
                WithSign_55(
                    (flag as i32),
                    ((((elem!(in_, triplet_index).read()) as i32) << 8)
                        + ((elem!(in_, (triplet_index).wrapping_add(1_u32)).read()) as i32)),
                )
            });
            dy = ({
                WithSign_55(
                    ((flag as i32) >> 1),
                    ((((elem!(in_, (triplet_index).wrapping_add(2_u32)).read()) as i32) << 8)
                        + ((elem!(in_, (triplet_index).wrapping_add(3_u32)).read()) as i32)),
                )
            });
        }
        triplet_index = { (triplet_index).wrapping_add(n_data_bytes) };
        if !({
            let _a: i32 = (*x.borrow());
            let _result: Ptr<i32> = (x.as_pointer());
            _SafeIntAddition_56(_a, dx, _result)
        }) {
            return false;
        }
        if !({
            let _a: i32 = (*y.borrow());
            let _result: Ptr<i32> = (y.as_pointer());
            _SafeIntAddition_56(_a, dy, _result)
        }) {
            return false;
        }
        let __rhs = woff2_Point {
            x: (*x.borrow()),
            y: (*y.borrow()),
            on_curve: on_curve,
        };
        result.postfix_inc().write(__rhs);
        i.prefix_inc();
    }
    in_bytes_consumed.write({ (triplet_index as usize) });
    return true;
}
pub fn StorePoints_58(
    mut n_points: u32,
    mut points: Ptr<woff2_Point>,
    mut n_contours: u32,
    mut instruction_length: u32,
    mut has_overlap_bit: bool,
    mut dst: Ptr<u8>,
    mut dst_size: usize,
    mut glyph_size: Ptr<usize>,
) -> bool {
    let mut flag_offset: u32 = (((((kEndPtsOfContoursOffset_51.with(|rc| *rc.borrow()))
        .wrapping_add((((2_u32).wrapping_mul(n_contours)) as usize)))
    .wrapping_add(2_usize))
    .wrapping_add((instruction_length as usize))) as u32);
    let mut last_flag: i32 = -1_i32;
    let mut repeat_count: i32 = 0;
    let mut last_x: i32 = 0;
    let mut last_y: i32 = 0;
    let mut x_bytes: u32 = 0_u32;
    let mut y_bytes: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    'loop_: while (i < n_points) {
        let point: Ptr<woff2_Point> = points.offset((i) as isize);
        let mut flag: i32 = if point.with(|__s| __s.on_curve) {
            kGlyfOnCurve_36.with(|rc| *rc.borrow())
        } else {
            0
        };
        if (has_overlap_bit) && (i == 0_u32) {
            flag |= kOverlapSimple_42.with(|rc| *rc.borrow());
        }
        let mut dx: i32 = ({ point.with(|__s| __s.x) } - { last_x });
        let mut dy: i32 = ({ point.with(|__s| __s.y) } - { last_y });
        if (dx == 0) {
            flag |= kGlyfThisXIsSame_40.with(|rc| *rc.borrow());
        } else if (dx > -256_i32) && (dx < 256) {
            flag |= (kGlyfXShort_37.with(|rc| *rc.borrow())
                | (if (dx > 0) {
                    kGlyfThisXIsSame_40.with(|rc| *rc.borrow())
                } else {
                    0
                }));
            x_bytes = { (x_bytes).wrapping_add(1_u32) };
        } else {
            x_bytes = { (x_bytes).wrapping_add(2_u32) };
        }
        if (dy == 0) {
            flag |= kGlyfThisYIsSame_41.with(|rc| *rc.borrow());
        } else if (dy > -256_i32) && (dy < 256) {
            flag |= (kGlyfYShort_38.with(|rc| *rc.borrow())
                | (if (dy > 0) {
                    kGlyfThisYIsSame_41.with(|rc| *rc.borrow())
                } else {
                    0
                }));
            y_bytes = { (y_bytes).wrapping_add(1_u32) };
        } else {
            y_bytes = { (y_bytes).wrapping_add(2_u32) };
        }
        if (flag == last_flag) && (repeat_count != 255) {
            elem!(dst, (flag_offset).wrapping_sub(1_u32)).write({
                (((elem!(dst, (flag_offset).wrapping_sub(1_u32)).read()) as i32)
                    | kGlyfRepeat_39.with(|rc| *rc.borrow())) as u8
            });
            repeat_count.postfix_inc();
        } else {
            if (repeat_count != 0) {
                if ((((flag_offset as usize) >= dst_size) as i64) != 0) {
                    return false;
                }
                let __rhs = (repeat_count as u8);
                elem!(dst, flag_offset.postfix_inc()).write(__rhs);
            }
            if ((((flag_offset as usize) >= dst_size) as i64) != 0) {
                return false;
            }
            let __rhs = (flag as u8);
            elem!(dst, flag_offset.postfix_inc()).write(__rhs);
            repeat_count = 0;
        }
        last_x = point.with(|__s| __s.x);
        last_y = point.with(|__s| __s.y);
        last_flag = flag;
        i.prefix_inc();
    }
    if (repeat_count != 0) {
        if ((((flag_offset as usize) >= dst_size) as i64) != 0) {
            return false;
        }
        let __rhs = (repeat_count as u8);
        elem!(dst, flag_offset.postfix_inc()).write(__rhs);
    }
    let mut xy_bytes: u32 = (x_bytes).wrapping_add(y_bytes);
    if (((((xy_bytes < x_bytes) || ((flag_offset).wrapping_add(xy_bytes) < flag_offset))
        || ((((flag_offset).wrapping_add(xy_bytes)) as usize) > dst_size)) as i64)
        != 0)
    {
        return false;
    }
    let mut x_offset: i32 = (flag_offset as i32);
    let mut y_offset: i32 = (((flag_offset).wrapping_add(x_bytes)) as i32);
    last_x = 0;
    last_y = 0;
    let mut i: u32 = 0_u32;
    'loop_: while (i < n_points) {
        let mut dx: i32 = ({ { (*elem!(points, i).upgrade().deref()).x } } - { last_x });
        if (dx == 0) {
        } else if (dx > -256_i32) && (dx < 256) {
            let __rhs = (dx.abs() as u8);
            elem!(dst, x_offset.postfix_inc()).write(__rhs);
        } else {
            let __rhs = (({ Store16_32((dst).clone(), (x_offset as usize), dx) }) as i32);
            x_offset = __rhs;
        }
        last_x += dx;
        let mut dy: i32 = ({ { (*elem!(points, i).upgrade().deref()).y } } - { last_y });
        if (dy == 0) {
        } else if (dy > -256_i32) && (dy < 256) {
            let __rhs = (dy.abs() as u8);
            elem!(dst, y_offset.postfix_inc()).write(__rhs);
        } else {
            let __rhs = (({ Store16_32((dst).clone(), (y_offset as usize), dy) }) as i32);
            y_offset = __rhs;
        }
        last_y += dy;
        i.prefix_inc();
    }
    glyph_size.write({ (y_offset as usize) });
    return true;
}
pub fn ComputeBbox_59(mut n_points: u32, mut points: Ptr<woff2_Point>, mut dst: Ptr<u8>) {
    let x_min: Value<i32> = Rc::new(RefCell::new(0));
    let y_min: Value<i32> = Rc::new(RefCell::new(0));
    let x_max: Value<i32> = Rc::new(RefCell::new(0));
    let y_max: Value<i32> = Rc::new(RefCell::new(0));
    if (n_points > 0_u32) {
        (*x_min.borrow_mut()) = { (*elem!(points, 0).upgrade().deref()).x };
        (*x_max.borrow_mut()) = { (*elem!(points, 0).upgrade().deref()).x };
        (*y_min.borrow_mut()) = { (*elem!(points, 0).upgrade().deref()).y };
        (*y_max.borrow_mut()) = { (*elem!(points, 0).upgrade().deref()).y };
    }
    let mut i: u32 = 1_u32;
    'loop_: while (i < n_points) {
        let x: Value<i32> = Rc::new(RefCell::new({ (*elem!(points, i).upgrade().deref()).x }));
        let y: Value<i32> = Rc::new(RefCell::new({ (*elem!(points, i).upgrade().deref()).y }));
        let __rhs = (if x.as_pointer().read() <= x_min.as_pointer().read() {
            x.as_pointer()
        } else {
            x_min.as_pointer()
        }
        .read());
        (*x_min.borrow_mut()) = __rhs;
        let __rhs = (if x.as_pointer().read() >= x_max.as_pointer().read() {
            x.as_pointer()
        } else {
            x_max.as_pointer()
        }
        .read());
        (*x_max.borrow_mut()) = __rhs;
        let __rhs = (if y.as_pointer().read() <= y_min.as_pointer().read() {
            y.as_pointer()
        } else {
            y_min.as_pointer()
        }
        .read());
        (*y_min.borrow_mut()) = __rhs;
        let __rhs = (if y.as_pointer().read() >= y_max.as_pointer().read() {
            y.as_pointer()
        } else {
            y_max.as_pointer()
        }
        .read());
        (*y_max.borrow_mut()) = __rhs;
        i.prefix_inc();
    }
    let mut offset: usize = 2_usize;
    let __rhs = ({ Store16_32((dst).clone(), offset, (*x_min.borrow())) });
    offset = __rhs;
    let __rhs = ({ Store16_32((dst).clone(), offset, (*y_min.borrow())) });
    offset = __rhs;
    let __rhs = ({ Store16_32((dst).clone(), offset, (*x_max.borrow())) });
    offset = __rhs;
    let __rhs = ({ Store16_32((dst).clone(), offset, (*y_max.borrow())) });
    offset = __rhs;
}
pub fn SizeOfComposite_60(
    composite_stream: woff2_Buffer,
    mut size: Ptr<usize>,
    mut have_instructions: Ptr<bool>,
) -> bool {
    let composite_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(composite_stream));
    let mut start_offset: usize = ({ woff2_BufferImpl::offset(&composite_stream.as_pointer()) });
    let mut we_have_instructions: bool = false;
    let flags: Value<u16> = Rc::new(RefCell::new(
        (FLAG_MORE_COMPONENTS_45.with(|rc| *rc.borrow()) as u16),
    ));
    'loop_: while ((((*flags.borrow()) as i32) & FLAG_MORE_COMPONENTS_45.with(|rc| *rc.borrow()))
        != 0)
    {
        if ((!({ woff2_BufferImpl::ReadU16(&composite_stream.as_pointer(), (flags.as_pointer())) })
            as i64)
            != 0)
        {
            return false;
        }
        we_have_instructions = {
            ((we_have_instructions as i32)
                | (((((*flags.borrow()) as i32)
                    & FLAG_WE_HAVE_INSTRUCTIONS_48.with(|rc| *rc.borrow()))
                    != 0) as i32))
                != 0
        };
        let mut arg_size: usize = 2_usize;
        if ((((*flags.borrow()) as i32) & FLAG_ARG_1_AND_2_ARE_WORDS_43.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(4_usize) };
        } else {
            arg_size = { (arg_size).wrapping_add(2_usize) };
        }
        if ((((*flags.borrow()) as i32) & FLAG_WE_HAVE_A_SCALE_44.with(|rc| *rc.borrow())) != 0) {
            arg_size = { (arg_size).wrapping_add(2_usize) };
        } else if ((((*flags.borrow()) as i32)
            & FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(4_usize) };
        } else if ((((*flags.borrow()) as i32)
            & FLAG_WE_HAVE_A_TWO_BY_TWO_47.with(|rc| *rc.borrow()))
            != 0)
        {
            arg_size = { (arg_size).wrapping_add(8_usize) };
        }
        if ((!({ woff2_BufferImpl::Skip(&composite_stream.as_pointer(), arg_size) }) as i64) != 0) {
            return false;
        }
    }
    let __rhs =
        ({ woff2_BufferImpl::offset(&composite_stream.as_pointer()) }).wrapping_sub(start_offset);
    size.write(__rhs);
    have_instructions.write({ we_have_instructions });
    return true;
}
pub fn Pad4_61(mut out: PtrDyn<dyn woff2_WOFF2Out>) -> bool {
    let zeroes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([0_u8, 0_u8, 0_u8])));
    if (((({ (*out.upgrade().deref_mut()).Size() }).wrapping_add(3_usize)
        < ({ (*out.upgrade().deref_mut()).Size() })) as i64)
        != 0)
    {
        return false;
    }
    let mut pad_bytes: u32 = ((({ Round4_29((({ (*out.upgrade().deref_mut()).Size() }) as u64)) })
        .wrapping_sub((({ (*out.upgrade().deref_mut()).Size() }) as u64)))
        as u32);
    if (pad_bytes > 0_u32) {
        if ((!({
            (*out.upgrade().deref_mut()).Write_2(
                ((zeroes.as_pointer()) as Ptr<u8>).to_any(),
                (pad_bytes as usize),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    return true;
}
pub fn StoreLoca_62(
    loca_values: Ptr<Vec<u32>>,
    mut index_format: i32,
    mut checksum: Ptr<u32>,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let mut loca_size: u64 = ((*loca_values.upgrade().deref()).len() as u64);
    let mut offset_size: u64 = (if (index_format != 0) { 4 } else { 2 } as u64);
    if (((((loca_size << 2) >> 2) != loca_size) as i64) != 0) {
        return false;
    }
    let loca_content: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((loca_size).wrapping_mul(offset_size) as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let mut dst: Ptr<u8> = ((loca_content.as_pointer() as Ptr<u8>).offset(0_usize));
    let mut offset: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*loca_values.upgrade().deref()).len() }) {
        let mut value: u32 =
            (elem!((Ptr::<Vec<u32>>::decay(&(loca_values)) as Ptr<u32>), i).read());
        if (index_format != 0) {
            let __rhs = ({ StoreU32_31((dst).clone(), offset, value) });
            offset = __rhs;
        } else {
            let __rhs = ({ Store16_32((dst).clone(), offset, ((value >> 1) as i32)) });
            offset = __rhs;
        }
        i.prefix_inc();
    }
    let __rhs = ({
        let _buf: Ptr<u8> = ((loca_content.as_pointer() as Ptr<u8>).offset(0_usize));
        let _size: usize = (*loca_content.borrow()).len();
        ComputeULongSum_26(_buf, _size)
    });
    checksum.write(__rhs);
    if ((!({
        let _buf: AnyPtr =
            (((loca_content.as_pointer() as Ptr<u8>).offset(0_usize)) as Ptr<u8>).to_any();
        let _n: usize = (*loca_content.borrow()).len();
        (*out.upgrade().deref_mut()).Write_2(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReconstructGlyf_63(
    mut data: Ptr<u8>,
    mut glyf_table: Ptr<woff2_Table>,
    mut glyf_checksum: Ptr<u32>,
    mut loca_table: Ptr<woff2_Table>,
    mut loca_checksum: Ptr<u32>,
    mut info: Ptr<woff2_WOFF2FontInfo>,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    thread_local!(
        static kNumSubStreams_64: Value<i32> = Rc::new(RefCell::new(7));
    );
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, {
        (glyf_table.with(|__s| __s.transform_length) as usize)
    })));
    let version: Value<u16> = Rc::new(RefCell::new(0_u16));
    let substreams: Value<Vec<(Value<Ptr<u8>>, Value<u64>)>> = Rc::new(RefCell::new(
        (0..(kNumSubStreams_64.with(|rc| *rc.borrow()) as usize) as usize)
            .map(|_| <(Value<Ptr<u8>>, Value<u64>)>::default())
            .collect::<Vec<_>>(),
    ));
    let mut glyf_start: usize = ({ (*out.upgrade().deref_mut()).Size() });
    if ((!({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (version.as_pointer())) }) as i64) != 0)
    {
        return false;
    }
    let flags: Value<u16> = Rc::new(RefCell::new(0_u16));
    if ((!({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (flags.as_pointer())) }) as i64) != 0) {
        return false;
    }
    let mut has_overlap_bitmap: bool =
        ((((*flags.borrow()) as i32) & FLAG_OVERLAP_SIMPLE_BITMAP_49.with(|rc| *rc.borrow())) != 0);
    if ((((!({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (field_ptr!(info, num_glyphs))) }))
        || (!({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (field_ptr!(info, index_format))) })))
        as i64)
        != 0)
    {
        return false;
    }
    let mut expected_loca_dst_length: u32 = ((if (info.with(|__s| __s.index_format) != 0) {
        4
    } else {
        2
    }) as u32)
        .wrapping_mul(((info.with(|__s| __s.num_glyphs) as u32).wrapping_add(1_u32)));
    if ((({ loca_table.with(|__s| __s.dst_length) } != { expected_loca_dst_length }) as i64) != 0) {
        return false;
    }
    let mut offset: u32 = (((2 + kNumSubStreams_64.with(|rc| *rc.borrow())) * 4) as u32);
    if ((({ offset } > { glyf_table.with(|__s| __s.transform_length) }) as i64) != 0) {
        return false;
    }
    let mut i: i32 = 0;
    'loop_: while (i < kNumSubStreams_64.with(|rc| *rc.borrow())) {
        let substream_size: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (substream_size.as_pointer())) })
            as i64)
            != 0)
        {
            return false;
        }
        if ((({ (*substream_size.borrow()) } > {
            (glyf_table.with(|__s| __s.transform_length)).wrapping_sub(offset)
        }) as i64)
            != 0)
        {
            return false;
        }
        let __rhs = (
            Rc::new(RefCell::new(
                data.offset((offset) as isize)
                    .try_into()
                    .expect("failed conversion"),
            )),
            Rc::new(RefCell::new(
                (*substream_size.borrow())
                    .try_into()
                    .expect("failed conversion"),
            )),
        );
        (*substreams.borrow_mut())[(i as usize)] = __rhs;
        offset = { (offset).wrapping_add((*substream_size.borrow())) };
        i.prefix_inc();
    }
    let n_contour_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[0_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[0_usize].1.borrow()) as usize) },
    )));
    let n_points_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[1_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[1_usize].1.borrow()) as usize) },
    )));
    let flag_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[2_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[2_usize].1.borrow()) as usize) },
    )));
    let glyph_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[3_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[3_usize].1.borrow()) as usize) },
    )));
    let composite_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[4_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[4_usize].1.borrow()) as usize) },
    )));
    let bbox_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[5_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[5_usize].1.borrow()) as usize) },
    )));
    let instruction_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (*(*substreams.borrow())[6_usize].0.borrow()).clone() },
        { ((*(*substreams.borrow())[6_usize].1.borrow()) as usize) },
    )));
    let mut overlap_bitmap: Ptr<u8> = Ptr::<u8>::null();
    let mut overlap_bitmap_length: u32 = 0_u32;
    if has_overlap_bitmap {
        overlap_bitmap_length = ((((info.with(|__s| __s.num_glyphs) as i32) + 7) >> 3) as u32);
        overlap_bitmap = data.offset((offset) as isize);
        if ((({ overlap_bitmap_length } > {
            (glyf_table.with(|__s| __s.transform_length)).wrapping_sub(offset)
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    let loca_values: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(((info.with(|__s| __s.num_glyphs) as i32) + 1) as usize) as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let mut n_points_vec: Vec<u32> = Vec::new();
    let points: Value<Option<Value<Box<[woff2_Point]>>>> = Rc::new(RefCell::new(None));
    let mut points_size: usize = 0_usize;
    let mut bbox_bitmap: Ptr<u8> = ({ woff2_BufferImpl::buffer(&bbox_stream.as_pointer()) });
    let mut bitmap_length: u32 =
        (((((info.with(|__s| __s.num_glyphs) as i32) + 31) >> 5) << 2) as u32);
    if !({ woff2_BufferImpl::Skip(&bbox_stream.as_pointer(), (bitmap_length as usize)) }) {
        return false;
    }
    let mut glyph_buf_size: usize = kDefaultGlyphBuf_53.with(|rc| *rc.borrow());
    let glyph_buf: Value<Option<Value<Box<[u8]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array((0..glyph_buf_size).map(|_| 0_u8).collect::<Box<[u8]>>()).to_owned_opt(),
    ));
    {
        let __a0 = (info.with(|__s| __s.num_glyphs) as usize) as usize;
        (*info.with(|__s| __s.x_mins.clone()).borrow_mut()).resize_with(__a0, || <i16>::default())
    };
    let mut i: u32 = 0_u32;
    'loop_: while ({ i } < { (info.with(|__s| __s.num_glyphs) as u32) }) {
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        let n_contours: Value<u16> = Rc::new(RefCell::new(0_u16));
        let mut have_bbox: bool = false;
        if (({ ((elem!(bbox_bitmap, (i >> 3)).read()) as i32) } & { (128 >> (i & 7_u32)) }) != 0) {
            have_bbox = true;
        }
        if ((!({
            woff2_BufferImpl::ReadU16(&n_contour_stream.as_pointer(), (n_contours.as_pointer()))
        }) as i64)
            != 0)
        {
            return false;
        }
        if (((*n_contours.borrow()) as i32) == 65535) {
            let have_instructions: Value<bool> = Rc::new(RefCell::new(false));
            let instruction_size: Value<u32> = Rc::new(RefCell::new(0_u32));
            if ((!(have_bbox) as i64) != 0) {
                return false;
            }
            let composite_size: Value<usize> = Rc::new(RefCell::new(0_usize));
            if ((!({
                SizeOfComposite_60(
                    (*composite_stream.borrow()).clone(),
                    (composite_size.as_pointer()),
                    (have_instructions.as_pointer()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if (*have_instructions.borrow()) {
                if ((!({
                    Read255UShort_12((glyph_stream.as_pointer()), (instruction_size.as_pointer()))
                }) as i64)
                    != 0)
                {
                    return false;
                }
            }
            let mut size_needed: usize = ((12_usize).wrapping_add((*composite_size.borrow())))
                .wrapping_add(((*instruction_size.borrow()) as usize));
            if (((glyph_buf_size < size_needed) as i64) != 0) {
                (glyph_buf.as_pointer() as Ptr<Option<Value<Box<[u8]>>>>).write(
                    Ptr::alloc_array((0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>())
                        .to_owned_opt(),
                );
                glyph_buf_size = size_needed;
            }
            let __rhs = ({
                Store16_32(
                    (*glyph_buf.borrow()).as_pointer(),
                    (*glyph_size.borrow()),
                    ((*n_contours.borrow()) as i32),
                )
            });
            (*glyph_size.borrow_mut()) = __rhs;
            if ((!({
                woff2_BufferImpl::Read(
                    &bbox_stream.as_pointer(),
                    (*glyph_buf.borrow())
                        .as_pointer()
                        .offset((*glyph_size.borrow()) as isize),
                    8_usize,
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            (*glyph_size.borrow_mut()) = { (*glyph_size.borrow()).wrapping_add(8_usize) };
            if ((!({
                woff2_BufferImpl::Read(
                    &composite_stream.as_pointer(),
                    (*glyph_buf.borrow())
                        .as_pointer()
                        .offset((*glyph_size.borrow()) as isize),
                    (*composite_size.borrow()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            (*glyph_size.borrow_mut()) =
                { (*glyph_size.borrow()).wrapping_add((*composite_size.borrow())) };
            if (*have_instructions.borrow()) {
                let __rhs = ({
                    Store16_32(
                        (*glyph_buf.borrow()).as_pointer(),
                        (*glyph_size.borrow()),
                        ((*instruction_size.borrow()) as i32),
                    )
                });
                (*glyph_size.borrow_mut()) = __rhs;
                if ((!({
                    woff2_BufferImpl::Read(
                        &instruction_stream.as_pointer(),
                        (*glyph_buf.borrow())
                            .as_pointer()
                            .offset((*glyph_size.borrow()) as isize),
                        ((*instruction_size.borrow()) as usize),
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
                (*glyph_size.borrow_mut()) = {
                    (*glyph_size.borrow()).wrapping_add(((*instruction_size.borrow()) as usize))
                };
            }
        } else if (((*n_contours.borrow()) as i32) > 0) {
            n_points_vec.clear();
            let mut total_n_points: u32 = 0_u32;
            let n_points_contour: Value<u32> = Rc::new(RefCell::new(0_u32));
            let mut j: u32 = 0_u32;
            'loop_: while (j < ((*n_contours.borrow()) as u32)) {
                if ((!({
                    Read255UShort_12(
                        (n_points_stream.as_pointer()),
                        (n_points_contour.as_pointer()),
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
                {
                    let a0_clone = (*n_points_contour.borrow()).clone();
                    n_points_vec.push(a0_clone)
                };
                if ((((total_n_points).wrapping_add((*n_points_contour.borrow())) < total_n_points)
                    as i64)
                    != 0)
                {
                    return false;
                }
                total_n_points = { (total_n_points).wrapping_add((*n_points_contour.borrow())) };
                j.prefix_inc();
            }
            let mut flag_size: u32 = total_n_points;
            if ((((flag_size as usize)
                > ({ woff2_BufferImpl::length(&flag_stream.as_pointer()) })
                    .wrapping_sub(({ woff2_BufferImpl::offset(&flag_stream.as_pointer()) })))
                as i64)
                != 0)
            {
                return false;
            }
            let mut flags_buf: Ptr<u8> = ({ woff2_BufferImpl::buffer(&flag_stream.as_pointer()) })
                .offset(({ woff2_BufferImpl::offset(&flag_stream.as_pointer()) }) as isize);
            let mut triplet_buf: Ptr<u8> =
                ({ woff2_BufferImpl::buffer(&glyph_stream.as_pointer()) })
                    .offset(({ woff2_BufferImpl::offset(&glyph_stream.as_pointer()) }) as isize);
            let mut triplet_size: usize =
                ({ woff2_BufferImpl::length(&glyph_stream.as_pointer()) })
                    .wrapping_sub(({ woff2_BufferImpl::offset(&glyph_stream.as_pointer()) }));
            let triplet_bytes_consumed: Value<usize> = Rc::new(RefCell::new(0_usize));
            if (points_size < (total_n_points as usize)) {
                points_size = (total_n_points as usize);
                (points.as_pointer() as Ptr<Option<Value<Box<[woff2_Point]>>>>).write(
                    Ptr::alloc_array(
                        (0..points_size)
                            .map(|_| <woff2_Point>::default())
                            .collect::<Box<[woff2_Point]>>(),
                    )
                    .to_owned_opt(),
                );
            }
            if ((!({
                TripletDecode_57(
                    (flags_buf).clone(),
                    (triplet_buf).clone(),
                    triplet_size,
                    total_n_points,
                    (*points.borrow()).as_pointer(),
                    (triplet_bytes_consumed.as_pointer()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((!({ woff2_BufferImpl::Skip(&flag_stream.as_pointer(), (flag_size as usize)) })
                as i64)
                != 0)
            {
                return false;
            }
            if ((!({
                woff2_BufferImpl::Skip(
                    &glyph_stream.as_pointer(),
                    (*triplet_bytes_consumed.borrow()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            let instruction_size: Value<u32> = Rc::new(RefCell::new(0_u32));
            if ((!({
                Read255UShort_12((glyph_stream.as_pointer()), (instruction_size.as_pointer()))
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((((total_n_points >= ((1 << 27) as u32))
                || ((*instruction_size.borrow()) >= ((1 << 30) as u32))) as i64)
                != 0)
            {
                return false;
            }
            let mut size_needed: usize = (((((12 + (2 * ((*n_contours.borrow()) as i32))) as u32)
                .wrapping_add((5_u32).wrapping_mul(total_n_points)))
            .wrapping_add((*instruction_size.borrow())))
                as usize);
            if (((glyph_buf_size < size_needed) as i64) != 0) {
                (glyph_buf.as_pointer() as Ptr<Option<Value<Box<[u8]>>>>).write(
                    Ptr::alloc_array((0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>())
                        .to_owned_opt(),
                );
                glyph_buf_size = size_needed;
            }
            let __rhs = ({
                Store16_32(
                    (*glyph_buf.borrow()).as_pointer(),
                    (*glyph_size.borrow()),
                    ((*n_contours.borrow()) as i32),
                )
            });
            (*glyph_size.borrow_mut()) = __rhs;
            if have_bbox {
                if ((!({
                    woff2_BufferImpl::Read(
                        &bbox_stream.as_pointer(),
                        (*glyph_buf.borrow())
                            .as_pointer()
                            .offset((*glyph_size.borrow()) as isize),
                        8_usize,
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                ({
                    ComputeBbox_59(
                        total_n_points,
                        (*points.borrow()).as_pointer(),
                        (*glyph_buf.borrow()).as_pointer(),
                    )
                });
            }
            (*glyph_size.borrow_mut()) = kEndPtsOfContoursOffset_51.with(|rc| *rc.borrow());
            let mut end_point: i32 = -1_i32;
            let mut contour_ix: u32 = 0_u32;
            'loop_: while (contour_ix < ((*n_contours.borrow()) as u32)) {
                {
                    let rhs_0 = ((end_point as u32)
                        .wrapping_add(n_points_vec[(contour_ix as usize)]))
                        as i32;
                    end_point = rhs_0
                };
                if (((end_point >= 65536) as i64) != 0) {
                    return false;
                }
                let __rhs = ({
                    Store16_32(
                        (*glyph_buf.borrow()).as_pointer(),
                        (*glyph_size.borrow()),
                        end_point,
                    )
                });
                (*glyph_size.borrow_mut()) = __rhs;
                contour_ix.prefix_inc();
            }
            let __rhs = ({
                Store16_32(
                    (*glyph_buf.borrow()).as_pointer(),
                    (*glyph_size.borrow()),
                    ((*instruction_size.borrow()) as i32),
                )
            });
            (*glyph_size.borrow_mut()) = __rhs;
            if ((!({
                woff2_BufferImpl::Read(
                    &instruction_stream.as_pointer(),
                    (*glyph_buf.borrow())
                        .as_pointer()
                        .offset((*glyph_size.borrow()) as isize),
                    ((*instruction_size.borrow()) as usize),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            (*glyph_size.borrow_mut()) =
                { (*glyph_size.borrow()).wrapping_add(((*instruction_size.borrow()) as usize)) };
            let mut has_overlap_bit: bool = (has_overlap_bitmap)
                && (({ ((elem!(overlap_bitmap, (i >> 3)).read()) as i32) } & {
                    (128 >> (i & 7_u32))
                }) != 0);
            if ((!({
                StorePoints_58(
                    total_n_points,
                    (*points.borrow()).as_pointer(),
                    ((*n_contours.borrow()) as u32),
                    (*instruction_size.borrow()),
                    has_overlap_bit,
                    (*glyph_buf.borrow()).as_pointer(),
                    glyph_buf_size,
                    (glyph_size.as_pointer()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            if ((have_bbox as i64) != 0) {
                eprintln!("Empty glyph has a bbox");
                return false;
            }
        }
        let __rhs = ((({ (*out.upgrade().deref_mut()).Size() }).wrapping_sub(glyf_start)) as u32);
        (*loca_values.borrow_mut())[(i as usize)] = __rhs;
        if ((!({
            (*out.upgrade().deref_mut()).Write_2(
                ((*glyph_buf.borrow()).as_pointer() as Ptr<u8>).to_any(),
                (*glyph_size.borrow()),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((!({ Pad4_61((out).clone()) }) as i64) != 0) {
            return false;
        }
        {
            let rhs_0 = (glyf_checksum.read()).wrapping_add(
                ({
                    ComputeULongSum_26((*glyph_buf.borrow()).as_pointer(), (*glyph_size.borrow()))
                }),
            );
            glyf_checksum.write(rhs_0)
        };
        if (((*n_contours.borrow()) as i32) > 0) {
            let x_min_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
                { (*glyph_buf.borrow()).as_pointer().offset((2) as isize) },
                { 2_usize },
            )));
            if ((!({
                woff2_BufferImpl::ReadS16(
                    &x_min_buf.as_pointer(),
                    ((info.with(|__s| __s.x_mins.as_pointer()) as Ptr<i16>).offset((i as usize))),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        }
        i.prefix_inc();
    }
    let __rhs = ((({ (*out.upgrade().deref_mut()).Size() })
        .wrapping_sub((glyf_table.with(|__s| __s.dst_offset) as usize))) as u32);
    field!(glyf_table, dst_length).write(__rhs);
    let __rhs = (({ (*out.upgrade().deref_mut()).Size() }) as u32);
    field!(loca_table, dst_offset).write(__rhs);
    elem!(
        (loca_values.as_pointer() as Ptr<u32>),
        (info.with(|__s| __s.num_glyphs) as usize)
    )
    .write(glyf_table.with(|__s| __s.dst_length));
    if ((!({
        StoreLoca_62(
            loca_values.as_pointer(),
            (info.with(|__s| __s.index_format) as i32),
            (loca_checksum).clone(),
            (out).clone(),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    let __rhs = ((({ (*out.upgrade().deref_mut()).Size() })
        .wrapping_sub((loca_table.with(|__s| __s.dst_offset) as usize))) as u32);
    field!(loca_table, dst_length).write(__rhs);
    return true;
}
pub fn FindTable_65(mut tables: Ptr<Vec<Ptr<woff2_Table>>>, mut tag: u32) -> Ptr<woff2_Table> {
    'loop_: for mut table in Ptr::<Vec<Ptr<woff2_Table>>>::decay(&(tables)) as Ptr<Ptr<woff2_Table>>
    {
        let mut table: Ptr<woff2_Table> = table.read();
        if ({ table.with(|__s| __s.tag) } == { tag }) {
            return table;
        }
    }
    return Ptr::<woff2_Table>::null();
}
pub fn ReadNumHMetrics_66(
    mut data: Ptr<u8>,
    mut data_size: usize,
    mut num_hmetrics: Ptr<u16>,
) -> bool {
    let buffer: Value<woff2_Buffer> =
        Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, {
            data_size
        })));
    if ((((!({ woff2_BufferImpl::Skip(&buffer.as_pointer(), 34_usize) }))
        || (!({ woff2_BufferImpl::ReadU16(&buffer.as_pointer(), (num_hmetrics).clone()) })))
        as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReconstructTransformedHmtx_67(
    mut transformed_buf: Ptr<u8>,
    mut transformed_size: usize,
    mut num_glyphs: u16,
    mut num_hmetrics: u16,
    x_mins: Ptr<Vec<i16>>,
    mut checksum: Ptr<u32>,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let hmtx_buff_in: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new(
        { (transformed_buf).clone() },
        { transformed_size },
    )));
    let hmtx_flags: Value<u8> = Rc::new(RefCell::new(0_u8));
    if ((!({ woff2_BufferImpl::ReadU8(&hmtx_buff_in.as_pointer(), (hmtx_flags.as_pointer())) })
        as i64)
        != 0)
    {
        return false;
    }
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut lsbs: Vec<i16> = Vec::new();
    let mut has_proportional_lsbs: bool = ((((*hmtx_flags.borrow()) as i32) & 1) == 0);
    let mut has_monospace_lsbs: bool = ((((*hmtx_flags.borrow()) as i32) & 2) == 0);
    if ((((*hmtx_flags.borrow()) as i32) & 252) != 0) {
        eprintln!("Illegal hmtx flags; bits 2-7 must be 0");
        return false;
    }
    if (has_proportional_lsbs) && (has_monospace_lsbs) {
        return false;
    }
    assert!(({ (*x_mins.upgrade().deref()).len() } == { (num_glyphs as usize) }));
    if ((((num_hmetrics as i32) > (num_glyphs as i32)) as i64) != 0) {
        return false;
    }
    if ((((num_hmetrics as i32) < 1) as i64) != 0) {
        return false;
    }
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < (num_hmetrics as i32)) {
        let advance_width: Value<u16> = Rc::new(RefCell::new(0_u16));
        if ((!({
            woff2_BufferImpl::ReadU16(&hmtx_buff_in.as_pointer(), (advance_width.as_pointer()))
        }) as i64)
            != 0)
        {
            return false;
        }
        {
            let a0_clone = (*advance_width.borrow()).clone();
            advance_widths.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < (num_hmetrics as i32)) {
        let lsb: Value<i16> = Rc::new(RefCell::new(0_i16));
        if has_proportional_lsbs {
            if ((!({ woff2_BufferImpl::ReadS16(&hmtx_buff_in.as_pointer(), (lsb.as_pointer())) })
                as i64)
                != 0)
            {
                return false;
            }
        } else {
            (*lsb.borrow_mut()) = (elem!(
                (Ptr::<Vec<i16>>::decay(&(x_mins)) as Ptr<i16>),
                (i as usize)
            )
            .read());
        }
        {
            let a0_clone = (*lsb.borrow()).clone();
            lsbs.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut i: u16 = num_hmetrics;
    'loop_: while ((i as i32) < (num_glyphs as i32)) {
        let lsb: Value<i16> = Rc::new(RefCell::new(0_i16));
        if has_monospace_lsbs {
            if ((!({ woff2_BufferImpl::ReadS16(&hmtx_buff_in.as_pointer(), (lsb.as_pointer())) })
                as i64)
                != 0)
            {
                return false;
            }
        } else {
            (*lsb.borrow_mut()) = (elem!(
                (Ptr::<Vec<i16>>::decay(&(x_mins)) as Ptr<i16>),
                (i as usize)
            )
            .read());
        }
        {
            let a0_clone = (*lsb.borrow()).clone();
            lsbs.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut hmtx_output_size: u32 =
        (((2 * (num_glyphs as i32)) + (2 * (num_hmetrics as i32))) as u32);
    let hmtx_table: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(hmtx_output_size as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let mut dst: Ptr<u8> = ((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize));
    let dst_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut i: u32 = 0_u32;
    'loop_: while (i < (num_glyphs as u32)) {
        if (i < (num_hmetrics as u32)) {
            ({
                Store16_34(
                    (advance_widths[(i as usize)] as i32),
                    (dst_offset.as_pointer()),
                    (dst).clone(),
                )
            });
        }
        ({
            Store16_34(
                (lsbs[(i as usize)] as i32),
                (dst_offset.as_pointer()),
                (dst).clone(),
            )
        });
        i.postfix_inc();
    }
    let __rhs = ({
        ComputeULongSum_26(
            ((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize)),
            (hmtx_output_size as usize),
        )
    });
    checksum.write(__rhs);
    if ((!({
        (*out.upgrade().deref_mut()).Write_2(
            (((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize)) as Ptr<u8>).to_any(),
            (hmtx_output_size as usize),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn Woff2Uncompress_68(
    mut dst_buf: Ptr<u8>,
    mut dst_size: usize,
    mut src_buf: Ptr<u8>,
    mut src_size: usize,
) -> bool {
    let uncompressed_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let mut result: ::brotli_sys::BrotliDecoderResult = {
        let __out_len = (uncompressed_size.as_pointer()).read();
        src_buf.with_slice(src_size, |__in| {
            (uncompressed_size.as_pointer()).with_mut(|_v2| {
                dst_buf.with_slice_mut(__out_len, |__out| unsafe {
                    ::brotli_sys::BrotliDecoderDecompress(
                        src_size,
                        __in.as_ptr(),
                        _v2 as *mut usize,
                        __out.as_mut_ptr(),
                    )
                })
            })
        })
    };
    if (((((result as i32) != (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32))
        || ((*uncompressed_size.borrow()) != dst_size)) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReadTableDirectory_69(
    mut file: Ptr<woff2_Buffer>,
    mut tables: Ptr<Vec<woff2_Table>>,
    mut num_tables: usize,
) -> bool {
    let mut src_offset: u32 = 0_u32;
    let mut i: usize = 0_usize;
    'loop_: while (i < num_tables) {
        let mut table: Ptr<woff2_Table> =
            (((Ptr::<Vec<woff2_Table>>::decay(&(tables))) as Ptr<woff2_Table>).offset(i));
        let flag_byte: Value<u8> = Rc::new(RefCell::new(0_u8));
        if ((!({ woff2_BufferImpl::ReadU8(&file, (flag_byte.as_pointer())) }) as i64) != 0) {
            return false;
        }
        let tag: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((((*flag_byte.borrow()) as i32) & 63) == 63) {
            if ((!({ woff2_BufferImpl::ReadU32(&file, (tag.as_pointer())) }) as i64) != 0) {
                return false;
            }
        } else {
            (*tag.borrow_mut()) = ({
                let __idx = (((*flag_byte.borrow()) as i32) & 63) as usize;
                kKnownTags_8.with(|rc| rc.borrow()[__idx])
            });
        }
        let mut flags: u32 = 0_u32;
        let mut xform_version: u8 = (((((*flag_byte.borrow()) as i32) >> 6) & 3) as u8);
        if ((*tag.borrow()) == kGlyfTableTag_0.with(|rc| *rc.borrow()))
            || ((*tag.borrow()) == kLocaTableTag_2.with(|rc| *rc.borrow()))
        {
            if ((xform_version as i32) == 0) {
                flags |= kWoff2FlagsTransform_21.with(|rc| *rc.borrow());
            }
        } else if ((xform_version as i32) != 0) {
            flags |= kWoff2FlagsTransform_21.with(|rc| *rc.borrow());
        }
        flags |= (xform_version as u32);
        let dst_length: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((!({ ReadBase128_17((file).clone(), (dst_length.as_pointer())) }) as i64) != 0) {
            return false;
        }
        let transform_length: Value<u32> = Rc::new(RefCell::new((*dst_length.borrow())));
        if ((flags & kWoff2FlagsTransform_21.with(|rc| *rc.borrow())) != 0_u32) {
            if ((!({ ReadBase128_17((file).clone(), (transform_length.as_pointer())) }) as i64)
                != 0)
            {
                return false;
            }
            if (((((*tag.borrow()) == kLocaTableTag_2.with(|rc| *rc.borrow()))
                && ((*transform_length.borrow()) != 0)) as i64)
                != 0)
            {
                return false;
            }
        }
        if ((((src_offset).wrapping_add((*transform_length.borrow())) < src_offset) as i64) != 0) {
            return false;
        }
        field!(table, src_offset).write(src_offset);
        field!(table, src_length).write((*transform_length.borrow()));
        src_offset = { (src_offset).wrapping_add((*transform_length.borrow())) };
        field!(table, tag).write((*tag.borrow()));
        field!(table, flags).write(flags);
        field!(table, transform_length).write((*transform_length.borrow()));
        field!(table, dst_length).write((*dst_length.borrow()));
        i.prefix_inc();
    }
    return true;
}
pub fn StoreOffsetTable_70(
    mut result: Ptr<u8>,
    mut offset: usize,
    mut flavor: u32,
    mut num_tables: u16,
) -> usize {
    let __rhs = ({ StoreU32_31((result).clone(), offset, flavor) });
    offset = __rhs;
    let __rhs = ({ Store16_32((result).clone(), offset, (num_tables as i32)) });
    offset = __rhs;
    let mut max_pow2: u32 = 0_u32;
    'loop_: while ((1_u32 << ((max_pow2).wrapping_add(1_u32))) <= (num_tables as u32)) {
        max_pow2.postfix_inc();
    }
    let mut output_search_range: u16 = (((1_u32 << max_pow2) << 4) as u16);
    let __rhs = ({ Store16_32((result).clone(), offset, (output_search_range as i32)) });
    offset = __rhs;
    let __rhs = ({ Store16_32((result).clone(), offset, (max_pow2 as i32)) });
    offset = __rhs;
    let __rhs = ({
        Store16_32(
            (result).clone(),
            offset,
            (((num_tables as i32) << 4) - (output_search_range as i32)),
        )
    });
    offset = __rhs;
    return offset;
}
pub fn StoreTableEntry_71(mut result: Ptr<u8>, mut offset: u32, mut tag: u32) -> usize {
    let __rhs = (({ StoreU32_31((result).clone(), (offset as usize), tag) }) as u32);
    offset = __rhs;
    let __rhs = (({ StoreU32_31((result).clone(), (offset as usize), 0_u32) }) as u32);
    offset = __rhs;
    let __rhs = (({ StoreU32_31((result).clone(), (offset as usize), 0_u32) }) as u32);
    offset = __rhs;
    let __rhs = (({ StoreU32_31((result).clone(), (offset as usize), 0_u32) }) as u32);
    offset = __rhs;
    return (offset as usize);
}
pub fn ComputeOffsetToFirstTable_72(hdr: Ptr<woff2_WOFF2Header>) -> u64 {
    let mut offset: u64 = (kSfntHeaderSize_23.with(|rc| *rc.borrow()) as u64).wrapping_add(
        (kSfntEntrySize_24.with(|rc| *rc.borrow()) as u64)
            .wrapping_mul((hdr.with(|__s| __s.num_tables) as u64)),
    );
    if (hdr.with(|__s| __s.header_version) != 0) {
        offset = (({
            let _header_version: u32 = hdr.with(|__s| __s.header_version);
            let _num_fonts: u32 = ((*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() as u32);
            CollectionHeaderSize_27(_header_version, _num_fonts)
        }) as u64)
            .wrapping_add(
                (kSfntHeaderSize_23.with(|rc| *rc.borrow()) as u64)
                    .wrapping_mul(((*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() as u64)),
            );
        'loop_: for mut ttc_font in hdr.with(|__s| __s.ttc_fonts.as_pointer()) as Ptr<woff2_TtcFont>
        {
            {
                let rhs_0 = (offset).wrapping_add(
                    (kSfntEntrySize_24.with(|rc| *rc.borrow()) as u64).wrapping_mul(
                        ((*ttc_font.with(|__s| __s.table_indices.clone()).borrow()).len() as u64),
                    ),
                );
                offset = rhs_0
            };
        }
    }
    return offset;
}
pub fn Tables_73(mut hdr: Ptr<woff2_WOFF2Header>, mut font_index: usize) -> Vec<Ptr<woff2_Table>> {
    let mut tables: Vec<Ptr<woff2_Table>> = Vec::new();
    if ((hdr.with(|__s| __s.header_version) as i64) != 0) {
        'loop_: for mut index in {
            (*elem!(
                (hdr.with(|__s| __s.ttc_fonts.as_pointer()) as Ptr<woff2_TtcFont>),
                font_index
            )
            .upgrade()
            .deref())
            .table_indices
            .as_pointer()
        } as Ptr<u16>
        {
            let mut index: u16 = index.read();
            {
                let __a1 = ((hdr.with(|__s| __s.tables.as_pointer()) as Ptr<woff2_Table>)
                    .offset((index as usize)));
                tables.push(__a1)
            };
        }
    } else {
        'loop_: for mut table in hdr.with(|__s| __s.tables.as_pointer()) as Ptr<woff2_Table> {
            {
                let __a1 = (table);
                tables.push(__a1)
            };
        }
    }
    return std::mem::take(&mut tables);
}
pub fn ReconstructFont_74(
    mut transformed_buf: Ptr<u8>,
    mut transformed_buf_size: u32,
    mut metadata: Ptr<woff2_RebuildMetadata>,
    mut hdr: Ptr<woff2_WOFF2Header>,
    mut font_index: usize,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let mut dest_offset: usize = ({ (*out.upgrade().deref_mut()).Size() });
    let table_entry: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..12).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let mut info: Ptr<woff2_WOFF2FontInfo> = ((metadata.with(|__s| __s.font_infos.as_pointer())
        as Ptr<woff2_WOFF2FontInfo>)
        .offset(font_index));
    let tables: Value<Vec<Ptr<woff2_Table>>> =
        Rc::new(RefCell::new(({ Tables_73((hdr).clone(), font_index) })));
    let mut glyf_table: Ptr<woff2_Table> = ({
        FindTable_65(
            (tables.as_pointer()),
            kGlyfTableTag_0.with(|rc| *rc.borrow()),
        )
    });
    let mut loca_table: Ptr<woff2_Table> = ({
        FindTable_65(
            (tables.as_pointer()),
            kLocaTableTag_2.with(|rc| *rc.borrow()),
        )
    });
    if ((({ (!(glyf_table).is_null() as i32) } != { (!(loca_table).is_null() as i32) }) as i64)
        != 0)
    {
        eprintln!("Cannot have just one of glyf/loca");
        return false;
    }
    if !((glyf_table).is_null()) {
        if ((({
            ({ glyf_table.with(|__s| __s.flags) } & {
                kWoff2FlagsTransform_21.with(|rc| *rc.borrow())
            })
        } != {
            ({ loca_table.with(|__s| __s.flags) } & {
                kWoff2FlagsTransform_21.with(|rc| *rc.borrow())
            })
        }) as i64)
            != 0)
        {
            eprintln!("Cannot transform just one of glyf/loca");
            return false;
        }
    }
    let mut font_checksum: u32 = metadata.with(|__s| __s.header_checksum);
    if (hdr.with(|__s| __s.header_version) != 0) {
        font_checksum = {
            (*elem!(
                (hdr.with(|__s| __s.ttc_fonts.as_pointer()) as Ptr<woff2_TtcFont>),
                font_index
            )
            .upgrade()
            .deref())
            .header_checksum
        };
    }
    let loca_checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*tables.borrow()).len()) {
        let table: Ptr<woff2_Table> = ((*tables.borrow())[i]).clone();
        let checksum_key: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                table
                    .with(|__s| __s.tag)
                    .try_into()
                    .expect("failed conversion"),
            )),
            Rc::new(RefCell::new(
                table
                    .with(|__s| __s.src_offset)
                    .try_into()
                    .expect("failed conversion"),
            )),
        )));
        let mut reused: bool = RefcountMapIter::find_key(
            (field_ptr!(metadata, checksums)
                as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>),
            &(*checksum_key.borrow()),
        ) != RefcountMapIter::end(
            (field_ptr!(metadata, checksums)
                as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>),
        );
        if ((((font_index == 0_usize) && (reused)) as i64) != 0) {
            return false;
        }
        if ((({
            (table.with(|__s| __s.src_offset) as u64)
                .wrapping_add((table.with(|__s| __s.src_length) as u64))
        } > { (transformed_buf_size as u64) }) as i64)
            != 0)
        {
            return false;
        }
        if ({ table.with(|__s| __s.tag) } == { kHheaTableTag_6.with(|rc| *rc.borrow()) }) {
            if !({
                let _data: Ptr<u8> =
                    transformed_buf.offset((table.with(|__s| __s.src_offset)) as isize);
                let _data_size: usize = (table.with(|__s| __s.src_length) as usize);
                ReadNumHMetrics_66(_data, _data_size, (field_ptr!(info, num_hmetrics)))
            }) {
                return false;
            }
        }
        let checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !(reused) {
            if ({
                ({ table.with(|__s| __s.flags) } & {
                    kWoff2FlagsTransform_21.with(|rc| *rc.borrow())
                })
            } != { kWoff2FlagsTransform_21.with(|rc| *rc.borrow()) })
            {
                if ({ table.with(|__s| __s.tag) } == { kHeadTableTag_1.with(|rc| *rc.borrow()) }) {
                    if (((table.with(|__s| __s.src_length) < 12_u32) as i64) != 0) {
                        return false;
                    }
                    ({
                        StoreU32_31(
                            transformed_buf.offset((table.with(|__s| __s.src_offset)) as isize),
                            8_usize,
                            0_u32,
                        )
                    });
                }
                field!(table, dst_offset).write((dest_offset as u32));
                (*checksum.borrow_mut()) = ({
                    let _buf: Ptr<u8> =
                        transformed_buf.offset((table.with(|__s| __s.src_offset)) as isize);
                    let _size: usize = (table.with(|__s| __s.src_length) as usize);
                    ComputeULongSum_26(_buf, _size)
                });
                if ((!({
                    let _buf: AnyPtr = (transformed_buf
                        .offset((table.with(|__s| __s.src_offset)) as isize)
                        as Ptr<u8>)
                        .to_any();
                    let _n: usize = (table.with(|__s| __s.src_length) as usize);
                    (*out.upgrade().deref_mut()).Write_2(_buf, _n)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                if ({ table.with(|__s| __s.tag) } == { kGlyfTableTag_0.with(|rc| *rc.borrow()) }) {
                    field!(table, dst_offset).write((dest_offset as u32));
                    let mut loca_table: Ptr<woff2_Table> = ({
                        FindTable_65(
                            (tables.as_pointer()),
                            kLocaTableTag_2.with(|rc| *rc.borrow()),
                        )
                    });
                    if ((!({
                        let _data: Ptr<u8> =
                            transformed_buf.offset((table.with(|__s| __s.src_offset)) as isize);
                        let _glyf_table: Ptr<woff2_Table> = (table).clone();
                        ReconstructGlyf_63(
                            _data,
                            _glyf_table,
                            (checksum.as_pointer()),
                            (loca_table).clone(),
                            (loca_checksum.as_pointer()),
                            (info).clone(),
                            (out).clone(),
                        )
                    }) as i64)
                        != 0)
                    {
                        return false;
                    }
                } else if ({ table.with(|__s| __s.tag) } == {
                    kLocaTableTag_2.with(|rc| *rc.borrow())
                }) {
                    (*checksum.borrow_mut()) = (*loca_checksum.borrow());
                } else if ({ table.with(|__s| __s.tag) } == {
                    kHmtxTableTag_5.with(|rc| *rc.borrow())
                }) {
                    field!(table, dst_offset).write((dest_offset as u32));
                    if ((!({
                        let _transformed_buf: Ptr<u8> =
                            transformed_buf.offset((table.with(|__s| __s.src_offset)) as isize);
                        let _transformed_size: usize = (table.with(|__s| __s.src_length) as usize);
                        let _num_glyphs: u16 = info.with(|__s| __s.num_glyphs);
                        let _num_hmetrics: u16 = info.with(|__s| __s.num_hmetrics);
                        let _x_mins: Ptr<Vec<i16>> = info.with(|__s| __s.x_mins.as_pointer());
                        ReconstructTransformedHmtx_67(
                            _transformed_buf,
                            _transformed_size,
                            _num_glyphs,
                            _num_hmetrics,
                            _x_mins,
                            (checksum.as_pointer()),
                            (out).clone(),
                        )
                    }) as i64)
                        != 0)
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            (field_ptr!(metadata, checksums)
                as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u32>>| {
                    __v.entry((*checksum_key.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .write((*checksum.borrow()));
        } else {
            (*checksum.borrow_mut()) = ((field_ptr!(metadata, checksums)
                as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u32>>| {
                    __v.entry((*checksum_key.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .read());
        }
        font_checksum = { (font_checksum).wrapping_add((*checksum.borrow())) };
        ({
            StoreU32_31(
                (table_entry.as_pointer() as Ptr<u8>),
                0_usize,
                (*checksum.borrow()),
            )
        });
        ({
            StoreU32_31(
                (table_entry.as_pointer() as Ptr<u8>),
                4_usize,
                table.with(|__s| __s.dst_offset),
            )
        });
        ({
            StoreU32_31(
                (table_entry.as_pointer() as Ptr<u8>),
                8_usize,
                table.with(|__s| __s.dst_length),
            )
        });
        if ((!({
            (*out.upgrade().deref_mut()).Write_3(
                ((table_entry.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                ((((field_ptr!(info, table_entry_by_tag) as Ptr<BTreeMap<u32, Value<u32>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                        __v.entry(table.with(|__s| __s.tag))
                            .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                            .as_pointer()
                    })
                    .read())
                .wrapping_add(4_u32)) as usize),
                12_usize,
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        {
            let rhs_0 = (font_checksum).wrapping_add(
                ({ ComputeULongSum_26((table_entry.as_pointer() as Ptr<u8>), 12_usize) }),
            );
            font_checksum = rhs_0
        };
        if ((!({ Pad4_61((out).clone()) }) as i64) != 0) {
            return false;
        }
        if ((({
            ((((table.with(|__s| __s.dst_offset)).wrapping_add(table.with(|__s| __s.dst_length)))
                as u64) as usize)
        } > { ({ (*out.upgrade().deref_mut()).Size() }) }) as i64)
            != 0)
        {
            return false;
        }
        dest_offset = ({ (*out.upgrade().deref_mut()).Size() });
        i.postfix_inc();
    }
    let mut head_table: Ptr<woff2_Table> = ({
        FindTable_65(
            (tables.as_pointer()),
            kHeadTableTag_1.with(|rc| *rc.borrow()),
        )
    });
    if !(head_table).is_null() {
        if (((head_table.with(|__s| __s.dst_length) < 12_u32) as i64) != 0) {
            return false;
        }
        let checksum_adjustment: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
        ({
            StoreU32_31(
                (checksum_adjustment.as_pointer() as Ptr<u8>),
                0_usize,
                (2981146554_u32 as u32).wrapping_sub(font_checksum),
            )
        });
        if ((!({
            (*out.upgrade().deref_mut()).Write_3(
                ((checksum_adjustment.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (((head_table.with(|__s| __s.dst_offset)).wrapping_add(8_u32)) as usize),
                4_usize,
            )
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    return true;
}
pub fn ReadWOFF2Header_75(
    mut data: Ptr<u8>,
    mut length: usize,
    mut hdr: Ptr<woff2_WOFF2Header>,
) -> bool {
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, {
        length
    })));
    let signature: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (((((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (signature.as_pointer())) }))
        || ((*signature.borrow()) != kWoff2Signature_20.with(|rc| *rc.borrow())))
        || (!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(hdr, flavor))) })))
        as i64)
        != 0)
    {
        return false;
    }
    let reported_length: Value<u32> = Rc::new(RefCell::new(0_u32));
    if ((((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (reported_length.as_pointer())) }))
        || (length != ((*reported_length.borrow()) as usize))) as i64)
        != 0)
    {
        return false;
    }
    if ((((!({ woff2_BufferImpl::ReadU16(&file.as_pointer(), (field_ptr!(hdr, num_tables))) }))
        || (!(hdr.with(|__s| __s.num_tables) != 0))) as i64)
        != 0)
    {
        return false;
    }
    if ((!({ woff2_BufferImpl::Skip(&file.as_pointer(), 6_usize) }) as i64) != 0) {
        return false;
    }
    if ((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(hdr, compressed_length))) })
        as i64)
        != 0)
    {
        return false;
    }
    if ((!({ woff2_BufferImpl::Skip(&file.as_pointer(), ((2 * 2) as usize)) }) as i64) != 0) {
        return false;
    }
    let meta_offset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let meta_length: Value<u32> = Rc::new(RefCell::new(0_u32));
    let meta_length_orig: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (((((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (meta_offset.as_pointer())) }))
        || (!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (meta_length.as_pointer())) })))
        || (!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (meta_length_orig.as_pointer())) })))
        as i64)
        != 0)
    {
        return false;
    }
    if ((*meta_offset.borrow()) != 0) {
        if ((((((*meta_offset.borrow()) as usize) >= length)
            || ((length).wrapping_sub(((*meta_offset.borrow()) as usize))
                < ((*meta_length.borrow()) as usize))) as i64)
            != 0)
        {
            return false;
        }
    }
    let priv_offset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let priv_length: Value<u32> = Rc::new(RefCell::new(0_u32));
    if ((((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (priv_offset.as_pointer())) }))
        || (!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (priv_length.as_pointer())) })))
        as i64)
        != 0)
    {
        return false;
    }
    if ((*priv_offset.borrow()) != 0) {
        if ((((((*priv_offset.borrow()) as usize) >= length)
            || ((length).wrapping_sub(((*priv_offset.borrow()) as usize))
                < ((*priv_length.borrow()) as usize))) as i64)
            != 0)
        {
            return false;
        }
    }
    {
        let __a0 = (hdr.with(|__s| __s.num_tables) as usize) as usize;
        (*hdr.with(|__s| __s.tables.clone()).borrow_mut())
            .resize_with(__a0, || <woff2_Table>::default())
    };
    if ((!({
        let _tables: Ptr<Vec<woff2_Table>> = (hdr.with(|__s| __s.tables.as_pointer()));
        let _num_tables: usize = (hdr.with(|__s| __s.num_tables) as usize);
        ReadTableDirectory_69((file.as_pointer()), _tables, _num_tables)
    }) as i64)
        != 0)
    {
        return false;
    }
    let last_table: Ptr<woff2_Table> =
        (hdr.with(|__s| __s.tables.as_pointer()) as Ptr<woff2_Table>).to_last();
    field!(hdr, uncompressed_size).write(
        (last_table.with(|__s| __s.src_offset)).wrapping_add(last_table.with(|__s| __s.src_length)),
    );
    if ((({ hdr.with(|__s| __s.uncompressed_size) } < { last_table.with(|__s| __s.src_offset) })
        as i64)
        != 0)
    {
        return false;
    }
    field!(hdr, header_version).write(0_u32);
    if ({ hdr.with(|__s| __s.flavor) } == { kTtcFontFlavor_22.with(|rc| *rc.borrow()) }) {
        if ((!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(hdr, header_version))) })
            as i64)
            != 0)
        {
            return false;
        }
        if ((((hdr.with(|__s| __s.header_version) != 65536_u32)
            && (hdr.with(|__s| __s.header_version) != 131072_u32)) as i64)
            != 0)
        {
            return false;
        }
        let num_fonts: Value<u32> = Rc::new(RefCell::new(0_u32));
        if ((((!({ Read255UShort_12((file.as_pointer()), (num_fonts.as_pointer())) }))
            || (!((*num_fonts.borrow()) != 0))) as i64)
            != 0)
        {
            return false;
        }
        {
            let __a0 = ((*num_fonts.borrow()) as usize) as usize;
            (*hdr.with(|__s| __s.ttc_fonts.clone()).borrow_mut())
                .resize_with(__a0, || <woff2_TtcFont>::default())
        };
        let mut i: u32 = 0_u32;
        'loop_: while (i < (*num_fonts.borrow())) {
            let ttc_font: Ptr<woff2_TtcFont> = (hdr.with(|__s| __s.ttc_fonts.as_pointer())
                as Ptr<woff2_TtcFont>)
                .offset((i as usize));
            let num_tables: Value<u32> = Rc::new(RefCell::new(0_u32));
            if ((((!({ Read255UShort_12((file.as_pointer()), (num_tables.as_pointer())) }))
                || (!((*num_tables.borrow()) != 0))) as i64)
                != 0)
            {
                return false;
            }
            if ((!({
                woff2_BufferImpl::ReadU32(&file.as_pointer(), (field_ptr!(ttc_font, flavor)))
            }) as i64)
                != 0)
            {
                return false;
            }
            {
                let __a0 = ((*num_tables.borrow()) as usize) as usize;
                (*ttc_font.with(|__s| __s.table_indices.clone()).borrow_mut())
                    .resize_with(__a0, || <u16>::default())
            };
            let mut glyf_idx: u32 = 0_u32;
            let mut loca_idx: u32 = 0_u32;
            let mut j: u32 = 0_u32;
            'loop_: while (j < (*num_tables.borrow())) {
                let table_idx: Value<u32> = Rc::new(RefCell::new(0_u32));
                if ((!({ Read255UShort_12((file.as_pointer()), (table_idx.as_pointer())) }) as i64)
                    != 0)
                    || ({ ((*table_idx.borrow()) as usize) } >= {
                        (*hdr.with(|__s| __s.tables.clone()).borrow()).len()
                    })
                {
                    return false;
                }
                elem!(
                    (ttc_font.with(|__s| __s.table_indices.as_pointer()) as Ptr<u16>),
                    (j as usize)
                )
                .write(((*table_idx.borrow()) as u16));
                let table: Ptr<woff2_Table> = (hdr.with(|__s| __s.tables.as_pointer())
                    as Ptr<woff2_Table>)
                    .offset(((*table_idx.borrow()) as usize));
                if ({ table.with(|__s| __s.tag) } == { kLocaTableTag_2.with(|rc| *rc.borrow()) }) {
                    loca_idx = (*table_idx.borrow());
                }
                if ({ table.with(|__s| __s.tag) } == { kGlyfTableTag_0.with(|rc| *rc.borrow()) }) {
                    glyf_idx = (*table_idx.borrow());
                }
                j.postfix_inc();
            }
            if (glyf_idx > 0_u32) || (loca_idx > 0_u32) {
                if ((((glyf_idx > loca_idx) || ((loca_idx).wrapping_sub(glyf_idx) != 1_u32))
                    as i64)
                    != 0)
                {
                    eprintln!("TTC font {} has non-consecutive glyf/loca", i);
                    return false;
                }
            }
            i.postfix_inc();
        }
    }
    let mut first_table_offset: u64 = ({ ComputeOffsetToFirstTable_72((hdr).clone()) });
    let __rhs = (({ woff2_BufferImpl::offset(&file.as_pointer()) }) as u64);
    field!(hdr, compressed_offset).write(__rhs);
    if ((({ hdr.with(|__s| __s.compressed_offset) } > { (<u32>::MAX as u64) }) as i64) != 0) {
        return false;
    }
    let mut src_offset: u64 = ({
        Round4_29(
            (hdr.with(|__s| __s.compressed_offset))
                .wrapping_add((hdr.with(|__s| __s.compressed_length) as u64)),
        )
    });
    let mut dst_offset: u64 = first_table_offset;
    if ((((src_offset as usize) > length) as i64) != 0) {
        eprintln!(
            "offset fail; src_offset {} length {} dst_offset {}",
            src_offset, length, dst_offset
        );
        return false;
    }
    if ((*meta_offset.borrow()) != 0) {
        if (((src_offset != ((*meta_offset.borrow()) as u64)) as i64) != 0) {
            return false;
        }
        src_offset =
            (({ Round4_30((*meta_offset.borrow()).wrapping_add((*meta_length.borrow()))) }) as u64);
        if (((src_offset > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if ((*priv_offset.borrow()) != 0) {
        if (((src_offset != ((*priv_offset.borrow()) as u64)) as i64) != 0) {
            return false;
        }
        src_offset =
            (({ Round4_30((*priv_offset.borrow()).wrapping_add((*priv_length.borrow()))) }) as u64);
        if (((src_offset > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if (((src_offset != ({ Round4_29((length as u64)) })) as i64) != 0) {
        return false;
    }
    return true;
}
pub fn WriteHeaders_76(
    mut data: Ptr<u8>,
    mut length: usize,
    mut metadata: Ptr<woff2_RebuildMetadata>,
    mut hdr: Ptr<woff2_WOFF2Header>,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let output: Value<Vec<u8>> = Rc::new(RefCell::new(vec![
        0_u8;
        (({ ComputeOffsetToFirstTable_72((hdr).clone(),) }) as usize)
            as usize
    ]));
    let sorted_tables: Value<Vec<woff2_Table>> = Rc::new(RefCell::new(
        (*hdr.with(|__s| __s.tables.clone()).borrow()).clone(),
    ));
    if (hdr.with(|__s| __s.header_version) != 0) {
        'loop_: for mut ttc_font in hdr.with(|__s| __s.ttc_fonts.as_pointer()) as Ptr<woff2_TtcFont>
        {
            let sorted_index_by_tag: Value<BTreeMap<u32, Value<u16>>> =
                Rc::new(RefCell::new(BTreeMap::new()));
            'loop_: for mut table_index in
                ttc_font.with(|__s| __s.table_indices.as_pointer()) as Ptr<u16>
            {
                let mut table_index: u16 = table_index.read();
                let __rhs = table_index;
                (sorted_index_by_tag.as_pointer() as Ptr<BTreeMap<u32, Value<u16>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u16>>| {
                        __v.entry({
                            (*elem!(
                                (hdr.with(|__s| __s.tables.as_pointer()) as Ptr<woff2_Table>),
                                (table_index as usize)
                            )
                            .upgrade()
                            .deref())
                            .tag
                        })
                        .or_insert_with(|| Rc::new(RefCell::new(<u16>::default())))
                        .as_pointer()
                    })
                    .write(__rhs);
            }
            let mut index: u16 = 0_u16;
            'loop_: for i in RefcountMapIter::begin(sorted_index_by_tag.as_pointer()) {
                elem!(
                    (ttc_font.with(|__s| __s.table_indices.as_pointer()) as Ptr<u16>),
                    (index.postfix_inc() as usize)
                )
                .write((*i.second().borrow()));
            }
        }
    } else {
        (sorted_tables.as_pointer() as Ptr<woff2_Table>).sort(
            (sorted_tables.as_pointer() as Ptr<woff2_Table>)
                .to_end()
                .get_offset(),
        );
    }
    let mut result: Ptr<u8> = ((output.as_pointer() as Ptr<u8>).offset(0_usize));
    let mut offset: usize = 0_usize;
    if (hdr.with(|__s| __s.header_version) != 0) {
        let __rhs = ({ StoreU32_31((result).clone(), offset, hdr.with(|__s| __s.flavor)) });
        offset = __rhs;
        let __rhs = ({ StoreU32_31((result).clone(), offset, hdr.with(|__s| __s.header_version)) });
        offset = __rhs;
        let __rhs = ({
            StoreU32_31(
                (result).clone(),
                offset,
                ((*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() as u32),
            )
        });
        offset = __rhs;
        let mut offset_table: usize = offset;
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() }) {
            let __rhs = ({ StoreU32_31((result).clone(), offset, 0_u32) });
            offset = __rhs;
            i.postfix_inc();
        }
        if (hdr.with(|__s| __s.header_version) == 131072_u32) {
            let __rhs = ({ StoreU32_31((result).clone(), offset, 0_u32) });
            offset = __rhs;
            let __rhs = ({ StoreU32_31((result).clone(), offset, 0_u32) });
            offset = __rhs;
            let __rhs = ({ StoreU32_31((result).clone(), offset, 0_u32) });
            offset = __rhs;
        }
        {
            let __a0 = (*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() as usize;
            (*metadata.with(|__s| __s.font_infos.clone()).borrow_mut())
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while ({ i } < { (*hdr.with(|__s| __s.ttc_fonts.clone()).borrow()).len() }) {
            let ttc_font: Ptr<woff2_TtcFont> =
                (hdr.with(|__s| __s.ttc_fonts.as_pointer()) as Ptr<woff2_TtcFont>).offset(i);
            let __rhs = ({ StoreU32_31((result).clone(), offset_table, (offset as u32)) });
            offset_table = __rhs;
            field!(ttc_font, dst_offset).write((offset as u32));
            let __rhs = ({
                let _flavor: u32 = ttc_font.with(|__s| __s.flavor);
                let _num_tables: u16 =
                    ((*ttc_font.with(|__s| __s.table_indices.clone()).borrow()).len() as u16);
                StoreOffsetTable_70((result).clone(), offset, _flavor, _num_tables)
            });
            offset = __rhs;
            'loop_: for table_index in
                ttc_font.with(|__s| __s.table_indices.as_pointer()) as Ptr<u16>
            {
                let mut table_index: u16 = table_index.read();
                let mut tag: u32 = {
                    (*elem!(
                        (hdr.with(|__s| __s.tables.as_pointer()) as Ptr<woff2_Table>),
                        (table_index as usize)
                    )
                    .upgrade()
                    .deref())
                    .tag
                };
                (field_ptr!(
                    (metadata.with(|__s| __s.font_infos.as_pointer()) as Ptr<woff2_WOFF2FontInfo>)
                        .offset(i),
                    table_entry_by_tag
                ) as Ptr<BTreeMap<u32, Value<u32>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                        __v.entry(tag)
                            .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                            .as_pointer()
                    })
                    .write((offset as u32));
                let __rhs = ({ StoreTableEntry_71((result).clone(), (offset as u32), tag) });
                offset = __rhs;
            }
            let __rhs = ({
                let _buf: Ptr<u8> = ((output.as_pointer() as Ptr<u8>)
                    .offset((ttc_font.with(|__s| __s.dst_offset) as usize)));
                let _size: usize =
                    (offset).wrapping_sub((ttc_font.with(|__s| __s.dst_offset) as usize));
                ComputeULongSum_26(_buf, _size)
            });
            field!(ttc_font, header_checksum).write(__rhs);
            i.postfix_inc();
        }
    } else {
        {
            let __a0 = 1_usize as usize;
            (*metadata.with(|__s| __s.font_infos.clone()).borrow_mut())
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let __rhs = ({
            let _flavor: u32 = hdr.with(|__s| __s.flavor);
            let _num_tables: u16 = hdr.with(|__s| __s.num_tables);
            StoreOffsetTable_70((result).clone(), offset, _flavor, _num_tables)
        });
        offset = __rhs;
        let mut i: u16 = 0_u16;
        'loop_: while ({ (i as i32) } < { (hdr.with(|__s| __s.num_tables) as i32) }) {
            (field_ptr!(
                (metadata.with(|__s| __s.font_infos.as_pointer()) as Ptr<woff2_WOFF2FontInfo>)
                    .offset(0_usize),
                table_entry_by_tag
            ) as Ptr<BTreeMap<u32, Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                    __v.entry({ (*sorted_tables.borrow())[(i as usize)].tag })
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .write((offset as u32));
            let __rhs = ({
                StoreTableEntry_71((result).clone(), (offset as u32), {
                    (*sorted_tables.borrow())[(i as usize)].tag
                })
            });
            offset = __rhs;
            i.prefix_inc();
        }
    }
    if ((!({
        let _buf: AnyPtr = (((output.as_pointer() as Ptr<u8>).offset(0_usize)) as Ptr<u8>).to_any();
        let _n: usize = (*output.borrow()).len();
        (*out.upgrade().deref_mut()).Write_2(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    let __rhs = ({
        let _buf: Ptr<u8> = ((output.as_pointer() as Ptr<u8>).offset(0_usize));
        let _size: usize = (*output.borrow()).len();
        ComputeULongSum_26(_buf, _size)
    });
    field!(metadata, header_checksum).write(__rhs);
    return true;
}
pub fn ComputeWOFF2FinalSize_77(mut data: Ptr<u8>, mut length: usize) -> usize {
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::new({ (data).clone() }, {
        length
    })));
    let total_length: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (!({ woff2_BufferImpl::Skip(&file.as_pointer(), 16_usize) }))
        || (!({ woff2_BufferImpl::ReadU32(&file.as_pointer(), (total_length.as_pointer())) }))
    {
        return 0_usize;
    }
    return ((*total_length.borrow()) as usize);
}
pub fn ConvertWOFF2ToTTF_78(
    mut result: Ptr<u8>,
    mut result_length: usize,
    mut data: Ptr<u8>,
    mut length: usize,
) -> bool {
    let out: Value<woff2_WOFF2MemoryOut> = Rc::new(RefCell::new(woff2_WOFF2MemoryOut::new(
        { (result).clone() },
        { result_length },
    )));
    return ({
        ConvertWOFF2ToTTF_79(
            data,
            length,
            (out.as_pointer()).to_dyn::<dyn woff2_WOFF2Out>(|w| w),
        )
    });
}
pub fn ConvertWOFF2ToTTF_79(
    mut data: Ptr<u8>,
    mut length: usize,
    mut out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let metadata: Value<woff2_RebuildMetadata> =
        Rc::new(RefCell::new(<woff2_RebuildMetadata>::default()));
    let hdr: Value<woff2_WOFF2Header> = Rc::new(RefCell::new(<woff2_WOFF2Header>::default()));
    if !({ ReadWOFF2Header_75((data).clone(), length, (hdr.as_pointer())) }) {
        return false;
    }
    if !({
        WriteHeaders_76(
            (data).clone(),
            length,
            (metadata.as_pointer()),
            (hdr.as_pointer()),
            (out).clone(),
        )
    }) {
        return false;
    }
    let mut compression_ratio: f32 =
        (({ (*hdr.borrow()).uncompressed_size } as f32) / (length as f32));
    if (compression_ratio > kMaxPlausibleCompressionRatio_54.with(|rc| *rc.borrow())) {
        eprintln!(
            "Implausible compression ratio {:.1}",
            (compression_ratio as f64)
        );
        return false;
    }
    let mut src_buf: Ptr<u8> = data.offset(({ (*hdr.borrow()).compressed_offset }) as isize);
    let uncompressed_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..({ (*hdr.borrow()).uncompressed_size } as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    if ((({ (*hdr.borrow()).uncompressed_size } < 1_u32) as i64) != 0) {
        return false;
    }
    if ((!({
        let _dst_size: usize = ({ (*hdr.borrow()).uncompressed_size } as usize);
        let _src_size: usize = ({ (*hdr.borrow()).compressed_length } as usize);
        Woff2Uncompress_68(
            ((uncompressed_buf.as_pointer() as Ptr<u8>).offset(0_usize)),
            _dst_size,
            (src_buf).clone(),
            _src_size,
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while (i < (*{ (*metadata.borrow()).font_infos.clone() }.borrow()).len()) {
        if ((!({
            let _transformed_buf_size: u32 = { (*hdr.borrow()).uncompressed_size };
            let _hdr: Ptr<woff2_WOFF2Header> = (hdr.as_pointer());
            ReconstructFont_74(
                ((uncompressed_buf.as_pointer() as Ptr<u8>).offset(0_usize)),
                _transformed_buf_size,
                (metadata.as_pointer()),
                _hdr,
                i,
                (out).clone(),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        i.postfix_inc();
    }
    return true;
}
impl woff2_WOFF2StringOut {
    pub fn new(mut buf: Ptr<Vec<i8>>) -> Self {
        Self {
            buf_: (buf).clone(),
            max_size_: kDefaultMaxSize_28.with(|rc| *rc.borrow()),
            offset_: 0_usize,
        }
    }
}
impl woff2_WOFF2MemoryOut {
    pub fn new(mut buf: Ptr<u8>, mut buf_size: usize) -> Self {
        Self {
            buf_: (buf).clone(),
            buf_size_: buf_size,
            offset_: 0_usize,
        }
    }
}
impl woff2_WOFF2StringOut {}
impl woff2_WOFF2MemoryOut {}
pub fn GetFileContent_80(filename: Vec<i8>) -> Vec<i8> {
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
pub fn SetFileContents_81(filename: Vec<i8>, start: Ptr<i8>, end: Ptr<i8>) {
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
    if (argc != 2) {
        eprintln!("One argument, the input filename, must be provided.");
        return 1;
    }
    let mut filename: Vec<i8> = {
        let mut __bytes = (elem!(argv, 1).read()).to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    let mut outfilename: Vec<i8> = {
        let mut __tmp2 = {
            let mut __tmp1 = filename[(0_usize) as usize
                ..::std::cmp::min(
                    (0_usize
                        + Ptr::<i8>::from_string_literal(b".").with_c_str(|__lookup| {
                            filename
                                .iter()
                                .take(filename.len().saturating_sub(1))
                                .rposition(|&x| __lookup.contains(&x))
                                .unwrap_or(usize::MAX)
                        })) as usize,
                    filename.len().saturating_sub(1),
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        };
        __tmp2.pop();
        Ptr::<i8>::from_string_literal(b".ttf").with_c_str(|__s| __tmp2.extend_from_slice(__s));
        __tmp2.push(0);
        __tmp2
    };
    let input: Value<Vec<i8>> = Rc::new(RefCell::new(({ GetFileContent_80((filename).clone()) })));
    let mut raw_input: Ptr<u8> = (input.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>();
    let output: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![
            0_i8;
            ({
                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                    (({
                        ComputeWOFF2FinalSize_77((raw_input).clone(), ((*input.borrow()).len() - 1))
                    }) as u64),
                ));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                    (kDefaultMaxSize_28.with(|rc| *rc.borrow()) as u64),
                ));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            } as usize) as usize
        ]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect(),
    ));
    let out: Value<woff2_WOFF2StringOut> = Rc::new(RefCell::new(woff2_WOFF2StringOut::new({
        (output.as_pointer())
    })));
    let mut ok: bool = ({
        ConvertWOFF2ToTTF_79(
            (raw_input).clone(),
            ((*input.borrow()).len() - 1),
            (out.as_pointer()).to_dyn::<dyn woff2_WOFF2Out>(|w| w),
        )
    });
    if ok {
        ({
            let _start: Ptr<i8> = (output.as_pointer() as Ptr<i8>);
            let _end: Ptr<i8> = (output.as_pointer() as Ptr<i8>)
                .offset((({ (*out.borrow_mut()).Size() }) as i64) as isize);
            SetFileContents_81((outfilename).clone(), _start, _end)
        });
    }
    return if ok { 0 } else { 1 };
}
impl woff2_WOFF2Out for woff2_WOFF2MemoryOut {
    fn Size(&mut self) -> usize {
        return { self.offset_ };
    }
    fn Write_2(&mut self, mut buf: AnyPtr, mut n: usize) -> bool {
        return ({
            let _offset: usize = { self.offset_ };
            self.Write_3(buf, _offset, n)
        });
    }
    fn Write_3(&mut self, mut buf: AnyPtr, mut offset: usize, mut n: usize) -> bool {
        if (offset > { self.buf_size_ }) || (n > ({ self.buf_size_ }).wrapping_sub(offset)) {
            return false;
        }
        {
            ({ self.buf_.clone() }.offset((offset) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(&buf, n as usize);
            ({ self.buf_.clone() }.offset((offset) as isize) as Ptr<u8>).to_any()
        };
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(({ self.offset_ } as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((offset).wrapping_add(n) as u64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        self.offset_ = __rhs;
        return true;
    }
}
impl woff2_WOFF2Out for woff2_WOFF2StringOut {
    fn Size(&mut self) -> usize {
        return { self.offset_ };
    }
    fn Write_2(&mut self, mut buf: AnyPtr, mut n: usize) -> bool {
        return ({
            let _offset: usize = { self.offset_ };
            self.Write_3(buf, _offset, n)
        });
    }
    fn Write_3(&mut self, mut buf: AnyPtr, mut offset: usize, mut n: usize) -> bool {
        if (offset > { self.max_size_ }) || (n > ({ self.max_size_ }).wrapping_sub(offset)) {
            return false;
        }
        if ({ offset } == { ((*{ self.buf_.clone() }.upgrade().deref()).len() - 1) }) {
            {
                ({ self.buf_.clone() } as Ptr<Vec<i8>>).with_mut(|__v: &mut Vec<i8>| {
                    __v.pop();
                    buf.reinterpret_cast::<i8>()
                        .with_slice(n as usize, |__s| __v.extend_from_slice(__s));
                    __v.push(0);
                });
                ({ self.buf_.clone() } as Ptr<Vec<i8>>)
            };
        } else {
            if ({ (offset).wrapping_add(n) } > {
                ((*{ self.buf_.clone() }.upgrade().deref()).len() - 1)
            }) {
                {
                    { self.buf_.clone() }.with_mut(|__v: &mut Vec<i8>| __v.pop());
                    { self.buf_.clone() }.with_mut(|__v: &mut Vec<i8>| {
                        __v.resize(
                            (*{ self.buf_.clone() }.upgrade().deref()).len()
                                + (((offset).wrapping_add(n) as u64).wrapping_sub(
                                    (((*{ self.buf_.clone() }.upgrade().deref()).len() - 1) as u64),
                                ) as usize) as usize,
                            0_i8,
                        )
                    });
                    { self.buf_.clone() }.with_mut(|__v: &mut Vec<i8>| __v.push(0));
                    (*{ self.buf_.clone() }.upgrade().deref()).clone()
                };
            }
            {
                let pos = offset as usize;
                let end = std::cmp::min(
                    pos + n as usize,
                    (*({ self.buf_.clone() } as Ptr<Vec<i8>>).upgrade().deref())
                        .len()
                        .saturating_sub(1),
                );
                ({ self.buf_.clone() } as Ptr<Vec<i8>>).with_mut(|__v: &mut Vec<i8>| {
                    buf.reinterpret_cast::<i8>().with_slice(n as usize, |__s| {
                        __v.splice(pos..end, __s.iter().copied());
                    });
                });
                ({ self.buf_.clone() } as Ptr<Vec<i8>>)
            };
        }
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(({ self.offset_ } as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((offset).wrapping_add(n) as u64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        self.offset_ = __rhs;
        return true;
    }
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
pub trait woff2_TableImpl {
    fn operator_lt(&self, other: Ptr<woff2_Table>) -> bool;
}
impl woff2_TableImpl for Ptr<woff2_Table> {
    fn operator_lt(&self, other: Ptr<woff2_Table>) -> bool {
        return ({ (*self).with(|__s| __s.tag) } < { other.with(|__s| __s.tag) });
    }
}
pub trait woff2_WOFF2StringOutImpl {
    fn MaxSize(&self) -> usize;
    fn SetMaxSize(&self, mut max_size: usize) {
        unimplemented!()
    }
}
impl woff2_WOFF2StringOutImpl for Ptr<woff2_WOFF2StringOut> {
    fn MaxSize(&self) -> usize {
        return (*self).with(|__s| __s.max_size_);
    }
    fn SetMaxSize(&self, mut max_size: usize) {
        field!((*self), max_size_).write(max_size);
        if ((*self).with(|__s| __s.offset_) > (*self).with(|__s| __s.max_size_)) {
            field!((*self), offset_).write((*self).with(|__s| __s.max_size_));
        }
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
    let _ = kDefaultMaxSize_28.with(|_| ());
    let _ = kGlyfOnCurve_36.with(|_| ());
    let _ = kGlyfXShort_37.with(|_| ());
    let _ = kGlyfYShort_38.with(|_| ());
    let _ = kGlyfRepeat_39.with(|_| ());
    let _ = kGlyfThisXIsSame_40.with(|_| ());
    let _ = kGlyfThisYIsSame_41.with(|_| ());
    let _ = kOverlapSimple_42.with(|_| ());
    let _ = FLAG_ARG_1_AND_2_ARE_WORDS_43.with(|_| ());
    let _ = FLAG_WE_HAVE_A_SCALE_44.with(|_| ());
    let _ = FLAG_MORE_COMPONENTS_45.with(|_| ());
    let _ = FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46.with(|_| ());
    let _ = FLAG_WE_HAVE_A_TWO_BY_TWO_47.with(|_| ());
    let _ = FLAG_WE_HAVE_INSTRUCTIONS_48.with(|_| ());
    let _ = FLAG_OVERLAP_SIMPLE_BITMAP_49.with(|_| ());
    let _ = kCheckSumAdjustmentOffset_50.with(|_| ());
    let _ = kEndPtsOfContoursOffset_51.with(|_| ());
    let _ = kCompositeGlyphBegin_52.with(|_| ());
    let _ = kDefaultGlyphBuf_53.with(|_| ());
    let _ = kMaxPlausibleCompressionRatio_54.with(|_| ());
}
