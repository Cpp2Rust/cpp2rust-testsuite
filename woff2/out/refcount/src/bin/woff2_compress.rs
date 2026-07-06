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
thread_local!();
thread_local!(
    pub static kKnownTags_8: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        ((((((('c' as u8) as i32) << 24) | ((('m' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('p' as u8) as i32)) as u32),
        ((((((('h' as u8) as i32) << 24) | ((('e' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('d' as u8) as i32)) as u32),
        ((((((('h' as u8) as i32) << 24) | ((('h' as u8) as i32) << 16))
            | ((('e' as u8) as i32) << 8))
            | (('a' as u8) as i32)) as u32),
        ((((((('h' as u8) as i32) << 24) | ((('m' as u8) as i32) << 16))
            | ((('t' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('m' as u8) as i32) << 24) | ((('a' as u8) as i32) << 16))
            | ((('x' as u8) as i32) << 8))
            | (('p' as u8) as i32)) as u32),
        ((((((('n' as u8) as i32) << 24) | ((('a' as u8) as i32) << 16))
            | ((('m' as u8) as i32) << 8))
            | (('e' as u8) as i32)) as u32),
        ((((((('O' as u8) as i32) << 24) | ((('S' as u8) as i32) << 16))
            | ((('/' as u8) as i32) << 8))
            | (('2' as u8) as i32)) as u32),
        ((((((('p' as u8) as i32) << 24) | ((('o' as u8) as i32) << 16))
            | ((('s' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('c' as u8) as i32) << 24) | ((('v' as u8) as i32) << 16))
            | ((('t' as u8) as i32) << 8))
            | ((' ' as u8) as i32)) as u32),
        ((((((('f' as u8) as i32) << 24) | ((('p' as u8) as i32) << 16))
            | ((('g' as u8) as i32) << 8))
            | (('m' as u8) as i32)) as u32),
        ((((((('g' as u8) as i32) << 24) | ((('l' as u8) as i32) << 16))
            | ((('y' as u8) as i32) << 8))
            | (('f' as u8) as i32)) as u32),
        ((((((('l' as u8) as i32) << 24) | ((('o' as u8) as i32) << 16))
            | ((('c' as u8) as i32) << 8))
            | (('a' as u8) as i32)) as u32),
        ((((((('p' as u8) as i32) << 24) | ((('r' as u8) as i32) << 16))
            | ((('e' as u8) as i32) << 8))
            | (('p' as u8) as i32)) as u32),
        ((((((('C' as u8) as i32) << 24) | ((('F' as u8) as i32) << 16))
            | ((('F' as u8) as i32) << 8))
            | ((' ' as u8) as i32)) as u32),
        ((((((('V' as u8) as i32) << 24) | ((('O' as u8) as i32) << 16))
            | ((('R' as u8) as i32) << 8))
            | (('G' as u8) as i32)) as u32),
        ((((((('E' as u8) as i32) << 24) | ((('B' as u8) as i32) << 16))
            | ((('D' as u8) as i32) << 8))
            | (('T' as u8) as i32)) as u32),
        ((((((('E' as u8) as i32) << 24) | ((('B' as u8) as i32) << 16))
            | ((('L' as u8) as i32) << 8))
            | (('C' as u8) as i32)) as u32),
        ((((((('g' as u8) as i32) << 24) | ((('a' as u8) as i32) << 16))
            | ((('s' as u8) as i32) << 8))
            | (('p' as u8) as i32)) as u32),
        ((((((('h' as u8) as i32) << 24) | ((('d' as u8) as i32) << 16))
            | ((('m' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('k' as u8) as i32) << 24) | ((('e' as u8) as i32) << 16))
            | ((('r' as u8) as i32) << 8))
            | (('n' as u8) as i32)) as u32),
        ((((((('L' as u8) as i32) << 24) | ((('T' as u8) as i32) << 16))
            | ((('S' as u8) as i32) << 8))
            | (('H' as u8) as i32)) as u32),
        ((((((('P' as u8) as i32) << 24) | ((('C' as u8) as i32) << 16))
            | ((('L' as u8) as i32) << 8))
            | (('T' as u8) as i32)) as u32),
        ((((((('V' as u8) as i32) << 24) | ((('D' as u8) as i32) << 16))
            | ((('M' as u8) as i32) << 8))
            | (('X' as u8) as i32)) as u32),
        ((((((('v' as u8) as i32) << 24) | ((('h' as u8) as i32) << 16))
            | ((('e' as u8) as i32) << 8))
            | (('a' as u8) as i32)) as u32),
        ((((((('v' as u8) as i32) << 24) | ((('m' as u8) as i32) << 16))
            | ((('t' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('B' as u8) as i32) << 24) | ((('A' as u8) as i32) << 16))
            | ((('S' as u8) as i32) << 8))
            | (('E' as u8) as i32)) as u32),
        ((((((('G' as u8) as i32) << 24) | ((('D' as u8) as i32) << 16))
            | ((('E' as u8) as i32) << 8))
            | (('F' as u8) as i32)) as u32),
        ((((((('G' as u8) as i32) << 24) | ((('P' as u8) as i32) << 16))
            | ((('O' as u8) as i32) << 8))
            | (('S' as u8) as i32)) as u32),
        ((((((('G' as u8) as i32) << 24) | ((('S' as u8) as i32) << 16))
            | ((('U' as u8) as i32) << 8))
            | (('B' as u8) as i32)) as u32),
        ((((((('E' as u8) as i32) << 24) | ((('B' as u8) as i32) << 16))
            | ((('S' as u8) as i32) << 8))
            | (('C' as u8) as i32)) as u32),
        ((((((('J' as u8) as i32) << 24) | ((('S' as u8) as i32) << 16))
            | ((('T' as u8) as i32) << 8))
            | (('F' as u8) as i32)) as u32),
        ((((((('M' as u8) as i32) << 24) | ((('A' as u8) as i32) << 16))
            | ((('T' as u8) as i32) << 8))
            | (('H' as u8) as i32)) as u32),
        ((((((('C' as u8) as i32) << 24) | ((('B' as u8) as i32) << 16))
            | ((('D' as u8) as i32) << 8))
            | (('T' as u8) as i32)) as u32),
        ((((((('C' as u8) as i32) << 24) | ((('B' as u8) as i32) << 16))
            | ((('L' as u8) as i32) << 8))
            | (('C' as u8) as i32)) as u32),
        ((((((('C' as u8) as i32) << 24) | ((('O' as u8) as i32) << 16))
            | ((('L' as u8) as i32) << 8))
            | (('R' as u8) as i32)) as u32),
        ((((((('C' as u8) as i32) << 24) | ((('P' as u8) as i32) << 16))
            | ((('A' as u8) as i32) << 8))
            | (('L' as u8) as i32)) as u32),
        ((((((('S' as u8) as i32) << 24) | ((('V' as u8) as i32) << 16))
            | ((('G' as u8) as i32) << 8))
            | ((' ' as u8) as i32)) as u32),
        ((((((('s' as u8) as i32) << 24) | ((('b' as u8) as i32) << 16))
            | ((('i' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('a' as u8) as i32) << 24) | ((('c' as u8) as i32) << 16))
            | ((('n' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('a' as u8) as i32) << 24) | ((('v' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('r' as u8) as i32)) as u32),
        ((((((('b' as u8) as i32) << 24) | ((('d' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('b' as u8) as i32) << 24) | ((('l' as u8) as i32) << 16))
            | ((('o' as u8) as i32) << 8))
            | (('c' as u8) as i32)) as u32),
        ((((((('b' as u8) as i32) << 24) | ((('s' as u8) as i32) << 16))
            | ((('l' as u8) as i32) << 8))
            | (('n' as u8) as i32)) as u32),
        ((((((('c' as u8) as i32) << 24) | ((('v' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('r' as u8) as i32)) as u32),
        ((((((('f' as u8) as i32) << 24) | ((('d' as u8) as i32) << 16))
            | ((('s' as u8) as i32) << 8))
            | (('c' as u8) as i32)) as u32),
        ((((((('f' as u8) as i32) << 24) | ((('e' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('f' as u8) as i32) << 24) | ((('m' as u8) as i32) << 16))
            | ((('t' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('f' as u8) as i32) << 24) | ((('v' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('r' as u8) as i32)) as u32),
        ((((((('g' as u8) as i32) << 24) | ((('v' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('r' as u8) as i32)) as u32),
        ((((((('h' as u8) as i32) << 24) | ((('s' as u8) as i32) << 16))
            | ((('t' as u8) as i32) << 8))
            | (('y' as u8) as i32)) as u32),
        ((((((('j' as u8) as i32) << 24) | ((('u' as u8) as i32) << 16))
            | ((('s' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('l' as u8) as i32) << 24) | ((('c' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('r' as u8) as i32)) as u32),
        ((((((('m' as u8) as i32) << 24) | ((('o' as u8) as i32) << 16))
            | ((('r' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('m' as u8) as i32) << 24) | ((('o' as u8) as i32) << 16))
            | ((('r' as u8) as i32) << 8))
            | (('x' as u8) as i32)) as u32),
        ((((((('o' as u8) as i32) << 24) | ((('p' as u8) as i32) << 16))
            | ((('b' as u8) as i32) << 8))
            | (('d' as u8) as i32)) as u32),
        ((((((('p' as u8) as i32) << 24) | ((('r' as u8) as i32) << 16))
            | ((('o' as u8) as i32) << 8))
            | (('p' as u8) as i32)) as u32),
        ((((((('t' as u8) as i32) << 24) | ((('r' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('k' as u8) as i32)) as u32),
        ((((((('Z' as u8) as i32) << 24) | ((('a' as u8) as i32) << 16))
            | ((('p' as u8) as i32) << 8))
            | (('f' as u8) as i32)) as u32),
        ((((((('S' as u8) as i32) << 24) | ((('i' as u8) as i32) << 16))
            | ((('l' as u8) as i32) << 8))
            | (('f' as u8) as i32)) as u32),
        ((((((('G' as u8) as i32) << 24) | ((('l' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('G' as u8) as i32) << 24) | ((('l' as u8) as i32) << 16))
            | ((('o' as u8) as i32) << 8))
            | (('c' as u8) as i32)) as u32),
        ((((((('F' as u8) as i32) << 24) | ((('e' as u8) as i32) << 16))
            | ((('a' as u8) as i32) << 8))
            | (('t' as u8) as i32)) as u32),
        ((((((('S' as u8) as i32) << 24) | ((('i' as u8) as i32) << 16))
            | ((('l' as u8) as i32) << 8))
            | (('l' as u8) as i32)) as u32),
    ])));
);
#[derive(Default)]
pub struct woff2_Buffer {
    buffer_: Value<Ptr<u8>>,
    length_: Value<usize>,
    offset_: Value<usize>,
}
impl woff2_Buffer {
    pub fn woff2_Buffer(data: Ptr<u8>, len: usize) -> Self {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let len: Value<usize> = Rc::new(RefCell::new(len));
        let mut this = Self {
            buffer_: Rc::new(RefCell::new((*data.borrow()).clone())),
            length_: Rc::new(RefCell::new((*len.borrow()))),
            offset_: Rc::new(RefCell::new(0_usize)),
        };
        this
    }
    pub fn Skip(&self, n_bytes: usize) -> bool {
        let n_bytes: Value<usize> = Rc::new(RefCell::new(n_bytes));
        return ({ self.Read(Ptr::<u8>::null(), (*n_bytes.borrow())) });
    }
    pub fn Read(&self, data: Ptr<u8>, n_bytes: usize) -> bool {
        let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
        let n_bytes: Value<usize> = Rc::new(RefCell::new(n_bytes));
        if ((*n_bytes.borrow()) > (((1024 * 1024) * 1024) as usize)) {
            return false;
        }
        if ((*self.offset_.borrow()).wrapping_add((*n_bytes.borrow())) > (*self.length_.borrow()))
            || ((*self.offset_.borrow())
                > (*self.length_.borrow()).wrapping_sub((*n_bytes.borrow())))
        {
            return false;
        }
        if !(*data.borrow()).is_null() {
            {
                ((*data.borrow()).clone() as Ptr<u8>).to_any().memcpy(
                    &((*self.buffer_.borrow()).offset((*self.offset_.borrow()) as isize)
                        as Ptr<u8>)
                        .to_any(),
                    (*n_bytes.borrow()) as usize,
                );
                ((*data.borrow()).clone() as Ptr<u8>).to_any().clone()
            };
        }
        let rhs_0 = (*self.offset_.borrow()).wrapping_add((*n_bytes.borrow()));
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn ReadU8(&self, value: Ptr<u8>) -> bool {
        let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 1_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(1_usize))
        {
            return false;
        }
        let __rhs = ((*self.buffer_.borrow())
            .offset((*self.offset_.borrow()) as isize)
            .read());
        (*value.borrow()).write(__rhs);
        (*self.offset_.borrow_mut()).prefix_inc();
        return true;
    }
    pub fn ReadU16(&self, value: Ptr<u16>) -> bool {
        let value: Value<Ptr<u16>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 2_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(2_usize))
        {
            return false;
        }
        {
            ((*value.borrow()).clone() as Ptr<u16>).to_any().memcpy(
                &((*self.buffer_.borrow()).offset((*self.offset_.borrow()) as isize) as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u16>() as usize,
            );
            ((*value.borrow()).clone() as Ptr<u16>).to_any().clone()
        };
        let __rhs = u16::from_be(((*value.borrow()).read()));
        (*value.borrow()).write(__rhs);
        let rhs_0 = (*self.offset_.borrow()).wrapping_add(2_usize);
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn ReadS16(&self, value: Ptr<i16>) -> bool {
        let value: Value<Ptr<i16>> = Rc::new(RefCell::new(value));
        return ({ self.ReadU16(((*value.borrow()).reinterpret_cast::<u16>()).clone()) });
    }
    pub fn ReadU24(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 3_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(3_usize))
        {
            return false;
        }
        let __rhs = ((((((*self.buffer_.borrow())
            .offset((*self.offset_.borrow()) as isize)
            .read()) as u32)
            << 16)
            | ((((*self.buffer_.borrow())
                .offset(((*self.offset_.borrow()).wrapping_add(1_usize)) as isize)
                .read()) as u32)
                << 8))
            | (((*self.buffer_.borrow())
                .offset(((*self.offset_.borrow()).wrapping_add(2_usize)) as isize)
                .read()) as u32));
        (*value.borrow()).write(__rhs);
        let rhs_0 = (*self.offset_.borrow()).wrapping_add(3_usize);
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn ReadU32(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 4_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            ((*value.borrow()).clone() as Ptr<u32>).to_any().memcpy(
                &((*self.buffer_.borrow()).offset((*self.offset_.borrow()) as isize) as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            ((*value.borrow()).clone() as Ptr<u32>).to_any().clone()
        };
        let __rhs = u32::from_be(((*value.borrow()).read()));
        (*value.borrow()).write(__rhs);
        let rhs_0 = (*self.offset_.borrow()).wrapping_add(4_usize);
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn ReadS32(&self, value: Ptr<i32>) -> bool {
        let value: Value<Ptr<i32>> = Rc::new(RefCell::new(value));
        return ({ self.ReadU32(((*value.borrow()).reinterpret_cast::<u32>()).clone()) });
    }
    pub fn ReadTag(&self, value: Ptr<u32>) -> bool {
        let value: Value<Ptr<u32>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 4_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(4_usize))
        {
            return false;
        }
        {
            ((*value.borrow()).clone() as Ptr<u32>).to_any().memcpy(
                &((*self.buffer_.borrow()).offset((*self.offset_.borrow()) as isize) as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u32>() as usize,
            );
            ((*value.borrow()).clone() as Ptr<u32>).to_any().clone()
        };
        let rhs_0 = (*self.offset_.borrow()).wrapping_add(4_usize);
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn ReadR64(&self, value: Ptr<u64>) -> bool {
        let value: Value<Ptr<u64>> = Rc::new(RefCell::new(value));
        if ((*self.length_.borrow()) < 8_usize)
            || ((*self.offset_.borrow()) > (*self.length_.borrow()).wrapping_sub(8_usize))
        {
            return false;
        }
        {
            ((*value.borrow()).clone() as Ptr<u64>).to_any().memcpy(
                &((*self.buffer_.borrow()).offset((*self.offset_.borrow()) as isize) as Ptr<u8>)
                    .to_any(),
                ::std::mem::size_of::<u64>() as usize,
            );
            ((*value.borrow()).clone() as Ptr<u64>).to_any().clone()
        };
        let rhs_0 = (*self.offset_.borrow()).wrapping_add(8_usize);
        (*self.offset_.borrow_mut()) = rhs_0;
        return true;
    }
    pub fn buffer(&self) -> Ptr<u8> {
        return (*self.buffer_.borrow()).clone();
    }
    pub fn offset(&self) -> usize {
        return (*self.offset_.borrow());
    }
    pub fn length(&self) -> usize {
        return (*self.length_.borrow());
    }
    pub fn set_offset(&self, newoffset: usize) -> bool {
        let newoffset: Value<usize> = Rc::new(RefCell::new(newoffset));
        if ((*newoffset.borrow()) > (*self.length_.borrow())) {
            return false;
        }
        (*self.offset_.borrow_mut()) = (*newoffset.borrow());
        return true;
    }
}
impl Clone for woff2_Buffer {
    fn clone(&self) -> Self {
        let mut this = Self {
            buffer_: Rc::new(RefCell::new((*self.buffer_.borrow()).clone())),
            length_: Rc::new(RefCell::new((*self.length_.borrow()))),
            offset_: Rc::new(RefCell::new((*self.offset_.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_Buffer {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.buffer_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.length_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.offset_.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            buffer_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            length_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
            offset_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
        }
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
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(((*value.borrow()) as u8)));
    } else if ((*value.borrow()) < 506) {
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(255_u8));
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) - 253) as u8)));
    } else if ((*value.borrow()) < 762) {
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(254_u8));
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) - 506) as u8)));
    } else {
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(253_u8));
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) >> 8) as u8)));
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) & 255) as u8)));
    }
}
pub fn Store255UShort_11(val: i32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let packed: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::new()));
    ({ Write255UShort_10((packed.as_pointer()), (*val.borrow())) });
    'loop_: for mut packed_byte in packed.as_pointer() as Ptr<u8> {
        let packed_byte: Value<u8> = Rc::new(RefCell::new(packed_byte.read().clone()));
        let __rhs = (*packed_byte.borrow());
        (*dst.borrow())
            .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
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
    if !({ (*(*buf.borrow()).upgrade().deref()).ReadU8((code.as_pointer())) }) {
        return false;
    }
    if (((*code.borrow()) as i32) == (*kWordCode_13.with(Value::clone).borrow())) {
        let result: Value<u16> = Rc::new(RefCell::new(0_u16));
        if !({ (*(*buf.borrow()).upgrade().deref()).ReadU16((result.as_pointer())) }) {
            return false;
        }
        let __rhs = ((*result.borrow()) as u32);
        (*value.borrow()).write(__rhs);
        return true;
    } else if (((*code.borrow()) as i32) == (*kOneMoreByteCode1_15.with(Value::clone).borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ (*(*buf.borrow()).upgrade().deref()).ReadU8((result.as_pointer())) }) {
            return false;
        }
        let __rhs =
            ((((*result.borrow()) as i32) + (*kLowestUCode_16.with(Value::clone).borrow())) as u32);
        (*value.borrow()).write(__rhs);
        return true;
    } else if (((*code.borrow()) as i32) == (*kOneMoreByteCode2_14.with(Value::clone).borrow())) {
        let result: Value<u8> = Rc::new(RefCell::new(0_u8));
        if !({ (*(*buf.borrow()).upgrade().deref()).ReadU8((result.as_pointer())) }) {
            return false;
        }
        let __rhs = ((((*result.borrow()) as i32)
            + ((*kLowestUCode_16.with(Value::clone).borrow()) * 2)) as u32);
        (*value.borrow()).write(__rhs);
        return true;
    } else {
        let __rhs = ((*code.borrow()) as u32);
        (*value.borrow()).write(__rhs);
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
        if !({ (*(*buf.borrow()).upgrade().deref()).ReadU8((code.as_pointer())) }) {
            return false;
        }
        if ((*i.borrow()) == 0_usize) && (((*code.borrow()) as i32) == 128) {
            return false;
        }
        if (((*result.borrow()) & 4261412864_u32) != 0) {
            return false;
        }
        let __rhs = (((*result.borrow()) << 7) | ((((*code.borrow()) as i32) & 127) as u32));
        (*result.borrow_mut()) = __rhs;
        if ((((*code.borrow()) as i32) & 128) == 0) {
            let __rhs = (*result.borrow());
            (*value.borrow()).write(__rhs);
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
        (*dst.borrow())
            .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
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
#[derive(Default)]
pub struct woff2_Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
    pub on_curve: Value<bool>,
}
impl Clone for woff2_Point {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
            on_curve: Rc::new(RefCell::new((*self.on_curve.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_Point {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
        (*self.on_curve.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            on_curve: Rc::new(RefCell::new(<bool>::from_bytes(&buf[8..9]))),
        }
    }
}
#[derive(Default)]
pub struct woff2_Table {
    pub tag: Value<u32>,
    pub flags: Value<u32>,
    pub src_offset: Value<u32>,
    pub src_length: Value<u32>,
    pub transform_length: Value<u32>,
    pub dst_offset: Value<u32>,
    pub dst_length: Value<u32>,
    pub dst_data: Value<Ptr<u8>>,
}
impl woff2_Table {
    pub fn lt(&self, other: Ptr<woff2_Table>) -> bool {
        return {
            let _lhs = (*self.tag.borrow());
            _lhs < (*(*other.upgrade().deref()).tag.borrow())
        };
    }
}
impl Ord for woff2_Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if self.lt(Rc::new(RefCell::new(other.clone())).as_pointer()) {
                std::cmp::Ordering::Less
            } else if other.lt(Rc::new(RefCell::new(self.clone())).as_pointer()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl PartialOrd for woff2_Table {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for woff2_Table {
    fn eq(&self, other: &Self) -> bool {
        {
            !(self.lt(Rc::new(RefCell::new(other.clone())).as_pointer()))
                && !(other.lt(Rc::new(RefCell::new(self.clone())).as_pointer()))
        }
    }
}
impl Eq for woff2_Table {}
impl Clone for woff2_Table {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            flags: Rc::new(RefCell::new((*self.flags.borrow()))),
            src_offset: Rc::new(RefCell::new((*self.src_offset.borrow()))),
            src_length: Rc::new(RefCell::new((*self.src_length.borrow()))),
            transform_length: Rc::new(RefCell::new((*self.transform_length.borrow()))),
            dst_offset: Rc::new(RefCell::new((*self.dst_offset.borrow()))),
            dst_length: Rc::new(RefCell::new((*self.dst_length.borrow()))),
            dst_data: Rc::new(RefCell::new((*self.dst_data.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for woff2_Table {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.flags.borrow()).to_bytes(&mut buf[4..8]);
        (*self.src_offset.borrow()).to_bytes(&mut buf[8..12]);
        (*self.src_length.borrow()).to_bytes(&mut buf[12..16]);
        (*self.transform_length.borrow()).to_bytes(&mut buf[16..20]);
        (*self.dst_offset.borrow()).to_bytes(&mut buf[20..24]);
        (*self.dst_length.borrow()).to_bytes(&mut buf[24..28]);
        (*self.dst_data.borrow()).to_bytes(&mut buf[32..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            flags: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            src_offset: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
            src_length: Rc::new(RefCell::new(<u32>::from_bytes(&buf[12..16]))),
            transform_length: Rc::new(RefCell::new(<u32>::from_bytes(&buf[16..20]))),
            dst_offset: Rc::new(RefCell::new(<u32>::from_bytes(&buf[20..24]))),
            dst_length: Rc::new(RefCell::new(<u32>::from_bytes(&buf[24..28]))),
            dst_data: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[32..40]))),
        }
    }
}
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
        let rhs_0 = (*checksum.borrow()).wrapping_add(
            ((((((((*buf.borrow()).offset((*i.borrow()) as isize).read()) as i32) << 24)
                | ((((*buf.borrow())
                    .offset(((*i.borrow()).wrapping_add(1_usize)) as isize)
                    .read()) as i32)
                    << 16))
                | ((((*buf.borrow())
                    .offset(((*i.borrow()).wrapping_add(2_usize)) as isize)
                    .read()) as i32)
                    << 8))
                | (((*buf.borrow())
                    .offset(((*i.borrow()).wrapping_add(3_usize)) as isize)
                    .read()) as i32)) as u32),
        );
        (*checksum.borrow_mut()) = rhs_0;
        let rhs_0 = (*i.borrow()).wrapping_add(4_usize);
        (*i.borrow_mut()) = rhs_0;
    }
    if ((*size.borrow()) != (*aligned_size.borrow())) {
        let v: Value<u32> = Rc::new(RefCell::new(0_u32));
        let i: Value<usize> = Rc::new(RefCell::new((*aligned_size.borrow())));
        'loop_: while ((*i.borrow()) < (*size.borrow())) {
            (*v.borrow_mut()) |= (({
                let _lhs = (((*buf.borrow()).offset((*i.borrow()) as isize).read()) as i32);
                _lhs << ((24_usize).wrapping_sub((8_usize).wrapping_mul(((*i.borrow()) & 3_usize))))
            }) as u32);
            (*i.borrow_mut()).prefix_inc();
        }
        let rhs_0 = (*checksum.borrow()).wrapping_add((*v.borrow()));
        (*checksum.borrow_mut()) = rhs_0;
    }
    return (*checksum.borrow());
}
pub fn CollectionHeaderSize_27(header_version: u32, num_fonts: u32) -> usize {
    let header_version: Value<u32> = Rc::new(RefCell::new(header_version));
    let num_fonts: Value<u32> = Rc::new(RefCell::new(num_fonts));
    let size: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*header_version.borrow()) == 131072_u32) {
        let rhs_0 = (*size.borrow()).wrapping_add(12_usize);
        (*size.borrow_mut()) = rhs_0;
    }
    if ((*header_version.borrow()) == 65536_u32) || ((*header_version.borrow()) == 131072_u32) {
        let rhs_0 = (*size.borrow()).wrapping_add(
            (((12_u32).wrapping_add((4_u32).wrapping_mul((*num_fonts.borrow())))) as usize),
        );
        (*size.borrow_mut()) = rhs_0;
    }
    return (*size.borrow());
}
#[derive(Default)]
pub struct woff2_Font_Table {
    pub tag: Value<u32>,
    pub checksum: Value<u32>,
    pub offset: Value<u32>,
    pub length: Value<u32>,
    pub data: Value<Ptr<u8>>,
    pub buffer: Value<Vec<u8>>,
    pub reuse_of: Value<Ptr<woff2_Font_Table>>,
    pub flag_byte: Value<u8>,
}
impl Clone for woff2_Font_Table {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            checksum: Rc::new(RefCell::new((*self.checksum.borrow()))),
            offset: Rc::new(RefCell::new((*self.offset.borrow()))),
            length: Rc::new(RefCell::new((*self.length.borrow()))),
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            buffer: Rc::new(RefCell::new((*self.buffer.borrow()).clone())),
            reuse_of: Rc::new(RefCell::new((*self.reuse_of.borrow()).clone())),
            flag_byte: Rc::new(RefCell::new((*self.flag_byte.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_Font_Table {}
#[derive(Default)]
pub struct woff2_Font {
    pub flavor: Value<u32>,
    pub num_tables: Value<u16>,
    pub tables: Value<BTreeMap<u32, Value<woff2_Font_Table>>>,
}
impl Clone for woff2_Font {
    fn clone(&self) -> Self {
        let mut this = Self {
            flavor: Rc::new(RefCell::new((*self.flavor.borrow()))),
            num_tables: Rc::new(RefCell::new((*self.num_tables.borrow()))),
            tables: Rc::new(RefCell::new(
                (*self.tables.borrow())
                    .iter()
                    .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                    .collect(),
            )),
        };
        this
    }
}
impl ByteRepr for woff2_Font {}
#[derive(Default)]
pub struct woff2_FontCollection {
    pub flavor: Value<u32>,
    pub header_version: Value<u32>,
    pub tables: Value<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>,
    pub fonts: Value<Vec<woff2_Font>>,
}
impl Clone for woff2_FontCollection {
    fn clone(&self) -> Self {
        let mut this = Self {
            flavor: Rc::new(RefCell::new((*self.flavor.borrow()))),
            header_version: Rc::new(RefCell::new((*self.header_version.borrow()))),
            tables: Rc::new(RefCell::new(
                (*self.tables.borrow())
                    .iter()
                    .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                    .collect(),
            )),
            fonts: Rc::new(RefCell::new((*self.fonts.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for woff2_FontCollection {}
pub fn StoreU32_28(dst: Ptr<u8>, offset: usize, x: u32) -> usize {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    let x: Value<u32> = Rc::new(RefCell::new(x));
    let __rhs = (((*x.borrow()) >> 24) as u8);
    (*dst.borrow())
        .offset((*offset.borrow()) as isize)
        .write(__rhs);
    let __rhs = (((*x.borrow()) >> 16) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).wrapping_add(1_usize)) as isize)
        .write(__rhs);
    let __rhs = (((*x.borrow()) >> 8) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).wrapping_add(2_usize)) as isize)
        .write(__rhs);
    let __rhs = ((*x.borrow()) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).wrapping_add(3_usize)) as isize)
        .write(__rhs);
    return (*offset.borrow()).wrapping_add(4_usize);
}
pub fn Store16_29(dst: Ptr<u8>, offset: usize, x: i32) -> usize {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let __rhs = (((*x.borrow()) >> 8) as u8);
    (*dst.borrow())
        .offset((*offset.borrow()) as isize)
        .write(__rhs);
    let __rhs = ((*x.borrow()) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).wrapping_add(1_usize)) as isize)
        .write(__rhs);
    return (*offset.borrow()).wrapping_add(2_usize);
}
pub fn StoreU32_30(val: u32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<u32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let __rhs = (((*val.borrow()) >> 24) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let __rhs = (((*val.borrow()) >> 16) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let __rhs = (((*val.borrow()) >> 8) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let __rhs = ((*val.borrow()) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
}
pub fn Store16_31(val: i32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let __rhs = (((*val.borrow()) >> 8) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    let __rhs = ((*val.borrow()) as u8);
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
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
            .memcpy(
                &((*data.borrow()).clone() as Ptr<u8>).to_any(),
                (*len.borrow()) as usize,
            );
        (((*dst.borrow()).offset(((*offset.borrow()).read()) as isize)) as Ptr<u8>)
            .to_any()
            .clone()
    };
    let rhs_0 = ((*offset.borrow()).read()).wrapping_add((*len.borrow()));
    (*offset.borrow()).write(rhs_0);
}
thread_local!();
impl woff2_Font {
    pub fn FindTable_u32(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
            Rc::new(RefCell::new(RefcountMapIter::find_key(
                (self.tables.as_pointer() as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
                &(*tag.borrow()),
            )));
        return if (*it.borrow())
            == RefcountMapIter::end(
                (self.tables.as_pointer() as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            ) {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*it.borrow()).second().as_pointer())
        };
    }
}
impl woff2_Font {
    pub fn FindTable_u32_const(&self, tag: u32) -> Ptr<woff2_Font_Table> {
        let tag: Value<u32> = Rc::new(RefCell::new(tag));
        let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
            Rc::new(RefCell::new(RefcountMapIter::find_key(
                (self.tables.as_pointer() as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
                &(*tag.borrow()),
            )));
        return if (*it.borrow())
            == RefcountMapIter::end(
                (self.tables.as_pointer() as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            ) {
            Ptr::<woff2_Font_Table>::null()
        } else {
            ((*it.borrow()).second().as_pointer())
        };
    }
}
impl woff2_Font {
    pub fn OutputOrderedTags(&self) -> Vec<u32> {
        let output_order: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        'loop_: for i in RefcountMapIter::begin(self.tables.as_pointer()) {
            let table: Ptr<woff2_Font_Table> = i.second().as_pointer();
            if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0) {
                continue 'loop_;
            }
            {
                let a0_clone = (*(*table.upgrade().deref()).tag.borrow()).clone();
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
                            && value_0.read() == (*kGlyfTableTag_0.with(Value::clone).borrow())
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
                            && value_0.read() == (*kLocaTableTag_2.with(Value::clone).borrow())
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
                let idx = (*loca_loc.borrow()).clone().get_offset();
                (output_order.as_pointer() as Ptr<Vec<u32>>)
                    .with_mut(|__v: &mut Vec<u32>| __v.remove(idx));
                (output_order.as_pointer() as Ptr<Vec<u32>>)
                    .to_strong()
                    .as_pointer() as Ptr<u32>
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
                                    && value_0.read()
                                        == (*kGlyfTableTag_0.with(Value::clone).borrow())
                            })
                            .unwrap_or(
                                (output_order.as_pointer() as Ptr<u32>)
                                    .to_end()
                                    .get_offset() as usize,
                            ) as isize,
                    )
                    .offset(1_i64 as isize)
                    .clone()
                    .get_offset();
                (*output_order.borrow_mut())
                    .insert(__off, (*kLocaTableTag_2.with(Value::clone).borrow()));
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
                                    && value_0.read()
                                        == (*kGlyfTableTag_0.with(Value::clone).borrow())
                            })
                            .unwrap_or(
                                (output_order.as_pointer() as Ptr<u32>)
                                    .to_end()
                                    .get_offset() as usize,
                            ) as isize,
                    )
                    .offset(1_i64 as isize)
                    .clone()
            };
        }
        return (*output_order.borrow_mut()).clone();
    }
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
        (*(*file.borrow()).upgrade().deref()).ReadU16(
            ((*(*font.borrow()).upgrade().deref())
                .num_tables
                .as_pointer()),
        )
    })) || (!({ (*(*file.borrow()).upgrade().deref()).Skip(6_usize) }))
    {
        return false;
    }
    let intervals: Value<BTreeMap<u32, Value<u32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let i: Value<u16> = Rc::new(RefCell::new(0_u16));
    'loop_: while {
        let _lhs = ((*i.borrow()) as i32);
        _lhs < ((*(*(*font.borrow()).upgrade().deref()).num_tables.borrow()) as i32)
    } {
        let table: Value<woff2_Font_Table> = Rc::new(RefCell::new(<woff2_Font_Table>::default()));
        (*(*table.borrow()).flag_byte.borrow_mut()) = 0_u8;
        (*(*table.borrow()).reuse_of.borrow_mut()) = Ptr::<woff2_Font_Table>::null();
        if (((!({
            (*(*file.borrow()).upgrade().deref()).ReadU32(((*table.borrow()).tag.as_pointer()))
        })) || (!({
            (*(*file.borrow()).upgrade().deref()).ReadU32(((*table.borrow()).checksum.as_pointer()))
        }))) || (!({
            (*(*file.borrow()).upgrade().deref()).ReadU32(((*table.borrow()).offset.as_pointer()))
        }))) || (!({
            (*(*file.borrow()).upgrade().deref()).ReadU32(((*table.borrow()).length.as_pointer()))
        })) {
            return false;
        }
        if ((((*(*table.borrow()).offset.borrow()) & 3_u32) != 0_u32)
            || (((*(*table.borrow()).length.borrow()) as usize) > (*len.borrow())))
            || ((*len.borrow()).wrapping_sub(((*(*table.borrow()).length.borrow()) as usize))
                < ((*(*table.borrow()).offset.borrow()) as usize))
        {
            return false;
        }
        let __rhs = (*(*table.borrow()).length.borrow());
        (intervals.as_pointer() as Ptr<BTreeMap<u32, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                __v.entry((*(*table.borrow()).offset.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        let __rhs = (*data.borrow()).offset((*(*table.borrow()).offset.borrow()) as isize);
        (*(*table.borrow()).data.borrow_mut()) = __rhs;
        if RefcountMapIter::find_key(
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &(*(*table.borrow()).tag.borrow()),
        ) != RefcountMapIter::end(
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
        ) {
            return false;
        }
        let __rhs = (*table.borrow()).clone();
        ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
            as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                __v.entry((*(*table.borrow()).tag.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let last_offset: Value<u32> =
        Rc::new(RefCell::new(
            (((12_u64 as u64).wrapping_add((16_u64 as u64).wrapping_mul(
                ((*(*(*font.borrow()).upgrade().deref()).num_tables.borrow()) as u64),
            ))) as u32),
        ));
    'loop_: for i in RefcountMapIter::begin(intervals.as_pointer()) {
        if ({
            let _lhs = (*i.first().borrow());
            _lhs < (*last_offset.borrow())
        }) || ({
            let _lhs = (*i.first().borrow()).wrapping_add((*i.second().borrow()));
            _lhs < (*i.first().borrow())
        }) {
            return false;
        }
        (*last_offset.borrow_mut()) = (*i.first().borrow()).wrapping_add((*i.second().borrow()));
    }
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHeadTableTag_1.with(Value::clone).borrow()))
        }),
    ));
    if (!((*head_table.borrow()).is_null()))
        && ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 52_u32)
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
    if !({
        (*(*file.borrow()).upgrade().deref())
            .ReadU32(((*(*font.borrow()).upgrade().deref()).flavor.as_pointer()))
    }) {
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
    'loop_: for entry in
        RefcountMapIter::begin((*(*font.borrow()).upgrade().deref()).tables.as_pointer())
    {
        let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
        if RefcountMapIter::find_key(
            ((*all_tables.borrow()).to_strong().as_pointer()
                as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>),
            &(*(*table.upgrade().deref()).offset.borrow()),
        ) == RefcountMapIter::end(
            ((*all_tables.borrow()).to_strong().as_pointer()
                as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>),
        ) {
            let __rhs = ({
                (*(*font.borrow()).upgrade().deref())
                    .FindTable_u32((*(*table.upgrade().deref()).tag.borrow()))
            });
            ((*all_tables.borrow()).clone() as Ptr<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>| {
                    __v.entry((*(*table.upgrade().deref()).offset.borrow()).clone())
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
                    __v.entry((*(*table.upgrade().deref()).offset.borrow()).clone())
                        .or_insert_with(|| {
                            Rc::new(RefCell::new(<Ptr<woff2_Font_Table>>::default()))
                        })
                        .as_pointer()
                })
                .read())
            .clone();
            (*(*table.upgrade().deref()).reuse_of.borrow_mut()) = __rhs;
            if {
                let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                _lhs != (*(*(*(*table.upgrade().deref()).reuse_of.borrow())
                    .upgrade()
                    .deref())
                .tag
                .borrow())
            } {
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
    let num_fonts: Value<u32> = <Value<u32>>::default();
    if (!({
        (*(*file.borrow()).upgrade().deref()).ReadU32(
            ((*(*font_collection.borrow()).upgrade().deref())
                .header_version
                .as_pointer()),
        )
    })) || (!({ (*(*file.borrow()).upgrade().deref()).ReadU32((num_fonts.as_pointer())) }))
    {
        return false;
    }
    let offsets: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < ((*num_fonts.borrow()) as usize)) {
        let offset: Value<u32> = <Value<u32>>::default();
        if !({ (*(*file.borrow()).upgrade().deref()).ReadU32((offset.as_pointer())) }) {
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
        (*(*(*font_collection.borrow()).upgrade().deref())
            .fonts
            .borrow_mut())
        .resize_with(__a0, || <woff2_Font>::default())
    };
    let font_it: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(
        ((*(*font_collection.borrow()).upgrade().deref())
            .fonts
            .as_pointer() as Ptr<woff2_Font>),
    ));
    let all_tables: Value<BTreeMap<u32, Value<Ptr<woff2_Font_Table>>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    'loop_: for offset in offsets.as_pointer() as Ptr<u32> {
        let offset: Value<u32> = Rc::new(RefCell::new(offset.read().clone()));
        if !({ (*(*file.borrow()).upgrade().deref()).set_offset(((*offset.borrow()) as usize)) }) {
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
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*len.borrow()),
    )));
    if !({ (*file.borrow()).ReadU32(((*(*font.borrow()).upgrade().deref()).flavor.as_pointer())) })
    {
        return false;
    }
    if {
        let _lhs = (*(*(*font.borrow()).upgrade().deref()).flavor.borrow());
        _lhs == (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
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
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*len.borrow()),
    )));
    if !({
        (*file.borrow()).ReadU32(
            ((*(*font_collection.borrow()).upgrade().deref())
                .flavor
                .as_pointer()),
        )
    }) {
        return false;
    }
    if {
        let _lhs = (*(*(*font_collection.borrow()).upgrade().deref())
            .flavor
            .borrow());
        _lhs != (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
        {
            let __a0 = 1_usize as usize;
            (*(*(*font_collection.borrow()).upgrade().deref())
                .fonts
                .borrow_mut())
            .resize_with(__a0, || <woff2_Font>::default())
        };
        let font: Ptr<woff2_Font> = ((*(*font_collection.borrow()).upgrade().deref())
            .fonts
            .as_pointer() as Ptr<woff2_Font>)
            .offset(0_usize);
        (*(*font.upgrade().deref()).flavor.borrow_mut()) =
            (*(*(*font_collection.borrow()).upgrade().deref())
                .flavor
                .borrow());
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
        (((12_u64 as u64).wrapping_add(
            (16_u64 as u64).wrapping_mul(((*(*font.upgrade().deref()).num_tables.borrow()) as u64)),
        )) as usize),
    ));
    'loop_: for i in RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer()) {
        let table: Ptr<woff2_Font_Table> = i.second().as_pointer();
        let padding_size: Value<usize> = Rc::new(RefCell::new(
            ((((4_u32).wrapping_sub(((*(*table.upgrade().deref()).length.borrow()) & 3_u32)))
                & 3_u32) as usize),
        ));
        let end_offset: Value<usize> = Rc::new(RefCell::new(
            ((*padding_size.borrow())
                .wrapping_add(((*(*table.upgrade().deref()).offset.borrow()) as usize)))
            .wrapping_add(((*(*table.upgrade().deref()).length.borrow()) as usize)),
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
    'loop_: for mut font in
        (*font_collection.upgrade().deref()).fonts.as_pointer() as Ptr<woff2_Font>
    {
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
    if {
        let _lhs = (*dst_size.borrow());
        _lhs < ((*offset.borrow()).read())
            .wrapping_add((*kSfntEntrySize_24.with(Value::clone).borrow()))
    } {
        return false;
    }
    if ({ (*(*table.borrow()).upgrade().deref()).IsReused() }) {
        let __rhs = (*(*(*table.borrow()).upgrade().deref()).reuse_of.borrow()).clone();
        (*table.borrow_mut()) = __rhs;
    }
    ({
        StoreU32_30(
            (*(*(*table.borrow()).upgrade().deref()).tag.borrow()),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*(*(*table.borrow()).upgrade().deref()).checksum.borrow()),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*(*(*table.borrow()).upgrade().deref()).offset.borrow()),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*(*(*table.borrow()).upgrade().deref()).length.borrow()),
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
    if !({ (*table.upgrade().deref()).IsReused() }) {
        if ({
            let _lhs = (*(*table.upgrade().deref()).offset.borrow())
                .wrapping_add((*(*table.upgrade().deref()).length.borrow()));
            _lhs < (*(*table.upgrade().deref()).offset.borrow())
        }) || ({
            let _lhs = (*dst_size.borrow());
            _lhs < (((*(*table.upgrade().deref()).offset.borrow())
                .wrapping_add((*(*table.upgrade().deref()).length.borrow())))
                as usize)
        }) {
            return false;
        }
        {
            ((*dst.borrow()).offset((*(*table.upgrade().deref()).offset.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .memcpy(
                    &((*(*table.upgrade().deref()).data.borrow()).clone() as Ptr<u8>).to_any(),
                    ((*(*table.upgrade().deref()).length.borrow()) as usize) as usize,
                );
            ((*dst.borrow()).offset((*(*table.upgrade().deref()).offset.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .clone()
        };
        let padding_size: Value<usize> = Rc::new(RefCell::new(
            ((((4_u32).wrapping_sub(((*(*table.upgrade().deref()).length.borrow()) & 3_u32)))
                & 3_u32) as usize),
        ));
        if ({
            let _lhs = (((*(*table.upgrade().deref()).offset.borrow())
                .wrapping_add((*(*table.upgrade().deref()).length.borrow())))
                as usize)
                .wrapping_add((*padding_size.borrow()));
            _lhs < (*padding_size.borrow())
        }) || ({
            let _lhs = (*dst_size.borrow());
            _lhs < (((*(*table.upgrade().deref()).offset.borrow())
                .wrapping_add((*(*table.upgrade().deref()).length.borrow())))
                as usize)
                .wrapping_add((*padding_size.borrow()))
        }) {
            return false;
        }
        {
            ((*dst.borrow())
                .offset((*(*table.upgrade().deref()).offset.borrow()) as isize)
                .offset((*(*table.upgrade().deref()).length.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .memset((0) as u8, (*padding_size.borrow()) as usize);
            ((*dst.borrow())
                .offset((*(*table.upgrade().deref()).offset.borrow()) as isize)
                .offset((*(*table.upgrade().deref()).length.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .clone()
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
    if {
        let _lhs = ((*dst_size.borrow()) as u64);
        _lhs < (12_u64 as u64).wrapping_add(
            (16_u64 as u64).wrapping_mul(((*(*font.upgrade().deref()).num_tables.borrow()) as u64)),
        )
    } {
        return false;
    }
    ({
        StoreU32_30(
            (*(*font.upgrade().deref()).flavor.borrow()),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*(*font.upgrade().deref()).num_tables.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    let max_pow2: Value<u16> = Rc::new(RefCell::new(
        (if ((*(*font.upgrade().deref()).num_tables.borrow()) != 0) {
            ({ Log2Floor_25(((*(*font.upgrade().deref()).num_tables.borrow()) as u32)) })
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
        (({
            let _lhs = (((*(*font.upgrade().deref()).num_tables.borrow()) as i32) << 4);
            _lhs - ((*search_range.borrow()) as i32)
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
    'loop_: for i in RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer()) {
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
    if {
        let _lhs = (*(*font_collection.upgrade().deref()).flavor.borrow());
        _lhs != (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
        return ({
            WriteFont_41(
                ((*font_collection.upgrade().deref()).fonts.as_pointer() as Ptr<woff2_Font>)
                    .offset(0_usize),
                (offset.as_pointer()),
                (*dst.borrow()).clone(),
                (*dst_size.borrow()),
            )
        });
    }
    ({
        StoreU32_30(
            (*kTtcFontFlavor_22.with(Value::clone).borrow()),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*(*font_collection.upgrade().deref()).header_version.borrow()),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            ((*(*font_collection.upgrade().deref()).fonts.borrow()).len() as u32),
            (offset.as_pointer()),
            (*dst.borrow()).clone(),
        )
    });
    let offset_table: Value<usize> = Rc::new(RefCell::new((*offset.borrow())));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*font_collection.upgrade().deref()).fonts.borrow()).len()
    } {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        (*i.borrow_mut()).postfix_inc();
    }
    if ((*(*font_collection.upgrade().deref()).header_version.borrow()) == 131072_u32) {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*dst.borrow()).clone()) });
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*font_collection.upgrade().deref()).fonts.borrow()).len()
    } {
        let font: Ptr<woff2_Font> = ((*font_collection.upgrade().deref()).fonts.as_pointer()
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
            let _tag: u32 = (*kHeadTableTag_1.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = (*kLocaTableTag_2.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    if (((*head_table.borrow()).is_null()) || ((*loca_table.borrow()).is_null()))
        || ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 52_u32)
    {
        return 0;
    }
    let index_fmt: Value<i32> = Rc::new(RefCell::new(({ IndexFormat_46((font).clone()) })));
    let loca_record_size: Value<i32> = Rc::new(RefCell::new(
        (if ((*index_fmt.borrow()) == 0) { 2 } else { 4 }),
    ));
    if {
        let _lhs = (*(*(*loca_table.borrow()).upgrade().deref()).length.borrow());
        _lhs < ((*loca_record_size.borrow()) as u32)
    } {
        return 0;
    }
    return ((((*(*(*loca_table.borrow()).upgrade().deref()).length.borrow())
        .wrapping_div(((*loca_record_size.borrow()) as u32)))
    .wrapping_sub(1_u32)) as i32);
}
pub fn IndexFormat_46(font: Ptr<woff2_Font>) -> i32 {
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = (*kHeadTableTag_1.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    if (*head_table.borrow()).is_null() {
        return 0;
    }
    return (((*(*(*head_table.borrow()).upgrade().deref()).data.borrow())
        .offset((51) as isize)
        .read()) as i32);
}
impl woff2_Font_Table {
    pub fn IsReused(&self) -> bool {
        return (!((*self.reuse_of.borrow()).is_null())).clone();
    }
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
            let _tag: u32 = (*kHeadTableTag_1.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = (*kLocaTableTag_2.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            let _tag: u32 = (*kGlyfTableTag_0.with(Value::clone).borrow());
            (*font.upgrade().deref()).FindTable_u32_const(_tag)
        }),
    ));
    if ((((*head_table.borrow()).is_null()) || ((*loca_table.borrow()).is_null()))
        || ((*glyf_table.borrow()).is_null()))
        || ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 52_u32)
    {
        return false;
    }
    let index_fmt: Value<i32> = Rc::new(RefCell::new(({ IndexFormat_46((font).clone()) })));
    let loca_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(*loca_table.borrow()).upgrade().deref()).data.borrow()).clone(),
        ((*(*(*loca_table.borrow()).upgrade().deref()).length.borrow()) as usize),
    )));
    if ((*index_fmt.borrow()) == 0) {
        let offset1: Value<u16> = <Value<u16>>::default();
        let offset2: Value<u16> = <Value<u16>>::default();
        if ((((!({ (*loca_buf.borrow()).Skip(((2 * (*glyph_index.borrow())) as usize)) }))
            || (!({ (*loca_buf.borrow()).ReadU16((offset1.as_pointer())) })))
            || (!({ (*loca_buf.borrow()).ReadU16((offset2.as_pointer())) })))
            || (((*offset2.borrow()) as i32) < ((*offset1.borrow()) as i32)))
            || ({
                let _lhs = ((2 * ((*offset2.borrow()) as i32)) as u32);
                _lhs > (*(*(*glyf_table.borrow()).upgrade().deref()).length.borrow())
            })
        {
            return false;
        }
        let __rhs = (*(*(*glyf_table.borrow()).upgrade().deref()).data.borrow())
            .offset((2 * ((*offset1.borrow()) as i32)) as isize);
        (*glyph_data.borrow()).write(__rhs);
        let __rhs = ((2 * (((*offset2.borrow()) as i32) - ((*offset1.borrow()) as i32))) as usize);
        (*glyph_size.borrow()).write(__rhs);
    } else {
        let offset1: Value<u32> = <Value<u32>>::default();
        let offset2: Value<u32> = <Value<u32>>::default();
        if ((((!({ (*loca_buf.borrow()).Skip(((4 * (*glyph_index.borrow())) as usize)) }))
            || (!({ (*loca_buf.borrow()).ReadU32((offset1.as_pointer())) })))
            || (!({ (*loca_buf.borrow()).ReadU32((offset2.as_pointer())) })))
            || ((*offset2.borrow()) < (*offset1.borrow())))
            || ({
                let _lhs = (*offset2.borrow());
                _lhs > (*(*(*glyf_table.borrow()).upgrade().deref()).length.borrow())
            })
        {
            return false;
        }
        let __rhs = (*(*(*glyf_table.borrow()).upgrade().deref()).data.borrow())
            .offset((*offset1.borrow()) as isize);
        (*glyph_data.borrow()).write(__rhs);
        let __rhs = (((*offset2.borrow()).wrapping_sub((*offset1.borrow()))) as usize);
        (*glyph_size.borrow()).write(__rhs);
    }
    return true;
}
pub fn RemoveDigitalSignature_48(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let it: Value<RefcountMapIter<u32, woff2_Font_Table>> =
        Rc::new(RefCell::new(RefcountMapIter::find_key(
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &(*kDsigTableTag_3.with(Value::clone).borrow()),
        )));
    if (*it.borrow())
        != RefcountMapIter::end(
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
        )
    {
        RefcountMapIter::erase(
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>),
            &(*it.borrow()).clone(),
        );
        let __rhs = ((*(*(*font.borrow()).upgrade().deref()).tables.borrow()).len() as u16);
        (*(*(*font.borrow()).upgrade().deref())
            .num_tables
            .borrow_mut()) = __rhs;
    }
    return true;
}
#[derive(Default)]
pub struct woff2_Glyph_Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
    pub on_curve: Value<bool>,
}
impl Clone for woff2_Glyph_Point {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
            on_curve: Rc::new(RefCell::new((*self.on_curve.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_Glyph_Point {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
        (*self.on_curve.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            on_curve: Rc::new(RefCell::new(<bool>::from_bytes(&buf[8..9]))),
        }
    }
}
#[derive()]
pub struct woff2_Glyph {
    pub x_min: Value<i16>,
    pub x_max: Value<i16>,
    pub y_min: Value<i16>,
    pub y_max: Value<i16>,
    pub instructions_size: Value<u16>,
    pub instructions_data: Value<Ptr<u8>>,
    pub overlap_simple_flag_set: Value<bool>,
    pub contours: Value<Vec<Value<Vec<woff2_Glyph_Point>>>>,
    pub composite_data: Value<Ptr<u8>>,
    pub composite_data_size: Value<u32>,
    pub have_instructions: Value<bool>,
}
impl woff2_Glyph {
    pub fn woff2_Glyph() -> Self {
        let mut this = Self {
            x_min: <Value<i16>>::default(),
            x_max: <Value<i16>>::default(),
            y_min: <Value<i16>>::default(),
            y_max: <Value<i16>>::default(),
            instructions_size: Rc::new(RefCell::new(0_u16)),
            instructions_data: Rc::new(RefCell::new(Ptr::<u8>::null())),
            overlap_simple_flag_set: Rc::new(RefCell::new(false)),
            contours: Rc::new(RefCell::new(Vec::new())),
            composite_data: Rc::new(RefCell::new(Ptr::<u8>::null())),
            composite_data_size: Rc::new(RefCell::new(0_u32)),
            have_instructions: <Value<bool>>::default(),
        };
        this
    }
}
impl Clone for woff2_Glyph {
    fn clone(&self) -> Self {
        let mut this = Self {
            x_min: Rc::new(RefCell::new((*self.x_min.borrow()))),
            x_max: Rc::new(RefCell::new((*self.x_max.borrow()))),
            y_min: Rc::new(RefCell::new((*self.y_min.borrow()))),
            y_max: Rc::new(RefCell::new((*self.y_max.borrow()))),
            instructions_size: Rc::new(RefCell::new((*self.instructions_size.borrow()))),
            instructions_data: Rc::new(RefCell::new((*self.instructions_data.borrow()).clone())),
            overlap_simple_flag_set: Rc::new(RefCell::new(
                (*self.overlap_simple_flag_set.borrow()),
            )),
            contours: Rc::new(RefCell::new(
                (*self.contours.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
            composite_data: Rc::new(RefCell::new((*self.composite_data.borrow()).clone())),
            composite_data_size: Rc::new(RefCell::new((*self.composite_data_size.borrow()))),
            have_instructions: Rc::new(RefCell::new((*self.have_instructions.borrow()))),
        };
        this
    }
}
impl Default for woff2_Glyph {
    fn default() -> Self {
        { woff2_Glyph::woff2_Glyph() }
    }
}
impl ByteRepr for woff2_Glyph {}
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
pub fn ReadCompositeGlyphData_62(buffer: Ptr<woff2_Buffer>, glyph: Ptr<woff2_Glyph>) -> bool {
    let buffer: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(buffer));
    let glyph: Value<Ptr<woff2_Glyph>> = Rc::new(RefCell::new(glyph));
    (*(*(*glyph.borrow()).upgrade().deref())
        .have_instructions
        .borrow_mut()) = false;
    (*(*(*glyph.borrow()).upgrade().deref())
        .composite_data
        .borrow_mut()) = ({ (*(*buffer.borrow()).upgrade().deref()).buffer() })
        .offset(({ (*(*buffer.borrow()).upgrade().deref()).offset() }) as isize);
    let start_offset: Value<usize> = Rc::new(RefCell::new(
        ({ (*(*buffer.borrow()).upgrade().deref()).offset() }),
    ));
    let flags: Value<u16> = Rc::new(RefCell::new(
        ((*kFLAG_MORE_COMPONENTS_58.with(Value::clone).borrow()) as u16),
    ));
    'loop_: while ((((*flags.borrow()) as i32)
        & (*kFLAG_MORE_COMPONENTS_58.with(Value::clone).borrow()))
        != 0)
    {
        if !({ (*(*buffer.borrow()).upgrade().deref()).ReadU16((flags.as_pointer())) }) {
            return false;
        }
        let rhs_0 = (((*(*(*glyph.borrow()).upgrade().deref())
            .have_instructions
            .borrow()) as i32)
            | (((((*flags.borrow()) as i32)
                & (*kFLAG_WE_HAVE_INSTRUCTIONS_61.with(Value::clone).borrow()))
                != 0) as i32))
            != 0;
        (*(*(*glyph.borrow()).upgrade().deref())
            .have_instructions
            .borrow_mut()) = rhs_0;
        let arg_size: Value<usize> = Rc::new(RefCell::new(2_usize));
        if ((((*flags.borrow()) as i32)
            & (*kFLAG_ARG_1_AND_2_ARE_WORDS_56.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(4_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(2_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        }
        if ((((*flags.borrow()) as i32) & (*kFLAG_WE_HAVE_A_SCALE_57.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(2_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else if ((((*flags.borrow()) as i32)
            & (*kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59
                .with(Value::clone)
                .borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(4_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else if ((((*flags.borrow()) as i32)
            & (*kFLAG_WE_HAVE_A_TWO_BY_TWO_60.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(8_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        }
        if !({ (*(*buffer.borrow()).upgrade().deref()).Skip((*arg_size.borrow())) }) {
            return false;
        }
    }
    if {
        let _lhs = ({ (*(*buffer.borrow()).upgrade().deref()).offset() })
            .wrapping_sub((*start_offset.borrow()));
        _lhs > (<u32>::MAX as usize)
    } {
        return false;
    }
    (*(*(*glyph.borrow()).upgrade().deref())
        .composite_data_size
        .borrow_mut()) = ((({ (*(*buffer.borrow()).upgrade().deref()).offset() })
        .wrapping_sub((*start_offset.borrow()))) as u32);
    return true;
}
pub fn ReadGlyph_63(data: Ptr<u8>, len: usize, glyph: Ptr<woff2_Glyph>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let glyph: Value<Ptr<woff2_Glyph>> = Rc::new(RefCell::new(glyph));
    let buffer: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*len.borrow()),
    )));
    let num_contours: Value<i16> = <Value<i16>>::default();
    if !({ (*buffer.borrow()).ReadS16((num_contours.as_pointer())) }) {
        return false;
    }
    if (((!({
        (*buffer.borrow()).ReadS16(((*(*glyph.borrow()).upgrade().deref()).x_min.as_pointer()))
    })) || (!({
        (*buffer.borrow()).ReadS16(((*(*glyph.borrow()).upgrade().deref()).y_min.as_pointer()))
    }))) || (!({
        (*buffer.borrow()).ReadS16(((*(*glyph.borrow()).upgrade().deref()).x_max.as_pointer()))
    }))) || (!({
        (*buffer.borrow()).ReadS16(((*(*glyph.borrow()).upgrade().deref()).y_max.as_pointer()))
    })) {
        return false;
    }
    if (((*num_contours.borrow()) as i32) == 0) {
        return true;
    }
    if (((*num_contours.borrow()) as i32) > 0) {
        {
            let _a0 = ((*num_contours.borrow()) as usize) as usize;
            ((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                as Ptr<Vec<Value<Vec<woff2_Glyph_Point>>>>)
                .with_mut(|__v: &mut Vec<Value<Vec<woff2_Glyph_Point>>>| {
                    __v.resize_with(_a0, <Value<Vec<woff2_Glyph_Point>>>::default)
                })
        };
        let last_point_index: Value<u16> = Rc::new(RefCell::new(0_u16));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*num_contours.borrow()) as i32)) {
            let point_index: Value<u16> = <Value<u16>>::default();
            if !({ (*buffer.borrow()).ReadU16((point_index.as_pointer())) }) {
                return false;
            }
            let num_points: Value<u16> = Rc::new(RefCell::new(
                (((((*point_index.borrow()) as i32) - ((*last_point_index.borrow()) as i32))
                    + (if ((*i.borrow()) == 0) { 1 } else { 0 })) as u16),
            ));
            {
                let __a0 = ((*num_points.borrow()) as usize) as usize;
                ((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .with_mut(|__v: &mut Value<Vec<woff2_Glyph_Point>>| {
                        (*__v.borrow_mut()).resize_with(__a0, || <woff2_Glyph_Point>::default())
                    })
            };
            (*last_point_index.borrow_mut()) = (*point_index.borrow());
            (*i.borrow_mut()).prefix_inc();
        }
        if !({
            (*buffer.borrow()).ReadU16(
                ((*(*glyph.borrow()).upgrade().deref())
                    .instructions_size
                    .as_pointer()),
            )
        }) {
            return false;
        }
        (*(*(*glyph.borrow()).upgrade().deref())
            .instructions_data
            .borrow_mut()) = (*data.borrow()).offset(({ (*buffer.borrow()).offset() }) as isize);
        if !({
            (*buffer.borrow()).Skip(
                ((*(*(*glyph.borrow()).upgrade().deref())
                    .instructions_size
                    .borrow()) as usize),
            )
        }) {
            return false;
        }
        let flags: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(
            (0..((*num_contours.borrow()) as usize) as usize)
                .map(|_| <Value<Vec<u8>>>::default())
                .collect::<Vec<_>>(),
        ));
        let flag: Value<u8> = Rc::new(RefCell::new(0_u8));
        let flag_repeat: Value<u8> = Rc::new(RefCell::new(0_u8));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*num_contours.borrow()) as i32)) {
            {
                let __a0 = (*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                    .upgrade()
                    .deref())
                .len() as usize;
                (flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                    .offset(((*i.borrow()) as usize))
                    .with_mut(|__v: &mut Value<Vec<u8>>| {
                        (*__v.borrow_mut()).resize_with(__a0, || <u8>::default())
                    })
            };
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while {
                let _lhs = (*j.borrow());
                _lhs < (*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                    .upgrade()
                    .deref())
                .len()
            } {
                if (((*flag_repeat.borrow()) as i32) == 0) {
                    if !({ (*buffer.borrow()).ReadU8((flag.as_pointer())) }) {
                        return false;
                    }
                    if ((((*flag.borrow()) as i32)
                        & (*kFLAG_REPEAT_52.with(Value::clone).borrow()))
                        != 0)
                    {
                        if !({ (*buffer.borrow()).ReadU8((flag_repeat.as_pointer())) }) {
                            return false;
                        }
                    }
                } else {
                    (*flag_repeat.borrow_mut()).postfix_dec();
                }
                ((flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<u8>)
                    .offset((*j.borrow()))
                    .write((*flag.borrow()));
                (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<woff2_Glyph_Point>)
                    .offset((*j.borrow()))
                    .upgrade()
                    .deref())
                .on_curve
                .borrow_mut()) = ((((*flag.borrow()) as i32)
                    & (*kFLAG_ONCURVE_49.with(Value::clone).borrow()))
                    != 0);
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        if (!(*flags.borrow()).is_empty())
            && (!(*((flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<u8>>)
                .upgrade()
                .deref())
            .is_empty())
        {
            (*(*(*glyph.borrow()).upgrade().deref())
                .overlap_simple_flag_set
                .borrow_mut()) = ((((((flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>)
                .offset(0_usize)
                .read()) as i32)
                & (*kFLAG_OVERLAP_SIMPLE_55.with(Value::clone).borrow()))
                != 0);
        }
        let prev_x: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*num_contours.borrow()) as i32)) {
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while {
                let _lhs = (*j.borrow());
                _lhs < (*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                    .upgrade()
                    .deref())
                .len()
            } {
                let flag: Value<u8> = Rc::new(RefCell::new(
                    (((flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<u8>)
                        .offset((*j.borrow()))
                        .read()),
                ));
                if ((((*flag.borrow()) as i32) & (*kFLAG_XSHORT_50.with(Value::clone).borrow()))
                    != 0)
                {
                    let x_delta: Value<u8> = <Value<u8>>::default();
                    if !({ (*buffer.borrow()).ReadU8((x_delta.as_pointer())) }) {
                        return false;
                    }
                    let sign: Value<i32> = Rc::new(RefCell::new(
                        if ((((*flag.borrow()) as i32)
                            & (*kFLAG_XREPEATSIGN_53.with(Value::clone).borrow()))
                            != 0)
                        {
                            1
                        } else {
                            -1_i32
                        },
                    ));
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .x
                    .borrow_mut()) =
                        ((*prev_x.borrow()) + ((*sign.borrow()) * ((*x_delta.borrow()) as i32)));
                } else {
                    let x_delta: Value<i16> = Rc::new(RefCell::new(0_i16));
                    if !((((*flag.borrow()) as i32)
                        & (*kFLAG_XREPEATSIGN_53.with(Value::clone).borrow()))
                        != 0)
                    {
                        if !({ (*buffer.borrow()).ReadS16((x_delta.as_pointer())) }) {
                            return false;
                        }
                    }
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .x
                    .borrow_mut()) = ((*prev_x.borrow()) + ((*x_delta.borrow()) as i32));
                }
                (*prev_x.borrow_mut()) =
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .x
                    .borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let prev_y: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < ((*num_contours.borrow()) as i32)) {
            let j: Value<usize> = Rc::new(RefCell::new(0_usize));
            'loop_: while {
                let _lhs = (*j.borrow());
                _lhs < (*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                    .upgrade()
                    .deref())
                .len()
            } {
                let flag: Value<u8> = Rc::new(RefCell::new(
                    (((flags.as_pointer() as Ptr<Value<Vec<u8>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<u8>)
                        .offset((*j.borrow()))
                        .read()),
                ));
                if ((((*flag.borrow()) as i32) & (*kFLAG_YSHORT_51.with(Value::clone).borrow()))
                    != 0)
                {
                    let y_delta: Value<u8> = <Value<u8>>::default();
                    if !({ (*buffer.borrow()).ReadU8((y_delta.as_pointer())) }) {
                        return false;
                    }
                    let sign: Value<i32> = Rc::new(RefCell::new(
                        if ((((*flag.borrow()) as i32)
                            & (*kFLAG_YREPEATSIGN_54.with(Value::clone).borrow()))
                            != 0)
                        {
                            1
                        } else {
                            -1_i32
                        },
                    ));
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .y
                    .borrow_mut()) =
                        ((*prev_y.borrow()) + ((*sign.borrow()) * ((*y_delta.borrow()) as i32)));
                } else {
                    let y_delta: Value<i16> = Rc::new(RefCell::new(0_i16));
                    if !((((*flag.borrow()) as i32)
                        & (*kFLAG_YREPEATSIGN_54.with(Value::clone).borrow()))
                        != 0)
                    {
                        if !({ (*buffer.borrow()).ReadS16((y_delta.as_pointer())) }) {
                            return false;
                        }
                    }
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .y
                    .borrow_mut()) = ((*prev_y.borrow()) + ((*y_delta.borrow()) as i32));
                }
                (*prev_y.borrow_mut()) =
                    (*(*(((*(*glyph.borrow()).upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset((*j.borrow()))
                        .upgrade()
                        .deref())
                    .y
                    .borrow());
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
    } else if (((*num_contours.borrow()) as i32) == -1_i32) {
        if !({ ReadCompositeGlyphData_62((buffer.as_pointer()), (*glyph.borrow()).clone()) }) {
            return false;
        }
        if (*(*(*glyph.borrow()).upgrade().deref())
            .have_instructions
            .borrow())
        {
            if !({
                (*buffer.borrow()).ReadU16(
                    ((*(*glyph.borrow()).upgrade().deref())
                        .instructions_size
                        .as_pointer()),
                )
            }) {
                return false;
            }
            (*(*(*glyph.borrow()).upgrade().deref())
                .instructions_data
                .borrow_mut()) =
                (*data.borrow()).offset(({ (*buffer.borrow()).offset() }) as isize);
            if !({
                (*buffer.borrow()).Skip(
                    ((*(*(*glyph.borrow()).upgrade().deref())
                        .instructions_size
                        .borrow()) as usize),
                )
            }) {
                return false;
            }
        } else {
            (*(*(*glyph.borrow()).upgrade().deref())
                .instructions_size
                .borrow_mut()) = 0_u16;
        }
    } else {
        return false;
    }
    return true;
}
pub fn StoreBbox_64(glyph: Ptr<woff2_Glyph>, offset: Ptr<usize>, dst: Ptr<u8>) {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    ({
        Store16_31(
            ((*(*glyph.upgrade().deref()).x_min.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*(*glyph.upgrade().deref()).y_min.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*(*glyph.upgrade().deref()).x_max.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*(*glyph.upgrade().deref()).y_max.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
}
pub fn StoreInstructions_65(glyph: Ptr<woff2_Glyph>, offset: Ptr<usize>, dst: Ptr<u8>) {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    ({
        Store16_31(
            ((*(*glyph.upgrade().deref()).instructions_size.borrow()) as i32),
            (*offset.borrow()).clone(),
            (*dst.borrow()).clone(),
        )
    });
    ({
        let _data: Ptr<u8> = (*(*glyph.upgrade().deref()).instructions_data.borrow()).clone();
        let _len: usize = ((*(*glyph.upgrade().deref()).instructions_size.borrow()) as usize);
        let _offset: Ptr<usize> = (*offset.borrow()).clone();
        StoreBytes_32(_data, _len, _offset, (*dst.borrow()).clone())
    });
}
pub fn StoreEndPtsOfContours_66(glyph: Ptr<woff2_Glyph>, offset: Ptr<usize>, dst: Ptr<u8>) -> bool {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let end_point: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: for mut contour in
        (*glyph.upgrade().deref()).contours.as_pointer() as Ptr<Value<Vec<woff2_Glyph_Point>>>
    {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.upgrade().deref().as_pointer();
        let rhs_0 = (((*end_point.borrow()) as usize)
            .wrapping_add((*contour.upgrade().deref()).len())) as i32;
        (*end_point.borrow_mut()) = rhs_0;
        if ({
            let _lhs = (*contour.upgrade().deref()).len();
            _lhs > (<u16>::MAX as usize)
        }) || ((*end_point.borrow()) > (<u16>::MAX as i32))
        {
            return false;
        }
        ({
            Store16_31(
                (*end_point.borrow()),
                (*offset.borrow()).clone(),
                (*dst.borrow()).clone(),
            )
        });
    }
    return true;
}
pub fn StorePoints_67(
    glyph: Ptr<woff2_Glyph>,
    offset: Ptr<usize>,
    dst: Ptr<u8>,
    dst_size: usize,
) -> bool {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let previous_flag: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let repeat_count: Value<i32> = Rc::new(RefCell::new(0));
    let last_x: Value<i32> = Rc::new(RefCell::new(0));
    let last_y: Value<i32> = Rc::new(RefCell::new(0));
    let x_bytes: Value<usize> = Rc::new(RefCell::new(0_usize));
    let y_bytes: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for mut contour in
        (*glyph.upgrade().deref()).contours.as_pointer() as Ptr<Value<Vec<woff2_Glyph_Point>>>
    {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.upgrade().deref().as_pointer();
        'loop_: for mut point in contour.to_strong().as_pointer() as Ptr<woff2_Glyph_Point> {
            let flag: Value<i32> = Rc::new(RefCell::new(
                if (*(*point.upgrade().deref()).on_curve.borrow()) {
                    (*kFLAG_ONCURVE_49.with(Value::clone).borrow())
                } else {
                    0
                },
            ));
            if ((*previous_flag.borrow()) == -1_i32)
                && (*(*glyph.upgrade().deref()).overlap_simple_flag_set.borrow())
            {
                let __rhs =
                    ((*flag.borrow()) | (*kFLAG_OVERLAP_SIMPLE_55.with(Value::clone).borrow()));
                (*flag.borrow_mut()) = __rhs;
            }
            let dx: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*(*point.upgrade().deref()).x.borrow());
                _lhs - (*last_x.borrow())
            }));
            let dy: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*(*point.upgrade().deref()).y.borrow());
                _lhs - (*last_y.borrow())
            }));
            if ((*dx.borrow()) == 0) {
                (*flag.borrow_mut()) |= (*kFLAG_XREPEATSIGN_53.with(Value::clone).borrow());
            } else if ((*dx.borrow()) > -256_i32) && ((*dx.borrow()) < 256) {
                (*flag.borrow_mut()) |= ((*kFLAG_XSHORT_50.with(Value::clone).borrow())
                    | (if ((*dx.borrow()) > 0) {
                        (*kFLAG_XREPEATSIGN_53.with(Value::clone).borrow())
                    } else {
                        0
                    }));
                let rhs_0 = (*x_bytes.borrow()).wrapping_add(1_usize);
                (*x_bytes.borrow_mut()) = rhs_0;
            } else {
                let rhs_0 = (*x_bytes.borrow()).wrapping_add(2_usize);
                (*x_bytes.borrow_mut()) = rhs_0;
            }
            if ((*dy.borrow()) == 0) {
                (*flag.borrow_mut()) |= (*kFLAG_YREPEATSIGN_54.with(Value::clone).borrow());
            } else if ((*dy.borrow()) > -256_i32) && ((*dy.borrow()) < 256) {
                (*flag.borrow_mut()) |= ((*kFLAG_YSHORT_51.with(Value::clone).borrow())
                    | (if ((*dy.borrow()) > 0) {
                        (*kFLAG_YREPEATSIGN_54.with(Value::clone).borrow())
                    } else {
                        0
                    }));
                let rhs_0 = (*y_bytes.borrow()).wrapping_add(1_usize);
                (*y_bytes.borrow_mut()) = rhs_0;
            } else {
                let rhs_0 = (*y_bytes.borrow()).wrapping_add(2_usize);
                (*y_bytes.borrow_mut()) = rhs_0;
            }
            if ((*flag.borrow()) == (*previous_flag.borrow())) && ((*repeat_count.borrow()) != 255)
            {
                let rhs_0 = ((((*dst.borrow())
                    .offset((((*offset.borrow()).read()).wrapping_sub(1_usize)) as isize)
                    .read()) as i32)
                    | (*kFLAG_REPEAT_52.with(Value::clone).borrow()))
                    as u8;
                (*dst.borrow())
                    .offset((((*offset.borrow()).read()).wrapping_sub(1_usize)) as isize)
                    .write(rhs_0);
                (*repeat_count.borrow_mut()).postfix_inc();
            } else {
                if ((*repeat_count.borrow()) != 0) {
                    if {
                        let _lhs = ((*offset.borrow()).read());
                        _lhs >= (*dst_size.borrow())
                    } {
                        return false;
                    }
                    let __rhs = ((*repeat_count.borrow()) as u8);
                    (*dst.borrow())
                        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
                        .write(__rhs);
                }
                if {
                    let _lhs = ((*offset.borrow()).read());
                    _lhs >= (*dst_size.borrow())
                } {
                    return false;
                }
                let __rhs = ((*flag.borrow()) as u8);
                (*dst.borrow())
                    .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
                    .write(__rhs);
                (*repeat_count.borrow_mut()) = 0;
            }
            (*last_x.borrow_mut()) = (*(*point.upgrade().deref()).x.borrow());
            (*last_y.borrow_mut()) = (*(*point.upgrade().deref()).y.borrow());
            (*previous_flag.borrow_mut()) = (*flag.borrow());
        }
    }
    if ((*repeat_count.borrow()) != 0) {
        if {
            let _lhs = ((*offset.borrow()).read());
            _lhs >= (*dst_size.borrow())
        } {
            return false;
        }
        let __rhs = ((*repeat_count.borrow()) as u8);
        (*dst.borrow())
            .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
            .write(__rhs);
    }
    if {
        let _lhs = (((*offset.borrow()).read()).wrapping_add((*x_bytes.borrow())))
            .wrapping_add((*y_bytes.borrow()));
        _lhs > (*dst_size.borrow())
    } {
        return false;
    }
    let x_offset: Value<usize> = Rc::new(RefCell::new(((*offset.borrow()).read())));
    let y_offset: Value<usize> = Rc::new(RefCell::new(
        ((*offset.borrow()).read()).wrapping_add((*x_bytes.borrow())),
    ));
    (*last_x.borrow_mut()) = 0;
    (*last_y.borrow_mut()) = 0;
    'loop_: for mut contour in
        (*glyph.upgrade().deref()).contours.as_pointer() as Ptr<Value<Vec<woff2_Glyph_Point>>>
    {
        let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.upgrade().deref().as_pointer();
        'loop_: for mut point in contour.to_strong().as_pointer() as Ptr<woff2_Glyph_Point> {
            let dx: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*(*point.upgrade().deref()).x.borrow());
                _lhs - (*last_x.borrow())
            }));
            let dy: Value<i32> = Rc::new(RefCell::new({
                let _lhs = (*(*point.upgrade().deref()).y.borrow());
                _lhs - (*last_y.borrow())
            }));
            if ((*dx.borrow()) == 0) {
            } else if ((*dx.borrow()) > -256_i32) && ((*dx.borrow()) < 256) {
                let __rhs = ((*dx.borrow()).abs() as u8);
                (*dst.borrow())
                    .offset(((*x_offset.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            } else {
                ({
                    Store16_31(
                        (*dx.borrow()),
                        (x_offset.as_pointer()),
                        (*dst.borrow()).clone(),
                    )
                });
            }
            if ((*dy.borrow()) == 0) {
            } else if ((*dy.borrow()) > -256_i32) && ((*dy.borrow()) < 256) {
                let __rhs = ((*dy.borrow()).abs() as u8);
                (*dst.borrow())
                    .offset(((*y_offset.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            } else {
                ({
                    Store16_31(
                        (*dy.borrow()),
                        (y_offset.as_pointer()),
                        (*dst.borrow()).clone(),
                    )
                });
            }
            (*last_x.borrow_mut()) += (*dx.borrow());
            (*last_y.borrow_mut()) += (*dy.borrow());
        }
    }
    let __rhs = (*y_offset.borrow());
    (*offset.borrow()).write(__rhs);
    return true;
}
pub fn StoreGlyph_68(glyph: Ptr<woff2_Glyph>, dst: Ptr<u8>, dst_size: Ptr<usize>) -> bool {
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<Ptr<usize>> = Rc::new(RefCell::new(dst_size));
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*(*glyph.upgrade().deref()).composite_data_size.borrow()) > 0_u32) {
        if {
            let _lhs = (((*dst_size.borrow()).read()) as u64);
            _lhs < (((10_u64 as u64)
                .wrapping_add(((*(*glyph.upgrade().deref()).composite_data_size.borrow()) as u64)))
            .wrapping_add(
                ((if (*(*glyph.upgrade().deref()).have_instructions.borrow()) {
                    2_u64
                } else {
                    0_u64
                })
                .wrapping_add(((*(*glyph.upgrade().deref()).instructions_size.borrow()) as u64))),
            ))
        } {
            return false;
        }
        ({ Store16_31(-1_i32, (offset.as_pointer()), (*dst.borrow()).clone()) });
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            StoreBbox_64(_glyph, _offset, _dst)
        });
        ({
            let _data: Ptr<u8> = (*(*glyph.upgrade().deref()).composite_data.borrow()).clone();
            let _len: usize = ((*(*glyph.upgrade().deref()).composite_data_size.borrow()) as usize);
            StoreBytes_32(_data, _len, (offset.as_pointer()), (*dst.borrow()).clone())
        });
        if (*(*glyph.upgrade().deref()).have_instructions.borrow()) {
            ({
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                let _offset: Ptr<usize> = (offset.as_pointer());
                let _dst: Ptr<u8> = (*dst.borrow()).clone();
                StoreInstructions_65(_glyph, _offset, _dst)
            });
        }
    } else if ((*(*glyph.upgrade().deref()).contours.borrow()).len() > 0_usize) {
        if {
            let _lhs = (*(*glyph.upgrade().deref()).contours.borrow()).len();
            _lhs > (<i16>::MAX as usize)
        } {
            return false;
        }
        if {
            let _lhs = (((*dst_size.borrow()).read()) as u64);
            _lhs < (((12_u64 as u64).wrapping_add(
                (((2_usize).wrapping_mul((*(*glyph.upgrade().deref()).contours.borrow()).len()))
                    as u64),
            ))
            .wrapping_add(((*(*glyph.upgrade().deref()).instructions_size.borrow()) as u64)))
        } {
            return false;
        }
        ({
            Store16_31(
                ((*(*glyph.upgrade().deref()).contours.borrow()).len() as i32),
                (offset.as_pointer()),
                (*dst.borrow()).clone(),
            )
        });
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            StoreBbox_64(_glyph, _offset, _dst)
        });
        if !({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            StoreEndPtsOfContours_66(_glyph, _offset, _dst)
        }) {
            return false;
        }
        ({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            StoreInstructions_65(_glyph, _offset, _dst)
        });
        if !({
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*dst.borrow()).clone();
            let _dst_size: usize = ((*dst_size.borrow()).read());
            StorePoints_67(_glyph, _offset, _dst, _dst_size)
        }) {
            return false;
        }
    }
    let __rhs = (*offset.borrow());
    (*dst_size.borrow()).write(__rhs);
    return true;
}
pub fn Round4_69(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ((<i32>::MAX - (*value.borrow())) < 3) {
        return (*value.borrow());
    }
    return (((*value.borrow()) + 3) & !3);
}
pub fn Round4_70(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    if ((<u64>::MAX as u64).wrapping_sub((*value.borrow())) < 3_u64) {
        return (*value.borrow());
    }
    return (((*value.borrow()).wrapping_add(3_u64)) & (!3 as u64));
}
pub fn Round4_71(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((<u32>::MAX as u32).wrapping_sub((*value.borrow())) < 3_u32) {
        return (*value.borrow());
    }
    return (((*value.borrow()).wrapping_add(3_u32)) & (!3 as u32));
}
thread_local!();
pub fn StoreLoca_72(index_fmt: i32, value: u32, offset: Ptr<usize>, dst: Ptr<u8>) {
    let index_fmt: Value<i32> = Rc::new(RefCell::new(index_fmt));
    let value: Value<u32> = Rc::new(RefCell::new(value));
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    if ((*index_fmt.borrow()) == 0) {
        ({
            Store16_31(
                (((*value.borrow()) >> 1) as i32),
                (*offset.borrow()).clone(),
                (*dst.borrow()).clone(),
            )
        });
    } else {
        ({
            StoreU32_30(
                (*value.borrow()),
                (*offset.borrow()).clone(),
                (*dst.borrow()).clone(),
            )
        });
    }
}
pub fn WriteNormalizedLoca_73(index_fmt: i32, num_glyphs: i32, font: Ptr<woff2_Font>) -> bool {
    let index_fmt: Value<i32> = Rc::new(RefCell::new(index_fmt));
    let num_glyphs: Value<i32> = Rc::new(RefCell::new(num_glyphs));
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kGlyfTableTag_0.with(Value::clone).borrow()))
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kLocaTableTag_2.with(Value::clone).borrow()))
        }),
    ));
    let glyph_sz: Value<i32> = Rc::new(RefCell::new(if ((*index_fmt.borrow()) == 0) {
        2
    } else {
        4
    }));
    {
        let __a0 = ((({ Round4_69(((*num_glyphs.borrow()) + 1)) }) * (*glyph_sz.borrow())) as usize)
            as usize;
        (*(*(*loca_table.borrow()).upgrade().deref())
            .buffer
            .borrow_mut())
        .resize_with(__a0, || <u8>::default())
    };
    (*(*(*loca_table.borrow()).upgrade().deref())
        .length
        .borrow_mut()) = ((((*num_glyphs.borrow()) + 1) * (*glyph_sz.borrow())) as u32);
    let glyf_dst: Value<Ptr<u8>> = Rc::new(RefCell::new(if ((*num_glyphs.borrow()) != 0) {
        (((*(*glyf_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(0_usize))
    } else {
        Ptr::<u8>::null()
    }));
    let loca_dst: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*loca_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(0_usize)),
    ));
    let glyf_offset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let loca_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*num_glyphs.borrow())) {
        ({
            StoreLoca_72(
                (*index_fmt.borrow()),
                (*glyf_offset.borrow()),
                (loca_offset.as_pointer()),
                (*loca_dst.borrow()).clone(),
            )
        });
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::woff2_Glyph()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (*font.borrow()).clone();
            let _glyph_index: i32 = (*i.borrow());
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
            ((*(*(*glyf_table.borrow()).upgrade().deref()).buffer.borrow()).len())
                .wrapping_sub(((*glyf_offset.borrow()) as usize)),
        ));
        if !({
            StoreGlyph_68(
                glyph.as_pointer(),
                (*glyf_dst.borrow()).offset((*glyf_offset.borrow()) as isize),
                (glyf_dst_size.as_pointer()),
            )
        }) {
            return false;
        }
        let __rhs = (({ Round4_70(((*glyf_dst_size.borrow()) as u64)) }) as usize);
        (*glyf_dst_size.borrow_mut()) = __rhs;
        if (((*glyf_dst_size.borrow()) > (<u32>::MAX as usize))
            || ((*glyf_offset.borrow()).wrapping_add(((*glyf_dst_size.borrow()) as u32))
                < (*glyf_offset.borrow())))
            || (((*index_fmt.borrow()) == 0)
                && (((*glyf_offset.borrow()) as usize).wrapping_add((*glyf_dst_size.borrow()))
                    >= ((1_u64 << 17) as usize)))
        {
            return false;
        }
        let rhs_0 =
            (((*glyf_offset.borrow()) as usize).wrapping_add((*glyf_dst_size.borrow()))) as u32;
        (*glyf_offset.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        StoreLoca_72(
            (*index_fmt.borrow()),
            (*glyf_offset.borrow()),
            (loca_offset.as_pointer()),
            (*loca_dst.borrow()).clone(),
        )
    });
    {
        let __a0 = ((*glyf_offset.borrow()) as usize) as usize;
        (*(*(*glyf_table.borrow()).upgrade().deref())
            .buffer
            .borrow_mut())
        .resize_with(__a0, || <u8>::default())
    };
    let __rhs = if ((*glyf_offset.borrow()) != 0) {
        (((*(*glyf_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(0_usize))
    } else {
        Ptr::<u8>::null()
    };
    (*(*(*glyf_table.borrow()).upgrade().deref())
        .data
        .borrow_mut()) = __rhs;
    (*(*(*glyf_table.borrow()).upgrade().deref())
        .length
        .borrow_mut()) = (*glyf_offset.borrow());
    let __rhs = if ((*loca_offset.borrow()) != 0) {
        (((*(*loca_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(0_usize))
    } else {
        Ptr::<u8>::null()
    };
    (*(*(*loca_table.borrow()).upgrade().deref())
        .data
        .borrow_mut()) = __rhs;
    return true;
}
pub fn MakeEditableBuffer_74(font: Ptr<woff2_Font>, tableTag: i32) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let tableTag: Value<i32> = Rc::new(RefCell::new(tableTag));
    let table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({ (*(*font.borrow()).upgrade().deref()).FindTable_u32(((*tableTag.borrow()) as u32)) }),
    ));
    if (*table.borrow()).is_null() {
        return false;
    }
    if ({ (*(*table.borrow()).upgrade().deref()).IsReused() }) {
        return true;
    }
    let sz: Value<i32> = Rc::new(RefCell::new(
        (({ Round4_71((*(*(*table.borrow()).upgrade().deref()).length.borrow())) }) as i32),
    ));
    {
        let __a0 = ((*sz.borrow()) as usize) as usize;
        (*(*(*table.borrow()).upgrade().deref()).buffer.borrow_mut())
            .resize_with(__a0, || <u8>::default())
    };
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*table.borrow()).upgrade().deref()).buffer.as_pointer() as Ptr<u8>).offset(0_usize)),
    ));
    {
        ((*buf.borrow()).clone() as Ptr<u8>).to_any().memcpy(
            &((*(*(*table.borrow()).upgrade().deref()).data.borrow()).clone() as Ptr<u8>).to_any(),
            ((*(*(*table.borrow()).upgrade().deref()).length.borrow()) as usize) as usize,
        );
        ((*buf.borrow()).clone() as Ptr<u8>).to_any().clone()
    };
    if ((({
        let _lhs = ((*sz.borrow()) as u32);
        _lhs > (*(*(*table.borrow()).upgrade().deref()).length.borrow())
    }) as i64)
        != 0)
    {
        {
            ((*buf.borrow())
                .offset((*(*(*table.borrow()).upgrade().deref()).length.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .memset(
                    (0) as u8,
                    ((((*sz.borrow()) as u32)
                        .wrapping_sub((*(*(*table.borrow()).upgrade().deref()).length.borrow())))
                        as usize) as usize,
                );
            ((*buf.borrow())
                .offset((*(*(*table.borrow()).upgrade().deref()).length.borrow()) as isize)
                as Ptr<u8>)
                .to_any()
                .clone()
        };
    }
    (*(*(*table.borrow()).upgrade().deref()).data.borrow_mut()) = (*buf.borrow()).clone();
    return true;
}
pub fn NormalizeGlyphs_75(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHeadTableTag_1.with(Value::clone).borrow()))
        }),
    ));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kGlyfTableTag_0.with(Value::clone).borrow()))
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kLocaTableTag_2.with(Value::clone).borrow()))
        }),
    ));
    if (*head_table.borrow()).is_null() {
        return false;
    }
    if ((*loca_table.borrow()).is_null()) && ((*glyf_table.borrow()).is_null()) {
        return true;
    }
    if {
        let _lhs = (((*glyf_table.borrow()).is_null()) as i32).clone();
        _lhs != (((*loca_table.borrow()).is_null()) as i32).clone()
    } {
        return false;
    }
    if {
        let _lhs = (({ (*(*loca_table.borrow()).upgrade().deref()).IsReused() }) as i32);
        _lhs != (({ (*(*glyf_table.borrow()).upgrade().deref()).IsReused() }) as i32)
    } {
        return false;
    }
    if ({ (*(*loca_table.borrow()).upgrade().deref()).IsReused() }) {
        return true;
    }
    let index_fmt: Value<i32> = Rc::new(RefCell::new(
        (((*(*(*head_table.borrow()).upgrade().deref()).data.borrow())
            .offset((51) as isize)
            .read()) as i32),
    ));
    let num_glyphs: Value<i32> =
        Rc::new(RefCell::new(({ NumGlyphs_45((*font.borrow()).clone()) })));
    let max_normalized_glyf_size: Value<usize> = Rc::new(RefCell::new(
        (({
            let _lhs =
                (1.1E+0 * ((*(*(*glyf_table.borrow()).upgrade().deref()).length.borrow()) as f64));
            _lhs + ((2 * (*num_glyphs.borrow())) as f64)
        }) as usize),
    ));
    {
        let __a0 = (*max_normalized_glyf_size.borrow()) as usize;
        (*(*(*glyf_table.borrow()).upgrade().deref())
            .buffer
            .borrow_mut())
        .resize_with(__a0, || <u8>::default())
    };
    if !({
        WriteNormalizedLoca_73(
            (*index_fmt.borrow()),
            (*num_glyphs.borrow()),
            (*font.borrow()).clone(),
        )
    }) {
        if ((*index_fmt.borrow()) != 0) {
            return false;
        }
        (*index_fmt.borrow_mut()) = 1;
        if !({
            WriteNormalizedLoca_73(
                (*index_fmt.borrow()),
                (*num_glyphs.borrow()),
                (*font.borrow()).clone(),
            )
        }) {
            return false;
        }
        ((*(*head_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(51_usize)
            .write(1_u8);
    }
    return true;
}
pub fn NormalizeOffsets_76(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let offset: Value<u32> = Rc::new(RefCell::new(
        ((12 + (16 * ((*(*(*font.borrow()).upgrade().deref()).num_tables.borrow()) as i32)))
            as u32),
    ));
    'loop_: for mut tag in Rc::new(RefCell::new(
        ({ (*(*font.borrow()).upgrade().deref()).OutputOrderedTags() }),
    ))
    .as_pointer() as Ptr<u32>
    {
        let tag: Value<u32> = Rc::new(RefCell::new(tag.read().clone()));
        let table: Ptr<woff2_Font_Table> =
            ((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                    __v.entry((*tag.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                        .as_pointer()
                });
        (*(*table.upgrade().deref()).offset.borrow_mut()) = (*offset.borrow());
        let rhs_0 = (*offset.borrow())
            .wrapping_add(({ Round4_71((*(*table.upgrade().deref()).length.borrow())) }));
        (*offset.borrow_mut()) = rhs_0;
    }
    return true;
}
pub fn ComputeHeaderChecksum_77(font: Ptr<woff2_Font>) -> u32 {
    let checksum: Value<u32> = Rc::new(RefCell::new((*(*font.upgrade().deref()).flavor.borrow())));
    let max_pow2: Value<u16> = Rc::new(RefCell::new(
        (if ((*(*font.upgrade().deref()).num_tables.borrow()) != 0) {
            ({ Log2Floor_25(((*(*font.upgrade().deref()).num_tables.borrow()) as u32)) })
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
        (({
            let _lhs = (((*(*font.upgrade().deref()).num_tables.borrow()) as i32) << 4);
            _lhs - ((*search_range.borrow()) as i32)
        }) as u16),
    ));
    let rhs_0 = (*checksum.borrow()).wrapping_add(
        (({
            let _lhs = (((*(*font.upgrade().deref()).num_tables.borrow()) as i32) << 16);
            _lhs | ((*search_range.borrow()) as i32)
        }) as u32),
    );
    (*checksum.borrow_mut()) = rhs_0;
    let rhs_0 = (*checksum.borrow()).wrapping_add(
        (((((*max_pow2.borrow()) as i32) << 16) | ((*range_shift.borrow()) as i32)) as u32),
    );
    (*checksum.borrow_mut()) = rhs_0;
    'loop_: for i in RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer()) {
        let table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new((i.second().as_pointer())));
        if ({ (*(*table.borrow()).upgrade().deref()).IsReused() }) {
            let __rhs = (*(*(*table.borrow()).upgrade().deref()).reuse_of.borrow()).clone();
            (*table.borrow_mut()) = __rhs;
        }
        let rhs_0 = (*checksum.borrow())
            .wrapping_add((*(*(*table.borrow()).upgrade().deref()).tag.borrow()));
        (*checksum.borrow_mut()) = rhs_0;
        let rhs_0 = (*checksum.borrow())
            .wrapping_add((*(*(*table.borrow()).upgrade().deref()).checksum.borrow()));
        (*checksum.borrow_mut()) = rhs_0;
        let rhs_0 = (*checksum.borrow())
            .wrapping_add((*(*(*table.borrow()).upgrade().deref()).offset.borrow()));
        (*checksum.borrow_mut()) = rhs_0;
        let rhs_0 = (*checksum.borrow())
            .wrapping_add((*(*(*table.borrow()).upgrade().deref()).length.borrow()));
        (*checksum.borrow_mut()) = rhs_0;
    }
    return (*checksum.borrow());
}
pub fn FixChecksums_78(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHeadTableTag_1.with(Value::clone).borrow()))
        }),
    ));
    if (*head_table.borrow()).is_null() {
        return false;
    }
    if !((*(*(*head_table.borrow()).upgrade().deref())
        .reuse_of
        .borrow())
    .is_null())
    {
        let __rhs = (*(*(*head_table.borrow()).upgrade().deref())
            .reuse_of
            .borrow())
        .clone();
        (*head_table.borrow_mut()) = __rhs;
    }
    if ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 12_u32) {
        return false;
    }
    let head_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*head_table.borrow()).upgrade().deref())
            .buffer
            .as_pointer() as Ptr<u8>)
            .offset(0_usize)),
    ));
    let offset: Value<usize> = Rc::new(RefCell::new(8_usize));
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (*head_buf.borrow()).clone()) });
    let file_checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    let head_checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: for i in
        RefcountMapIter::begin((*(*font.borrow()).upgrade().deref()).tables.as_pointer())
    {
        let table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new((i.second().as_pointer())));
        if ({ (*(*table.borrow()).upgrade().deref()).IsReused() }) {
            let __rhs = (*(*(*table.borrow()).upgrade().deref()).reuse_of.borrow()).clone();
            (*table.borrow_mut()) = __rhs;
        }
        let __rhs = ({
            let _buf: Ptr<u8> = (*(*(*table.borrow()).upgrade().deref()).data.borrow()).clone();
            let _size: usize = ((*(*(*table.borrow()).upgrade().deref()).length.borrow()) as usize);
            ComputeULongSum_26(_buf, _size)
        });
        (*(*(*table.borrow()).upgrade().deref()).checksum.borrow_mut()) = __rhs;
        let rhs_0 = (*file_checksum.borrow())
            .wrapping_add((*(*(*table.borrow()).upgrade().deref()).checksum.borrow()));
        (*file_checksum.borrow_mut()) = rhs_0;
        if {
            let _lhs = (*(*(*table.borrow()).upgrade().deref()).tag.borrow());
            _lhs == (*kHeadTableTag_1.with(Value::clone).borrow())
        } {
            (*head_checksum.borrow_mut()) =
                (*(*(*table.borrow()).upgrade().deref()).checksum.borrow());
        }
    }
    let rhs_0 = (*file_checksum.borrow())
        .wrapping_add(({ ComputeHeaderChecksum_77((*font.borrow()).clone()) }));
    (*file_checksum.borrow_mut()) = rhs_0;
    (*offset.borrow_mut()) = 8_usize;
    ({
        StoreU32_30(
            (2981146554_u32 as u32).wrapping_sub((*file_checksum.borrow())),
            (offset.as_pointer()),
            (*head_buf.borrow()).clone(),
        )
    });
    return true;
}
pub fn MarkTransformed_79(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHeadTableTag_1.with(Value::clone).borrow()))
        }),
    ));
    if (*head_table.borrow()).is_null() {
        return false;
    }
    if !((*(*(*head_table.borrow()).upgrade().deref())
        .reuse_of
        .borrow())
    .is_null())
    {
        let __rhs = (*(*(*head_table.borrow()).upgrade().deref())
            .reuse_of
            .borrow())
        .clone();
        (*head_table.borrow_mut()) = __rhs;
    }
    if ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 17_u32) {
        return false;
    }
    let head_flags: Value<i32> = Rc::new(RefCell::new(
        (((*(*(*head_table.borrow()).upgrade().deref()).data.borrow())
            .offset((16) as isize)
            .read()) as i32),
    ));
    ((*(*head_table.borrow()).upgrade().deref())
        .buffer
        .as_pointer() as Ptr<u8>)
        .offset(16_usize)
        .write((((*head_flags.borrow()) | 8) as u8));
    return true;
}
pub fn NormalizeWithoutFixingChecksums_80(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    return ((((({
        MakeEditableBuffer_74(
            (*font.borrow()).clone(),
            ((*kHeadTableTag_1.with(Value::clone).borrow()) as i32),
        )
    }) && ({ RemoveDigitalSignature_48((*font.borrow()).clone()) }))
        && ({ MarkTransformed_79((*font.borrow()).clone()) }))
        && ({ NormalizeGlyphs_75((*font.borrow()).clone()) }))
        && ({ NormalizeOffsets_76((*font.borrow()).clone()) }));
}
pub fn NormalizeFont_81(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    return (({ NormalizeWithoutFixingChecksums_80((*font.borrow()).clone()) })
        && ({ FixChecksums_78((*font.borrow()).clone()) }));
}
pub fn NormalizeFontCollection_82(font_collection: Ptr<woff2_FontCollection>) -> bool {
    let font_collection: Value<Ptr<woff2_FontCollection>> = Rc::new(RefCell::new(font_collection));
    if ((*(*(*font_collection.borrow()).upgrade().deref())
        .fonts
        .borrow())
    .len()
        == 1_usize)
    {
        return ({
            NormalizeFont_81(
                (((*(*font_collection.borrow()).upgrade().deref())
                    .fonts
                    .as_pointer() as Ptr<woff2_Font>)
                    .offset(0_usize)),
            )
        });
    }
    let offset: Value<u32> = Rc::new(RefCell::new(
        (({
            let _header_version: u32 = (*(*(*font_collection.borrow()).upgrade().deref())
                .header_version
                .borrow());
            let _num_fonts: u32 = ((*(*(*font_collection.borrow()).upgrade().deref())
                .fonts
                .borrow())
            .len() as u32);
            CollectionHeaderSize_27(_header_version, _num_fonts)
        }) as u32),
    ));
    'loop_: for mut font in (*(*font_collection.borrow()).upgrade().deref())
        .fonts
        .as_pointer() as Ptr<woff2_Font>
    {
        if !({ NormalizeWithoutFixingChecksums_80((font).clone()) }) {
            eprintln!("Font normalization failed.");
            return false;
        }
        let rhs_0 = (((*offset.borrow()) as usize).wrapping_add(
            (*kSfntHeaderSize_23.with(Value::clone).borrow()).wrapping_add(
                (*kSfntEntrySize_24.with(Value::clone).borrow())
                    .wrapping_mul(((*(*font.upgrade().deref()).num_tables.borrow()) as usize)),
            ),
        )) as u32;
        (*offset.borrow_mut()) = rhs_0;
    }
    'loop_: for mut font in (*(*font_collection.borrow()).upgrade().deref())
        .fonts
        .as_pointer() as Ptr<woff2_Font>
    {
        'loop_: for mut tag in Rc::new(RefCell::new(
            ({ (*font.upgrade().deref()).OutputOrderedTags() }),
        ))
        .as_pointer() as Ptr<u32>
        {
            let tag: Value<u32> = Rc::new(RefCell::new(tag.read().clone()));
            let table: Ptr<woff2_Font_Table> = ((*font.upgrade().deref()).tables.as_pointer()
                as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                    __v.entry((*tag.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                        .as_pointer()
                });
            if ({ (*table.upgrade().deref()).IsReused() }) {
                let __rhs = (*(*(*(*table.upgrade().deref()).reuse_of.borrow())
                    .upgrade()
                    .deref())
                .offset
                .borrow());
                (*(*table.upgrade().deref()).offset.borrow_mut()) = __rhs;
            } else {
                (*(*table.upgrade().deref()).offset.borrow_mut()) = (*offset.borrow());
                let rhs_0 = (*offset.borrow())
                    .wrapping_add(({ Round4_71((*(*table.upgrade().deref()).length.borrow())) }));
                (*offset.borrow_mut()) = rhs_0;
            }
        }
    }
    'loop_: for mut font in (*(*font_collection.borrow()).upgrade().deref())
        .fonts
        .as_pointer() as Ptr<woff2_Font>
    {
        if !({ FixChecksums_78((font).clone()) }) {
            eprintln!("Failed to fix checksums");
            return false;
        }
    }
    return true;
}
thread_local!();
thread_local!(
    pub static FLAG_ARG_1_AND_2_ARE_WORDS_83: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
thread_local!(
    pub static FLAG_WE_HAVE_INSTRUCTIONS_84: Value<i32> = Rc::new(RefCell::new((1 << 8)));
);
thread_local!(
    pub static FLAG_OVERLAP_SIMPLE_BITMAP_85: Value<i32> = Rc::new(RefCell::new((1 << 0)));
);
pub fn WriteBytes_86(out: Ptr<Vec<u8>>, data: Ptr<u8>, len: usize) {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    if ((*len.borrow()) == 0_usize) {
        return;
    }
    let offset: Value<usize> = Rc::new(RefCell::new((*(*out.borrow()).upgrade().deref()).len()));
    {
        let __a0 = (*offset.borrow()).wrapping_add((*len.borrow())) as usize;
        (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.resize_with(__a0, || <u8>::default()))
    };
    {
        (((((*out.borrow()).to_strong().as_pointer()) as Ptr<u8>).offset((*offset.borrow())))
            as Ptr<u8>)
            .to_any()
            .memcpy(
                &((*data.borrow()).clone() as Ptr<u8>).to_any(),
                (*len.borrow()) as usize,
            );
        (((((*out.borrow()).to_strong().as_pointer()) as Ptr<u8>).offset((*offset.borrow())))
            as Ptr<u8>)
            .to_any()
            .clone()
    };
}
pub fn WriteBytes_87(out: Ptr<Vec<u8>>, in_: Ptr<Vec<u8>>) {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*in_.upgrade().deref()).len()
    } {
        {
            let a0_clone = ((in_.to_strong().as_pointer() as Ptr<u8>)
                .offset((*i.borrow()))
                .read())
            .clone();
            (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(a0_clone))
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn WriteUShort_88(out: Ptr<Vec<u8>>, value: i32) {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) >> 8) as u8)));
    (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) & 255) as u8)));
}
pub fn WriteLong_89(out: Ptr<Vec<u8>>, value: i32) {
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(out));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    (*out.borrow())
        .with_mut(|__v: &mut Vec<u8>| __v.push(((((*value.borrow()) >> 24) & 255) as u8)));
    (*out.borrow())
        .with_mut(|__v: &mut Vec<u8>| __v.push(((((*value.borrow()) >> 16) & 255) as u8)));
    (*out.borrow())
        .with_mut(|__v: &mut Vec<u8>| __v.push(((((*value.borrow()) >> 8) & 255) as u8)));
    (*out.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push((((*value.borrow()) & 255) as u8)));
}
#[derive(Default)]
pub struct woff2_GlyfEncoder {
    n_contour_stream_: Value<Vec<u8>>,
    n_points_stream_: Value<Vec<u8>>,
    flag_byte_stream_: Value<Vec<u8>>,
    composite_stream_: Value<Vec<u8>>,
    bbox_bitmap_: Value<Vec<u8>>,
    bbox_stream_: Value<Vec<u8>>,
    glyph_stream_: Value<Vec<u8>>,
    instruction_stream_: Value<Vec<u8>>,
    overlap_bitmap_: Value<Vec<u8>>,
    n_glyphs_: Value<i32>,
}
impl woff2_GlyfEncoder {
    pub fn woff2_GlyfEncoder(num_glyphs: i32) -> Self {
        let num_glyphs: Value<i32> = Rc::new(RefCell::new(num_glyphs));
        let mut this = Self {
            n_contour_stream_: Rc::new(RefCell::new(Vec::new())),
            n_points_stream_: Rc::new(RefCell::new(Vec::new())),
            flag_byte_stream_: Rc::new(RefCell::new(Vec::new())),
            composite_stream_: Rc::new(RefCell::new(Vec::new())),
            bbox_bitmap_: Rc::new(RefCell::new(Vec::new())),
            bbox_stream_: Rc::new(RefCell::new(Vec::new())),
            glyph_stream_: Rc::new(RefCell::new(Vec::new())),
            instruction_stream_: Rc::new(RefCell::new(Vec::new())),
            overlap_bitmap_: Rc::new(RefCell::new(Vec::new())),
            n_glyphs_: Rc::new(RefCell::new((*num_glyphs.borrow()))),
        };
        {
            let __a0 = (((((*num_glyphs.borrow()) + 31) >> 5) << 2) as usize) as usize;
            (*this.bbox_bitmap_.borrow_mut()).resize_with(__a0, || <u8>::default())
        };
        this
    }
    pub fn Encode(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>) -> bool {
        let glyph_id: Value<i32> = Rc::new(RefCell::new(glyph_id));
        if ((*(*glyph.upgrade().deref()).composite_data_size.borrow()) > 0_u32) {
            ({
                let _glyph_id: i32 = (*glyph_id.borrow());
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                self.WriteCompositeGlyph(_glyph_id, _glyph)
            });
        } else if ((*(*glyph.upgrade().deref()).contours.borrow()).len() > 0_usize) {
            ({
                let _glyph_id: i32 = (*glyph_id.borrow());
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                self.WriteSimpleGlyph(_glyph_id, _glyph)
            });
        } else {
            ({ WriteUShort_88((self.n_contour_stream_.as_pointer()), 0) });
        }
        return true;
    }
    pub fn GetTransformedGlyfBytes(&self, result: Ptr<Vec<u8>>) {
        let result: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(result));
        ({ WriteUShort_88((*result.borrow()).clone(), 0) });
        ({
            WriteUShort_88(
                (*result.borrow()).clone(),
                if (*self.overlap_bitmap_.borrow()).is_empty() {
                    0
                } else {
                    (*FLAG_OVERLAP_SIMPLE_BITMAP_85.with(Value::clone).borrow())
                },
            )
        });
        ({ WriteUShort_88((*result.borrow()).clone(), (*self.n_glyphs_.borrow())) });
        ({ WriteUShort_88((*result.borrow()).clone(), 0) });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.n_contour_stream_.borrow()).len() as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.n_points_stream_.borrow()).len() as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.flag_byte_stream_.borrow()).len() as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.glyph_stream_.borrow()).len() as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.composite_stream_.borrow()).len() as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((((*self.bbox_bitmap_.borrow()).len())
                    .wrapping_add((*self.bbox_stream_.borrow()).len())) as i32),
            )
        });
        ({
            WriteLong_89(
                (*result.borrow()).clone(),
                ((*self.instruction_stream_.borrow()).len() as i32),
            )
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.n_contour_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.n_points_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.flag_byte_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.glyph_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.composite_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.bbox_bitmap_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.bbox_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        ({
            let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
            let _in: Ptr<Vec<u8>> = self.instruction_stream_.as_pointer();
            WriteBytes_87(_out, _in)
        });
        if !(*self.overlap_bitmap_.borrow()).is_empty() {
            ({
                let _out: Ptr<Vec<u8>> = (*result.borrow()).clone();
                let _in: Ptr<Vec<u8>> = self.overlap_bitmap_.as_pointer();
                WriteBytes_87(_out, _in)
            });
        }
    }
    fn WriteInstructions(&self, glyph: Ptr<woff2_Glyph>) {
        ({
            Write255UShort_10(
                (self.glyph_stream_.as_pointer()),
                ((*(*glyph.upgrade().deref()).instructions_size.borrow()) as i32),
            )
        });
        ({
            let _data: Ptr<u8> = (*(*glyph.upgrade().deref()).instructions_data.borrow()).clone();
            let _len: usize = ((*(*glyph.upgrade().deref()).instructions_size.borrow()) as usize);
            WriteBytes_86((self.instruction_stream_.as_pointer()), _data, _len)
        });
    }
    fn ShouldWriteSimpleGlyphBbox(&self, glyph: Ptr<woff2_Glyph>) -> bool {
        if ((*(*glyph.upgrade().deref()).contours.borrow()).is_empty())
            || ((*(((*glyph.upgrade().deref()).contours.as_pointer()
                as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                .upgrade()
                .deref())
            .is_empty())
        {
            return ((((*(*glyph.upgrade().deref()).x_min.borrow()) != 0)
                || ((*(*glyph.upgrade().deref()).y_min.borrow()) != 0))
                || ((*(*glyph.upgrade().deref()).x_max.borrow()) != 0))
                || ((*(*glyph.upgrade().deref()).y_max.borrow()) != 0);
        }
        let x_min: Value<i16> = Rc::new(RefCell::new(
            ((*(*(((*glyph.upgrade().deref()).contours.as_pointer()
                as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<woff2_Glyph_Point>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .x
            .borrow()) as i16),
        ));
        let y_min: Value<i16> = Rc::new(RefCell::new(
            ((*(*(((*glyph.upgrade().deref()).contours.as_pointer()
                as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<woff2_Glyph_Point>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .y
            .borrow()) as i16),
        ));
        let x_max: Value<i16> = Rc::new(RefCell::new((*x_min.borrow())));
        let y_max: Value<i16> = Rc::new(RefCell::new((*y_min.borrow())));
        'loop_: for mut contour in
            (*glyph.upgrade().deref()).contours.as_pointer() as Ptr<Value<Vec<woff2_Glyph_Point>>>
        {
            let contour: Ptr<Vec<woff2_Glyph_Point>> = contour.upgrade().deref().as_pointer();
            'loop_: for mut point in contour.to_strong().as_pointer() as Ptr<woff2_Glyph_Point> {
                if {
                    let _lhs = (*(*point.upgrade().deref()).x.borrow());
                    _lhs < ((*x_min.borrow()) as i32)
                } {
                    (*x_min.borrow_mut()) = ((*(*point.upgrade().deref()).x.borrow()) as i16);
                }
                if {
                    let _lhs = (*(*point.upgrade().deref()).x.borrow());
                    _lhs > ((*x_max.borrow()) as i32)
                } {
                    (*x_max.borrow_mut()) = ((*(*point.upgrade().deref()).x.borrow()) as i16);
                }
                if {
                    let _lhs = (*(*point.upgrade().deref()).y.borrow());
                    _lhs < ((*y_min.borrow()) as i32)
                } {
                    (*y_min.borrow_mut()) = ((*(*point.upgrade().deref()).y.borrow()) as i16);
                }
                if {
                    let _lhs = (*(*point.upgrade().deref()).y.borrow());
                    _lhs > ((*y_max.borrow()) as i32)
                } {
                    (*y_max.borrow_mut()) = ((*(*point.upgrade().deref()).y.borrow()) as i16);
                }
            }
        }
        if {
            let _lhs = ((*(*glyph.upgrade().deref()).x_min.borrow()) as i32);
            _lhs != ((*x_min.borrow()) as i32)
        } {
            return true;
        }
        if {
            let _lhs = ((*(*glyph.upgrade().deref()).y_min.borrow()) as i32);
            _lhs != ((*y_min.borrow()) as i32)
        } {
            return true;
        }
        if {
            let _lhs = ((*(*glyph.upgrade().deref()).x_max.borrow()) as i32);
            _lhs != ((*x_max.borrow()) as i32)
        } {
            return true;
        }
        if {
            let _lhs = ((*(*glyph.upgrade().deref()).y_max.borrow()) as i32);
            _lhs != ((*y_max.borrow()) as i32)
        } {
            return true;
        }
        return false;
    }
    fn WriteSimpleGlyph(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        let glyph_id: Value<i32> = Rc::new(RefCell::new(glyph_id));
        if (*(*glyph.upgrade().deref()).overlap_simple_flag_set.borrow()) {
            ({ self.EnsureOverlapBitmap() });
            let rhs_0 = ((((self.overlap_bitmap_.as_pointer() as Ptr<u8>)
                .offset((((*glyph_id.borrow()) >> 3) as usize))
                .read()) as i32)
                | (128 >> ((*glyph_id.borrow()) & 7))) as u8;
            (self.overlap_bitmap_.as_pointer() as Ptr<u8>)
                .offset((((*glyph_id.borrow()) >> 3) as usize))
                .write(rhs_0);
        }
        let num_contours: Value<i32> = Rc::new(RefCell::new(
            ((*(*glyph.upgrade().deref()).contours.borrow()).len() as i32),
        ));
        ({
            WriteUShort_88(
                (self.n_contour_stream_.as_pointer()),
                (*num_contours.borrow()),
            )
        });
        if ({ self.ShouldWriteSimpleGlyphBbox((glyph).clone()) }) {
            ({
                let _glyph_id: i32 = (*glyph_id.borrow());
                let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
                self.WriteBbox(_glyph_id, _glyph)
            });
        }
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*num_contours.borrow())) {
            ({
                Write255UShort_10(
                    (self.n_points_stream_.as_pointer()),
                    ((*(((*glyph.upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                        .upgrade()
                        .deref())
                    .len() as i32),
                )
            });
            (*i.borrow_mut()).postfix_inc();
        }
        let lastX: Value<i32> = Rc::new(RefCell::new(0));
        let lastY: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*num_contours.borrow())) {
            let num_points: Value<i32> = Rc::new(RefCell::new(
                ((*(((*glyph.upgrade().deref()).contours.as_pointer()
                    as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                    .offset(((*i.borrow()) as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<Vec<woff2_Glyph_Point>>)
                    .upgrade()
                    .deref())
                .len() as i32),
            ));
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*j.borrow()) < (*num_points.borrow())) {
                let x: Value<i32> = Rc::new(RefCell::new(
                    (*(*(((*glyph.upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset(((*j.borrow()) as usize))
                        .upgrade()
                        .deref())
                    .x
                    .borrow()),
                ));
                let y: Value<i32> = Rc::new(RefCell::new(
                    (*(*(((*glyph.upgrade().deref()).contours.as_pointer()
                        as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                        .offset(((*i.borrow()) as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<woff2_Glyph_Point>)
                        .offset(((*j.borrow()) as usize))
                        .upgrade()
                        .deref())
                    .y
                    .borrow()),
                ));
                let dx: Value<i32> = Rc::new(RefCell::new(((*x.borrow()) - (*lastX.borrow()))));
                let dy: Value<i32> = Rc::new(RefCell::new(((*y.borrow()) - (*lastY.borrow()))));
                ({
                    self.WriteTriplet(
                        (*(*(((*glyph.upgrade().deref()).contours.as_pointer()
                            as Ptr<Value<Vec<woff2_Glyph_Point>>>)
                            .offset(((*i.borrow()) as usize))
                            .upgrade()
                            .deref()
                            .as_pointer() as Ptr<woff2_Glyph_Point>)
                            .offset(((*j.borrow()) as usize))
                            .upgrade()
                            .deref())
                        .on_curve
                        .borrow()),
                        (*dx.borrow()),
                        (*dy.borrow()),
                    )
                });
                (*lastX.borrow_mut()) = (*x.borrow());
                (*lastY.borrow_mut()) = (*y.borrow());
                (*j.borrow_mut()).postfix_inc();
            }
            (*i.borrow_mut()).postfix_inc();
        }
        if ((*num_contours.borrow()) > 0) {
            ({ self.WriteInstructions((glyph).clone()) });
        }
    }
    fn WriteCompositeGlyph(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        let glyph_id: Value<i32> = Rc::new(RefCell::new(glyph_id));
        ({ WriteUShort_88((self.n_contour_stream_.as_pointer()), -1_i32) });
        ({
            let _glyph_id: i32 = (*glyph_id.borrow());
            let _glyph: Ptr<woff2_Glyph> = (glyph).clone();
            self.WriteBbox(_glyph_id, _glyph)
        });
        ({
            let _data: Ptr<u8> = (*(*glyph.upgrade().deref()).composite_data.borrow()).clone();
            let _len: usize = ((*(*glyph.upgrade().deref()).composite_data_size.borrow()) as usize);
            WriteBytes_86((self.composite_stream_.as_pointer()), _data, _len)
        });
        if (*(*glyph.upgrade().deref()).have_instructions.borrow()) {
            ({ self.WriteInstructions((glyph).clone()) });
        }
    }
    fn WriteBbox(&self, glyph_id: i32, glyph: Ptr<woff2_Glyph>) {
        let glyph_id: Value<i32> = Rc::new(RefCell::new(glyph_id));
        let rhs_0 = ((((self.bbox_bitmap_.as_pointer() as Ptr<u8>)
            .offset((((*glyph_id.borrow()) >> 3) as usize))
            .read()) as i32)
            | (128 >> ((*glyph_id.borrow()) & 7))) as u8;
        (self.bbox_bitmap_.as_pointer() as Ptr<u8>)
            .offset((((*glyph_id.borrow()) >> 3) as usize))
            .write(rhs_0);
        ({
            WriteUShort_88(
                (self.bbox_stream_.as_pointer()),
                ((*(*glyph.upgrade().deref()).x_min.borrow()) as i32),
            )
        });
        ({
            WriteUShort_88(
                (self.bbox_stream_.as_pointer()),
                ((*(*glyph.upgrade().deref()).y_min.borrow()) as i32),
            )
        });
        ({
            WriteUShort_88(
                (self.bbox_stream_.as_pointer()),
                ((*(*glyph.upgrade().deref()).x_max.borrow()) as i32),
            )
        });
        ({
            WriteUShort_88(
                (self.bbox_stream_.as_pointer()),
                ((*(*glyph.upgrade().deref()).y_max.borrow()) as i32),
            )
        });
    }
    fn WriteTriplet(&self, on_curve: bool, x: i32, y: i32) {
        let on_curve: Value<bool> = Rc::new(RefCell::new(on_curve));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let abs_x: Value<i32> = Rc::new(RefCell::new((*x.borrow()).abs()));
        let abs_y: Value<i32> = Rc::new(RefCell::new((*y.borrow()).abs()));
        let on_curve_bit: Value<i32> =
            Rc::new(RefCell::new(if (*on_curve.borrow()) { 0 } else { 128 }));
        let x_sign_bit: Value<i32> = Rc::new(RefCell::new(if ((*x.borrow()) < 0) { 0 } else { 1 }));
        let y_sign_bit: Value<i32> = Rc::new(RefCell::new(if ((*y.borrow()) < 0) { 0 } else { 1 }));
        let xy_sign_bits: Value<i32> = Rc::new(RefCell::new(
            ((*x_sign_bit.borrow()) + (2 * (*y_sign_bit.borrow()))),
        ));
        if ((*x.borrow()) == 0) && ((*abs_y.borrow()) < 1280) {
            (*self.flag_byte_stream_.borrow_mut()).push(
                ((((*on_curve_bit.borrow()) + (((*abs_y.borrow()) & 3840) >> 7))
                    + (*y_sign_bit.borrow())) as u8),
            );
            (*self.glyph_stream_.borrow_mut()).push((((*abs_y.borrow()) & 255) as u8));
        } else if ((*y.borrow()) == 0) && ((*abs_x.borrow()) < 1280) {
            (*self.flag_byte_stream_.borrow_mut()).push(
                (((((*on_curve_bit.borrow()) + 10) + (((*abs_x.borrow()) & 3840) >> 7))
                    + (*x_sign_bit.borrow())) as u8),
            );
            (*self.glyph_stream_.borrow_mut()).push((((*abs_x.borrow()) & 255) as u8));
        } else if ((*abs_x.borrow()) < 65) && ((*abs_y.borrow()) < 65) {
            (*self.flag_byte_stream_.borrow_mut()).push(
                ((((((*on_curve_bit.borrow()) + 20) + (((*abs_x.borrow()) - 1) & 48))
                    + ((((*abs_y.borrow()) - 1) & 48) >> 2))
                    + (*xy_sign_bits.borrow())) as u8),
            );
            (*self.glyph_stream_.borrow_mut()).push(
                ((((((*abs_x.borrow()) - 1) & 15) << 4) | (((*abs_y.borrow()) - 1) & 15)) as u8),
            );
        } else if ((*abs_x.borrow()) < 769) && ((*abs_y.borrow()) < 769) {
            (*self.flag_byte_stream_.borrow_mut()).push(
                ((((((*on_curve_bit.borrow()) + 84)
                    + (12 * ((((*abs_x.borrow()) - 1) & 768) >> 8)))
                    + ((((*abs_y.borrow()) - 1) & 768) >> 6))
                    + (*xy_sign_bits.borrow())) as u8),
            );
            (*self.glyph_stream_.borrow_mut()).push(((((*abs_x.borrow()) - 1) & 255) as u8));
            (*self.glyph_stream_.borrow_mut()).push(((((*abs_y.borrow()) - 1) & 255) as u8));
        } else if ((*abs_x.borrow()) < 4096) && ((*abs_y.borrow()) < 4096) {
            (*self.flag_byte_stream_.borrow_mut())
                .push(((((*on_curve_bit.borrow()) + 120) + (*xy_sign_bits.borrow())) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_x.borrow()) >> 4) as u8));
            (*self.glyph_stream_.borrow_mut())
                .push((((((*abs_x.borrow()) & 15) << 4) | ((*abs_y.borrow()) >> 8)) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_y.borrow()) & 255) as u8));
        } else {
            (*self.flag_byte_stream_.borrow_mut())
                .push(((((*on_curve_bit.borrow()) + 124) + (*xy_sign_bits.borrow())) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_x.borrow()) >> 8) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_x.borrow()) & 255) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_y.borrow()) >> 8) as u8));
            (*self.glyph_stream_.borrow_mut()).push((((*abs_y.borrow()) & 255) as u8));
        }
    }
    fn EnsureOverlapBitmap(&self) {
        if (*self.overlap_bitmap_.borrow()).is_empty() {
            {
                let __a0 = ((((*self.n_glyphs_.borrow()) + 7) >> 3) as usize) as usize;
                (*self.overlap_bitmap_.borrow_mut()).resize_with(__a0, || <u8>::default())
            };
        }
    }
}
impl Clone for woff2_GlyfEncoder {
    fn clone(&self) -> Self {
        let mut this = Self {
            n_contour_stream_: Rc::new(RefCell::new((*self.n_contour_stream_.borrow()).clone())),
            n_points_stream_: Rc::new(RefCell::new((*self.n_points_stream_.borrow()).clone())),
            flag_byte_stream_: Rc::new(RefCell::new((*self.flag_byte_stream_.borrow()).clone())),
            composite_stream_: Rc::new(RefCell::new((*self.composite_stream_.borrow()).clone())),
            bbox_bitmap_: Rc::new(RefCell::new((*self.bbox_bitmap_.borrow()).clone())),
            bbox_stream_: Rc::new(RefCell::new((*self.bbox_stream_.borrow()).clone())),
            glyph_stream_: Rc::new(RefCell::new((*self.glyph_stream_.borrow()).clone())),
            instruction_stream_: Rc::new(RefCell::new(
                (*self.instruction_stream_.borrow()).clone(),
            )),
            overlap_bitmap_: Rc::new(RefCell::new((*self.overlap_bitmap_.borrow()).clone())),
            n_glyphs_: Rc::new(RefCell::new((*self.n_glyphs_.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_GlyfEncoder {}
pub fn TransformGlyfAndLocaTables_90(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kGlyfTableTag_0.with(Value::clone).borrow()))
        }),
    ));
    let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kLocaTableTag_2.with(Value::clone).borrow()))
        }),
    ));
    if ((*loca_table.borrow()).is_null()) && ((*glyf_table.borrow()).is_null()) {
        return true;
    }
    if {
        let _lhs = (((*glyf_table.borrow()).is_null()) as i32).clone();
        _lhs != (((*loca_table.borrow()).is_null()) as i32).clone()
    } {
        return false;
    }
    if {
        let _lhs = (({ (*(*loca_table.borrow()).upgrade().deref()).IsReused() }) as i32);
        _lhs != (({ (*(*glyf_table.borrow()).upgrade().deref()).IsReused() }) as i32)
    } {
        return false;
    }
    if ({ (*(*loca_table.borrow()).upgrade().deref()).IsReused() }) {
        return true;
    }
    let transformed_glyf: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        (((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
            as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                __v.entry(((*kGlyfTableTag_0.with(Value::clone).borrow()) ^ 2155905152_u32).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                    .as_pointer()
            })),
    ));
    let transformed_loca: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        (((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
            as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                __v.entry(((*kLocaTableTag_2.with(Value::clone).borrow()) ^ 2155905152_u32).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                    .as_pointer()
            })),
    ));
    let num_glyphs: Value<i32> =
        Rc::new(RefCell::new(({ NumGlyphs_45((*font.borrow()).clone()) })));
    let encoder: Value<woff2_GlyfEncoder> = Rc::new(RefCell::new(
        woff2_GlyfEncoder::woff2_GlyfEncoder((*num_glyphs.borrow())),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*num_glyphs.borrow())) {
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::woff2_Glyph()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (*font.borrow()).clone();
            let _glyph_index: i32 = (*i.borrow());
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
        ({ (*encoder.borrow()).Encode((*i.borrow()), glyph.as_pointer()) });
        (*i.borrow_mut()).prefix_inc();
    }
    ({
        (*encoder.borrow()).GetTransformedGlyfBytes(
            ((*(*transformed_glyf.borrow()).upgrade().deref())
                .buffer
                .as_pointer()),
        )
    });
    let head_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHeadTableTag_1.with(Value::clone).borrow()))
        }),
    ));
    if ((*head_table.borrow()).is_null())
        || ((*(*(*head_table.borrow()).upgrade().deref()).length.borrow()) < 52_u32)
    {
        return false;
    }
    let __rhs = ((*(*(*head_table.borrow()).upgrade().deref()).data.borrow())
        .offset((51) as isize)
        .read());
    ((*(*transformed_glyf.borrow()).upgrade().deref())
        .buffer
        .as_pointer() as Ptr<u8>)
        .offset(7_usize)
        .write(__rhs);
    (*(*(*transformed_glyf.borrow()).upgrade().deref())
        .tag
        .borrow_mut()) = ((*kGlyfTableTag_0.with(Value::clone).borrow()) ^ 2155905152_u32);
    let __rhs = ((*(*(*transformed_glyf.borrow()).upgrade().deref())
        .buffer
        .borrow())
    .len() as u32);
    (*(*(*transformed_glyf.borrow()).upgrade().deref())
        .length
        .borrow_mut()) = __rhs;
    let __rhs = ((*(*transformed_glyf.borrow()).upgrade().deref())
        .buffer
        .as_pointer() as Ptr<u8>);
    (*(*(*transformed_glyf.borrow()).upgrade().deref())
        .data
        .borrow_mut()) = __rhs;
    (*(*(*transformed_loca.borrow()).upgrade().deref())
        .tag
        .borrow_mut()) = ((*kLocaTableTag_2.with(Value::clone).borrow()) ^ 2155905152_u32);
    (*(*(*transformed_loca.borrow()).upgrade().deref())
        .length
        .borrow_mut()) = 0_u32;
    (*(*(*transformed_loca.borrow()).upgrade().deref())
        .data
        .borrow_mut()) = Ptr::<u8>::null();
    return true;
}
pub fn TransformHmtxTable_91(font: Ptr<woff2_Font>) -> bool {
    let font: Value<Ptr<woff2_Font>> = Rc::new(RefCell::new(font));
    let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kGlyfTableTag_0.with(Value::clone).borrow()))
        }),
    ));
    let hmtx_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHmtxTableTag_5.with(Value::clone).borrow()))
        }),
    ));
    let hhea_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        ({
            (*(*font.borrow()).upgrade().deref())
                .FindTable_u32((*kHheaTableTag_6.with(Value::clone).borrow()))
        }),
    ));
    if ((*hmtx_table.borrow()).is_null()) || ((*glyf_table.borrow()).is_null()) {
        return true;
    }
    if (*hhea_table.borrow()).is_null() {
        return false;
    }
    let hhea_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(*hhea_table.borrow()).upgrade().deref()).data.borrow()).clone(),
        ((*(*(*hhea_table.borrow()).upgrade().deref()).length.borrow()) as usize),
    )));
    let num_hmetrics: Value<u16> = <Value<u16>>::default();
    if (!({ (*hhea_buf.borrow()).Skip(34_usize) }))
        || (!({ (*hhea_buf.borrow()).ReadU16((num_hmetrics.as_pointer())) }))
    {
        return false;
    }
    if (((*num_hmetrics.borrow()) as i32) < 1) {
        return false;
    }
    let num_glyphs: Value<i32> =
        Rc::new(RefCell::new(({ NumGlyphs_45((*font.borrow()).clone()) })));
    let advance_widths: Value<Vec<u16>> = Rc::new(RefCell::new(Vec::new()));
    let proportional_lsbs: Value<Vec<i16>> = Rc::new(RefCell::new(Vec::new()));
    let monospace_lsbs: Value<Vec<i16>> = Rc::new(RefCell::new(Vec::new()));
    let remove_proportional_lsb: Value<bool> = Rc::new(RefCell::new(true));
    let remove_monospace_lsb: Value<bool> = Rc::new(RefCell::new(
        (((*num_glyphs.borrow()) - ((*num_hmetrics.borrow()) as i32)) > 0),
    ));
    let hmtx_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(*hmtx_table.borrow()).upgrade().deref()).data.borrow()).clone(),
        ((*(*(*hmtx_table.borrow()).upgrade().deref()).length.borrow()) as usize),
    )));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*num_glyphs.borrow())) {
        let glyph: Value<woff2_Glyph> = Rc::new(RefCell::new(woff2_Glyph::woff2_Glyph()));
        let glyph_data: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        if (!({
            let _font: Ptr<woff2_Font> = (*font.borrow()).clone();
            let _glyph_index: i32 = (*i.borrow());
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
        if ((*i.borrow()) < ((*num_hmetrics.borrow()) as i32)) {
            if !({ (*hmtx_buf.borrow()).ReadU16((advance_width.as_pointer())) }) {
                return false;
            }
            if !({ (*hmtx_buf.borrow()).ReadS16((lsb.as_pointer())) }) {
                return false;
            }
            if ((*glyph_size.borrow()) > 0_usize)
                && (((*(*glyph.borrow()).x_min.borrow()) as i32) != ((*lsb.borrow()) as i32))
            {
                (*remove_proportional_lsb.borrow_mut()) = false;
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
            if !({ (*hmtx_buf.borrow()).ReadS16((lsb.as_pointer())) }) {
                return false;
            }
            if ((*glyph_size.borrow()) > 0_usize)
                && (((*(*glyph.borrow()).x_min.borrow()) as i32) != ((*lsb.borrow()) as i32))
            {
                (*remove_monospace_lsb.borrow_mut()) = false;
            }
            {
                let a0_clone = (*lsb.borrow()).clone();
                (*monospace_lsbs.borrow_mut()).push(a0_clone)
            };
        }
        if (!(*remove_proportional_lsb.borrow())) && (!(*remove_monospace_lsb.borrow())) {
            return true;
        }
        (*i.borrow_mut()).postfix_inc();
    }
    let transformed_hmtx: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
        (((*(*font.borrow()).upgrade().deref()).tables.as_pointer()
            as Ptr<BTreeMap<u32, Value<woff2_Font_Table>>>)
            .with_mut(|__v: &mut BTreeMap<u32, Value<woff2_Font_Table>>| {
                __v.entry(((*kHmtxTableTag_5.with(Value::clone).borrow()) ^ 2155905152_u32).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<woff2_Font_Table>::default())))
                    .as_pointer()
            })),
    ));
    let flags: Value<u8> = Rc::new(RefCell::new(0_u8));
    let transformed_size: Value<usize> = Rc::new(RefCell::new(
        (1_usize).wrapping_add((2_usize).wrapping_mul((*advance_widths.borrow()).len())),
    ));
    if (*remove_proportional_lsb.borrow()) {
        let rhs_0 = (((*flags.borrow()) as i32) | 1) as u8;
        (*flags.borrow_mut()) = rhs_0;
    } else {
        let rhs_0 = (((*transformed_size.borrow()) as u64)
            .wrapping_add(((2_usize).wrapping_mul((*proportional_lsbs.borrow()).len()) as u64)))
            as usize;
        (*transformed_size.borrow_mut()) = rhs_0;
    }
    if (*remove_monospace_lsb.borrow()) {
        let rhs_0 = (((*flags.borrow()) as i32) | (1 << 1)) as u8;
        (*flags.borrow_mut()) = rhs_0;
    } else {
        let rhs_0 = (((*transformed_size.borrow()) as u64)
            .wrapping_add(((2_usize).wrapping_mul((*monospace_lsbs.borrow()).len()) as u64)))
            as usize;
        (*transformed_size.borrow_mut()) = rhs_0;
    }
    if (*transformed_size.borrow()) as usize
        > (*(*(*transformed_hmtx.borrow()).upgrade().deref())
            .buffer
            .borrow())
        .capacity() as usize
    {
        let len_0 = (*(*(*transformed_hmtx.borrow()).upgrade().deref())
            .buffer
            .borrow())
        .len();
        (*(*(*transformed_hmtx.borrow()).upgrade().deref())
            .buffer
            .borrow_mut())
        .reserve_exact((*transformed_size.borrow()) as usize - len_0 as usize);
    };
    let out: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(
        ((*(*transformed_hmtx.borrow()).upgrade().deref())
            .buffer
            .as_pointer()),
    ));
    ({ WriteBytes_86((*out.borrow()).clone(), (flags.as_pointer()), 1_usize) });
    'loop_: for mut advance_width in advance_widths.as_pointer() as Ptr<u16> {
        let advance_width: Value<u16> = Rc::new(RefCell::new(advance_width.read().clone()));
        ({ WriteUShort_88((*out.borrow()).clone(), ((*advance_width.borrow()) as i32)) });
    }
    if !(*remove_proportional_lsb.borrow()) {
        'loop_: for mut lsb in proportional_lsbs.as_pointer() as Ptr<i16> {
            let lsb: Value<i16> = Rc::new(RefCell::new(lsb.read().clone()));
            ({ WriteUShort_88((*out.borrow()).clone(), ((*lsb.borrow()) as i32)) });
        }
    }
    if !(*remove_monospace_lsb.borrow()) {
        'loop_: for mut lsb in monospace_lsbs.as_pointer() as Ptr<i16> {
            let lsb: Value<i16> = Rc::new(RefCell::new(lsb.read().clone()));
            ({ WriteUShort_88((*out.borrow()).clone(), ((*lsb.borrow()) as i32)) });
        }
    }
    (*(*(*transformed_hmtx.borrow()).upgrade().deref())
        .tag
        .borrow_mut()) = ((*kHmtxTableTag_5.with(Value::clone).borrow()) ^ 2155905152_u32);
    (*(*(*transformed_hmtx.borrow()).upgrade().deref())
        .flag_byte
        .borrow_mut()) = ((1 << 6) as u8);
    let __rhs = ((*(*(*transformed_hmtx.borrow()).upgrade().deref())
        .buffer
        .borrow())
    .len() as u32);
    (*(*(*transformed_hmtx.borrow()).upgrade().deref())
        .length
        .borrow_mut()) = __rhs;
    let __rhs = ((*(*transformed_hmtx.borrow()).upgrade().deref())
        .buffer
        .as_pointer() as Ptr<u8>);
    (*(*(*transformed_hmtx.borrow()).upgrade().deref())
        .data
        .borrow_mut()) = __rhs;
    return true;
}
#[derive()]
pub struct woff2_WOFF2Params {
    pub extended_metadata: Value<Vec<u8>>,
    pub brotli_quality: Value<i32>,
    pub allow_transforms: Value<bool>,
}
impl woff2_WOFF2Params {
    pub fn woff2_WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: Rc::new(RefCell::new(
                Ptr::from_string_literal(b"")
                    .to_c_string_iterator()
                    .chain(std::iter::once(0))
                    .collect::<Vec<u8>>(),
            )),
            brotli_quality: Rc::new(RefCell::new(11)),
            allow_transforms: Rc::new(RefCell::new(true)),
        };
        this
    }
}
impl Clone for woff2_WOFF2Params {
    fn clone(&self) -> Self {
        let mut this = Self {
            extended_metadata: Rc::new(RefCell::new((*self.extended_metadata.borrow()).clone())),
            brotli_quality: Rc::new(RefCell::new((*self.brotli_quality.borrow()))),
            allow_transforms: Rc::new(RefCell::new((*self.allow_transforms.borrow()))),
        };
        this
    }
}
impl Default for woff2_WOFF2Params {
    fn default() -> Self {
        { woff2_WOFF2Params::woff2_WOFF2Params() }
    }
}
impl ByteRepr for woff2_WOFF2Params {}
thread_local!();
thread_local!(
    pub static kWoff2HeaderSize_92: Value<usize> = Rc::new(RefCell::new(48_usize));
);
thread_local!(
    pub static kWoff2EntrySize_93: Value<usize> = Rc::new(RefCell::new(20_usize));
);
pub fn Compress_94(
    data: Ptr<u8>,
    len: usize,
    result: Ptr<u8>,
    result_len: Ptr<u32>,
    mode: ::brotli_sys::BrotliEncoderMode,
    quality: i32,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_len: Value<Ptr<u32>> = Rc::new(RefCell::new(result_len));
    let mode: Value<::brotli_sys::BrotliEncoderMode> = Rc::new(RefCell::new(mode));
    let quality: Value<i32> = Rc::new(RefCell::new(quality));
    let compressed_len: Value<usize> =
        Rc::new(RefCell::new((((*result_len.borrow()).read()) as usize)));
    if ((compressed_len.as_pointer()).with_mut(|_v5| {
        (*result.borrow()).with_mut(|_v6| unsafe {
            ::brotli_sys::BrotliEncoderCompress(
                (*quality.borrow()),
                22,
                (*mode.borrow()),
                (*len.borrow()),
                &*(*data.borrow()).upgrade().deref() as *const u8,
                _v5 as *mut usize,
                _v6,
            )
        })
    }) == 0)
    {
        return false;
    }
    let __rhs = ((*compressed_len.borrow()) as u32);
    (*result_len.borrow()).write(__rhs);
    return true;
}
pub fn Woff2Compress_95(
    data: Ptr<u8>,
    len: usize,
    result: Ptr<u8>,
    result_len: Ptr<u32>,
    quality: i32,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_len: Value<Ptr<u32>> = Rc::new(RefCell::new(result_len));
    let quality: Value<i32> = Rc::new(RefCell::new(quality));
    return ({
        Compress_94(
            (*data.borrow()).clone(),
            (*len.borrow()),
            (*result.borrow()).clone(),
            (*result_len.borrow()).clone(),
            ::brotli_sys::BROTLI_MODE_FONT,
            (*quality.borrow()),
        )
    });
}
pub fn TextCompress_96(
    data: Ptr<u8>,
    len: usize,
    result: Ptr<u8>,
    result_len: Ptr<u32>,
    quality: i32,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let len: Value<usize> = Rc::new(RefCell::new(len));
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_len: Value<Ptr<u32>> = Rc::new(RefCell::new(result_len));
    let quality: Value<i32> = Rc::new(RefCell::new(quality));
    return ({
        Compress_94(
            (*data.borrow()).clone(),
            (*len.borrow()),
            (*result.borrow()).clone(),
            (*result_len.borrow()).clone(),
            ::brotli_sys::BROTLI_MODE_TEXT,
            (*quality.borrow()),
        )
    });
}
pub fn KnownTableIndex_97(tag: u32) -> i32 {
    let tag: Value<u32> = Rc::new(RefCell::new(tag));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 63) {
        if ((*tag.borrow()) == (*kKnownTags_8.with(Value::clone).borrow())[(*i.borrow()) as usize])
        {
            return (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return 63;
}
pub fn StoreTableEntry_98(table: Ptr<woff2_Table>, offset: Ptr<usize>, dst: Ptr<u8>) {
    let offset: Value<Ptr<usize>> = Rc::new(RefCell::new(offset));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let flag_byte: Value<u8> = Rc::new(RefCell::new(
        (({
            let _lhs = ((*(*table.upgrade().deref()).flags.borrow()) & 192_u32);
            _lhs | (({ KnownTableIndex_97((*(*table.upgrade().deref()).tag.borrow())) }) as u32)
        }) as u8),
    ));
    let __rhs = (*flag_byte.borrow());
    (*dst.borrow())
        .offset(((*offset.borrow()).with_mut(|__v| __v.postfix_inc())) as isize)
        .write(__rhs);
    if ((((*flag_byte.borrow()) as i32) & 63) == 63) {
        ({
            StoreU32_30(
                (*(*table.upgrade().deref()).tag.borrow()),
                (*offset.borrow()).clone(),
                (*dst.borrow()).clone(),
            )
        });
    }
    ({
        let _len: usize = ((*(*table.upgrade().deref()).src_length.borrow()) as usize);
        let _offset: Ptr<usize> = (*offset.borrow()).clone();
        StoreBase128_19(_len, _offset, (*dst.borrow()).clone())
    });
    if (({
        let _lhs = (*(*table.upgrade().deref()).flags.borrow());
        _lhs & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
    }) != 0_u32)
    {
        ({
            let _len: usize = ((*(*table.upgrade().deref()).transform_length.borrow()) as usize);
            let _offset: Ptr<usize> = (*offset.borrow()).clone();
            StoreBase128_19(_len, _offset, (*dst.borrow()).clone())
        });
    }
}
pub fn TableEntrySize_99(table: Ptr<woff2_Table>) -> usize {
    let flag_byte: Value<u8> = Rc::new(RefCell::new(
        (({ KnownTableIndex_97((*(*table.upgrade().deref()).tag.borrow())) }) as u8),
    ));
    let size: Value<usize> = Rc::new(RefCell::new(
        (if ((((*flag_byte.borrow()) as i32) & 63) != 63) {
            1
        } else {
            5
        } as usize),
    ));
    let rhs_0 = (*size.borrow()).wrapping_add(
        ({ Base128Size_18(((*(*table.upgrade().deref()).src_length.borrow()) as usize)) }),
    );
    (*size.borrow_mut()) = rhs_0;
    if (({
        let _lhs = (*(*table.upgrade().deref()).flags.borrow());
        _lhs & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
    }) != 0_u32)
    {
        let rhs_0 = (*size.borrow()).wrapping_add(
            ({
                Base128Size_18(((*(*table.upgrade().deref()).transform_length.borrow()) as usize))
            }),
        );
        (*size.borrow_mut()) = rhs_0;
    }
    return (*size.borrow());
}
pub fn ComputeWoff2Length_100(
    font_collection: Ptr<woff2_FontCollection>,
    tables: Ptr<Vec<woff2_Table>>,
    index_by_tag_offset: BTreeMap<(Value<u32>, Value<u32>), Value<u16>>,
    compressed_data_length: usize,
    extended_metadata_length: usize,
) -> usize {
    let index_by_tag_offset: Value<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>> =
        Rc::new(RefCell::new(index_by_tag_offset));
    let compressed_data_length: Value<usize> = Rc::new(RefCell::new(compressed_data_length));
    let extended_metadata_length: Value<usize> = Rc::new(RefCell::new(extended_metadata_length));
    let size: Value<usize> = Rc::new(RefCell::new(
        (*kWoff2HeaderSize_92.with(Value::clone).borrow()),
    ));
    'loop_: for mut table in tables.to_strong().as_pointer() as Ptr<woff2_Table> {
        let rhs_0 = (*size.borrow()).wrapping_add(({ TableEntrySize_99((table).clone()) }));
        (*size.borrow_mut()) = rhs_0;
    }
    if {
        let _lhs = (*(*font_collection.upgrade().deref()).flavor.borrow());
        _lhs == (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
        let rhs_0 = (*size.borrow()).wrapping_add(4_usize);
        (*size.borrow_mut()) = rhs_0;
        let rhs_0 = (*size.borrow()).wrapping_add(
            ({
                Size255UShort_9(
                    ((*(*font_collection.upgrade().deref()).fonts.borrow()).len() as u16),
                )
            }),
        );
        (*size.borrow_mut()) = rhs_0;
        let rhs_0 = (((*size.borrow()) as u64).wrapping_add(
            ((4_usize).wrapping_mul((*(*font_collection.upgrade().deref()).fonts.borrow()).len())
                as u64),
        )) as usize;
        (*size.borrow_mut()) = rhs_0;
        'loop_: for mut font in
            (*font_collection.upgrade().deref()).fonts.as_pointer() as Ptr<woff2_Font>
        {
            let rhs_0 = (*size.borrow()).wrapping_add(
                ({ Size255UShort_9(((*(*font.upgrade().deref()).tables.borrow()).len() as u16)) }),
            );
            (*size.borrow_mut()) = rhs_0;
            'loop_: for entry in
                RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer())
            {
                let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
                if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(
                        (*(*table.upgrade().deref()).tag.borrow())
                            .try_into()
                            .expect("failed conversion"),
                    )),
                    Rc::new(RefCell::new(
                        (*(*table.upgrade().deref()).offset.borrow())
                            .try_into()
                            .expect("failed conversion"),
                    )),
                )));
                let table_index: Value<u16> = Rc::new(RefCell::new(
                    ((index_by_tag_offset.as_pointer()
                        as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>)
                        .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u16>>| {
                            __v.entry((*tag_offset.borrow()).clone())
                                .or_insert_with(|| Rc::new(RefCell::new(<u16>::default())))
                                .as_pointer()
                        })
                        .read()),
                ));
                let rhs_0 =
                    (*size.borrow()).wrapping_add(({ Size255UShort_9((*table_index.borrow())) }));
                (*size.borrow_mut()) = rhs_0;
            }
        }
    }
    let rhs_0 = (*size.borrow()).wrapping_add((*compressed_data_length.borrow()));
    (*size.borrow_mut()) = rhs_0;
    let __rhs = (({ Round4_70(((*size.borrow()) as u64)) }) as usize);
    (*size.borrow_mut()) = __rhs;
    let rhs_0 = (*size.borrow()).wrapping_add((*extended_metadata_length.borrow()));
    (*size.borrow_mut()) = rhs_0;
    return (*size.borrow());
}
pub fn ComputeUncompressedLength_101(font: Ptr<woff2_Font>) -> usize {
    let size: Value<usize> = Rc::new(RefCell::new(
        ((12 + (16 * ((*(*font.upgrade().deref()).num_tables.borrow()) as i32))) as usize),
    ));
    'loop_: for entry in RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer()) {
        let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
        if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0) {
            continue 'loop_;
        }
        if ({ (*table.upgrade().deref()).IsReused() }) {
            continue 'loop_;
        }
        let rhs_0 = (*size.borrow()).wrapping_add(
            (({ Round4_71((*(*table.upgrade().deref()).length.borrow())) }) as usize),
        );
        (*size.borrow_mut()) = rhs_0;
    }
    return (*size.borrow());
}
pub fn ComputeUncompressedLength_102(font_collection: Ptr<woff2_FontCollection>) -> usize {
    if {
        let _lhs = (*(*font_collection.upgrade().deref()).flavor.borrow());
        _lhs != (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
        return ({
            ComputeUncompressedLength_101(
                ((*font_collection.upgrade().deref()).fonts.as_pointer() as Ptr<woff2_Font>)
                    .offset(0_usize),
            )
        });
    }
    let size: Value<usize> = Rc::new(RefCell::new(
        ({
            let _header_version: u32 =
                (*(*font_collection.upgrade().deref()).header_version.borrow());
            let _num_fonts: u32 =
                ((*(*font_collection.upgrade().deref()).fonts.borrow()).len() as u32);
            CollectionHeaderSize_27(_header_version, _num_fonts)
        }),
    ));
    'loop_: for mut font in
        (*font_collection.upgrade().deref()).fonts.as_pointer() as Ptr<woff2_Font>
    {
        let rhs_0 =
            (*size.borrow()).wrapping_add(({ ComputeUncompressedLength_101((font).clone()) }));
        (*size.borrow_mut()) = rhs_0;
    }
    return (*size.borrow());
}
pub fn ComputeTotalTransformLength_103(font: Ptr<woff2_Font>) -> usize {
    let total: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for i in RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer()) {
        let table: Ptr<woff2_Font_Table> = i.second().as_pointer();
        if ({ (*table.upgrade().deref()).IsReused() }) {
            continue 'loop_;
        }
        if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0)
            || (!!({
                let _tag: u32 = ((*(*table.upgrade().deref()).tag.borrow()) ^ 2155905152_u32);
                (*font.upgrade().deref()).FindTable_u32_const(_tag)
            })
            .is_null())
        {
            let rhs_0 = (*total.borrow())
                .wrapping_add(((*(*table.upgrade().deref()).length.borrow()) as usize));
            (*total.borrow_mut()) = rhs_0;
        }
    }
    return (*total.borrow());
}
pub fn MaxWOFF2CompressedSize_104(data: Ptr<u8>, length: usize) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return ({
        let _extended_metadata: Value<Vec<u8>> = Rc::new(RefCell::new(
            Ptr::from_string_literal(b"")
                .to_c_string_iterator()
                .chain(std::iter::once(0))
                .collect::<Vec<u8>>(),
        ));
        MaxWOFF2CompressedSize_105(
            (*data.borrow()).clone(),
            (*length.borrow()),
            _extended_metadata.as_pointer(),
        )
    });
}
pub fn MaxWOFF2CompressedSize_105(
    data: Ptr<u8>,
    length: usize,
    extended_metadata: Ptr<Vec<u8>>,
) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    return (((*length.borrow()).wrapping_add(1024_usize) as u64)
        .wrapping_add((((*extended_metadata.upgrade().deref()).len() - 1) as u64))
        as usize);
}
pub fn CompressedBufferSize_106(original_size: u32) -> u32 {
    let original_size: Value<u32> = Rc::new(RefCell::new(original_size));
    return (((1.2E+0 * ((*original_size.borrow()) as f64)) + 10240_f64) as u32);
}
pub fn TransformFontCollection_107(font_collection: Ptr<woff2_FontCollection>) -> bool {
    let font_collection: Value<Ptr<woff2_FontCollection>> = Rc::new(RefCell::new(font_collection));
    'loop_: for mut font in (*(*font_collection.borrow()).upgrade().deref())
        .fonts
        .as_pointer() as Ptr<woff2_Font>
    {
        if !({ TransformGlyfAndLocaTables_90((font).clone()) }) {
            eprintln!("glyf/loca transformation failed.");
            return false;
        }
    }
    return true;
}
pub fn ConvertTTFToWOFF2_108(
    data: Ptr<u8>,
    length: usize,
    result: Ptr<u8>,
    result_length: Ptr<usize>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_length: Value<Ptr<usize>> = Rc::new(RefCell::new(result_length));
    let params: Value<woff2_WOFF2Params> =
        Rc::new(RefCell::new(woff2_WOFF2Params::woff2_WOFF2Params()));
    return ({
        let _length: usize = (*length.borrow());
        let _result_length: Ptr<usize> = (*result_length.borrow()).clone();
        ConvertTTFToWOFF2_109(
            (*data.borrow()).clone(),
            _length,
            (*result.borrow()).clone(),
            _result_length,
            params.as_pointer(),
        )
    });
}
pub fn ConvertTTFToWOFF2_109(
    data: Ptr<u8>,
    length: usize,
    result: Ptr<u8>,
    result_length: Ptr<usize>,
    params: Ptr<woff2_WOFF2Params>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_length: Value<Ptr<usize>> = Rc::new(RefCell::new(result_length));
    let font_collection: Value<woff2_FontCollection> =
        Rc::new(RefCell::new(<woff2_FontCollection>::default()));
    if !({
        ReadFontCollection_37(
            (*data.borrow()).clone(),
            (*length.borrow()),
            (font_collection.as_pointer()),
        )
    }) {
        eprintln!("Parsing of the input font failed.");
        return false;
    }
    if !({ NormalizeFontCollection_82((font_collection.as_pointer())) }) {
        return false;
    }
    if (*(*params.upgrade().deref()).allow_transforms.borrow())
        && (!({ TransformFontCollection_107((font_collection.as_pointer())) }))
    {
        return false;
    } else {
        'loop_: for mut font in (*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font> {
            let glyf_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
                ({
                    let _tag: u32 = (*kGlyfTableTag_0.with(Value::clone).borrow());
                    (*font.upgrade().deref()).FindTable_u32(_tag)
                }),
            ));
            let loca_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
                ({
                    let _tag: u32 = (*kLocaTableTag_2.with(Value::clone).borrow());
                    (*font.upgrade().deref()).FindTable_u32(_tag)
                }),
            ));
            if !(*glyf_table.borrow()).is_null() {
                let rhs_0 = (((*(*(*glyf_table.borrow()).upgrade().deref())
                    .flag_byte
                    .borrow()) as i32)
                    | 192) as u8;
                (*(*(*glyf_table.borrow()).upgrade().deref())
                    .flag_byte
                    .borrow_mut()) = rhs_0;
            }
            if !(*loca_table.borrow()).is_null() {
                let rhs_0 = (((*(*(*loca_table.borrow()).upgrade().deref())
                    .flag_byte
                    .borrow()) as i32)
                    | 192) as u8;
                (*(*(*loca_table.borrow()).upgrade().deref())
                    .flag_byte
                    .borrow_mut()) = rhs_0;
            }
        }
    }
    let total_transform_length: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for mut font in (*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font> {
        let rhs_0 = (*total_transform_length.borrow())
            .wrapping_add(({ ComputeTotalTransformLength_103((font).clone()) }));
        (*total_transform_length.borrow_mut()) = rhs_0;
    }
    let compression_buffer_size: Value<usize> = Rc::new(RefCell::new(
        (({ CompressedBufferSize_106(((*total_transform_length.borrow()) as u32)) }) as usize),
    ));
    let compression_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(*compression_buffer_size.borrow()) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let total_compressed_length: Value<u32> =
        Rc::new(RefCell::new(((*compression_buffer_size.borrow()) as u32)));
    let transform_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(*total_transform_length.borrow()) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let transform_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: for mut font in (*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font> {
        'loop_: for tag in Rc::new(RefCell::new(
            ({ (*font.upgrade().deref()).OutputOrderedTags() }),
        ))
        .as_pointer() as Ptr<u32>
        {
            let tag: Value<u32> = Rc::new(RefCell::new(tag.read().clone()));
            let original: Ptr<woff2_Font_Table> = (*(*font.upgrade().deref()).tables.borrow())
                .get(&(*tag.borrow()))
                .expect("out of range!")
                .as_pointer();
            if ({ (*original.upgrade().deref()).IsReused() }) {
                continue 'loop_;
            }
            if (((*tag.borrow()) & 2155905152_u32) != 0) {
                continue 'loop_;
            }
            let table_to_store: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
                ({
                    let _tag: u32 = ((*tag.borrow()) ^ 2155905152_u32);
                    (*font.upgrade().deref()).FindTable_u32_const(_tag)
                }),
            ));
            if (*table_to_store.borrow()).is_null() {
                (*table_to_store.borrow_mut()) = (original).clone();
            }
            ({
                let _data: Ptr<u8> = (*(*(*table_to_store.borrow()).upgrade().deref())
                    .data
                    .borrow())
                .clone();
                let _len: usize = ((*(*(*table_to_store.borrow()).upgrade().deref())
                    .length
                    .borrow()) as usize);
                StoreBytes_32(
                    _data,
                    _len,
                    (transform_offset.as_pointer()),
                    ((transform_buf.as_pointer() as Ptr<u8>).offset(0_usize)),
                )
            });
        }
    }
    if !({
        Woff2Compress_95(
            (transform_buf.as_pointer() as Ptr<u8>),
            (*total_transform_length.borrow()),
            ((compression_buf.as_pointer() as Ptr<u8>).offset(0_usize)),
            (total_compressed_length.as_pointer()),
            (*(*params.upgrade().deref()).brotli_quality.borrow()),
        )
    }) {
        eprintln!("Compression of combined table failed.");
        return false;
    }
    eprintln!(
        "Compressed {} to {}.",
        (*total_transform_length.borrow()),
        (*total_compressed_length.borrow())
    );
    let compressed_metadata_buf_length: Value<u32> = Rc::new(RefCell::new(
        ({
            CompressedBufferSize_106(
                (((*(*params.upgrade().deref()).extended_metadata.borrow()).len() - 1) as u32),
            )
        }),
    ));
    let compressed_metadata_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((*compressed_metadata_buf_length.borrow()) as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    if (((*(*params.upgrade().deref()).extended_metadata.borrow()).len() - 1) > 0_usize) {
        if !({
            let _data: Ptr<u8> = ((*params.upgrade().deref()).extended_metadata.as_pointer()
                as Ptr<u8>)
                .reinterpret_cast::<u8>();
            let _len: usize = ((*(*params.upgrade().deref()).extended_metadata.borrow()).len() - 1);
            let _quality: i32 = (*(*params.upgrade().deref()).brotli_quality.borrow());
            TextCompress_96(
                _data,
                _len,
                (compressed_metadata_buf.as_pointer() as Ptr<u8>),
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
    let index_by_tag_offset: Value<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    'loop_: for mut font in (*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font> {
        'loop_: for tag in Rc::new(RefCell::new(
            ({ (*font.upgrade().deref()).OutputOrderedTags() }),
        ))
        .as_pointer() as Ptr<u32>
        {
            let tag: Value<u32> = Rc::new(RefCell::new(tag.read().clone()));
            let src_table: Ptr<woff2_Font_Table> = (*(*font.upgrade().deref()).tables.borrow())
                .get(&(*tag.borrow()))
                .expect("out of range!")
                .as_pointer();
            if ({ (*src_table.upgrade().deref()).IsReused() }) {
                continue 'loop_;
            }
            let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new(
                    (*(*src_table.upgrade().deref()).tag.borrow())
                        .try_into()
                        .expect("failed conversion"),
                )),
                Rc::new(RefCell::new(
                    (*(*src_table.upgrade().deref()).offset.borrow())
                        .try_into()
                        .expect("failed conversion"),
                )),
            )));
            if RefcountMapIter::find_key(
                (index_by_tag_offset.as_pointer()
                    as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>),
                &(*tag_offset.borrow()),
            ) == RefcountMapIter::end(
                (index_by_tag_offset.as_pointer()
                    as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>),
            ) {
                (index_by_tag_offset.as_pointer()
                    as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>)
                    .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u16>>| {
                        __v.entry((*tag_offset.borrow()).clone())
                            .or_insert_with(|| Rc::new(RefCell::new(<u16>::default())))
                            .as_pointer()
                    })
                    .write(((*tables.borrow()).len() as u16));
            } else {
                return false;
            }
            let table: Value<woff2_Table> = Rc::new(RefCell::new(<woff2_Table>::default()));
            (*(*table.borrow()).tag.borrow_mut()) = (*(*src_table.upgrade().deref()).tag.borrow());
            (*(*table.borrow()).flags.borrow_mut()) =
                ((*(*src_table.upgrade().deref()).flag_byte.borrow()) as u32);
            (*(*table.borrow()).src_length.borrow_mut()) =
                (*(*src_table.upgrade().deref()).length.borrow());
            (*(*table.borrow()).transform_length.borrow_mut()) =
                (*(*src_table.upgrade().deref()).length.borrow());
            let transformed_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
                (*(*src_table.upgrade().deref()).data.borrow()).clone(),
            ));
            let transformed_table: Value<Ptr<woff2_Font_Table>> = Rc::new(RefCell::new(
                ({
                    let _tag: u32 =
                        ((*(*src_table.upgrade().deref()).tag.borrow()) ^ 2155905152_u32);
                    (*font.upgrade().deref()).FindTable_u32_const(_tag)
                }),
            ));
            if !((*transformed_table.borrow()).is_null()) {
                (*(*table.borrow()).flags.borrow_mut()) =
                    ((*(*(*transformed_table.borrow()).upgrade().deref())
                        .flag_byte
                        .borrow()) as u32);
                (*(*table.borrow()).flags.borrow_mut()) |=
                    (*kWoff2FlagsTransform_21.with(Value::clone).borrow());
                (*(*table.borrow()).transform_length.borrow_mut()) =
                    (*(*(*transformed_table.borrow()).upgrade().deref())
                        .length
                        .borrow());
                (*transformed_data.borrow_mut()) =
                    (*(*(*transformed_table.borrow()).upgrade().deref())
                        .data
                        .borrow())
                    .clone();
            }
            {
                let a0_clone = (*table.borrow()).clone();
                (*tables.borrow_mut()).push(a0_clone)
            };
        }
    }
    let woff2_length: Value<usize> = Rc::new(RefCell::new(
        ({
            ComputeWoff2Length_100(
                font_collection.as_pointer(),
                tables.as_pointer(),
                (*index_by_tag_offset.borrow())
                    .iter()
                    .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                    .collect(),
                ((*total_compressed_length.borrow()) as usize),
                ((*compressed_metadata_buf_length.borrow()) as usize),
            )
        }),
    ));
    if {
        let _lhs = (*woff2_length.borrow());
        _lhs > ((*result_length.borrow()).read())
    } {
        eprintln!(
            "Result allocation was too small ({} vs {} bytes).",
            ((*result_length.borrow()).read()),
            (*woff2_length.borrow())
        );
        return false;
    }
    let __rhs = (*woff2_length.borrow());
    (*result_length.borrow()).write(__rhs);
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({
        StoreU32_30(
            (*kWoff2Signature_20.with(Value::clone).borrow()),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    if ((*(*font_collection.borrow()).flavor.borrow())
        != (*kTtcFontFlavor_22.with(Value::clone).borrow()))
    {
        ({
            StoreU32_30(
                (*(*((*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font>)
                    .offset(0_usize)
                    .upgrade()
                    .deref())
                .flavor
                .borrow()),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
    } else {
        ({
            StoreU32_30(
                (*kTtcFontFlavor_22.with(Value::clone).borrow()),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
    }
    ({
        StoreU32_30(
            ((*woff2_length.borrow()) as u32),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    ({
        Store16_31(
            ((*tables.borrow()).len() as i32),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    ({ Store16_31(0, (offset.as_pointer()), (*result.borrow()).clone()) });
    ({
        StoreU32_30(
            (({ ComputeUncompressedLength_102(font_collection.as_pointer()) }) as u32),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    ({
        StoreU32_30(
            (*total_compressed_length.borrow()),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    ({ Store16_31(1, (offset.as_pointer()), (*result.borrow()).clone()) });
    ({ Store16_31(0, (offset.as_pointer()), (*result.borrow()).clone()) });
    if ((*compressed_metadata_buf_length.borrow()) > 0_u32) {
        ({
            StoreU32_30(
                (((*woff2_length.borrow())
                    .wrapping_sub(((*compressed_metadata_buf_length.borrow()) as usize)))
                    as u32),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
        ({
            StoreU32_30(
                (*compressed_metadata_buf_length.borrow()),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
        ({
            StoreU32_30(
                (((*(*params.upgrade().deref()).extended_metadata.borrow()).len() - 1) as u32),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
    } else {
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*result.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*result.borrow()).clone()) });
        ({ StoreU32_30(0_u32, (offset.as_pointer()), (*result.borrow()).clone()) });
    }
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (*result.borrow()).clone()) });
    ({ StoreU32_30(0_u32, (offset.as_pointer()), (*result.borrow()).clone()) });
    'loop_: for mut table in tables.as_pointer() as Ptr<woff2_Table> {
        ({
            let _table: Ptr<woff2_Table> = (table).clone();
            let _offset: Ptr<usize> = (offset.as_pointer());
            let _dst: Ptr<u8> = (*result.borrow()).clone();
            StoreTableEntry_98(_table, _offset, _dst)
        });
    }
    if ((*(*font_collection.borrow()).flavor.borrow())
        == (*kTtcFontFlavor_22.with(Value::clone).borrow()))
    {
        ({
            StoreU32_30(
                (*(*font_collection.borrow()).header_version.borrow()),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
        ({
            Store255UShort_11(
                ((*(*font_collection.borrow()).fonts.borrow()).len() as i32),
                (offset.as_pointer()),
                (*result.borrow()).clone(),
            )
        });
        'loop_: for mut font in (*font_collection.borrow()).fonts.as_pointer() as Ptr<woff2_Font> {
            let num_tables: Value<u16> = Rc::new(RefCell::new(0_u16));
            'loop_: for entry in
                RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer())
            {
                let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
                if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                (*num_tables.borrow_mut()).postfix_inc();
            }
            ({
                Store255UShort_11(
                    ((*num_tables.borrow()) as i32),
                    (offset.as_pointer()),
                    (*result.borrow()).clone(),
                )
            });
            ({
                StoreU32_30(
                    (*(*font.upgrade().deref()).flavor.borrow()),
                    (offset.as_pointer()),
                    (*result.borrow()).clone(),
                )
            });
            'loop_: for entry in
                RefcountMapIter::begin((*font.upgrade().deref()).tables.as_pointer())
            {
                let table: Ptr<woff2_Font_Table> = entry.second().as_pointer();
                if (((*(*table.upgrade().deref()).tag.borrow()) & 2155905152_u32) != 0) {
                    continue 'loop_;
                }
                let table_offset: Value<u32> = Rc::new(RefCell::new(
                    if ({ (*table.upgrade().deref()).IsReused() }) {
                        (*(*(*(*table.upgrade().deref()).reuse_of.borrow())
                            .upgrade()
                            .deref())
                        .offset
                        .borrow())
                    } else {
                        (*(*table.upgrade().deref()).offset.borrow())
                    },
                ));
                let table_length: Value<u32> = Rc::new(RefCell::new(
                    if ({ (*table.upgrade().deref()).IsReused() }) {
                        (*(*(*(*table.upgrade().deref()).reuse_of.borrow())
                            .upgrade()
                            .deref())
                        .length
                        .borrow())
                    } else {
                        (*(*table.upgrade().deref()).length.borrow())
                    },
                ));
                let tag_offset: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(
                        (*(*table.upgrade().deref()).tag.borrow())
                            .try_into()
                            .expect("failed conversion"),
                    )),
                    Rc::new(RefCell::new(
                        (*table_offset.borrow())
                            .try_into()
                            .expect("failed conversion"),
                    )),
                )));
                if RefcountMapIter::find_key(
                    (index_by_tag_offset.as_pointer()
                        as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>),
                    &(*tag_offset.borrow()),
                ) == RefcountMapIter::end(
                    (index_by_tag_offset.as_pointer()
                        as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>),
                ) {
                    eprintln!(
                        "Missing table index for offset 0x{:08x}",
                        (*table_offset.borrow())
                    );
                    return false;
                }
                let index: Value<u16> = Rc::new(RefCell::new(
                    ((index_by_tag_offset.as_pointer()
                        as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u16>>>)
                        .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u16>>| {
                            __v.entry((*tag_offset.borrow()).clone())
                                .or_insert_with(|| Rc::new(RefCell::new(<u16>::default())))
                                .as_pointer()
                        })
                        .read()),
                ));
                ({
                    Store255UShort_11(
                        ((*index.borrow()) as i32),
                        (offset.as_pointer()),
                        (*result.borrow()).clone(),
                    )
                });
            }
        }
    }
    ({
        StoreBytes_32(
            ((compression_buf.as_pointer() as Ptr<u8>).offset(0_usize)),
            ((*total_compressed_length.borrow()) as usize),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    let __rhs = (({ Round4_70(((*offset.borrow()) as u64)) }) as usize);
    (*offset.borrow_mut()) = __rhs;
    ({
        StoreBytes_32(
            (compressed_metadata_buf.as_pointer() as Ptr<u8>),
            ((*compressed_metadata_buf_length.borrow()) as usize),
            (offset.as_pointer()),
            (*result.borrow()).clone(),
        )
    });
    if {
        let _lhs = ((*result_length.borrow()).read());
        _lhs != (*offset.borrow())
    } {
        eprintln!(
            "Mismatch between computed and actual length ({} vs {})",
            ((*result_length.borrow()).read()),
            (*offset.borrow())
        );
        return false;
    }
    return true;
}
pub fn GetFileContent_110(filename: Vec<u8>) -> Vec<u8> {
    let filename: Value<Vec<u8>> = Rc::new(RefCell::new(filename));
    let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open((filename.as_pointer() as Ptr<u8>).to_string())
            .expect("Failed to open file"),
    ));
    return {
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &(*ifs.borrow()).try_clone().unwrap();
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        __bytes
    };
}
pub fn SetFileContents_111(filename: Vec<u8>, start: Ptr<u8>, end: Ptr<u8>) {
    let filename: Value<Vec<u8>> = Rc::new(RefCell::new(filename));
    let start: Value<Ptr<u8>> = Rc::new(RefCell::new(start));
    let end: Value<Ptr<u8>> = Rc::new(RefCell::new(end));
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create((filename.as_pointer() as Ptr<u8>).to_string())
            .expect("Failed to open file"),
    ));
    {
        (*ofs.borrow_mut()).try_clone().unwrap().write_all(
            (*start.borrow())
                .clone()
                .slice_until(&(*end.borrow()).clone())
                .as_slice(),
        );
        (*ofs.borrow_mut())
            .try_clone()
            .unwrap()
            .try_clone()
            .unwrap()
    };
}
pub fn main() {
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
        argv.iter()
            .map(|x| {
                x.borrow_mut().push(0);
                x.as_pointer()
            })
            .collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<u8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(argv));
    if ((*argc.borrow()) != 2) {
        eprintln!("One argument, the input filename, must be provided.");
        return 1;
    }
    let filename: Value<Vec<u8>> = Rc::new(RefCell::new(
        ((*argv.borrow()).offset((1) as isize).read())
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    let outfilename: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp2 = {
            let mut __tmp1 = (*filename.borrow())[(0_usize) as usize
                ..::std::cmp::min(
                    (0_usize + {
                        let __lookup: Vec<u8> = Ptr::from_string_literal(b".")
                            .to_c_string_iterator()
                            .collect();
                        (*filename.borrow())
                            .iter()
                            .take((*filename.borrow()).len().saturating_sub(1))
                            .rposition(|&x| __lookup.contains(&x))
                            .unwrap_or(usize::MAX)
                    }) as usize,
                    (*filename.borrow()).len().saturating_sub(1),
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }
        .clone();
        __tmp2.pop();
        __tmp2.extend(Ptr::from_string_literal(b".woff2").to_c_string_iterator());
        __tmp2.push(0);
        __tmp2
    }));
    println!(
        "Processing {} => {}",
        (filename.as_pointer() as Ptr<u8>),
        (outfilename.as_pointer() as Ptr<u8>)
    );
    let input: Value<Vec<u8>> = Rc::new(RefCell::new(
        ({ GetFileContent_110((*filename.borrow()).clone()) }),
    ));
    let input_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (input.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
    ));
    let output_size: Value<usize> = Rc::new(RefCell::new(
        ({
            MaxWOFF2CompressedSize_104(
                (*input_data.borrow()).clone(),
                ((*input.borrow()).len() - 1),
            )
        }),
    ));
    let output: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![0_u8; (*output_size.borrow()) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let output_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((output.as_pointer() as Ptr<u8>).offset(0_usize)).reinterpret_cast::<u8>(),
    ));
    let params: Value<woff2_WOFF2Params> =
        Rc::new(RefCell::new(woff2_WOFF2Params::woff2_WOFF2Params()));
    if !({
        ConvertTTFToWOFF2_109(
            (*input_data.borrow()).clone(),
            ((*input.borrow()).len() - 1),
            (*output_data.borrow()).clone(),
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
        let _start: Ptr<u8> = (output.as_pointer() as Ptr<u8>);
        let _end: Ptr<u8> = (output.as_pointer() as Ptr<u8>).to_last();
        SetFileContents_111((*outfilename.borrow()).clone(), _start, _end)
    });
    return 0;
}
