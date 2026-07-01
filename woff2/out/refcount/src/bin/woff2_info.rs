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
            .offset(0_usize as isize);
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
                    .offset(0_usize as isize),
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
            .offset((*i.borrow()) as isize);
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
pub fn GetFileContent_49(filename: Vec<u8>) -> Vec<u8> {
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
pub fn SetFileContents_50(filename: Vec<u8>, start: Ptr<u8>, end: Ptr<u8>) {
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
thread_local!();
pub fn PrintTag_51(tag: i32) -> Vec<u8> {
    let tag: Value<i32> = Rc::new(RefCell::new(tag));
    if ((((*tag.borrow()) as u32) & 2155905152_u32) != 0) {
        return Ptr::from_string_literal(b"_xfm")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>();
    }
    let printable: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((((*tag.borrow()) >> 24) & 255) as u8),
        ((((*tag.borrow()) >> 16) & 255) as u8),
        ((((*tag.borrow()) >> 8) & 255) as u8),
        (((*tag.borrow()) & 255) as u8),
    ])));
    return (printable.as_pointer() as Ptr<u8>)
        .map(|c| c.read())
        .take(4_usize as usize)
        .chain(std::iter::once(0))
        .collect::<Vec<u8>>();
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
        ({ GetFileContent_49((*filename.borrow()).clone()) }),
    ));
    let file: Value<woff2_Buffer> = Rc::new(RefCell::new(woff2_Buffer::woff2_Buffer(
        (input.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
        ((*input.borrow()).len() - 1),
    )));
    println!("WOFF2Header");
    let signature: Value<u32> = <Value<u32>>::default();
    let flavor: Value<u32> = <Value<u32>>::default();
    let length: Value<u32> = <Value<u32>>::default();
    let totalSfntSize: Value<u32> = <Value<u32>>::default();
    let totalCompressedSize: Value<u32> = <Value<u32>>::default();
    let metaOffset: Value<u32> = <Value<u32>>::default();
    let metaLength: Value<u32> = <Value<u32>>::default();
    let metaOrigLength: Value<u32> = <Value<u32>>::default();
    let privOffset: Value<u32> = <Value<u32>>::default();
    let privLength: Value<u32> = <Value<u32>>::default();
    let num_tables: Value<u16> = <Value<u16>>::default();
    let reserved: Value<u16> = <Value<u16>>::default();
    let major: Value<u16> = <Value<u16>>::default();
    let minor: Value<u16> = <Value<u16>>::default();
    if !({ (*file.borrow()).ReadU32((signature.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((flavor.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((length.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU16((num_tables.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU16((reserved.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((totalSfntSize.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((totalCompressedSize.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU16((major.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU16((minor.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((metaOffset.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((metaLength.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((metaOrigLength.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((privOffset.as_pointer())) }) {
        return 1;
    }
    if !({ (*file.borrow()).ReadU32((privLength.as_pointer())) }) {
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
        ({ (*file.borrow()).offset() })
    );
    println!("Entry offset flags tag  origLength txLength");
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < ((*num_tables.borrow()) as i32)) {
        let offset: Value<usize> = Rc::new(RefCell::new(({ (*file.borrow()).offset() })));
        let flags: Value<u8> = <Value<u8>>::default();
        let tag: Value<u32> = <Value<u32>>::default();
        let origLength: Value<u32> = <Value<u32>>::default();
        let transformLength: Value<u32> = <Value<u32>>::default();
        if !({ (*file.borrow()).ReadU8((flags.as_pointer())) }) {
            return 1;
        }
        if ((((*flags.borrow()) as i32) & 63) == 63) {
            if !({ (*file.borrow()).ReadU32((tag.as_pointer())) }) {
                return 1;
            }
        } else {
            (*tag.borrow_mut()) = (*kKnownTags_8.with(Value::clone).borrow())
                [(((*flags.borrow()) as i32) & 63) as usize];
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
                as Ptr<u8>),
            (*origLength.borrow())
        );
        let xform_version: Value<u8> = Rc::new(RefCell::new(
            (((((*flags.borrow()) as i32) >> 6) & 3) as u8),
        ));
        if ((*tag.borrow()) == (*kGlyfTableTag_0.with(Value::clone).borrow()))
            || ((*tag.borrow()) == (*kLocaTableTag_2.with(Value::clone).borrow()))
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
    if ((*flavor.borrow()) == (*kTtcFontFlavor_22.with(Value::clone).borrow())) {
        let version: Value<u32> = <Value<u32>>::default();
        let numFonts: Value<u32> = <Value<u32>>::default();
        if !({ (*file.borrow()).ReadU32((version.as_pointer())) }) {
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
            let numTables: Value<u32> = <Value<u32>>::default();
            let flavor: Value<u32> = <Value<u32>>::default();
            if !({ Read255UShort_12((file.as_pointer()), (numTables.as_pointer())) }) {
                return 1;
            }
            if !({ (*file.borrow()).ReadU32((flavor.as_pointer())) }) {
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
                let table_idx: Value<u32> = <Value<u32>>::default();
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
                                (((table_tags.as_pointer() as Ptr<u32>)
                                    .offset(((*table_idx.borrow()) as usize) as isize)
                                    .read()) as i32),
                            )
                        })
                    ))
                    .as_pointer() as Ptr<u8>),
                    (*table_idx.borrow())
                );
                (*j.borrow_mut()).postfix_inc();
            }
            (*i.borrow_mut()).postfix_inc();
        }
    }
    println!(
        "TableDirectory ends at +{}",
        ({ (*file.borrow()).offset() })
    );
    return 0;
}
