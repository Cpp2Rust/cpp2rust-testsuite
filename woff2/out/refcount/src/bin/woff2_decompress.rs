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
thread_local!(
    pub static kDefaultMaxSize_28: Value<usize> =
        Rc::new(RefCell::new((((128 * 1024) * 1024) as usize)));
);
pub trait woff2_WOFF2Out {
    fn Write_AnyPtr_usize(&self, buf: AnyPtr, n: usize) -> bool;
    fn Write_AnyPtr_usize_usize(&self, buf: AnyPtr, offset: usize, n: usize) -> bool;
    fn Size(&self) -> usize;
}
pub fn Round4_29(value: u64) -> u64 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    if ((<u64>::MAX as u64).wrapping_sub((*value.borrow())) < 3_u64) {
        return (*value.borrow());
    }
    return (((*value.borrow()).wrapping_add(3_u64)) & (!3 as u64));
}
pub fn Round4_30(value: u32) -> u32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    if ((<u32>::MAX as u32).wrapping_sub((*value.borrow())) < 3_u32) {
        return (*value.borrow());
    }
    return (((*value.borrow()).wrapping_add(3_u32)) & (!3 as u32));
}
pub fn StoreU32_31(dst: Ptr<u8>, offset: usize, x: u32) -> usize {
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
pub fn Store16_32(dst: Ptr<u8>, offset: usize, x: i32) -> usize {
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
pub fn StoreU32_33(val: u32, offset: Ptr<usize>, dst: Ptr<u8>) {
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
pub fn Store16_34(val: i32, offset: Ptr<usize>, dst: Ptr<u8>) {
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
pub fn StoreBytes_35(data: Ptr<u8>, len: usize, offset: Ptr<usize>, dst: Ptr<u8>) {
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
#[derive(Default)]
pub struct woff2_TtcFont {
    pub flavor: Value<u32>,
    pub dst_offset: Value<u32>,
    pub header_checksum: Value<u32>,
    pub table_indices: Value<Vec<u16>>,
}
impl Clone for woff2_TtcFont {
    fn clone(&self) -> Self {
        let mut this = Self {
            flavor: Rc::new(RefCell::new((*self.flavor.borrow()))),
            dst_offset: Rc::new(RefCell::new((*self.dst_offset.borrow()))),
            header_checksum: Rc::new(RefCell::new((*self.header_checksum.borrow()))),
            table_indices: Rc::new(RefCell::new((*self.table_indices.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for woff2_TtcFont {}
#[derive(Default)]
pub struct woff2_WOFF2Header {
    pub flavor: Value<u32>,
    pub header_version: Value<u32>,
    pub num_tables: Value<u16>,
    pub compressed_offset: Value<u64>,
    pub compressed_length: Value<u32>,
    pub uncompressed_size: Value<u32>,
    pub tables: Value<Vec<woff2_Table>>,
    pub ttc_fonts: Value<Vec<woff2_TtcFont>>,
}
impl Clone for woff2_WOFF2Header {
    fn clone(&self) -> Self {
        let mut this = Self {
            flavor: Rc::new(RefCell::new((*self.flavor.borrow()))),
            header_version: Rc::new(RefCell::new((*self.header_version.borrow()))),
            num_tables: Rc::new(RefCell::new((*self.num_tables.borrow()))),
            compressed_offset: Rc::new(RefCell::new((*self.compressed_offset.borrow()))),
            compressed_length: Rc::new(RefCell::new((*self.compressed_length.borrow()))),
            uncompressed_size: Rc::new(RefCell::new((*self.uncompressed_size.borrow()))),
            tables: Rc::new(RefCell::new((*self.tables.borrow()).clone())),
            ttc_fonts: Rc::new(RefCell::new((*self.ttc_fonts.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for woff2_WOFF2Header {}
#[derive(Default)]
pub struct woff2_WOFF2FontInfo {
    pub num_glyphs: Value<u16>,
    pub index_format: Value<u16>,
    pub num_hmetrics: Value<u16>,
    pub x_mins: Value<Vec<i16>>,
    pub table_entry_by_tag: Value<BTreeMap<u32, Value<u32>>>,
}
impl Clone for woff2_WOFF2FontInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            num_glyphs: Rc::new(RefCell::new((*self.num_glyphs.borrow()))),
            index_format: Rc::new(RefCell::new((*self.index_format.borrow()))),
            num_hmetrics: Rc::new(RefCell::new((*self.num_hmetrics.borrow()))),
            x_mins: Rc::new(RefCell::new((*self.x_mins.borrow()).clone())),
            table_entry_by_tag: Rc::new(RefCell::new(
                (*self.table_entry_by_tag.borrow())
                    .iter()
                    .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                    .collect(),
            )),
        };
        this
    }
}
impl ByteRepr for woff2_WOFF2FontInfo {}
#[derive(Default)]
pub struct woff2_RebuildMetadata {
    pub header_checksum: Value<u32>,
    pub font_infos: Value<Vec<woff2_WOFF2FontInfo>>,
    pub checksums: Value<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>,
}
impl Clone for woff2_RebuildMetadata {
    fn clone(&self) -> Self {
        let mut this = Self {
            header_checksum: Rc::new(RefCell::new((*self.header_checksum.borrow()))),
            font_infos: Rc::new(RefCell::new((*self.font_infos.borrow()).clone())),
            checksums: Rc::new(RefCell::new(
                (*self.checksums.borrow())
                    .iter()
                    .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                    .collect(),
            )),
        };
        this
    }
}
impl ByteRepr for woff2_RebuildMetadata {}
pub fn WithSign_55(flag: i32, baseval: i32) -> i32 {
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let baseval: Value<i32> = Rc::new(RefCell::new(baseval));
    return if (((*flag.borrow()) & 1) != 0) {
        (*baseval.borrow())
    } else {
        -(*baseval.borrow())
    };
}
pub fn _SafeIntAddition_56(a: i32, b: i32, result: Ptr<i32>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let result: Value<Ptr<i32>> = Rc::new(RefCell::new(result));
    if ((((((*a.borrow()) > 0) && ((*b.borrow()) > (<i32>::MAX - (*a.borrow()))))
        || (((*a.borrow()) < 0) && ((*b.borrow()) < (<i32>::MIN - (*a.borrow())))))
        as i64)
        != 0)
    {
        return false;
    }
    let __rhs = ((*a.borrow()) + (*b.borrow()));
    (*result.borrow()).write(__rhs);
    return true;
}
pub fn TripletDecode_57(
    flags_in: Ptr<u8>,
    in_: Ptr<u8>,
    in_size: usize,
    n_points: u32,
    result: Ptr<woff2_Point>,
    in_bytes_consumed: Ptr<usize>,
) -> bool {
    let flags_in: Value<Ptr<u8>> = Rc::new(RefCell::new(flags_in));
    let in_: Value<Ptr<u8>> = Rc::new(RefCell::new(in_));
    let in_size: Value<usize> = Rc::new(RefCell::new(in_size));
    let n_points: Value<u32> = Rc::new(RefCell::new(n_points));
    let result: Value<Ptr<woff2_Point>> = Rc::new(RefCell::new(result));
    let in_bytes_consumed: Value<Ptr<usize>> = Rc::new(RefCell::new(in_bytes_consumed));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    if (((((*n_points.borrow()) as usize) > (*in_size.borrow())) as i64) != 0) {
        return false;
    }
    let triplet_index: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*n_points.borrow())) {
        let flag: Value<u8> = Rc::new(RefCell::new(
            ((*flags_in.borrow()).offset((*i.borrow()) as isize).read()),
        ));
        let on_curve: Value<bool> = Rc::new(RefCell::new(!((((*flag.borrow()) as i32) >> 7) != 0)));
        let rhs_0 = (((*flag.borrow()) as i32) & 127) as u8;
        (*flag.borrow_mut()) = rhs_0;
        let n_data_bytes: Value<u32> = <Value<u32>>::default();
        if (((*flag.borrow()) as i32) < 84) {
            (*n_data_bytes.borrow_mut()) = 1_u32;
        } else if (((*flag.borrow()) as i32) < 120) {
            (*n_data_bytes.borrow_mut()) = 2_u32;
        } else if (((*flag.borrow()) as i32) < 124) {
            (*n_data_bytes.borrow_mut()) = 3_u32;
        } else {
            (*n_data_bytes.borrow_mut()) = 4_u32;
        }
        if (((((((*triplet_index.borrow()).wrapping_add((*n_data_bytes.borrow()))) as usize)
            > (*in_size.borrow()))
            || ((*triplet_index.borrow()).wrapping_add((*n_data_bytes.borrow()))
                < (*triplet_index.borrow()))) as i64)
            != 0)
        {
            return false;
        }
        let dx: Value<i32> = <Value<i32>>::default();
        let dy: Value<i32> = <Value<i32>>::default();
        if (((*flag.borrow()) as i32) < 10) {
            (*dx.borrow_mut()) = 0;
            (*dy.borrow_mut()) = ({
                let _flag: i32 = ((*flag.borrow()) as i32);
                let _baseval: i32 = {
                    let _lhs = ((((*flag.borrow()) as i32) & 14) << 7);
                    _lhs + (((*in_.borrow())
                        .offset((*triplet_index.borrow()) as isize)
                        .read()) as i32)
                };
                WithSign_55(_flag, _baseval)
            });
        } else if (((*flag.borrow()) as i32) < 20) {
            (*dx.borrow_mut()) = ({
                let _flag: i32 = ((*flag.borrow()) as i32);
                let _baseval: i32 = {
                    let _lhs = (((((*flag.borrow()) as i32) - 10) & 14) << 7);
                    _lhs + (((*in_.borrow())
                        .offset((*triplet_index.borrow()) as isize)
                        .read()) as i32)
                };
                WithSign_55(_flag, _baseval)
            });
            (*dy.borrow_mut()) = 0;
        } else if (((*flag.borrow()) as i32) < 84) {
            let b0: Value<i32> = Rc::new(RefCell::new((((*flag.borrow()) as i32) - 20)));
            let b1: Value<i32> = Rc::new(RefCell::new(
                (((*in_.borrow())
                    .offset((*triplet_index.borrow()) as isize)
                    .read()) as i32),
            ));
            (*dx.borrow_mut()) = ({
                WithSign_55(
                    ((*flag.borrow()) as i32),
                    ((1 + ((*b0.borrow()) & 48)) + ((*b1.borrow()) >> 4)),
                )
            });
            (*dy.borrow_mut()) = ({
                WithSign_55(
                    (((*flag.borrow()) as i32) >> 1),
                    ((1 + (((*b0.borrow()) & 12) << 2)) + ((*b1.borrow()) & 15)),
                )
            });
        } else if (((*flag.borrow()) as i32) < 120) {
            let b0: Value<i32> = Rc::new(RefCell::new((((*flag.borrow()) as i32) - 84)));
            (*dx.borrow_mut()) = ({
                WithSign_55(((*flag.borrow()) as i32), {
                    let _lhs = (1 + (((*b0.borrow()) / 12) << 8));
                    _lhs + (((*in_.borrow())
                        .offset((*triplet_index.borrow()) as isize)
                        .read()) as i32)
                })
            });
            (*dy.borrow_mut()) = ({
                WithSign_55((((*flag.borrow()) as i32) >> 1), {
                    let _lhs = (1 + ((((*b0.borrow()) % 12) >> 2) << 8));
                    _lhs + (((*in_.borrow())
                        .offset(((*triplet_index.borrow()).wrapping_add(1_u32)) as isize)
                        .read()) as i32)
                })
            });
        } else if (((*flag.borrow()) as i32) < 124) {
            let b2: Value<i32> = Rc::new(RefCell::new(
                (((*in_.borrow())
                    .offset(((*triplet_index.borrow()).wrapping_add(1_u32)) as isize)
                    .read()) as i32),
            ));
            (*dx.borrow_mut()) = ({
                WithSign_55(((*flag.borrow()) as i32), {
                    let _lhs = ((((*in_.borrow())
                        .offset((*triplet_index.borrow()) as isize)
                        .read()) as i32)
                        << 4);
                    _lhs + ((*b2.borrow()) >> 4)
                })
            });
            (*dy.borrow_mut()) = ({
                WithSign_55((((*flag.borrow()) as i32) >> 1), {
                    let _lhs = (((*b2.borrow()) & 15) << 8);
                    _lhs + (((*in_.borrow())
                        .offset(((*triplet_index.borrow()).wrapping_add(2_u32)) as isize)
                        .read()) as i32)
                })
            });
        } else {
            (*dx.borrow_mut()) = ({
                WithSign_55(
                    ((*flag.borrow()) as i32),
                    (((((*in_.borrow())
                        .offset((*triplet_index.borrow()) as isize)
                        .read()) as i32)
                        << 8)
                        + (((*in_.borrow())
                            .offset(((*triplet_index.borrow()).wrapping_add(1_u32)) as isize)
                            .read()) as i32)),
                )
            });
            (*dy.borrow_mut()) = ({
                WithSign_55(
                    (((*flag.borrow()) as i32) >> 1),
                    (((((*in_.borrow())
                        .offset(((*triplet_index.borrow()).wrapping_add(2_u32)) as isize)
                        .read()) as i32)
                        << 8)
                        + (((*in_.borrow())
                            .offset(((*triplet_index.borrow()).wrapping_add(3_u32)) as isize)
                            .read()) as i32)),
                )
            });
        }
        let rhs_0 = (*triplet_index.borrow()).wrapping_add((*n_data_bytes.borrow()));
        (*triplet_index.borrow_mut()) = rhs_0;
        if !({
            let _a: i32 = (*x.borrow());
            let _result: Ptr<i32> = (x.as_pointer());
            _SafeIntAddition_56(_a, (*dx.borrow()), _result)
        }) {
            return false;
        }
        if !({
            let _a: i32 = (*y.borrow());
            let _result: Ptr<i32> = (y.as_pointer());
            _SafeIntAddition_56(_a, (*dy.borrow()), _result)
        }) {
            return false;
        }
        let __rhs = woff2_Point {
            x: Rc::new(RefCell::new((*x.borrow()))),
            y: Rc::new(RefCell::new((*y.borrow()))),
            on_curve: Rc::new(RefCell::new((*on_curve.borrow()))),
        };
        (*result.borrow_mut()).postfix_inc().write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ((*triplet_index.borrow()) as usize);
    (*in_bytes_consumed.borrow()).write(__rhs);
    return true;
}
pub fn StorePoints_58(
    n_points: u32,
    points: Ptr<woff2_Point>,
    n_contours: u32,
    instruction_length: u32,
    has_overlap_bit: bool,
    dst: Ptr<u8>,
    dst_size: usize,
    glyph_size: Ptr<usize>,
) -> bool {
    let n_points: Value<u32> = Rc::new(RefCell::new(n_points));
    let points: Value<Ptr<woff2_Point>> = Rc::new(RefCell::new(points));
    let n_contours: Value<u32> = Rc::new(RefCell::new(n_contours));
    let instruction_length: Value<u32> = Rc::new(RefCell::new(instruction_length));
    let has_overlap_bit: Value<bool> = Rc::new(RefCell::new(has_overlap_bit));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let glyph_size: Value<Ptr<usize>> = Rc::new(RefCell::new(glyph_size));
    let flag_offset: Value<u32> = Rc::new(RefCell::new(
        (((((*kEndPtsOfContoursOffset_51.with(Value::clone).borrow())
            .wrapping_add((((2_u32).wrapping_mul((*n_contours.borrow()))) as usize)))
        .wrapping_add(2_usize))
        .wrapping_add(((*instruction_length.borrow()) as usize))) as u32),
    ));
    let last_flag: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let repeat_count: Value<i32> = Rc::new(RefCell::new(0));
    let last_x: Value<i32> = Rc::new(RefCell::new(0));
    let last_y: Value<i32> = Rc::new(RefCell::new(0));
    let x_bytes: Value<u32> = Rc::new(RefCell::new(0_u32));
    let y_bytes: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*n_points.borrow())) {
        let point: Ptr<woff2_Point> = (*points.borrow()).offset((*i.borrow()) as isize);
        let flag: Value<i32> = Rc::new(RefCell::new(
            if (*(*point.upgrade().deref()).on_curve.borrow()) {
                (*kGlyfOnCurve_36.with(Value::clone).borrow())
            } else {
                0
            },
        ));
        if (*has_overlap_bit.borrow()) && ((*i.borrow()) == 0_u32) {
            (*flag.borrow_mut()) |= (*kOverlapSimple_42.with(Value::clone).borrow());
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
            (*flag.borrow_mut()) |= (*kGlyfThisXIsSame_40.with(Value::clone).borrow());
        } else if ((*dx.borrow()) > -256_i32) && ((*dx.borrow()) < 256) {
            (*flag.borrow_mut()) |= ((*kGlyfXShort_37.with(Value::clone).borrow())
                | (if ((*dx.borrow()) > 0) {
                    (*kGlyfThisXIsSame_40.with(Value::clone).borrow())
                } else {
                    0
                }));
            let rhs_0 = (*x_bytes.borrow()).wrapping_add(1_u32);
            (*x_bytes.borrow_mut()) = rhs_0;
        } else {
            let rhs_0 = (*x_bytes.borrow()).wrapping_add(2_u32);
            (*x_bytes.borrow_mut()) = rhs_0;
        }
        if ((*dy.borrow()) == 0) {
            (*flag.borrow_mut()) |= (*kGlyfThisYIsSame_41.with(Value::clone).borrow());
        } else if ((*dy.borrow()) > -256_i32) && ((*dy.borrow()) < 256) {
            (*flag.borrow_mut()) |= ((*kGlyfYShort_38.with(Value::clone).borrow())
                | (if ((*dy.borrow()) > 0) {
                    (*kGlyfThisYIsSame_41.with(Value::clone).borrow())
                } else {
                    0
                }));
            let rhs_0 = (*y_bytes.borrow()).wrapping_add(1_u32);
            (*y_bytes.borrow_mut()) = rhs_0;
        } else {
            let rhs_0 = (*y_bytes.borrow()).wrapping_add(2_u32);
            (*y_bytes.borrow_mut()) = rhs_0;
        }
        if ((*flag.borrow()) == (*last_flag.borrow())) && ((*repeat_count.borrow()) != 255) {
            let rhs_0 = ((((*dst.borrow())
                .offset(((*flag_offset.borrow()).wrapping_sub(1_u32)) as isize)
                .read()) as i32)
                | (*kGlyfRepeat_39.with(Value::clone).borrow())) as u8;
            (*dst.borrow())
                .offset(((*flag_offset.borrow()).wrapping_sub(1_u32)) as isize)
                .write(rhs_0);
            (*repeat_count.borrow_mut()).postfix_inc();
        } else {
            if ((*repeat_count.borrow()) != 0) {
                if (((((*flag_offset.borrow()) as usize) >= (*dst_size.borrow())) as i64) != 0) {
                    return false;
                }
                let __rhs = ((*repeat_count.borrow()) as u8);
                (*dst.borrow())
                    .offset(((*flag_offset.borrow_mut()).postfix_inc()) as isize)
                    .write(__rhs);
            }
            if (((((*flag_offset.borrow()) as usize) >= (*dst_size.borrow())) as i64) != 0) {
                return false;
            }
            let __rhs = ((*flag.borrow()) as u8);
            (*dst.borrow())
                .offset(((*flag_offset.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
            (*repeat_count.borrow_mut()) = 0;
        }
        (*last_x.borrow_mut()) = (*(*point.upgrade().deref()).x.borrow());
        (*last_y.borrow_mut()) = (*(*point.upgrade().deref()).y.borrow());
        (*last_flag.borrow_mut()) = (*flag.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*repeat_count.borrow()) != 0) {
        if (((((*flag_offset.borrow()) as usize) >= (*dst_size.borrow())) as i64) != 0) {
            return false;
        }
        let __rhs = ((*repeat_count.borrow()) as u8);
        (*dst.borrow())
            .offset(((*flag_offset.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
    }
    let xy_bytes: Value<u32> = Rc::new(RefCell::new(
        (*x_bytes.borrow()).wrapping_add((*y_bytes.borrow())),
    ));
    if ((((((*xy_bytes.borrow()) < (*x_bytes.borrow()))
        || ((*flag_offset.borrow()).wrapping_add((*xy_bytes.borrow())) < (*flag_offset.borrow())))
        || ((((*flag_offset.borrow()).wrapping_add((*xy_bytes.borrow()))) as usize)
            > (*dst_size.borrow()))) as i64)
        != 0)
    {
        return false;
    }
    let x_offset: Value<i32> = Rc::new(RefCell::new(((*flag_offset.borrow()) as i32)));
    let y_offset: Value<i32> = Rc::new(RefCell::new(
        (((*flag_offset.borrow()).wrapping_add((*x_bytes.borrow()))) as i32),
    ));
    (*last_x.borrow_mut()) = 0;
    (*last_y.borrow_mut()) = 0;
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*n_points.borrow())) {
        let dx: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*(*points.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow());
            _lhs - (*last_x.borrow())
        }));
        if ((*dx.borrow()) == 0) {
        } else if ((*dx.borrow()) > -256_i32) && ((*dx.borrow()) < 256) {
            let __rhs = ((*dx.borrow()).abs() as u8);
            (*dst.borrow())
                .offset(((*x_offset.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
        } else {
            let __rhs = (({
                Store16_32(
                    (*dst.borrow()).clone(),
                    ((*x_offset.borrow()) as usize),
                    (*dx.borrow()),
                )
            }) as i32);
            (*x_offset.borrow_mut()) = __rhs;
        }
        (*last_x.borrow_mut()) += (*dx.borrow());
        let dy: Value<i32> = Rc::new(RefCell::new({
            let _lhs = (*(*(*points.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow());
            _lhs - (*last_y.borrow())
        }));
        if ((*dy.borrow()) == 0) {
        } else if ((*dy.borrow()) > -256_i32) && ((*dy.borrow()) < 256) {
            let __rhs = ((*dy.borrow()).abs() as u8);
            (*dst.borrow())
                .offset(((*y_offset.borrow_mut()).postfix_inc()) as isize)
                .write(__rhs);
        } else {
            let __rhs = (({
                Store16_32(
                    (*dst.borrow()).clone(),
                    ((*y_offset.borrow()) as usize),
                    (*dy.borrow()),
                )
            }) as i32);
            (*y_offset.borrow_mut()) = __rhs;
        }
        (*last_y.borrow_mut()) += (*dy.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ((*y_offset.borrow()) as usize);
    (*glyph_size.borrow()).write(__rhs);
    return true;
}
pub fn ComputeBbox_59(n_points: u32, points: Ptr<woff2_Point>, dst: Ptr<u8>) {
    let n_points: Value<u32> = Rc::new(RefCell::new(n_points));
    let points: Value<Ptr<woff2_Point>> = Rc::new(RefCell::new(points));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(dst));
    let x_min: Value<i32> = Rc::new(RefCell::new(0));
    let y_min: Value<i32> = Rc::new(RefCell::new(0));
    let x_max: Value<i32> = Rc::new(RefCell::new(0));
    let y_max: Value<i32> = Rc::new(RefCell::new(0));
    if ((*n_points.borrow()) > 0_u32) {
        (*x_min.borrow_mut()) = (*(*(*points.borrow()).offset((0) as isize).upgrade().deref())
            .x
            .borrow());
        (*x_max.borrow_mut()) = (*(*(*points.borrow()).offset((0) as isize).upgrade().deref())
            .x
            .borrow());
        (*y_min.borrow_mut()) = (*(*(*points.borrow()).offset((0) as isize).upgrade().deref())
            .y
            .borrow());
        (*y_max.borrow_mut()) = (*(*(*points.borrow()).offset((0) as isize).upgrade().deref())
            .y
            .borrow());
    }
    let i: Value<u32> = Rc::new(RefCell::new(1_u32));
    'loop_: while ((*i.borrow()) < (*n_points.borrow())) {
        let x: Value<i32> = Rc::new(RefCell::new(
            (*(*(*points.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow()),
        ));
        let y: Value<i32> = Rc::new(RefCell::new(
            (*(*(*points.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow()),
        ));
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
        (*i.borrow_mut()).prefix_inc();
    }
    let offset: Value<usize> = Rc::new(RefCell::new(2_usize));
    let __rhs = ({
        Store16_32(
            (*dst.borrow()).clone(),
            (*offset.borrow()),
            (*x_min.borrow()),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*dst.borrow()).clone(),
            (*offset.borrow()),
            (*y_min.borrow()),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*dst.borrow()).clone(),
            (*offset.borrow()),
            (*x_max.borrow()),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*dst.borrow()).clone(),
            (*offset.borrow()),
            (*y_max.borrow()),
        )
    });
    (*offset.borrow_mut()) = __rhs;
}
pub fn SizeOfComposite_60(
    composite_stream: woff2_Buffer,
    size: Ptr<usize>,
    have_instructions: Ptr<bool>,
) -> bool {
    let composite_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(composite_stream));
    let size: Value<Ptr<usize>> = Rc::new(RefCell::new(size));
    let have_instructions: Value<Ptr<bool>> = Rc::new(RefCell::new(have_instructions));
    let start_offset: Value<usize> =
        Rc::new(RefCell::new(({ (*composite_stream.borrow()).offset() })));
    let we_have_instructions: Value<bool> = Rc::new(RefCell::new(false));
    let flags: Value<u16> = Rc::new(RefCell::new(
        ((*FLAG_MORE_COMPONENTS_45.with(Value::clone).borrow()) as u16),
    ));
    'loop_: while ((((*flags.borrow()) as i32)
        & (*FLAG_MORE_COMPONENTS_45.with(Value::clone).borrow()))
        != 0)
    {
        if ((!({ (*composite_stream.borrow()).ReadU16((flags.as_pointer())) }) as i64) != 0) {
            return false;
        }
        let rhs_0 = (((*we_have_instructions.borrow()) as i32)
            | (((((*flags.borrow()) as i32)
                & (*FLAG_WE_HAVE_INSTRUCTIONS_48.with(Value::clone).borrow()))
                != 0) as i32))
            != 0;
        (*we_have_instructions.borrow_mut()) = rhs_0;
        let arg_size: Value<usize> = Rc::new(RefCell::new(2_usize));
        if ((((*flags.borrow()) as i32)
            & (*FLAG_ARG_1_AND_2_ARE_WORDS_43.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(4_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(2_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        }
        if ((((*flags.borrow()) as i32) & (*FLAG_WE_HAVE_A_SCALE_44.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(2_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else if ((((*flags.borrow()) as i32)
            & (*FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(4_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        } else if ((((*flags.borrow()) as i32)
            & (*FLAG_WE_HAVE_A_TWO_BY_TWO_47.with(Value::clone).borrow()))
            != 0)
        {
            let rhs_0 = (*arg_size.borrow()).wrapping_add(8_usize);
            (*arg_size.borrow_mut()) = rhs_0;
        }
        if ((!({ (*composite_stream.borrow()).Skip((*arg_size.borrow())) }) as i64) != 0) {
            return false;
        }
    }
    let __rhs = ({ (*composite_stream.borrow()).offset() }).wrapping_sub((*start_offset.borrow()));
    (*size.borrow()).write(__rhs);
    let __rhs = (*we_have_instructions.borrow());
    (*have_instructions.borrow()).write(__rhs);
    return true;
}
pub fn Pad4_61(out: PtrDyn<dyn woff2_WOFF2Out>) -> bool {
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let zeroes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([0_u8, 0_u8, 0_u8])));
    if (((({ (*(*out.borrow()).upgrade().deref()).Size() }).wrapping_add(3_usize)
        < ({ (*(*out.borrow()).upgrade().deref()).Size() })) as i64)
        != 0)
    {
        return false;
    }
    let pad_bytes: Value<u32> = Rc::new(RefCell::new(
        ((({ Round4_29((({ (*(*out.borrow()).upgrade().deref()).Size() }) as u64)) })
            .wrapping_sub((({ (*(*out.borrow()).upgrade().deref()).Size() }) as u64)))
            as u32),
    ));
    if ((*pad_bytes.borrow()) > 0_u32) {
        if ((!({
            (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(
                ((zeroes.as_pointer()) as Ptr<u8>).to_any(),
                ((*pad_bytes.borrow()) as usize),
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
    index_format: i32,
    checksum: Ptr<u32>,
    out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let index_format: Value<i32> = Rc::new(RefCell::new(index_format));
    let checksum: Value<Ptr<u32>> = Rc::new(RefCell::new(checksum));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let loca_size: Value<u64> = Rc::new(RefCell::new(
        ((*loca_values.upgrade().deref()).len() as u64),
    ));
    let offset_size: Value<u64> = Rc::new(RefCell::new(
        (if ((*index_format.borrow()) != 0) {
            4
        } else {
            2
        } as u64),
    ));
    if ((((((*loca_size.borrow()) << 2) >> 2) != (*loca_size.borrow())) as i64) != 0) {
        return false;
    }
    let loca_content: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((*loca_size.borrow()).wrapping_mul((*offset_size.borrow()))) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((loca_content.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
    ));
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*loca_values.upgrade().deref()).len()
    } {
        let value: Value<u32> = Rc::new(RefCell::new(
            ((loca_values.to_strong().as_pointer() as Ptr<u32>)
                .offset((*i.borrow()) as isize)
                .read()),
        ));
        if ((*index_format.borrow()) != 0) {
            let __rhs = ({
                StoreU32_31(
                    (*dst.borrow()).clone(),
                    (*offset.borrow()),
                    (*value.borrow()),
                )
            });
            (*offset.borrow_mut()) = __rhs;
        } else {
            let __rhs = ({
                Store16_32(
                    (*dst.borrow()).clone(),
                    (*offset.borrow()),
                    (((*value.borrow()) >> 1) as i32),
                )
            });
            (*offset.borrow_mut()) = __rhs;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ({
        let _buf: Ptr<u8> = ((loca_content.as_pointer() as Ptr<u8>).offset(0_usize as isize));
        let _size: usize = (*loca_content.borrow()).len();
        ComputeULongSum_26(_buf, _size)
    });
    (*checksum.borrow()).write(__rhs);
    if ((!({
        let _buf: AnyPtr =
            (((loca_content.as_pointer() as Ptr<u8>).offset(0_usize as isize)) as Ptr<u8>).to_any();
        let _n: usize = (*loca_content.borrow()).len();
        (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReconstructGlyf_63(
    data: Ptr<u8>,
    glyf_table: Ptr<woff2_Table>,
    glyf_checksum: Ptr<u32>,
    loca_table: Ptr<woff2_Table>,
    loca_checksum: Ptr<u32>,
    info: Ptr<woff2_WOFF2FontInfo>,
    out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let glyf_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(glyf_table));
    let glyf_checksum: Value<Ptr<u32>> = Rc::new(RefCell::new(glyf_checksum));
    let loca_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(loca_table));
    let loca_checksum: Value<Ptr<u32>> = Rc::new(RefCell::new(loca_checksum));
    let info: Value<Ptr<woff2_WOFF2FontInfo>> = Rc::new(RefCell::new(info));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    thread_local!(
        static kNumSubStreams_64: Value<i32> = Rc::new(RefCell::new(7));
    );
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        ((*(*(*glyf_table.borrow()).upgrade().deref())
            .transform_length
            .borrow()) as usize),
    )));
    let version: Value<u16> = <Value<u16>>::default();
    let substreams: Value<Vec<(Value<Ptr<u8>>, Value<u64>)>> = Rc::new(RefCell::new(
        (0..((*kNumSubStreams_64.with(Value::clone).borrow()) as usize) as usize)
            .map(|_| <(Value<Ptr<u8>>, Value<u64>)>::default())
            .collect::<Vec<_>>(),
    ));
    let glyf_start: Value<usize> = Rc::new(RefCell::new(
        ({ (*(*out.borrow()).upgrade().deref()).Size() }),
    ));
    if ((!({ (*file.borrow()).ReadU16((version.as_pointer())) }) as i64) != 0) {
        return false;
    }
    let flags: Value<u16> = <Value<u16>>::default();
    if ((!({ (*file.borrow()).ReadU16((flags.as_pointer())) }) as i64) != 0) {
        return false;
    }
    let has_overlap_bitmap: Value<bool> = Rc::new(RefCell::new(
        ((((*flags.borrow()) as i32)
            & (*FLAG_OVERLAP_SIMPLE_BITMAP_49.with(Value::clone).borrow()))
            != 0),
    ));
    if ((((!({
        (*file.borrow()).ReadU16(
            ((*(*info.borrow()).upgrade().deref())
                .num_glyphs
                .as_pointer()),
        )
    })) || (!({
        (*file.borrow()).ReadU16(
            ((*(*info.borrow()).upgrade().deref())
                .index_format
                .as_pointer()),
        )
    }))) as i64)
        != 0)
    {
        return false;
    }
    let expected_loca_dst_length: Value<u32> = Rc::new(RefCell::new(
        ((if ((*(*(*info.borrow()).upgrade().deref()).index_format.borrow()) != 0) {
            4
        } else {
            2
        }) as u32)
            .wrapping_mul(
                (((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as u32)
                    .wrapping_add(1_u32)),
            ),
    ));
    if ((({
        let _lhs = (*(*(*loca_table.borrow()).upgrade().deref())
            .dst_length
            .borrow());
        _lhs != (*expected_loca_dst_length.borrow())
    }) as i64)
        != 0)
    {
        return false;
    }
    let offset: Value<u32> = Rc::new(RefCell::new(
        (((2 + (*kNumSubStreams_64.with(Value::clone).borrow())) * 4) as u32),
    ));
    if ((({
        let _lhs = (*offset.borrow());
        _lhs > (*(*(*glyf_table.borrow()).upgrade().deref())
            .transform_length
            .borrow())
    }) as i64)
        != 0)
    {
        return false;
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*kNumSubStreams_64.with(Value::clone).borrow())) {
        let substream_size: Value<u32> = <Value<u32>>::default();
        if ((!({ (*file.borrow()).ReadU32((substream_size.as_pointer())) }) as i64) != 0) {
            return false;
        }
        if ((({
            let _lhs = (*substream_size.borrow());
            _lhs > (*(*(*glyf_table.borrow()).upgrade().deref())
                .transform_length
                .borrow())
            .wrapping_sub((*offset.borrow()))
        }) as i64)
            != 0)
        {
            return false;
        }
        (substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(((*i.borrow()) as usize) as isize)
            .write((
                Rc::new(RefCell::new(
                    (*data.borrow())
                        .offset((*offset.borrow()) as isize)
                        .try_into()
                        .expect("failed conversion"),
                )),
                Rc::new(RefCell::new(
                    (*substream_size.borrow())
                        .try_into()
                        .expect("failed conversion"),
                )),
            ));
        let rhs_0 = (*offset.borrow()).wrapping_add((*substream_size.borrow()));
        (*offset.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    let n_contour_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let n_points_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let flag_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(2_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(2_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let glyph_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(3_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(3_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let composite_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(4_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(4_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let bbox_stream: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(5_usize as isize)
            .upgrade()
            .deref())
        .0
        .borrow())
        .clone(),
        ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
            .offset(5_usize as isize)
            .upgrade()
            .deref())
        .1
        .borrow()) as usize),
    )));
    let instruction_stream: Value<woff2_Buffer> =
        Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
            (*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
                .offset(6_usize as isize)
                .upgrade()
                .deref())
            .0
            .borrow())
            .clone(),
            ((*(*(substreams.as_pointer() as Ptr<(Value<Ptr<u8>>, Value<u64>)>)
                .offset(6_usize as isize)
                .upgrade()
                .deref())
            .1
            .borrow()) as usize),
        )));
    let overlap_bitmap: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    let overlap_bitmap_length: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (*has_overlap_bitmap.borrow()) {
        (*overlap_bitmap_length.borrow_mut()) =
            (((((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as i32) + 7) >> 3)
                as u32);
        (*overlap_bitmap.borrow_mut()) = (*data.borrow()).offset((*offset.borrow()) as isize);
        if ((({
            let _lhs = (*overlap_bitmap_length.borrow());
            _lhs > (*(*(*glyf_table.borrow()).upgrade().deref())
                .transform_length
                .borrow())
            .wrapping_sub((*offset.borrow()))
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    let loca_values: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..((((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as i32) + 1) as usize)
            as usize)
            .map(|_| <u32>::default())
            .collect::<Vec<_>>(),
    ));
    let n_points_vec: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let points: Value<Option<Value<Box<[woff2_Point]>>>> = Rc::new(RefCell::new(None));
    let points_size: Value<usize> = Rc::new(RefCell::new(0_usize));
    let bbox_bitmap: Value<Ptr<u8>> = Rc::new(RefCell::new(({ (*bbox_stream.borrow()).buffer() })));
    let bitmap_length: Value<u32> = Rc::new(RefCell::new(
        ((((((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as i32) + 31) >> 5) << 2)
            as u32),
    ));
    if !({ (*bbox_stream.borrow()).Skip(((*bitmap_length.borrow()) as usize)) }) {
        return false;
    }
    let glyph_buf_size: Value<usize> = Rc::new(RefCell::new(
        (*kDefaultGlyphBuf_53.with(Value::clone).borrow()),
    ));
    let glyph_buf: Value<Option<Value<Box<[u8]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array(
            (0..(*glyph_buf_size.borrow()))
                .map(|_| <u8>::default())
                .collect::<Box<[u8]>>(),
        )
        .to_owned_opt(),
    ));
    {
        let __a0 = ((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as usize) as usize;
        (*(*(*info.borrow()).upgrade().deref()).x_mins.borrow_mut())
            .resize_with(__a0, || <i16>::default())
    };
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < ((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as u32)
    } {
        let glyph_size: Value<usize> = Rc::new(RefCell::new(0_usize));
        let n_contours: Value<u16> = Rc::new(RefCell::new(0_u16));
        let have_bbox: Value<bool> = Rc::new(RefCell::new(false));
        if ({
            let _lhs = (((*bbox_bitmap.borrow())
                .offset(((*i.borrow()) >> 3) as isize)
                .read()) as i32);
            _lhs & (128 >> ((*i.borrow()) & 7_u32))
        } != 0)
        {
            (*have_bbox.borrow_mut()) = true;
        }
        if ((!({ (*n_contour_stream.borrow()).ReadU16((n_contours.as_pointer())) }) as i64) != 0) {
            return false;
        }
        if (((*n_contours.borrow()) as i32) == 65535) {
            let have_instructions: Value<bool> = Rc::new(RefCell::new(false));
            let instruction_size: Value<u32> = Rc::new(RefCell::new(0_u32));
            if ((!(*have_bbox.borrow()) as i64) != 0) {
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
            let size_needed: Value<usize> = Rc::new(RefCell::new(
                ((12_usize).wrapping_add((*composite_size.borrow())))
                    .wrapping_add(((*instruction_size.borrow()) as usize)),
            ));
            if ((((*glyph_buf_size.borrow()) < (*size_needed.borrow())) as i64) != 0) {
                (*glyph_buf.borrow_mut()) = Ptr::alloc_array(
                    (0..(*size_needed.borrow()))
                        .map(|_| <u8>::default())
                        .collect::<Box<[u8]>>(),
                )
                .to_owned_opt();
                (*glyph_buf_size.borrow_mut()) = (*size_needed.borrow());
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
                (*bbox_stream.borrow()).Read(
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
            let rhs_0 = (*glyph_size.borrow()).wrapping_add(8_usize);
            (*glyph_size.borrow_mut()) = rhs_0;
            if ((!({
                (*composite_stream.borrow()).Read(
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
            let rhs_0 = (*glyph_size.borrow()).wrapping_add((*composite_size.borrow()));
            (*glyph_size.borrow_mut()) = rhs_0;
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
                    (*instruction_stream.borrow()).Read(
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
                let rhs_0 =
                    (*glyph_size.borrow()).wrapping_add(((*instruction_size.borrow()) as usize));
                (*glyph_size.borrow_mut()) = rhs_0;
            }
        } else if (((*n_contours.borrow()) as i32) > 0) {
            (*n_points_vec.borrow_mut()).clear();
            let total_n_points: Value<u32> = Rc::new(RefCell::new(0_u32));
            let n_points_contour: Value<u32> = <Value<u32>>::default();
            let j: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*j.borrow()) < ((*n_contours.borrow()) as u32)) {
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
                    (*n_points_vec.borrow_mut()).push(a0_clone)
                };
                if ((((*total_n_points.borrow()).wrapping_add((*n_points_contour.borrow()))
                    < (*total_n_points.borrow())) as i64)
                    != 0)
                {
                    return false;
                }
                let rhs_0 = (*total_n_points.borrow()).wrapping_add((*n_points_contour.borrow()));
                (*total_n_points.borrow_mut()) = rhs_0;
                (*j.borrow_mut()).prefix_inc();
            }
            let flag_size: Value<u32> = Rc::new(RefCell::new((*total_n_points.borrow())));
            if (((((*flag_size.borrow()) as usize)
                > ({ (*flag_stream.borrow()).length() })
                    .wrapping_sub(({ (*flag_stream.borrow()).offset() }))) as i64)
                != 0)
            {
                return false;
            }
            let flags_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
                ({ (*flag_stream.borrow()).buffer() })
                    .offset(({ (*flag_stream.borrow()).offset() }) as isize),
            ));
            let triplet_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
                ({ (*glyph_stream.borrow()).buffer() })
                    .offset(({ (*glyph_stream.borrow()).offset() }) as isize),
            ));
            let triplet_size: Value<usize> = Rc::new(RefCell::new(
                ({ (*glyph_stream.borrow()).length() })
                    .wrapping_sub(({ (*glyph_stream.borrow()).offset() })),
            ));
            let triplet_bytes_consumed: Value<usize> = Rc::new(RefCell::new(0_usize));
            if ((*points_size.borrow()) < ((*total_n_points.borrow()) as usize)) {
                (*points_size.borrow_mut()) = ((*total_n_points.borrow()) as usize);
                (*points.borrow_mut()) = Ptr::alloc_array(
                    (0..(*points_size.borrow()))
                        .map(|_| <woff2_Point>::default())
                        .collect::<Box<[woff2_Point]>>(),
                )
                .to_owned_opt();
            }
            if ((!({
                TripletDecode_57(
                    (*flags_buf.borrow()).clone(),
                    (*triplet_buf.borrow()).clone(),
                    (*triplet_size.borrow()),
                    (*total_n_points.borrow()),
                    (*points.borrow()).as_pointer(),
                    (triplet_bytes_consumed.as_pointer()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((!({ (*flag_stream.borrow()).Skip(((*flag_size.borrow()) as usize)) }) as i64) != 0)
            {
                return false;
            }
            if ((!({ (*glyph_stream.borrow()).Skip((*triplet_bytes_consumed.borrow())) }) as i64)
                != 0)
            {
                return false;
            }
            let instruction_size: Value<u32> = <Value<u32>>::default();
            if ((!({
                Read255UShort_12((glyph_stream.as_pointer()), (instruction_size.as_pointer()))
            }) as i64)
                != 0)
            {
                return false;
            }
            if (((((*total_n_points.borrow()) >= ((1 << 27) as u32))
                || ((*instruction_size.borrow()) >= ((1 << 30) as u32))) as i64)
                != 0)
            {
                return false;
            }
            let size_needed: Value<usize> = Rc::new(RefCell::new(
                (((((12 + (2 * ((*n_contours.borrow()) as i32))) as u32)
                    .wrapping_add((5_u32).wrapping_mul((*total_n_points.borrow()))))
                .wrapping_add((*instruction_size.borrow()))) as usize),
            ));
            if ((((*glyph_buf_size.borrow()) < (*size_needed.borrow())) as i64) != 0) {
                (*glyph_buf.borrow_mut()) = Ptr::alloc_array(
                    (0..(*size_needed.borrow()))
                        .map(|_| <u8>::default())
                        .collect::<Box<[u8]>>(),
                )
                .to_owned_opt();
                (*glyph_buf_size.borrow_mut()) = (*size_needed.borrow());
            }
            let __rhs = ({
                Store16_32(
                    (*glyph_buf.borrow()).as_pointer(),
                    (*glyph_size.borrow()),
                    ((*n_contours.borrow()) as i32),
                )
            });
            (*glyph_size.borrow_mut()) = __rhs;
            if (*have_bbox.borrow()) {
                if ((!({
                    (*bbox_stream.borrow()).Read(
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
                        (*total_n_points.borrow()),
                        (*points.borrow()).as_pointer(),
                        (*glyph_buf.borrow()).as_pointer(),
                    )
                });
            }
            (*glyph_size.borrow_mut()) = (*kEndPtsOfContoursOffset_51.with(Value::clone).borrow());
            let end_point: Value<i32> = Rc::new(RefCell::new(-1_i32));
            let contour_ix: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*contour_ix.borrow()) < ((*n_contours.borrow()) as u32)) {
                let rhs_0 = (((*end_point.borrow()) as u32).wrapping_add(
                    ((n_points_vec.as_pointer() as Ptr<u32>)
                        .offset(((*contour_ix.borrow()) as usize) as isize)
                        .read()),
                )) as i32;
                (*end_point.borrow_mut()) = rhs_0;
                if ((((*end_point.borrow()) >= 65536) as i64) != 0) {
                    return false;
                }
                let __rhs = ({
                    Store16_32(
                        (*glyph_buf.borrow()).as_pointer(),
                        (*glyph_size.borrow()),
                        (*end_point.borrow()),
                    )
                });
                (*glyph_size.borrow_mut()) = __rhs;
                (*contour_ix.borrow_mut()).prefix_inc();
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
                (*instruction_stream.borrow()).Read(
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
            let rhs_0 =
                (*glyph_size.borrow()).wrapping_add(((*instruction_size.borrow()) as usize));
            (*glyph_size.borrow_mut()) = rhs_0;
            let has_overlap_bit: Value<bool> = Rc::new(RefCell::new(
                (*has_overlap_bitmap.borrow())
                    && ({
                        let _lhs = (((*overlap_bitmap.borrow())
                            .offset(((*i.borrow()) >> 3) as isize)
                            .read()) as i32);
                        _lhs & (128 >> ((*i.borrow()) & 7_u32))
                    } != 0),
            ));
            if ((!({
                StorePoints_58(
                    (*total_n_points.borrow()),
                    (*points.borrow()).as_pointer(),
                    ((*n_contours.borrow()) as u32),
                    (*instruction_size.borrow()),
                    (*has_overlap_bit.borrow()),
                    (*glyph_buf.borrow()).as_pointer(),
                    (*glyph_buf_size.borrow()),
                    (glyph_size.as_pointer()),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            if (((*have_bbox.borrow()) as i64) != 0) {
                eprintln!("Empty glyph has a bbox");
                return false;
            }
        }
        (loca_values.as_pointer() as Ptr<u32>)
            .offset(((*i.borrow()) as usize) as isize)
            .write(
                ((({ (*(*out.borrow()).upgrade().deref()).Size() })
                    .wrapping_sub((*glyf_start.borrow()))) as u32),
            );
        if ((!({
            (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(
                ((*glyph_buf.borrow()).as_pointer() as Ptr<u8>).to_any(),
                (*glyph_size.borrow()),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((!({ Pad4_61((*out.borrow()).clone()) }) as i64) != 0) {
            return false;
        }
        let rhs_0 = ((*glyf_checksum.borrow()).read()).wrapping_add(
            ({ ComputeULongSum_26((*glyph_buf.borrow()).as_pointer(), (*glyph_size.borrow())) }),
        );
        (*glyf_checksum.borrow()).write(rhs_0);
        if (((*n_contours.borrow()) as i32) > 0) {
            let x_min_buf: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
                (*glyph_buf.borrow()).as_pointer().offset((2) as isize),
                2_usize,
            )));
            if ((!({
                (*x_min_buf.borrow()).ReadS16(
                    (((*(*info.borrow()).upgrade().deref()).x_mins.as_pointer() as Ptr<i16>)
                        .offset(((*i.borrow()) as usize) as isize)),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = ((({ (*(*out.borrow()).upgrade().deref()).Size() }).wrapping_sub(
        ((*(*(*glyf_table.borrow()).upgrade().deref())
            .dst_offset
            .borrow()) as usize),
    )) as u32);
    (*(*(*glyf_table.borrow()).upgrade().deref())
        .dst_length
        .borrow_mut()) = __rhs;
    (*(*(*loca_table.borrow()).upgrade().deref())
        .dst_offset
        .borrow_mut()) = (({ (*(*out.borrow()).upgrade().deref()).Size() }) as u32);
    (loca_values.as_pointer() as Ptr<u32>)
        .offset(((*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow()) as usize) as isize)
        .write(
            (*(*(*glyf_table.borrow()).upgrade().deref())
                .dst_length
                .borrow()),
        );
    if ((!({
        StoreLoca_62(
            loca_values.as_pointer(),
            ((*(*(*info.borrow()).upgrade().deref()).index_format.borrow()) as i32),
            (*loca_checksum.borrow()).clone(),
            (*out.borrow()).clone(),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    let __rhs = ((({ (*(*out.borrow()).upgrade().deref()).Size() }).wrapping_sub(
        ((*(*(*loca_table.borrow()).upgrade().deref())
            .dst_offset
            .borrow()) as usize),
    )) as u32);
    (*(*(*loca_table.borrow()).upgrade().deref())
        .dst_length
        .borrow_mut()) = __rhs;
    return true;
}
pub fn FindTable_65(tables: Ptr<Vec<Ptr<woff2_Table>>>, tag: u32) -> Ptr<woff2_Table> {
    let tables: Value<Ptr<Vec<Ptr<woff2_Table>>>> = Rc::new(RefCell::new(tables));
    let tag: Value<u32> = Rc::new(RefCell::new(tag));
    'loop_: for mut table in (*tables.borrow()).to_strong().as_pointer() as Ptr<Ptr<woff2_Table>> {
        let table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(table.read().clone()));
        if {
            let _lhs = (*(*(*table.borrow()).upgrade().deref()).tag.borrow());
            _lhs == (*tag.borrow())
        } {
            return (*table.borrow()).clone();
        }
    }
    return Ptr::<woff2_Table>::null();
}
pub fn ReadNumHMetrics_66(data: Ptr<u8>, data_size: usize, num_hmetrics: Ptr<u16>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let data_size: Value<usize> = Rc::new(RefCell::new(data_size));
    let num_hmetrics: Value<Ptr<u16>> = Rc::new(RefCell::new(num_hmetrics));
    let buffer: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*data_size.borrow()),
    )));
    if ((((!({ (*buffer.borrow()).Skip(34_usize) }))
        || (!({ (*buffer.borrow()).ReadU16((*num_hmetrics.borrow()).clone()) }))) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReconstructTransformedHmtx_67(
    transformed_buf: Ptr<u8>,
    transformed_size: usize,
    num_glyphs: u16,
    num_hmetrics: u16,
    x_mins: Ptr<Vec<i16>>,
    checksum: Ptr<u32>,
    out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let transformed_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(transformed_buf));
    let transformed_size: Value<usize> = Rc::new(RefCell::new(transformed_size));
    let num_glyphs: Value<u16> = Rc::new(RefCell::new(num_glyphs));
    let num_hmetrics: Value<u16> = Rc::new(RefCell::new(num_hmetrics));
    let checksum: Value<Ptr<u32>> = Rc::new(RefCell::new(checksum));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let hmtx_buff_in: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*transformed_buf.borrow()).clone(),
        (*transformed_size.borrow()),
    )));
    let hmtx_flags: Value<u8> = <Value<u8>>::default();
    if ((!({ (*hmtx_buff_in.borrow()).ReadU8((hmtx_flags.as_pointer())) }) as i64) != 0) {
        return false;
    }
    let advance_widths: Value<Vec<u16>> = Rc::new(RefCell::new(Vec::new()));
    let lsbs: Value<Vec<i16>> = Rc::new(RefCell::new(Vec::new()));
    let has_proportional_lsbs: Value<bool> =
        Rc::new(RefCell::new(((((*hmtx_flags.borrow()) as i32) & 1) == 0)));
    let has_monospace_lsbs: Value<bool> =
        Rc::new(RefCell::new(((((*hmtx_flags.borrow()) as i32) & 2) == 0)));
    if ((((*hmtx_flags.borrow()) as i32) & 252) != 0) {
        eprintln!("Illegal hmtx flags; bits 2-7 must be 0");
        return false;
    }
    if (*has_proportional_lsbs.borrow()) && (*has_monospace_lsbs.borrow()) {
        return false;
    }
    assert!({
        let _lhs = (*x_mins.upgrade().deref()).len();
        _lhs == ((*num_glyphs.borrow()) as usize)
    });
    if (((((*num_hmetrics.borrow()) as i32) > ((*num_glyphs.borrow()) as i32)) as i64) != 0) {
        return false;
    }
    if (((((*num_hmetrics.borrow()) as i32) < 1) as i64) != 0) {
        return false;
    }
    let i: Value<u16> = Rc::new(RefCell::new(0_u16));
    'loop_: while (((*i.borrow()) as i32) < ((*num_hmetrics.borrow()) as i32)) {
        let advance_width: Value<u16> = <Value<u16>>::default();
        if ((!({ (*hmtx_buff_in.borrow()).ReadU16((advance_width.as_pointer())) }) as i64) != 0) {
            return false;
        }
        {
            let a0_clone = (*advance_width.borrow()).clone();
            (*advance_widths.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<u16> = Rc::new(RefCell::new(0_u16));
    'loop_: while (((*i.borrow()) as i32) < ((*num_hmetrics.borrow()) as i32)) {
        let lsb: Value<i16> = <Value<i16>>::default();
        if (*has_proportional_lsbs.borrow()) {
            if ((!({ (*hmtx_buff_in.borrow()).ReadS16((lsb.as_pointer())) }) as i64) != 0) {
                return false;
            }
        } else {
            (*lsb.borrow_mut()) = ((x_mins.to_strong().as_pointer() as Ptr<i16>)
                .offset(((*i.borrow()) as usize) as isize)
                .read());
        }
        {
            let a0_clone = (*lsb.borrow()).clone();
            (*lsbs.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<u16> = Rc::new(RefCell::new((*num_hmetrics.borrow())));
    'loop_: while (((*i.borrow()) as i32) < ((*num_glyphs.borrow()) as i32)) {
        let lsb: Value<i16> = <Value<i16>>::default();
        if (*has_monospace_lsbs.borrow()) {
            if ((!({ (*hmtx_buff_in.borrow()).ReadS16((lsb.as_pointer())) }) as i64) != 0) {
                return false;
            }
        } else {
            (*lsb.borrow_mut()) = ((x_mins.to_strong().as_pointer() as Ptr<i16>)
                .offset(((*i.borrow()) as usize) as isize)
                .read());
        }
        {
            let a0_clone = (*lsb.borrow()).clone();
            (*lsbs.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let hmtx_output_size: Value<u32> = Rc::new(RefCell::new(
        (((2 * ((*num_glyphs.borrow()) as i32)) + (2 * ((*num_hmetrics.borrow()) as i32))) as u32),
    ));
    let hmtx_table: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((*hmtx_output_size.borrow()) as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    let dst: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
    ));
    let dst_offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ((*num_glyphs.borrow()) as u32)) {
        if ((*i.borrow()) < ((*num_hmetrics.borrow()) as u32)) {
            ({
                Store16_34(
                    (((advance_widths.as_pointer() as Ptr<u16>)
                        .offset(((*i.borrow()) as usize) as isize)
                        .read()) as i32),
                    (dst_offset.as_pointer()),
                    (*dst.borrow()).clone(),
                )
            });
        }
        ({
            Store16_34(
                (((lsbs.as_pointer() as Ptr<i16>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .read()) as i32),
                (dst_offset.as_pointer()),
                (*dst.borrow()).clone(),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let __rhs = ({
        ComputeULongSum_26(
            ((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
            ((*hmtx_output_size.borrow()) as usize),
        )
    });
    (*checksum.borrow()).write(__rhs);
    if ((!({
        (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(
            (((hmtx_table.as_pointer() as Ptr<u8>).offset(0_usize as isize)) as Ptr<u8>).to_any(),
            ((*hmtx_output_size.borrow()) as usize),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn Woff2Uncompress_68(
    dst_buf: Ptr<u8>,
    dst_size: usize,
    src_buf: Ptr<u8>,
    src_size: usize,
) -> bool {
    let dst_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(dst_buf));
    let dst_size: Value<usize> = Rc::new(RefCell::new(dst_size));
    let src_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(src_buf));
    let src_size: Value<usize> = Rc::new(RefCell::new(src_size));
    let uncompressed_size: Value<usize> = Rc::new(RefCell::new((*dst_size.borrow())));
    let result: Value<::brotli_sys::BrotliDecoderResult> = Rc::new(RefCell::new(
        (uncompressed_size.as_pointer()).with_mut(|_v2| {
            (*dst_buf.borrow()).with_mut(|_v3| unsafe {
                ::brotli_sys::BrotliDecoderDecompress(
                    (*src_size.borrow()),
                    &*(*src_buf.borrow()).upgrade().deref(),
                    _v2 as *mut usize,
                    _v3,
                )
            })
        }),
    ));
    if ((((((*result.borrow()) as i32) != (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32))
        || ((*uncompressed_size.borrow()) != (*dst_size.borrow()))) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub fn ReadTableDirectory_69(
    file: Ptr<woff2_Buffer>,
    tables: Ptr<Vec<woff2_Table>>,
    num_tables: usize,
) -> bool {
    let file: Value<Ptr<woff2_Buffer>> = Rc::new(RefCell::new(file));
    let tables: Value<Ptr<Vec<woff2_Table>>> = Rc::new(RefCell::new(tables));
    let num_tables: Value<usize> = Rc::new(RefCell::new(num_tables));
    let src_offset: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*num_tables.borrow())) {
        let table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(
            ((((*tables.borrow()).to_strong().as_pointer()) as Ptr<woff2_Table>)
                .offset((*i.borrow()) as isize)),
        ));
        let flag_byte: Value<u8> = <Value<u8>>::default();
        if ((!({ (*(*file.borrow()).upgrade().deref()).ReadU8((flag_byte.as_pointer())) }) as i64)
            != 0)
        {
            return false;
        }
        let tag: Value<u32> = <Value<u32>>::default();
        if ((((*flag_byte.borrow()) as i32) & 63) == 63) {
            if ((!({ (*(*file.borrow()).upgrade().deref()).ReadU32((tag.as_pointer())) }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            (*tag.borrow_mut()) = (*kKnownTags_8.with(Value::clone).borrow())
                [(((*flag_byte.borrow()) as i32) & 63) as usize];
        }
        let flags: Value<u32> = Rc::new(RefCell::new(0_u32));
        let xform_version: Value<u8> = Rc::new(RefCell::new(
            (((((*flag_byte.borrow()) as i32) >> 6) & 3) as u8),
        ));
        if ((*tag.borrow()) == (*kGlyfTableTag_0.with(Value::clone).borrow()))
            || ((*tag.borrow()) == (*kLocaTableTag_2.with(Value::clone).borrow()))
        {
            if (((*xform_version.borrow()) as i32) == 0) {
                let rhs_0 = (((*flags.borrow()) as u32)
                    | (*kWoff2FlagsTransform_21.with(Value::clone).borrow()))
                    as u32;
                (*flags.borrow_mut()) = rhs_0;
            }
        } else if (((*xform_version.borrow()) as i32) != 0) {
            let rhs_0 = (((*flags.borrow()) as u32)
                | (*kWoff2FlagsTransform_21.with(Value::clone).borrow()))
                as u32;
            (*flags.borrow_mut()) = rhs_0;
        }
        (*flags.borrow_mut()) |= ((*xform_version.borrow()) as u32);
        let dst_length: Value<u32> = <Value<u32>>::default();
        if ((!({ ReadBase128_17((*file.borrow()).clone(), (dst_length.as_pointer())) }) as i64)
            != 0)
        {
            return false;
        }
        let transform_length: Value<u32> = Rc::new(RefCell::new((*dst_length.borrow())));
        if (((*flags.borrow()) & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())) != 0_u32) {
            if ((!({ ReadBase128_17((*file.borrow()).clone(), (transform_length.as_pointer())) })
                as i64)
                != 0)
            {
                return false;
            }
            if (((((*tag.borrow()) == (*kLocaTableTag_2.with(Value::clone).borrow()))
                && ((*transform_length.borrow()) != 0)) as i64)
                != 0)
            {
                return false;
            }
        }
        if ((((*src_offset.borrow()).wrapping_add((*transform_length.borrow()))
            < (*src_offset.borrow())) as i64)
            != 0)
        {
            return false;
        }
        (*(*(*table.borrow()).upgrade().deref())
            .src_offset
            .borrow_mut()) = (*src_offset.borrow());
        (*(*(*table.borrow()).upgrade().deref())
            .src_length
            .borrow_mut()) = (*transform_length.borrow());
        let rhs_0 = (*src_offset.borrow()).wrapping_add((*transform_length.borrow()));
        (*src_offset.borrow_mut()) = rhs_0;
        (*(*(*table.borrow()).upgrade().deref()).tag.borrow_mut()) = (*tag.borrow());
        (*(*(*table.borrow()).upgrade().deref()).flags.borrow_mut()) = (*flags.borrow());
        (*(*(*table.borrow()).upgrade().deref())
            .transform_length
            .borrow_mut()) = (*transform_length.borrow());
        (*(*(*table.borrow()).upgrade().deref())
            .dst_length
            .borrow_mut()) = (*dst_length.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn StoreOffsetTable_70(result: Ptr<u8>, offset: usize, flavor: u32, num_tables: u16) -> usize {
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let offset: Value<usize> = Rc::new(RefCell::new(offset));
    let flavor: Value<u32> = Rc::new(RefCell::new(flavor));
    let num_tables: Value<u16> = Rc::new(RefCell::new(num_tables));
    let __rhs = ({
        StoreU32_31(
            (*result.borrow()).clone(),
            (*offset.borrow()),
            (*flavor.borrow()),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*result.borrow()).clone(),
            (*offset.borrow()),
            ((*num_tables.borrow()) as i32),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let max_pow2: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((1_u32 << ((*max_pow2.borrow()).wrapping_add(1_u32)))
        <= ((*num_tables.borrow()) as u32))
    {
        (*max_pow2.borrow_mut()).postfix_inc();
    }
    let output_search_range: Value<u16> = Rc::new(RefCell::new(
        (((1_u32 << (*max_pow2.borrow())) << 4) as u16),
    ));
    let __rhs = ({
        Store16_32(
            (*result.borrow()).clone(),
            (*offset.borrow()),
            ((*output_search_range.borrow()) as i32),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*result.borrow()).clone(),
            (*offset.borrow()),
            ((*max_pow2.borrow()) as i32),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    let __rhs = ({
        Store16_32(
            (*result.borrow()).clone(),
            (*offset.borrow()),
            ((((*num_tables.borrow()) as i32) << 4) - ((*output_search_range.borrow()) as i32)),
        )
    });
    (*offset.borrow_mut()) = __rhs;
    return (*offset.borrow());
}
pub fn StoreTableEntry_71(result: Ptr<u8>, offset: u32, tag: u32) -> usize {
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let offset: Value<u32> = Rc::new(RefCell::new(offset));
    let tag: Value<u32> = Rc::new(RefCell::new(tag));
    let __rhs = (({
        StoreU32_31(
            (*result.borrow()).clone(),
            ((*offset.borrow()) as usize),
            (*tag.borrow()),
        )
    }) as u32);
    (*offset.borrow_mut()) = __rhs;
    let __rhs = (({
        StoreU32_31(
            (*result.borrow()).clone(),
            ((*offset.borrow()) as usize),
            0_u32,
        )
    }) as u32);
    (*offset.borrow_mut()) = __rhs;
    let __rhs = (({
        StoreU32_31(
            (*result.borrow()).clone(),
            ((*offset.borrow()) as usize),
            0_u32,
        )
    }) as u32);
    (*offset.borrow_mut()) = __rhs;
    let __rhs = (({
        StoreU32_31(
            (*result.borrow()).clone(),
            ((*offset.borrow()) as usize),
            0_u32,
        )
    }) as u32);
    (*offset.borrow_mut()) = __rhs;
    return ((*offset.borrow()) as usize);
}
pub fn ComputeOffsetToFirstTable_72(hdr: Ptr<woff2_WOFF2Header>) -> u64 {
    let offset: Value<u64> = Rc::new(RefCell::new(
        ((*kSfntHeaderSize_23.with(Value::clone).borrow()) as u64).wrapping_add(
            ((*kSfntEntrySize_24.with(Value::clone).borrow()) as u64)
                .wrapping_mul(((*(*hdr.upgrade().deref()).num_tables.borrow()) as u64)),
        ),
    ));
    if ((*(*hdr.upgrade().deref()).header_version.borrow()) != 0) {
        (*offset.borrow_mut()) = (({
            let _header_version: u32 = (*(*hdr.upgrade().deref()).header_version.borrow());
            let _num_fonts: u32 = ((*(*hdr.upgrade().deref()).ttc_fonts.borrow()).len() as u32);
            CollectionHeaderSize_27(_header_version, _num_fonts)
        }) as u64)
            .wrapping_add(
                ((*kSfntHeaderSize_23.with(Value::clone).borrow()) as u64)
                    .wrapping_mul(((*(*hdr.upgrade().deref()).ttc_fonts.borrow()).len() as u64)),
            );
        'loop_: for mut ttc_font in
            (*hdr.upgrade().deref()).ttc_fonts.as_pointer() as Ptr<woff2_TtcFont>
        {
            let rhs_0 = (((*offset.borrow()) as u64).wrapping_add(
                ((*kSfntEntrySize_24.with(Value::clone).borrow()) as u64).wrapping_mul(
                    ((*(*ttc_font.upgrade().deref()).table_indices.borrow()).len() as u64),
                ),
            )) as u64;
            (*offset.borrow_mut()) = rhs_0;
        }
    }
    return (*offset.borrow());
}
pub fn Tables_73(hdr: Ptr<woff2_WOFF2Header>, font_index: usize) -> Vec<Ptr<woff2_Table>> {
    let hdr: Value<Ptr<woff2_WOFF2Header>> = Rc::new(RefCell::new(hdr));
    let font_index: Value<usize> = Rc::new(RefCell::new(font_index));
    let tables: Value<Vec<Ptr<woff2_Table>>> = Rc::new(RefCell::new(Vec::new()));
    if (((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) as i64) != 0) {
        'loop_: for mut index in (*((*(*hdr.borrow()).upgrade().deref()).ttc_fonts.as_pointer()
            as Ptr<woff2_TtcFont>)
            .offset((*font_index.borrow()) as isize)
            .upgrade()
            .deref())
        .table_indices
        .as_pointer() as Ptr<u16>
        {
            let index: Value<u16> = Rc::new(RefCell::new(index.read().clone()));
            (*tables.borrow_mut()).push(
                (((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer() as Ptr<woff2_Table>)
                    .offset(((*index.borrow()) as usize) as isize)),
            );
        }
    } else {
        'loop_: for mut table in
            (*(*hdr.borrow()).upgrade().deref()).tables.as_pointer() as Ptr<woff2_Table>
        {
            (*tables.borrow_mut()).push((table));
        }
    }
    return (*tables.borrow_mut()).clone();
}
pub fn ReconstructFont_74(
    transformed_buf: Ptr<u8>,
    transformed_buf_size: u32,
    metadata: Ptr<woff2_RebuildMetadata>,
    hdr: Ptr<woff2_WOFF2Header>,
    font_index: usize,
    out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let transformed_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(transformed_buf));
    let transformed_buf_size: Value<u32> = Rc::new(RefCell::new(transformed_buf_size));
    let metadata: Value<Ptr<woff2_RebuildMetadata>> = Rc::new(RefCell::new(metadata));
    let hdr: Value<Ptr<woff2_WOFF2Header>> = Rc::new(RefCell::new(hdr));
    let font_index: Value<usize> = Rc::new(RefCell::new(font_index));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let dest_offset: Value<usize> = Rc::new(RefCell::new(
        ({ (*(*out.borrow()).upgrade().deref()).Size() }),
    ));
    let table_entry: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..12).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let info: Value<Ptr<woff2_WOFF2FontInfo>> = Rc::new(RefCell::new(
        (((*(*metadata.borrow()).upgrade().deref())
            .font_infos
            .as_pointer() as Ptr<woff2_WOFF2FontInfo>)
            .offset((*font_index.borrow()) as isize)),
    ));
    let tables: Value<Vec<Ptr<woff2_Table>>> = Rc::new(RefCell::new(
        ({ Tables_73((*hdr.borrow()).clone(), (*font_index.borrow())) }),
    ));
    let glyf_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(
        ({
            FindTable_65(
                (tables.as_pointer()),
                (*kGlyfTableTag_0.with(Value::clone).borrow()),
            )
        }),
    ));
    let loca_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(
        ({
            FindTable_65(
                (tables.as_pointer()),
                (*kLocaTableTag_2.with(Value::clone).borrow()),
            )
        }),
    ));
    if ((({
        let _lhs = (!(*glyf_table.borrow()).is_null() as i32).clone();
        _lhs != (!(*loca_table.borrow()).is_null() as i32).clone()
    }) as i64)
        != 0)
    {
        eprintln!("Cannot have just one of glyf/loca");
        return false;
    }
    if !((*glyf_table.borrow()).is_null()) {
        if ((({
            let _lhs = ({
                let _lhs = (*(*(*glyf_table.borrow()).upgrade().deref()).flags.borrow());
                _lhs & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
            });
            _lhs != ({
                let _lhs = (*(*(*loca_table.borrow()).upgrade().deref()).flags.borrow());
                _lhs & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
            })
        }) as i64)
            != 0)
        {
            eprintln!("Cannot transform just one of glyf/loca");
            return false;
        }
    }
    let font_checksum: Value<u32> = Rc::new(RefCell::new(
        (*(*(*metadata.borrow()).upgrade().deref())
            .header_checksum
            .borrow()),
    ));
    if ((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) != 0) {
        (*font_checksum.borrow_mut()) = (*(*((*(*hdr.borrow()).upgrade().deref())
            .ttc_fonts
            .as_pointer() as Ptr<woff2_TtcFont>)
            .offset((*font_index.borrow()) as isize)
            .upgrade()
            .deref())
        .header_checksum
        .borrow());
    }
    let loca_checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*tables.borrow()).len()) {
        let table: Ptr<woff2_Table> = ((tables.as_pointer() as Ptr<Ptr<woff2_Table>>)
            .offset((*i.borrow()) as isize)
            .read())
        .clone();
        let checksum_key: Value<(Value<u32>, Value<u32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                (*(*table.upgrade().deref()).tag.borrow())
                    .try_into()
                    .expect("failed conversion"),
            )),
            Rc::new(RefCell::new(
                (*(*table.upgrade().deref()).src_offset.borrow())
                    .try_into()
                    .expect("failed conversion"),
            )),
        )));
        let reused: Value<bool> = Rc::new(RefCell::new(
            RefcountMapIter::find_key(
                ((*(*metadata.borrow()).upgrade().deref())
                    .checksums
                    .as_pointer()
                    as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>),
                &(*checksum_key.borrow()),
            ) != RefcountMapIter::end(
                ((*(*metadata.borrow()).upgrade().deref())
                    .checksums
                    .as_pointer()
                    as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>),
            ),
        ));
        if (((((*font_index.borrow()) == 0_usize) && (*reused.borrow())) as i64) != 0) {
            return false;
        }
        if ((({
            let _lhs = ((*(*table.upgrade().deref()).src_offset.borrow()) as u64)
                .wrapping_add(((*(*table.upgrade().deref()).src_length.borrow()) as u64));
            _lhs > ((*transformed_buf_size.borrow()) as u64)
        }) as i64)
            != 0)
        {
            return false;
        }
        if {
            let _lhs = (*(*table.upgrade().deref()).tag.borrow());
            _lhs == (*kHheaTableTag_6.with(Value::clone).borrow())
        } {
            if !({
                let _data: Ptr<u8> = (*transformed_buf.borrow())
                    .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize);
                let _data_size: usize =
                    ((*(*table.upgrade().deref()).src_length.borrow()) as usize);
                ReadNumHMetrics_66(
                    _data,
                    _data_size,
                    ((*(*info.borrow()).upgrade().deref())
                        .num_hmetrics
                        .as_pointer()),
                )
            }) {
                return false;
            }
        }
        let checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
        if !(*reused.borrow()) {
            if {
                let _lhs = ({
                    let _lhs = (*(*table.upgrade().deref()).flags.borrow());
                    _lhs & (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
                });
                _lhs != (*kWoff2FlagsTransform_21.with(Value::clone).borrow())
            } {
                if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kHeadTableTag_1.with(Value::clone).borrow())
                } {
                    if ((((*(*table.upgrade().deref()).src_length.borrow()) < 12_u32) as i64) != 0)
                    {
                        return false;
                    }
                    ({
                        StoreU32_31(
                            (*transformed_buf.borrow())
                                .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize),
                            8_usize,
                            0_u32,
                        )
                    });
                }
                (*(*table.upgrade().deref()).dst_offset.borrow_mut()) =
                    ((*dest_offset.borrow()) as u32);
                (*checksum.borrow_mut()) = ({
                    let _buf: Ptr<u8> = (*transformed_buf.borrow())
                        .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize);
                    let _size: usize = ((*(*table.upgrade().deref()).src_length.borrow()) as usize);
                    ComputeULongSum_26(_buf, _size)
                });
                if ((!({
                    let _buf: AnyPtr = ((*transformed_buf.borrow())
                        .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize)
                        as Ptr<u8>)
                        .to_any();
                    let _n: usize = ((*(*table.upgrade().deref()).src_length.borrow()) as usize);
                    (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(_buf, _n)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kGlyfTableTag_0.with(Value::clone).borrow())
                } {
                    (*(*table.upgrade().deref()).dst_offset.borrow_mut()) =
                        ((*dest_offset.borrow()) as u32);
                    let loca_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(
                        ({
                            FindTable_65(
                                (tables.as_pointer()),
                                (*kLocaTableTag_2.with(Value::clone).borrow()),
                            )
                        }),
                    ));
                    if ((!({
                        let _data: Ptr<u8> = (*transformed_buf.borrow())
                            .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize);
                        let _glyf_table: Ptr<woff2_Table> = (table).clone();
                        ReconstructGlyf_63(
                            _data,
                            _glyf_table,
                            (checksum.as_pointer()),
                            (*loca_table.borrow()).clone(),
                            (loca_checksum.as_pointer()),
                            (*info.borrow()).clone(),
                            (*out.borrow()).clone(),
                        )
                    }) as i64)
                        != 0)
                    {
                        return false;
                    }
                } else if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kLocaTableTag_2.with(Value::clone).borrow())
                } {
                    (*checksum.borrow_mut()) = (*loca_checksum.borrow());
                } else if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kHmtxTableTag_5.with(Value::clone).borrow())
                } {
                    (*(*table.upgrade().deref()).dst_offset.borrow_mut()) =
                        ((*dest_offset.borrow()) as u32);
                    if ((!({
                        let _transformed_buf: Ptr<u8> = (*transformed_buf.borrow())
                            .offset((*(*table.upgrade().deref()).src_offset.borrow()) as isize);
                        let _transformed_size: usize =
                            ((*(*table.upgrade().deref()).src_length.borrow()) as usize);
                        let _num_glyphs: u16 =
                            (*(*(*info.borrow()).upgrade().deref()).num_glyphs.borrow());
                        let _num_hmetrics: u16 =
                            (*(*(*info.borrow()).upgrade().deref()).num_hmetrics.borrow());
                        let _x_mins: Ptr<Vec<i16>> =
                            (*(*info.borrow()).upgrade().deref()).x_mins.as_pointer();
                        ReconstructTransformedHmtx_67(
                            _transformed_buf,
                            _transformed_size,
                            _num_glyphs,
                            _num_hmetrics,
                            _x_mins,
                            (checksum.as_pointer()),
                            (*out.borrow()).clone(),
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
            ((*(*metadata.borrow()).upgrade().deref())
                .checksums
                .as_pointer() as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u32>>| {
                    __v.entry((*checksum_key.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .write((*checksum.borrow()));
        } else {
            (*checksum.borrow_mut()) = (((*(*metadata.borrow()).upgrade().deref())
                .checksums
                .as_pointer()
                as Ptr<BTreeMap<(Value<u32>, Value<u32>), Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<(Value<u32>, Value<u32>), Value<u32>>| {
                    __v.entry((*checksum_key.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .read());
        }
        let rhs_0 = (*font_checksum.borrow()).wrapping_add((*checksum.borrow()));
        (*font_checksum.borrow_mut()) = rhs_0;
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
                (*(*table.upgrade().deref()).dst_offset.borrow()),
            )
        });
        ({
            StoreU32_31(
                (table_entry.as_pointer() as Ptr<u8>),
                8_usize,
                (*(*table.upgrade().deref()).dst_length.borrow()),
            )
        });
        if ((!({
            (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize_usize(
                ((table_entry.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (((((*(*info.borrow()).upgrade().deref())
                    .table_entry_by_tag
                    .as_pointer() as Ptr<BTreeMap<u32, Value<u32>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                        __v.entry((*(*table.upgrade().deref()).tag.borrow()).clone())
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
        let rhs_0 = (*font_checksum.borrow()).wrapping_add(
            ({ ComputeULongSum_26((table_entry.as_pointer() as Ptr<u8>), 12_usize) }),
        );
        (*font_checksum.borrow_mut()) = rhs_0;
        if ((!({ Pad4_61((*out.borrow()).clone()) }) as i64) != 0) {
            return false;
        }
        if ((({
            let _lhs = ((((*(*table.upgrade().deref()).dst_offset.borrow())
                .wrapping_add((*(*table.upgrade().deref()).dst_length.borrow())))
                as u64) as usize);
            _lhs > ({ (*(*out.borrow()).upgrade().deref()).Size() })
        }) as i64)
            != 0)
        {
            return false;
        }
        (*dest_offset.borrow_mut()) = ({ (*(*out.borrow()).upgrade().deref()).Size() });
        (*i.borrow_mut()).postfix_inc();
    }
    let head_table: Value<Ptr<woff2_Table>> = Rc::new(RefCell::new(
        ({
            FindTable_65(
                (tables.as_pointer()),
                (*kHeadTableTag_1.with(Value::clone).borrow()),
            )
        }),
    ));
    if !(*head_table.borrow()).is_null() {
        if ((((*(*(*head_table.borrow()).upgrade().deref())
            .dst_length
            .borrow())
            < 12_u32) as i64)
            != 0)
        {
            return false;
        }
        let checksum_adjustment: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        ({
            StoreU32_31(
                (checksum_adjustment.as_pointer() as Ptr<u8>),
                0_usize,
                (2981146554_u32 as u32).wrapping_sub((*font_checksum.borrow())),
            )
        });
        if ((!({
            (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize_usize(
                ((checksum_adjustment.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (((*(*(*head_table.borrow()).upgrade().deref())
                    .dst_offset
                    .borrow())
                .wrapping_add(8_u32)) as usize),
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
pub fn ReadWOFF2Header_75(data: Ptr<u8>, length: usize, hdr: Ptr<woff2_WOFF2Header>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let hdr: Value<Ptr<woff2_WOFF2Header>> = Rc::new(RefCell::new(hdr));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*length.borrow()),
    )));
    let signature: Value<u32> = <Value<u32>>::default();
    if (((((!({ (*file.borrow()).ReadU32((signature.as_pointer())) }))
        || ((*signature.borrow()) != (*kWoff2Signature_20.with(Value::clone).borrow())))
        || (!({
            (*file.borrow()).ReadU32(((*(*hdr.borrow()).upgrade().deref()).flavor.as_pointer()))
        }))) as i64)
        != 0)
    {
        return false;
    }
    let reported_length: Value<u32> = <Value<u32>>::default();
    if ((((!({ (*file.borrow()).ReadU32((reported_length.as_pointer())) }))
        || ((*length.borrow()) != ((*reported_length.borrow()) as usize))) as i64)
        != 0)
    {
        return false;
    }
    if ((((!({
        (*file.borrow()).ReadU16(((*(*hdr.borrow()).upgrade().deref()).num_tables.as_pointer()))
    })) || (!((*(*(*hdr.borrow()).upgrade().deref()).num_tables.borrow()) != 0))) as i64)
        != 0)
    {
        return false;
    }
    if ((!({ (*file.borrow()).Skip(6_usize) }) as i64) != 0) {
        return false;
    }
    if ((!({
        (*file.borrow()).ReadU32(
            ((*(*hdr.borrow()).upgrade().deref())
                .compressed_length
                .as_pointer()),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    if ((!({ (*file.borrow()).Skip(((2 * 2) as usize)) }) as i64) != 0) {
        return false;
    }
    let meta_offset: Value<u32> = <Value<u32>>::default();
    let meta_length: Value<u32> = <Value<u32>>::default();
    let meta_length_orig: Value<u32> = <Value<u32>>::default();
    if (((((!({ (*file.borrow()).ReadU32((meta_offset.as_pointer())) }))
        || (!({ (*file.borrow()).ReadU32((meta_length.as_pointer())) })))
        || (!({ (*file.borrow()).ReadU32((meta_length_orig.as_pointer())) }))) as i64)
        != 0)
    {
        return false;
    }
    if ((*meta_offset.borrow()) != 0) {
        if ((((((*meta_offset.borrow()) as usize) >= (*length.borrow()))
            || ((*length.borrow()).wrapping_sub(((*meta_offset.borrow()) as usize))
                < ((*meta_length.borrow()) as usize))) as i64)
            != 0)
        {
            return false;
        }
    }
    let priv_offset: Value<u32> = <Value<u32>>::default();
    let priv_length: Value<u32> = <Value<u32>>::default();
    if ((((!({ (*file.borrow()).ReadU32((priv_offset.as_pointer())) }))
        || (!({ (*file.borrow()).ReadU32((priv_length.as_pointer())) }))) as i64)
        != 0)
    {
        return false;
    }
    if ((*priv_offset.borrow()) != 0) {
        if ((((((*priv_offset.borrow()) as usize) >= (*length.borrow()))
            || ((*length.borrow()).wrapping_sub(((*priv_offset.borrow()) as usize))
                < ((*priv_length.borrow()) as usize))) as i64)
            != 0)
        {
            return false;
        }
    }
    {
        let __a0 = ((*(*(*hdr.borrow()).upgrade().deref()).num_tables.borrow()) as usize) as usize;
        (*(*(*hdr.borrow()).upgrade().deref()).tables.borrow_mut())
            .resize_with(__a0, || <woff2_Table>::default())
    };
    if ((!({
        let _tables: Ptr<Vec<woff2_Table>> =
            ((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer());
        let _num_tables: usize =
            ((*(*(*hdr.borrow()).upgrade().deref()).num_tables.borrow()) as usize);
        ReadTableDirectory_69((file.as_pointer()), _tables, _num_tables)
    }) as i64)
        != 0)
    {
        return false;
    }
    let last_table: Ptr<woff2_Table> =
        ((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer() as Ptr<woff2_Table>).to_last();
    (*(*(*hdr.borrow()).upgrade().deref())
        .uncompressed_size
        .borrow_mut()) = (*(*last_table.upgrade().deref()).src_offset.borrow())
        .wrapping_add((*(*last_table.upgrade().deref()).src_length.borrow()));
    if ((({
        let _lhs = (*(*(*hdr.borrow()).upgrade().deref())
            .uncompressed_size
            .borrow());
        _lhs < (*(*last_table.upgrade().deref()).src_offset.borrow())
    }) as i64)
        != 0)
    {
        return false;
    }
    (*(*(*hdr.borrow()).upgrade().deref())
        .header_version
        .borrow_mut()) = 0_u32;
    if {
        let _lhs = (*(*(*hdr.borrow()).upgrade().deref()).flavor.borrow());
        _lhs == (*kTtcFontFlavor_22.with(Value::clone).borrow())
    } {
        if ((!({
            (*file.borrow()).ReadU32(
                ((*(*hdr.borrow()).upgrade().deref())
                    .header_version
                    .as_pointer()),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        if (((((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) != 65536_u32)
            && ((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) != 131072_u32))
            as i64)
            != 0)
        {
            return false;
        }
        let num_fonts: Value<u32> = <Value<u32>>::default();
        if ((((!({ Read255UShort_12((file.as_pointer()), (num_fonts.as_pointer())) }))
            || (!((*num_fonts.borrow()) != 0))) as i64)
            != 0)
        {
            return false;
        }
        {
            let __a0 = ((*num_fonts.borrow()) as usize) as usize;
            (*(*(*hdr.borrow()).upgrade().deref()).ttc_fonts.borrow_mut())
                .resize_with(__a0, || <woff2_TtcFont>::default())
        };
        let i: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*i.borrow()) < (*num_fonts.borrow())) {
            let ttc_font: Ptr<woff2_TtcFont> =
                ((*(*hdr.borrow()).upgrade().deref()).ttc_fonts.as_pointer() as Ptr<woff2_TtcFont>)
                    .offset(((*i.borrow()) as usize) as isize);
            let num_tables: Value<u32> = <Value<u32>>::default();
            if ((((!({ Read255UShort_12((file.as_pointer()), (num_tables.as_pointer())) }))
                || (!((*num_tables.borrow()) != 0))) as i64)
                != 0)
            {
                return false;
            }
            if ((!({
                (*file.borrow()).ReadU32(((*ttc_font.upgrade().deref()).flavor.as_pointer()))
            }) as i64)
                != 0)
            {
                return false;
            }
            {
                let __a0 = ((*num_tables.borrow()) as usize) as usize;
                (*(*ttc_font.upgrade().deref()).table_indices.borrow_mut())
                    .resize_with(__a0, || <u16>::default())
            };
            let glyf_idx: Value<u32> = Rc::new(RefCell::new(0_u32));
            let loca_idx: Value<u32> = Rc::new(RefCell::new(0_u32));
            let j: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*j.borrow()) < (*num_tables.borrow())) {
                let table_idx: Value<u32> = <Value<u32>>::default();
                if ((!({ Read255UShort_12((file.as_pointer()), (table_idx.as_pointer())) }) as i64)
                    != 0)
                    || ({
                        let _lhs = ((*table_idx.borrow()) as usize);
                        _lhs >= (*(*(*hdr.borrow()).upgrade().deref()).tables.borrow()).len()
                    })
                {
                    return false;
                }
                ((*ttc_font.upgrade().deref()).table_indices.as_pointer() as Ptr<u16>)
                    .offset(((*j.borrow()) as usize) as isize)
                    .write(((*table_idx.borrow()) as u16));
                let table: Ptr<woff2_Table> =
                    ((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer() as Ptr<woff2_Table>)
                        .offset(((*table_idx.borrow()) as usize) as isize);
                if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kLocaTableTag_2.with(Value::clone).borrow())
                } {
                    (*loca_idx.borrow_mut()) = (*table_idx.borrow());
                }
                if {
                    let _lhs = (*(*table.upgrade().deref()).tag.borrow());
                    _lhs == (*kGlyfTableTag_0.with(Value::clone).borrow())
                } {
                    (*glyf_idx.borrow_mut()) = (*table_idx.borrow());
                }
                (*j.borrow_mut()).postfix_inc();
            }
            if ((*glyf_idx.borrow()) > 0_u32) || ((*loca_idx.borrow()) > 0_u32) {
                if (((((*glyf_idx.borrow()) > (*loca_idx.borrow()))
                    || ((*loca_idx.borrow()).wrapping_sub((*glyf_idx.borrow())) != 1_u32))
                    as i64)
                    != 0)
                {
                    eprintln!("TTC font {} has non-consecutive glyf/loca", (*i.borrow()));
                    return false;
                }
            }
            (*i.borrow_mut()).postfix_inc();
        }
    }
    let first_table_offset: Value<u64> = Rc::new(RefCell::new(
        ({ ComputeOffsetToFirstTable_72((*hdr.borrow()).clone()) }),
    ));
    (*(*(*hdr.borrow()).upgrade().deref())
        .compressed_offset
        .borrow_mut()) = (({ (*file.borrow()).offset() }) as u64);
    if ((({
        let _lhs = (*(*(*hdr.borrow()).upgrade().deref())
            .compressed_offset
            .borrow());
        _lhs > (<u32>::MAX as u64)
    }) as i64)
        != 0)
    {
        return false;
    }
    let src_offset: Value<u64> = Rc::new(RefCell::new(
        ({
            Round4_29(
                (*(*(*hdr.borrow()).upgrade().deref())
                    .compressed_offset
                    .borrow())
                .wrapping_add(
                    ((*(*(*hdr.borrow()).upgrade().deref())
                        .compressed_length
                        .borrow()) as u64),
                ),
            )
        }),
    ));
    let dst_offset: Value<u64> = Rc::new(RefCell::new((*first_table_offset.borrow())));
    if (((((*src_offset.borrow()) as usize) > (*length.borrow())) as i64) != 0) {
        eprintln!(
            "offset fail; src_offset {} length {} dst_offset {}",
            (*src_offset.borrow()),
            (*length.borrow()),
            (*dst_offset.borrow())
        );
        return false;
    }
    if ((*meta_offset.borrow()) != 0) {
        if ((((*src_offset.borrow()) != ((*meta_offset.borrow()) as u64)) as i64) != 0) {
            return false;
        }
        (*src_offset.borrow_mut()) =
            (({ Round4_30((*meta_offset.borrow()).wrapping_add((*meta_length.borrow()))) }) as u64);
        if ((((*src_offset.borrow()) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if ((*priv_offset.borrow()) != 0) {
        if ((((*src_offset.borrow()) != ((*priv_offset.borrow()) as u64)) as i64) != 0) {
            return false;
        }
        (*src_offset.borrow_mut()) =
            (({ Round4_30((*priv_offset.borrow()).wrapping_add((*priv_length.borrow()))) }) as u64);
        if ((((*src_offset.borrow()) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if ((((*src_offset.borrow()) != ({ Round4_29(((*length.borrow()) as u64)) })) as i64) != 0) {
        return false;
    }
    return true;
}
pub fn WriteHeaders_76(
    data: Ptr<u8>,
    length: usize,
    metadata: Ptr<woff2_RebuildMetadata>,
    hdr: Ptr<woff2_WOFF2Header>,
    out: PtrDyn<dyn woff2_WOFF2Out>,
) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let metadata: Value<Ptr<woff2_RebuildMetadata>> = Rc::new(RefCell::new(metadata));
    let hdr: Value<Ptr<woff2_WOFF2Header>> = Rc::new(RefCell::new(hdr));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let output: Value<Vec<u8>> = Rc::new(RefCell::new(vec![
        0_u8;
        ({ ComputeOffsetToFirstTable_72((*hdr.borrow()).clone(),) })
            as usize
    ]));
    let sorted_tables: Value<Vec<woff2_Table>> = Rc::new(RefCell::new(
        (*(*(*hdr.borrow()).upgrade().deref()).tables.borrow()).clone(),
    ));
    if ((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) != 0) {
        'loop_: for mut ttc_font in
            (*(*hdr.borrow()).upgrade().deref()).ttc_fonts.as_pointer() as Ptr<woff2_TtcFont>
        {
            let sorted_index_by_tag: Value<BTreeMap<u32, Value<u16>>> =
                Rc::new(RefCell::new(BTreeMap::new()));
            'loop_: for mut table_index in
                (*ttc_font.upgrade().deref()).table_indices.as_pointer() as Ptr<u16>
            {
                let table_index: Value<u16> = Rc::new(RefCell::new(table_index.read().clone()));
                let __rhs = (*table_index.borrow());
                (sorted_index_by_tag.as_pointer() as Ptr<BTreeMap<u32, Value<u16>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u16>>| {
                        __v.entry(
                            (*(*((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer()
                                as Ptr<woff2_Table>)
                                .offset(((*table_index.borrow()) as usize) as isize)
                                .upgrade()
                                .deref())
                            .tag
                            .borrow())
                            .clone(),
                        )
                        .or_insert_with(|| Rc::new(RefCell::new(<u16>::default())))
                        .as_pointer()
                    })
                    .write(__rhs);
            }
            let index: Value<u16> = Rc::new(RefCell::new(0_u16));
            'loop_: for i in RefcountMapIter::begin(sorted_index_by_tag.as_pointer()) {
                ((*ttc_font.upgrade().deref()).table_indices.as_pointer() as Ptr<u16>)
                    .offset(((*index.borrow_mut()).postfix_inc() as usize) as isize)
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
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((output.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
    ));
    let offset: Value<usize> = Rc::new(RefCell::new(0_usize));
    if ((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) != 0) {
        let __rhs = ({
            StoreU32_31(
                (*result.borrow()).clone(),
                (*offset.borrow()),
                (*(*(*hdr.borrow()).upgrade().deref()).flavor.borrow()),
            )
        });
        (*offset.borrow_mut()) = __rhs;
        let __rhs = ({
            StoreU32_31(
                (*result.borrow()).clone(),
                (*offset.borrow()),
                (*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()),
            )
        });
        (*offset.borrow_mut()) = __rhs;
        let __rhs = ({
            StoreU32_31(
                (*result.borrow()).clone(),
                (*offset.borrow()),
                ((*(*(*hdr.borrow()).upgrade().deref()).ttc_fonts.borrow()).len() as u32),
            )
        });
        (*offset.borrow_mut()) = __rhs;
        let offset_table: Value<usize> = Rc::new(RefCell::new((*offset.borrow())));
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*(*hdr.borrow()).upgrade().deref()).ttc_fonts.borrow()).len()
        } {
            let __rhs = ({ StoreU32_31((*result.borrow()).clone(), (*offset.borrow()), 0_u32) });
            (*offset.borrow_mut()) = __rhs;
            (*i.borrow_mut()).postfix_inc();
        }
        if ((*(*(*hdr.borrow()).upgrade().deref()).header_version.borrow()) == 131072_u32) {
            let __rhs = ({ StoreU32_31((*result.borrow()).clone(), (*offset.borrow()), 0_u32) });
            (*offset.borrow_mut()) = __rhs;
            let __rhs = ({ StoreU32_31((*result.borrow()).clone(), (*offset.borrow()), 0_u32) });
            (*offset.borrow_mut()) = __rhs;
            let __rhs = ({ StoreU32_31((*result.borrow()).clone(), (*offset.borrow()), 0_u32) });
            (*offset.borrow_mut()) = __rhs;
        }
        {
            let __a0 = (*(*(*hdr.borrow()).upgrade().deref()).ttc_fonts.borrow()).len() as usize;
            (*(*(*metadata.borrow()).upgrade().deref())
                .font_infos
                .borrow_mut())
            .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < (*(*(*hdr.borrow()).upgrade().deref()).ttc_fonts.borrow()).len()
        } {
            let ttc_font: Ptr<woff2_TtcFont> =
                ((*(*hdr.borrow()).upgrade().deref()).ttc_fonts.as_pointer() as Ptr<woff2_TtcFont>)
                    .offset((*i.borrow()) as isize);
            let __rhs = ({
                StoreU32_31(
                    (*result.borrow()).clone(),
                    (*offset_table.borrow()),
                    ((*offset.borrow()) as u32),
                )
            });
            (*offset_table.borrow_mut()) = __rhs;
            (*(*ttc_font.upgrade().deref()).dst_offset.borrow_mut()) = ((*offset.borrow()) as u32);
            let __rhs = ({
                let _flavor: u32 = (*(*ttc_font.upgrade().deref()).flavor.borrow());
                let _num_tables: u16 =
                    ((*(*ttc_font.upgrade().deref()).table_indices.borrow()).len() as u16);
                StoreOffsetTable_70(
                    (*result.borrow()).clone(),
                    (*offset.borrow()),
                    _flavor,
                    _num_tables,
                )
            });
            (*offset.borrow_mut()) = __rhs;
            'loop_: for table_index in
                (*ttc_font.upgrade().deref()).table_indices.as_pointer() as Ptr<u16>
            {
                let table_index: Value<u16> = Rc::new(RefCell::new(table_index.read().clone()));
                let tag: Value<u32> = Rc::new(RefCell::new(
                    (*(*((*(*hdr.borrow()).upgrade().deref()).tables.as_pointer()
                        as Ptr<woff2_Table>)
                        .offset(((*table_index.borrow()) as usize) as isize)
                        .upgrade()
                        .deref())
                    .tag
                    .borrow()),
                ));
                ((*((*(*metadata.borrow()).upgrade().deref())
                    .font_infos
                    .as_pointer() as Ptr<woff2_WOFF2FontInfo>)
                    .offset((*i.borrow()) as isize)
                    .upgrade()
                    .deref())
                .table_entry_by_tag
                .as_pointer() as Ptr<BTreeMap<u32, Value<u32>>>)
                    .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                        __v.entry((*tag.borrow()).clone())
                            .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                            .as_pointer()
                    })
                    .write(((*offset.borrow()) as u32));
                let __rhs = ({
                    StoreTableEntry_71(
                        (*result.borrow()).clone(),
                        ((*offset.borrow()) as u32),
                        (*tag.borrow()),
                    )
                });
                (*offset.borrow_mut()) = __rhs;
            }
            let __rhs = ({
                let _buf: Ptr<u8> = ((output.as_pointer() as Ptr<u8>).offset(
                    ((*(*ttc_font.upgrade().deref()).dst_offset.borrow()) as usize) as isize,
                ));
                let _size: usize = (*offset.borrow())
                    .wrapping_sub(((*(*ttc_font.upgrade().deref()).dst_offset.borrow()) as usize));
                ComputeULongSum_26(_buf, _size)
            });
            (*(*ttc_font.upgrade().deref()).header_checksum.borrow_mut()) = __rhs;
            (*i.borrow_mut()).postfix_inc();
        }
    } else {
        {
            let __a0 = 1_usize as usize;
            (*(*(*metadata.borrow()).upgrade().deref())
                .font_infos
                .borrow_mut())
            .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let __rhs = ({
            let _flavor: u32 = (*(*(*hdr.borrow()).upgrade().deref()).flavor.borrow());
            let _num_tables: u16 = (*(*(*hdr.borrow()).upgrade().deref()).num_tables.borrow());
            StoreOffsetTable_70(
                (*result.borrow()).clone(),
                (*offset.borrow()),
                _flavor,
                _num_tables,
            )
        });
        (*offset.borrow_mut()) = __rhs;
        let i: Value<u16> = Rc::new(RefCell::new(0_u16));
        'loop_: while {
            let _lhs = ((*i.borrow()) as i32);
            _lhs < ((*(*(*hdr.borrow()).upgrade().deref()).num_tables.borrow()) as i32)
        } {
            ((*((*(*metadata.borrow()).upgrade().deref())
                .font_infos
                .as_pointer() as Ptr<woff2_WOFF2FontInfo>)
                .offset(0_usize as isize)
                .upgrade()
                .deref())
            .table_entry_by_tag
            .as_pointer() as Ptr<BTreeMap<u32, Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<u32, Value<u32>>| {
                    __v.entry(
                        (*(*(sorted_tables.as_pointer() as Ptr<woff2_Table>)
                            .offset(((*i.borrow()) as usize) as isize)
                            .upgrade()
                            .deref())
                        .tag
                        .borrow())
                        .clone(),
                    )
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
                })
                .write(((*offset.borrow()) as u32));
            let __rhs = ({
                StoreTableEntry_71(
                    (*result.borrow()).clone(),
                    ((*offset.borrow()) as u32),
                    (*(*(sorted_tables.as_pointer() as Ptr<woff2_Table>)
                        .offset(((*i.borrow()) as usize) as isize)
                        .upgrade()
                        .deref())
                    .tag
                    .borrow()),
                )
            });
            (*offset.borrow_mut()) = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
    }
    if ((!({
        let _buf: AnyPtr =
            (((output.as_pointer() as Ptr<u8>).offset(0_usize as isize)) as Ptr<u8>).to_any();
        let _n: usize = (*output.borrow()).len();
        (*(*out.borrow()).upgrade().deref()).Write_AnyPtr_usize(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    (*(*(*metadata.borrow()).upgrade().deref())
        .header_checksum
        .borrow_mut()) = ({
        let _buf: Ptr<u8> = ((output.as_pointer() as Ptr<u8>).offset(0_usize as isize));
        let _size: usize = (*output.borrow()).len();
        ComputeULongSum_26(_buf, _size)
    });
    return true;
}
pub fn ComputeWOFF2FinalSize_77(data: Ptr<u8>, length: usize) -> usize {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (*data.borrow()).clone(),
        (*length.borrow()),
    )));
    let total_length: Value<u32> = <Value<u32>>::default();
    if (!({ (*file.borrow()).Skip(16_usize) }))
        || (!({ (*file.borrow()).ReadU32((total_length.as_pointer())) }))
    {
        return 0_usize;
    }
    return ((*total_length.borrow()) as usize);
}
pub fn ConvertWOFF2ToTTF_78(
    result: Ptr<u8>,
    result_length: usize,
    data: Ptr<u8>,
    length: usize,
) -> bool {
    let result: Value<Ptr<u8>> = Rc::new(RefCell::new(result));
    let result_length: Value<usize> = Rc::new(RefCell::new(result_length));
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let out: Value<woff2_WOFF2MemoryOut> =
        Rc::new(RefCell::new(woff2_WOFF2MemoryOut::woff2_WOFF2MemoryOut(
            (*result.borrow()).clone(),
            (*result_length.borrow()),
        )));
    return ({
        ConvertWOFF2ToTTF_79(
            (*data.borrow()).clone(),
            (*length.borrow()),
            ((out.as_pointer()).to_strong() as Value<dyn woff2_WOFF2Out>).as_pointer_dyn(),
        )
    });
}
pub fn ConvertWOFF2ToTTF_79(data: Ptr<u8>, length: usize, out: PtrDyn<dyn woff2_WOFF2Out>) -> bool {
    let data: Value<Ptr<u8>> = Rc::new(RefCell::new(data));
    let length: Value<usize> = Rc::new(RefCell::new(length));
    let out: Value<PtrDyn<dyn woff2_WOFF2Out>> = Rc::new(RefCell::new(out));
    let metadata: Value<woff2_RebuildMetadata> =
        Rc::new(RefCell::new(<woff2_RebuildMetadata>::default()));
    let hdr: Value<woff2_WOFF2Header> = Rc::new(RefCell::new(<woff2_WOFF2Header>::default()));
    if !({
        ReadWOFF2Header_75(
            (*data.borrow()).clone(),
            (*length.borrow()),
            (hdr.as_pointer()),
        )
    }) {
        return false;
    }
    if !({
        WriteHeaders_76(
            (*data.borrow()).clone(),
            (*length.borrow()),
            (metadata.as_pointer()),
            (hdr.as_pointer()),
            (*out.borrow()).clone(),
        )
    }) {
        return false;
    }
    let compression_ratio: Value<f32> = Rc::new(RefCell::new(
        (((*(*hdr.borrow()).uncompressed_size.borrow()) as f32) / ((*length.borrow()) as f32)),
    ));
    if ((*compression_ratio.borrow())
        > (*kMaxPlausibleCompressionRatio_54.with(Value::clone).borrow()))
    {
        eprintln!(
            "Implausible compression ratio {:.1}",
            ((*compression_ratio.borrow()) as f64)
        );
        return false;
    }
    let src_buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*data.borrow()).offset((*(*hdr.borrow()).compressed_offset.borrow()) as isize),
    ));
    let uncompressed_buf: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..((*(*hdr.borrow()).uncompressed_size.borrow()) as usize) as usize)
            .map(|_| <u8>::default())
            .collect::<Vec<_>>(),
    ));
    if ((((*(*hdr.borrow()).uncompressed_size.borrow()) < 1_u32) as i64) != 0) {
        return false;
    }
    if ((!({
        let _dst_size: usize = ((*(*hdr.borrow()).uncompressed_size.borrow()) as usize);
        let _src_size: usize = ((*(*hdr.borrow()).compressed_length.borrow()) as usize);
        Woff2Uncompress_68(
            ((uncompressed_buf.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
            _dst_size,
            (*src_buf.borrow()).clone(),
            _src_size,
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*(*metadata.borrow()).font_infos.borrow()).len()) {
        if ((!({
            let _transformed_buf_size: u32 = (*(*hdr.borrow()).uncompressed_size.borrow());
            let _hdr: Ptr<woff2_WOFF2Header> = (hdr.as_pointer());
            ReconstructFont_74(
                ((uncompressed_buf.as_pointer() as Ptr<u8>).offset(0_usize as isize)),
                _transformed_buf_size,
                (metadata.as_pointer()),
                _hdr,
                (*i.borrow()),
                (*out.borrow()).clone(),
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return true;
}
#[derive(Default)]
pub struct woff2_WOFF2StringOut {
    buf_: Value<Ptr<Vec<u8>>>,
    max_size_: Value<usize>,
    offset_: Value<usize>,
}
impl woff2_WOFF2StringOut {
    pub fn woff2_WOFF2StringOut(buf: Ptr<Vec<u8>>) -> Self {
        let buf: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(buf));
        let mut this = Self {
            buf_: Rc::new(RefCell::new((*buf.borrow()).clone())),
            max_size_: Rc::new(RefCell::new(
                (*kDefaultMaxSize_28.with(Value::clone).borrow()),
            )),
            offset_: Rc::new(RefCell::new(0_usize)),
        };
        this
    }
    pub fn MaxSize(&self) -> usize {
        return (*self.max_size_.borrow());
    }
}
impl woff2_WOFF2Out for woff2_WOFF2StringOut {
    fn Write_AnyPtr_usize(&self, buf: AnyPtr, n: usize) -> bool {
        let buf: Value<AnyPtr> = Rc::new(RefCell::new(buf));
        let n: Value<usize> = Rc::new(RefCell::new(n));
        return ({
            let _offset: usize = (*self.offset_.borrow());
            self.Write_AnyPtr_usize_usize((*buf.borrow()).clone(), _offset, (*n.borrow()))
        });
    }
    fn Write_AnyPtr_usize_usize(&self, buf: AnyPtr, offset: usize, n: usize) -> bool {
        let buf: Value<AnyPtr> = Rc::new(RefCell::new(buf));
        let offset: Value<usize> = Rc::new(RefCell::new(offset));
        let n: Value<usize> = Rc::new(RefCell::new(n));
        if ((*offset.borrow()) > (*self.max_size_.borrow()))
            || ((*n.borrow()) > (*self.max_size_.borrow()).wrapping_sub((*offset.borrow())))
        {
            return false;
        }
        if {
            let _lhs = (*offset.borrow());
            _lhs == ((*(*self.buf_.borrow()).upgrade().deref()).len() - 1)
        } {
            {
                ((*self.buf_.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>).with_mut(
                    |__v: &mut Vec<u8>| {
                        __v.pop();
                        __v.extend(
                            (*buf.borrow())
                                .cast::<u8>()
                                .expect("ub:wrong type")
                                .map(|c| c.read())
                                .take((*n.borrow()) as usize),
                        );
                        __v.push(0);
                    },
                );
                ((*self.buf_.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
            };
        } else {
            if {
                let _lhs = (*offset.borrow()).wrapping_add((*n.borrow()));
                _lhs > ((*(*self.buf_.borrow()).upgrade().deref()).len() - 1)
            } {
                {
                    (*self.buf_.borrow()).with_mut(|__v: &mut Vec<u8>| __v.pop());
                    (*self.buf_.borrow()).with_mut(|__v: &mut Vec<u8>| {
                        __v.resize(
                            (*(*self.buf_.borrow()).upgrade().deref()).len()
                                + (((*offset.borrow()).wrapping_add((*n.borrow())) as u64)
                                    .wrapping_sub(
                                        (((*(*self.buf_.borrow()).upgrade().deref()).len() - 1)
                                            as u64),
                                    ) as usize) as usize,
                            0_u8,
                        )
                    });
                    (*self.buf_.borrow()).with_mut(|__v: &mut Vec<u8>| __v.push(0));
                    (*(*self.buf_.borrow()).upgrade().deref()).clone()
                };
            }
            {
                let pos = (*offset.borrow()) as usize;
                let end = std::cmp::min(
                    pos + (*n.borrow()) as usize,
                    (*((*self.buf_.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
                        .upgrade()
                        .deref())
                    .len()
                    .saturating_sub(1),
                );
                ((*self.buf_.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>).with_mut(
                    |__v: &mut Vec<u8>| {
                        __v.splice(
                            pos..end,
                            (*buf.borrow())
                                .cast::<u8>()
                                .expect("ub:wrong type")
                                .map(|c| c.read())
                                .take((*n.borrow()) as usize),
                        );
                    },
                );
                ((*self.buf_.borrow()).to_strong().as_pointer() as Ptr<Vec<u8>>)
            };
        }
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*self.offset_.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                ((*offset.borrow()).wrapping_add((*n.borrow())) as u64),
            ));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        (*self.offset_.borrow_mut()) = __rhs;
        return true;
    }
    fn Size(&self) -> usize {
        return (*self.offset_.borrow());
    }
}
impl Clone for woff2_WOFF2StringOut {
    fn clone(&self) -> Self {
        let mut this = Self {
            buf_: Rc::new(RefCell::new((*self.buf_.borrow()).clone())),
            max_size_: Rc::new(RefCell::new((*self.max_size_.borrow()))),
            offset_: Rc::new(RefCell::new((*self.offset_.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_WOFF2StringOut {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.buf_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.max_size_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.offset_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            buf_: Rc::new(RefCell::new(<Ptr<Vec<u8>>>::from_bytes(&buf[8..16]))),
            max_size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
            offset_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
#[derive(Default)]
pub struct woff2_WOFF2MemoryOut {
    buf_: Value<Ptr<u8>>,
    buf_size_: Value<usize>,
    offset_: Value<usize>,
}
impl woff2_WOFF2MemoryOut {
    pub fn woff2_WOFF2MemoryOut(buf: Ptr<u8>, buf_size: usize) -> Self {
        let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
        let buf_size: Value<usize> = Rc::new(RefCell::new(buf_size));
        let mut this = Self {
            buf_: Rc::new(RefCell::new((*buf.borrow()).clone())),
            buf_size_: Rc::new(RefCell::new((*buf_size.borrow()))),
            offset_: Rc::new(RefCell::new(0_usize)),
        };
        this
    }
}
impl woff2_WOFF2Out for woff2_WOFF2MemoryOut {
    fn Write_AnyPtr_usize(&self, buf: AnyPtr, n: usize) -> bool {
        let buf: Value<AnyPtr> = Rc::new(RefCell::new(buf));
        let n: Value<usize> = Rc::new(RefCell::new(n));
        return ({
            let _offset: usize = (*self.offset_.borrow());
            self.Write_AnyPtr_usize_usize((*buf.borrow()).clone(), _offset, (*n.borrow()))
        });
    }
    fn Write_AnyPtr_usize_usize(&self, buf: AnyPtr, offset: usize, n: usize) -> bool {
        let buf: Value<AnyPtr> = Rc::new(RefCell::new(buf));
        let offset: Value<usize> = Rc::new(RefCell::new(offset));
        let n: Value<usize> = Rc::new(RefCell::new(n));
        if ((*offset.borrow()) > (*self.buf_size_.borrow()))
            || ((*n.borrow()) > (*self.buf_size_.borrow()).wrapping_sub((*offset.borrow())))
        {
            return false;
        }
        {
            ((*self.buf_.borrow()).offset((*offset.borrow()) as isize) as Ptr<u8>)
                .to_any()
                .memcpy(&(*buf.borrow()), (*n.borrow()) as usize);
            ((*self.buf_.borrow()).offset((*offset.borrow()) as isize) as Ptr<u8>)
                .to_any()
                .clone()
        };
        let __rhs = ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*self.offset_.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                ((*offset.borrow()).wrapping_add((*n.borrow())) as u64),
            ));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize);
        (*self.offset_.borrow_mut()) = __rhs;
        return true;
    }
    fn Size(&self) -> usize {
        return (*self.offset_.borrow());
    }
}
impl Clone for woff2_WOFF2MemoryOut {
    fn clone(&self) -> Self {
        let mut this = Self {
            buf_: Rc::new(RefCell::new((*self.buf_.borrow()).clone())),
            buf_size_: Rc::new(RefCell::new((*self.buf_size_.borrow()))),
            offset_: Rc::new(RefCell::new((*self.offset_.borrow()))),
        };
        this
    }
}
impl ByteRepr for woff2_WOFF2MemoryOut {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.buf_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.buf_size_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.offset_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            buf_: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[8..16]))),
            buf_size_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[16..24]))),
            offset_: Rc::new(RefCell::new(<usize>::from_bytes(&buf[24..32]))),
        }
    }
}
impl woff2_WOFF2StringOut {}
impl woff2_WOFF2StringOut {
    pub fn SetMaxSize(&self, max_size: usize) {
        let max_size: Value<usize> = Rc::new(RefCell::new(max_size));
        (*self.max_size_.borrow_mut()) = (*max_size.borrow());
        if ((*self.offset_.borrow()) > (*self.max_size_.borrow())) {
            (*self.offset_.borrow_mut()) = (*self.max_size_.borrow());
        }
    }
}
impl woff2_WOFF2MemoryOut {}
pub fn GetFileContent_80(filename: Vec<u8>) -> Vec<u8> {
    let filename: Value<Vec<u8>> = Rc::new(RefCell::new(filename));
    let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open((filename.as_pointer() as Ptr<u8>).to_string())
            .expect("Failed to open file"),
    ));
    return {
        let mut __buf: Vec<u8> = Vec::new();
        let mut __f = &(*ifs.borrow()).try_clone().unwrap();
        __f.read_to_end(&mut __buf).expect("couldn't read the file");
        __buf.push(0);
        __buf
    };
}
pub fn SetFileContents_81(filename: Vec<u8>, start: Ptr<u8>, end: Ptr<u8>) {
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
        __tmp2.extend(Ptr::from_string_literal(b".ttf").to_c_string_iterator());
        __tmp2.push(0);
        __tmp2
    }));
    let input: Value<Vec<u8>> = Rc::new(RefCell::new(
        ({ GetFileContent_80((*filename.borrow()).clone()) }),
    ));
    let raw_input: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (input.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
    ));
    let output: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![
            0_u8;
            ({
                let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                    (({
                        ComputeWOFF2FinalSize_77(
                            (*raw_input.borrow()).clone(),
                            ((*input.borrow()).len() - 1),
                        )
                    }) as u64),
                ));
                let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                    ((*kDefaultMaxSize_28.with(Value::clone).borrow()) as u64),
                ));
                (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    __tmp_0.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            }) as usize
        ]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect(),
    ));
    let out: Value<woff2_WOFF2StringOut> = Rc::new(RefCell::new(
        woff2_WOFF2StringOut::woff2_WOFF2StringOut((output.as_pointer())),
    ));
    let ok: Value<bool> = Rc::new(RefCell::new(
        ({
            ConvertWOFF2ToTTF_79(
                (*raw_input.borrow()).clone(),
                ((*input.borrow()).len() - 1),
                ((out.as_pointer()).to_strong() as Value<dyn woff2_WOFF2Out>).as_pointer_dyn(),
            )
        }),
    ));
    if (*ok.borrow()) {
        ({
            let _start: Ptr<u8> = (output.as_pointer() as Ptr<u8>);
            let _end: Ptr<u8> = (output.as_pointer() as Ptr<u8>)
                .offset((({ (*out.borrow()).Size() }) as i64) as isize);
            SetFileContents_81((*outfilename.borrow()).clone(), _start, _end)
        });
    }
    return if (*ok.borrow()) { 0 } else { 1 };
}
