extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

// table_tags.rs
pub static woff2_kGlyfTableTag: u32 = 1735162214_u32;
pub static woff2_kHeadTableTag: u32 = 1751474532_u32;
pub static woff2_kLocaTableTag: u32 = 1819239265_u32;
pub static woff2_kDsigTableTag: u32 = 1146308935_u32;
pub static woff2_kCffTableTag: u32 = 1128678944_u32;
pub static woff2_kHmtxTableTag: u32 = 1752003704_u32;
pub static woff2_kHheaTableTag: u32 = 1751672161_u32;
pub static woff2_kMaxpTableTag: u32 = 1835104368_u32;
pub static woff2_kKnownTags: [u32; 63] = [
    ((((((('c' as u8) as i32) << (24)) | ((('m' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('p' as u8) as i32)) as u32),
    ((((((('h' as u8) as i32) << (24)) | ((('e' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('d' as u8) as i32)) as u32),
    ((((((('h' as u8) as i32) << (24)) | ((('h' as u8) as i32) << (16)))
        | ((('e' as u8) as i32) << (8)))
        | (('a' as u8) as i32)) as u32),
    ((((((('h' as u8) as i32) << (24)) | ((('m' as u8) as i32) << (16)))
        | ((('t' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('m' as u8) as i32) << (24)) | ((('a' as u8) as i32) << (16)))
        | ((('x' as u8) as i32) << (8)))
        | (('p' as u8) as i32)) as u32),
    ((((((('n' as u8) as i32) << (24)) | ((('a' as u8) as i32) << (16)))
        | ((('m' as u8) as i32) << (8)))
        | (('e' as u8) as i32)) as u32),
    ((((((('O' as u8) as i32) << (24)) | ((('S' as u8) as i32) << (16)))
        | ((('/' as u8) as i32) << (8)))
        | (('2' as u8) as i32)) as u32),
    ((((((('p' as u8) as i32) << (24)) | ((('o' as u8) as i32) << (16)))
        | ((('s' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('c' as u8) as i32) << (24)) | ((('v' as u8) as i32) << (16)))
        | ((('t' as u8) as i32) << (8)))
        | ((' ' as u8) as i32)) as u32),
    ((((((('f' as u8) as i32) << (24)) | ((('p' as u8) as i32) << (16)))
        | ((('g' as u8) as i32) << (8)))
        | (('m' as u8) as i32)) as u32),
    ((((((('g' as u8) as i32) << (24)) | ((('l' as u8) as i32) << (16)))
        | ((('y' as u8) as i32) << (8)))
        | (('f' as u8) as i32)) as u32),
    ((((((('l' as u8) as i32) << (24)) | ((('o' as u8) as i32) << (16)))
        | ((('c' as u8) as i32) << (8)))
        | (('a' as u8) as i32)) as u32),
    ((((((('p' as u8) as i32) << (24)) | ((('r' as u8) as i32) << (16)))
        | ((('e' as u8) as i32) << (8)))
        | (('p' as u8) as i32)) as u32),
    ((((((('C' as u8) as i32) << (24)) | ((('F' as u8) as i32) << (16)))
        | ((('F' as u8) as i32) << (8)))
        | ((' ' as u8) as i32)) as u32),
    ((((((('V' as u8) as i32) << (24)) | ((('O' as u8) as i32) << (16)))
        | ((('R' as u8) as i32) << (8)))
        | (('G' as u8) as i32)) as u32),
    ((((((('E' as u8) as i32) << (24)) | ((('B' as u8) as i32) << (16)))
        | ((('D' as u8) as i32) << (8)))
        | (('T' as u8) as i32)) as u32),
    ((((((('E' as u8) as i32) << (24)) | ((('B' as u8) as i32) << (16)))
        | ((('L' as u8) as i32) << (8)))
        | (('C' as u8) as i32)) as u32),
    ((((((('g' as u8) as i32) << (24)) | ((('a' as u8) as i32) << (16)))
        | ((('s' as u8) as i32) << (8)))
        | (('p' as u8) as i32)) as u32),
    ((((((('h' as u8) as i32) << (24)) | ((('d' as u8) as i32) << (16)))
        | ((('m' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('k' as u8) as i32) << (24)) | ((('e' as u8) as i32) << (16)))
        | ((('r' as u8) as i32) << (8)))
        | (('n' as u8) as i32)) as u32),
    ((((((('L' as u8) as i32) << (24)) | ((('T' as u8) as i32) << (16)))
        | ((('S' as u8) as i32) << (8)))
        | (('H' as u8) as i32)) as u32),
    ((((((('P' as u8) as i32) << (24)) | ((('C' as u8) as i32) << (16)))
        | ((('L' as u8) as i32) << (8)))
        | (('T' as u8) as i32)) as u32),
    ((((((('V' as u8) as i32) << (24)) | ((('D' as u8) as i32) << (16)))
        | ((('M' as u8) as i32) << (8)))
        | (('X' as u8) as i32)) as u32),
    ((((((('v' as u8) as i32) << (24)) | ((('h' as u8) as i32) << (16)))
        | ((('e' as u8) as i32) << (8)))
        | (('a' as u8) as i32)) as u32),
    ((((((('v' as u8) as i32) << (24)) | ((('m' as u8) as i32) << (16)))
        | ((('t' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('B' as u8) as i32) << (24)) | ((('A' as u8) as i32) << (16)))
        | ((('S' as u8) as i32) << (8)))
        | (('E' as u8) as i32)) as u32),
    ((((((('G' as u8) as i32) << (24)) | ((('D' as u8) as i32) << (16)))
        | ((('E' as u8) as i32) << (8)))
        | (('F' as u8) as i32)) as u32),
    ((((((('G' as u8) as i32) << (24)) | ((('P' as u8) as i32) << (16)))
        | ((('O' as u8) as i32) << (8)))
        | (('S' as u8) as i32)) as u32),
    ((((((('G' as u8) as i32) << (24)) | ((('S' as u8) as i32) << (16)))
        | ((('U' as u8) as i32) << (8)))
        | (('B' as u8) as i32)) as u32),
    ((((((('E' as u8) as i32) << (24)) | ((('B' as u8) as i32) << (16)))
        | ((('S' as u8) as i32) << (8)))
        | (('C' as u8) as i32)) as u32),
    ((((((('J' as u8) as i32) << (24)) | ((('S' as u8) as i32) << (16)))
        | ((('T' as u8) as i32) << (8)))
        | (('F' as u8) as i32)) as u32),
    ((((((('M' as u8) as i32) << (24)) | ((('A' as u8) as i32) << (16)))
        | ((('T' as u8) as i32) << (8)))
        | (('H' as u8) as i32)) as u32),
    ((((((('C' as u8) as i32) << (24)) | ((('B' as u8) as i32) << (16)))
        | ((('D' as u8) as i32) << (8)))
        | (('T' as u8) as i32)) as u32),
    ((((((('C' as u8) as i32) << (24)) | ((('B' as u8) as i32) << (16)))
        | ((('L' as u8) as i32) << (8)))
        | (('C' as u8) as i32)) as u32),
    ((((((('C' as u8) as i32) << (24)) | ((('O' as u8) as i32) << (16)))
        | ((('L' as u8) as i32) << (8)))
        | (('R' as u8) as i32)) as u32),
    ((((((('C' as u8) as i32) << (24)) | ((('P' as u8) as i32) << (16)))
        | ((('A' as u8) as i32) << (8)))
        | (('L' as u8) as i32)) as u32),
    ((((((('S' as u8) as i32) << (24)) | ((('V' as u8) as i32) << (16)))
        | ((('G' as u8) as i32) << (8)))
        | ((' ' as u8) as i32)) as u32),
    ((((((('s' as u8) as i32) << (24)) | ((('b' as u8) as i32) << (16)))
        | ((('i' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('a' as u8) as i32) << (24)) | ((('c' as u8) as i32) << (16)))
        | ((('n' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('a' as u8) as i32) << (24)) | ((('v' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('r' as u8) as i32)) as u32),
    ((((((('b' as u8) as i32) << (24)) | ((('d' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('b' as u8) as i32) << (24)) | ((('l' as u8) as i32) << (16)))
        | ((('o' as u8) as i32) << (8)))
        | (('c' as u8) as i32)) as u32),
    ((((((('b' as u8) as i32) << (24)) | ((('s' as u8) as i32) << (16)))
        | ((('l' as u8) as i32) << (8)))
        | (('n' as u8) as i32)) as u32),
    ((((((('c' as u8) as i32) << (24)) | ((('v' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('r' as u8) as i32)) as u32),
    ((((((('f' as u8) as i32) << (24)) | ((('d' as u8) as i32) << (16)))
        | ((('s' as u8) as i32) << (8)))
        | (('c' as u8) as i32)) as u32),
    ((((((('f' as u8) as i32) << (24)) | ((('e' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('f' as u8) as i32) << (24)) | ((('m' as u8) as i32) << (16)))
        | ((('t' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('f' as u8) as i32) << (24)) | ((('v' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('r' as u8) as i32)) as u32),
    ((((((('g' as u8) as i32) << (24)) | ((('v' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('r' as u8) as i32)) as u32),
    ((((((('h' as u8) as i32) << (24)) | ((('s' as u8) as i32) << (16)))
        | ((('t' as u8) as i32) << (8)))
        | (('y' as u8) as i32)) as u32),
    ((((((('j' as u8) as i32) << (24)) | ((('u' as u8) as i32) << (16)))
        | ((('s' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('l' as u8) as i32) << (24)) | ((('c' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('r' as u8) as i32)) as u32),
    ((((((('m' as u8) as i32) << (24)) | ((('o' as u8) as i32) << (16)))
        | ((('r' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('m' as u8) as i32) << (24)) | ((('o' as u8) as i32) << (16)))
        | ((('r' as u8) as i32) << (8)))
        | (('x' as u8) as i32)) as u32),
    ((((((('o' as u8) as i32) << (24)) | ((('p' as u8) as i32) << (16)))
        | ((('b' as u8) as i32) << (8)))
        | (('d' as u8) as i32)) as u32),
    ((((((('p' as u8) as i32) << (24)) | ((('r' as u8) as i32) << (16)))
        | ((('o' as u8) as i32) << (8)))
        | (('p' as u8) as i32)) as u32),
    ((((((('t' as u8) as i32) << (24)) | ((('r' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('k' as u8) as i32)) as u32),
    ((((((('Z' as u8) as i32) << (24)) | ((('a' as u8) as i32) << (16)))
        | ((('p' as u8) as i32) << (8)))
        | (('f' as u8) as i32)) as u32),
    ((((((('S' as u8) as i32) << (24)) | ((('i' as u8) as i32) << (16)))
        | ((('l' as u8) as i32) << (8)))
        | (('f' as u8) as i32)) as u32),
    ((((((('G' as u8) as i32) << (24)) | ((('l' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('G' as u8) as i32) << (24)) | ((('l' as u8) as i32) << (16)))
        | ((('o' as u8) as i32) << (8)))
        | (('c' as u8) as i32)) as u32),
    ((((((('F' as u8) as i32) << (24)) | ((('e' as u8) as i32) << (16)))
        | ((('a' as u8) as i32) << (8)))
        | (('t' as u8) as i32)) as u32),
    ((((((('S' as u8) as i32) << (24)) | ((('i' as u8) as i32) << (16)))
        | ((('l' as u8) as i32) << (8)))
        | (('l' as u8) as i32)) as u32),
];

// variable_length.rs
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_Buffer {
    buffer_: *const u8,
    length_: u64,
    offset_: u64,
}
impl woff2_Buffer {
    pub unsafe fn woff2_Buffer(mut data: *const u8, mut len: u64) -> Self {
        let mut this = Self {
            buffer_: data,
            length_: len,
            offset_: 0_u64,
        };
        this
    }
    pub unsafe fn Skip(&mut self, mut n_bytes: u64) -> bool {
        return (unsafe {
            let _data: *mut u8 = Default::default();
            let _n_bytes: u64 = n_bytes;
            self.Read(_data, _n_bytes)
        });
    }
    pub unsafe fn Read(&mut self, mut data: *mut u8, mut n_bytes: u64) -> bool {
        if ((n_bytes) > ((((1024) * (1024)) * (1024)) as u64)) {
            return false;
        }
        if (((self.offset_).wrapping_add(n_bytes)) > (self.length_))
            || ((self.offset_) > ((self.length_).wrapping_sub(n_bytes)))
        {
            return false;
        }
        if !(data).is_null() {
            {
                if n_bytes != 0 {
                    ::std::ptr::copy_nonoverlapping(
                        (self.buffer_.offset((self.offset_) as isize) as *const u8
                            as *const ::libc::c_void),
                        (data as *mut u8 as *mut ::libc::c_void),
                        n_bytes as usize,
                    )
                }
                (data as *mut u8 as *mut ::libc::c_void)
            };
        }
        self.offset_ = (self.offset_).wrapping_add(n_bytes);
        return true;
    }
    pub unsafe fn ReadU8(&mut self, mut value: *mut u8) -> bool {
        if ((self.length_) < (1_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(1_u64))) {
            return false;
        }
        (*value) = (*self.buffer_.offset((self.offset_) as isize));
        self.offset_.prefix_inc();
        return true;
    }
    pub unsafe fn ReadU16(&mut self, mut value: *mut u16) -> bool {
        if ((self.length_) < (2_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(2_u64))) {
            return false;
        }
        {
            if ::std::mem::size_of::<u16>() as u64 != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u16 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u16>() as u64 as usize,
                )
            }
            (value as *mut u16 as *mut ::libc::c_void)
        };
        (*value) = u16::from_be((*value));
        self.offset_ = (self.offset_).wrapping_add(2_u64);
        return true;
    }
    pub unsafe fn ReadS16(&mut self, mut value: *mut i16) -> bool {
        return (unsafe {
            let _value: *mut u16 = (value as *mut u16);
            self.ReadU16(_value)
        });
    }
    pub unsafe fn ReadU24(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (3_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(3_u64))) {
            return false;
        }
        (*value) = (((((*self.buffer_.offset((self.offset_) as isize)) as u32) << (16))
            | (((*self
                .buffer_
                .offset(((self.offset_).wrapping_add(1_u64)) as isize)) as u32)
                << (8)))
            | ((*self
                .buffer_
                .offset(((self.offset_).wrapping_add(2_u64)) as isize)) as u32));
        self.offset_ = (self.offset_).wrapping_add(3_u64);
        return true;
    }
    pub unsafe fn ReadU32(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (4_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(4_u64))) {
            return false;
        }
        {
            if ::std::mem::size_of::<u32>() as u64 != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u32 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u32>() as u64 as usize,
                )
            }
            (value as *mut u32 as *mut ::libc::c_void)
        };
        (*value) = u32::from_be((*value));
        self.offset_ = (self.offset_).wrapping_add(4_u64);
        return true;
    }
    pub unsafe fn ReadS32(&mut self, mut value: *mut i32) -> bool {
        return (unsafe {
            let _value: *mut u32 = (value as *mut u32);
            self.ReadU32(_value)
        });
    }
    pub unsafe fn ReadTag(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (4_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(4_u64))) {
            return false;
        }
        {
            if ::std::mem::size_of::<u32>() as u64 != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u32 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u32>() as u64 as usize,
                )
            }
            (value as *mut u32 as *mut ::libc::c_void)
        };
        self.offset_ = (self.offset_).wrapping_add(4_u64);
        return true;
    }
    pub unsafe fn ReadR64(&mut self, mut value: *mut u64) -> bool {
        if ((self.length_) < (8_u64)) || ((self.offset_) > ((self.length_).wrapping_sub(8_u64))) {
            return false;
        }
        {
            if ::std::mem::size_of::<u64>() as u64 != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u64 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u64>() as u64 as usize,
                )
            }
            (value as *mut u64 as *mut ::libc::c_void)
        };
        self.offset_ = (self.offset_).wrapping_add(8_u64);
        return true;
    }
    pub unsafe fn buffer(&self) -> *const u8 {
        return self.buffer_;
    }
    pub unsafe fn offset(&self) -> u64 {
        return self.offset_;
    }
    pub unsafe fn length(&self) -> u64 {
        return self.length_;
    }
    pub unsafe fn set_offset(&mut self, mut newoffset: u64) -> bool {
        if ((newoffset) > (self.length_)) {
            return false;
        }
        self.offset_ = newoffset;
        return true;
    }
}
pub unsafe fn Size255UShort_0(mut value: u16) -> u64 {
    let mut result: u64 = 3_u64;
    if ((value as i32) < (253)) {
        result = 1_u64;
    } else if ((value as i32) < (762)) {
        result = 2_u64;
    } else {
        result = 3_u64;
    }
    return result;
}
pub unsafe fn Write255UShort_1(mut out: *mut Vec<u8>, mut value: i32) {
    if ((value) < (253)) {
        (*out).push((value as u8));
    } else if ((value) < (506)) {
        (*out).push(255_u8);
        (*out).push((((value) - (253)) as u8));
    } else if ((value) < (762)) {
        (*out).push(254_u8);
        (*out).push((((value) - (506)) as u8));
    } else {
        (*out).push(253_u8);
        (*out).push((((value) >> (8)) as u8));
        (*out).push((((value) & (255)) as u8));
    }
}
pub unsafe fn Store255UShort_2(mut val: i32, mut offset: *mut u64, mut dst: *mut u8) {
    let mut packed: Vec<u8> = Vec::new();
    (unsafe {
        let _out: *mut Vec<u8> = (&mut packed as *mut Vec<u8>);
        let _value: i32 = val;
        Write255UShort_1(_out, _value)
    });
    'loop_: for packed_byte in 0..(packed.len()) {
        let mut packed_byte = packed[packed_byte].clone();
        (*dst.offset(((*offset).postfix_inc()) as isize)) = packed_byte;
    }
}
pub unsafe fn Read255UShort_3(mut buf: *mut woff2_Buffer, mut value: *mut u32) -> bool {
    static kWordCode: i32 = 253;;
    static kOneMoreByteCode2: i32 = 254;;
    static kOneMoreByteCode1: i32 = 255;;
    static kLowestUCode: i32 = 253;;
    let mut code: u8 = 0_u8;
    if !(unsafe {
        let _value: *mut u8 = (&mut code as *mut u8);
        (*buf).ReadU8(_value)
    }) {
        return false;
    }
    if ((code as i32) == (kWordCode)) {
        let mut result: u16 = 0_u16;
        if !(unsafe {
            let _value: *mut u16 = (&mut result as *mut u16);
            (*buf).ReadU16(_value)
        }) {
            return false;
        }
        (*value) = (result as u32);
        return true;
    } else if ((code as i32) == (kOneMoreByteCode1)) {
        let mut result: u8 = 0_u8;
        if !(unsafe {
            let _value: *mut u8 = (&mut result as *mut u8);
            (*buf).ReadU8(_value)
        }) {
            return false;
        }
        (*value) = (((result as i32) + (kLowestUCode)) as u32);
        return true;
    } else if ((code as i32) == (kOneMoreByteCode2)) {
        let mut result: u8 = 0_u8;
        if !(unsafe {
            let _value: *mut u8 = (&mut result as *mut u8);
            (*buf).ReadU8(_value)
        }) {
            return false;
        }
        (*value) = (((result as i32) + ((kLowestUCode) * (2))) as u32);
        return true;
    } else {
        (*value) = (code as u32);
        return true;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn ReadBase128_4(mut buf: *mut woff2_Buffer, mut value: *mut u32) -> bool {
    let mut result: u32 = 0_u32;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (5_u64)) {
        let mut code: u8 = 0_u8;
        if !(unsafe {
            let _value: *mut u8 = (&mut code as *mut u8);
            (*buf).ReadU8(_value)
        }) {
            return false;
        }
        if ((i) == (0_u64)) && ((code as i32) == (128)) {
            return false;
        }
        if (((result) & (4261412864_u32)) != 0) {
            return false;
        }
        result = (((result) << (7)) | (((code as i32) & (127)) as u32));
        if (((code as i32) & (128)) == (0)) {
            (*value) = result;
            return true;
        }
        i.prefix_inc();
    }
    return false;
}
pub unsafe fn Base128Size_5(mut n: u64) -> u64 {
    let mut size: u64 = 1_u64;
    'loop_: while ((n) >= (128_u64)) {
        size.prefix_inc();
        n >>= 7;
    }
    return size;
}
pub unsafe fn StoreBase128_6(mut len: u64, mut offset: *mut u64, mut dst: *mut u8) {
    let mut size: u64 = (unsafe {
        let _n: u64 = len;
        Base128Size_5(_n)
    });
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (size)) {
        let mut b: i32 = ((((len)
            >> ((7_u64).wrapping_mul((((size).wrapping_sub(i)).wrapping_sub(1_u64)))))
            & (127_u64)) as i32);
        if ((i) < ((size).wrapping_sub(1_u64))) {
            b |= 128;
        }
        (*dst.offset(((*offset).postfix_inc()) as isize)) = (b as u8);
        i.prefix_inc();
    }
}
// woff2_common.rs
pub static woff2_kWoff2Signature: u32 = 2001684018_u32;
pub static woff2_kWoff2FlagsTransform: u32 = (((1) << (8)) as u32);
pub static woff2_kTtcFontFlavor: u32 = 1953784678_u32;
pub static woff2_kSfntHeaderSize: u64 = 12_u64;
pub static woff2_kSfntEntrySize: u64 = 16_u64;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_Point {
    pub x: i32,
    pub y: i32,
    pub on_curve: bool,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_Table {
    pub tag: u32,
    pub flags: u32,
    pub src_offset: u32,
    pub src_length: u32,
    pub transform_length: u32,
    pub dst_offset: u32,
    pub dst_length: u32,
    pub dst_data: *const u8,
}
impl woff2_Table {
    pub unsafe fn lt(&self, other: *const woff2_Table) -> bool {
        return ((self.tag) < ((*other).tag));
    }
}
impl Ord for woff2_Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if self.lt(other) {
                std::cmp::Ordering::Less
            } else if other.lt(other) {
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
        unsafe { !(self.lt(other)) && !(other.lt(other)) }
    }
}
impl Eq for woff2_Table {}
pub unsafe fn Log2Floor_7(mut n: u32) -> i32 {
    return if ((n) == (0_u32)) {
        -1_i32
    } else {
        ((31) ^ (n.leading_zeros() as i32))
    };
}
pub unsafe fn ComputeULongSum_8(mut buf: *const u8, mut size: u64) -> u32 {
    let mut checksum: u32 = 0_u32;
    let mut aligned_size: u64 = ((size) & (!3 as u64));
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (aligned_size)) {
        checksum = (checksum).wrapping_add(
            (((((((*buf.offset((i) as isize)) as i32) << (24))
                | (((*buf.offset(((i).wrapping_add(1_u64)) as isize)) as i32) << (16)))
                | (((*buf.offset(((i).wrapping_add(2_u64)) as isize)) as i32) << (8)))
                | ((*buf.offset(((i).wrapping_add(3_u64)) as isize)) as i32)) as u32),
        );
        i = (i).wrapping_add(4_u64);
    }
    if ((size) != (aligned_size)) {
        let mut v: u32 = 0_u32;
        let mut i: u64 = aligned_size;
        'loop_: while ((i) < (size)) {
            v |= ((((*buf.offset((i) as isize)) as i32)
                << ((24_u64).wrapping_sub((8_u64).wrapping_mul(((i) & (3_u64))))))
                as u32);
            i.prefix_inc();
        }
        checksum = (checksum).wrapping_add(v);
    }
    return checksum;
}
pub unsafe fn CollectionHeaderSize_9(mut header_version: u32, mut num_fonts: u32) -> u64 {
    let mut size: u64 = 0_u64;
    if ((header_version) == (131072_u32)) {
        size = (size).wrapping_add(12_u64);
    }
    if ((header_version) == (65536_u32)) || ((header_version) == (131072_u32)) {
        size =
            (size).wrapping_add((((12_u32).wrapping_add((4_u32).wrapping_mul(num_fonts))) as u64));
    }
    return size;
}
// font.rs
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_Font_Table {
    pub tag: u32,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *const u8,
    pub buffer: Vec<u8>,
    pub reuse_of: *mut woff2_Font_Table,
    pub flag_byte: u8,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_Font {
    pub flavor: u32,
    pub num_tables: u16,
    pub tables: BTreeMap<u32, Box<woff2_Font_Table>>,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_FontCollection {
    pub flavor: u32,
    pub header_version: u32,
    pub tables: BTreeMap<u32, Box<*mut woff2_Font_Table>>,
    pub fonts: Vec<woff2_Font>,
}
pub unsafe fn StoreU32_10(mut dst: *mut u8, mut offset: u64, mut x: u32) -> u64 {
    (*dst.offset((offset) as isize)) = (((x) >> (24)) as u8);
    (*dst.offset(((offset).wrapping_add(1_u64)) as isize)) = (((x) >> (16)) as u8);
    (*dst.offset(((offset).wrapping_add(2_u64)) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(3_u64)) as isize)) = (x as u8);
    return (offset).wrapping_add(4_u64);
}
pub unsafe fn Store16_11(mut dst: *mut u8, mut offset: u64, mut x: i32) -> u64 {
    (*dst.offset((offset) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(1_u64)) as isize)) = (x as u8);
    return (offset).wrapping_add(2_u64);
}
pub unsafe fn StoreU32_12(mut val: u32, mut offset: *mut u64, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (24)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (16)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn Store16_13(mut val: i32, mut offset: *mut u64, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn StoreBytes_14(
    mut data: *const u8,
    mut len: u64,
    mut offset: *mut u64,
    mut dst: *mut u8,
) {
    {
        if len != 0 {
            ::std::ptr::copy_nonoverlapping(
                (data as *const u8 as *const ::libc::c_void),
                ((&mut (*dst.offset((*offset) as isize)) as *mut u8) as *mut u8
                    as *mut ::libc::c_void),
                len as usize,
            )
        }
        ((&mut (*dst.offset((*offset) as isize)) as *mut u8) as *mut u8 as *mut ::libc::c_void)
    };
    (*offset) = (*offset).wrapping_add(len);
}
impl woff2_Font {
    pub unsafe fn FindTable_u32(&mut self, mut tag: u32) -> *mut woff2_Font_Table {
        let mut it: UnsafeMapIterator<u32, woff2_Font_Table> = UnsafeMapIterator::find_key(
            &self.tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            &tag,
        );
        return if it
            == UnsafeMapIterator::end(&self.tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
        {
            Default::default()
        } else {
            (&mut *it.second() as *mut woff2_Font_Table)
        };
    }
}
impl woff2_Font {
    pub unsafe fn FindTable_u32_const(&self, mut tag: u32) -> *const woff2_Font_Table {
        let mut it: UnsafeMapIterator<u32, woff2_Font_Table> = UnsafeMapIterator::find_key(
            &self.tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            &tag,
        );
        return if it
            == UnsafeMapIterator::end(&self.tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
        {
            Default::default()
        } else {
            (&*it.second() as *const woff2_Font_Table)
        };
    }
}
impl woff2_Font {
    pub unsafe fn OutputOrderedTags(&self) -> Vec<u32> {
        let mut output_order: Vec<u32> = Vec::new();
        'loop_: for i in
            UnsafeMapIterator::begin(&self.tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
        {
            let table: *const woff2_Font_Table = &*i.second() as *const woff2_Font_Table;
            if ((((*table).tag) & (2155905152_u32)) != 0) {
                continue 'loop_;
            }
            {
                let a0_clone = (*table).tag.clone();
                output_order.push(a0_clone)
            };
        }
        let mut glyf_loc: *mut u32 = {
            let mut it = output_order.as_mut_ptr();
            while it != output_order.as_mut_ptr().add(output_order.len())
                && *it != woff2_kGlyfTableTag
            {
                it = it.add(1);
            }
            it
        };
        let mut loca_loc: *mut u32 = {
            let mut it = output_order.as_mut_ptr();
            while it != output_order.as_mut_ptr().add(output_order.len())
                && *it != woff2_kLocaTableTag
            {
                it = it.add(1);
            }
            it
        };
        if (glyf_loc != output_order.as_mut_ptr().add(output_order.len()))
            && (loca_loc != output_order.as_mut_ptr().add(output_order.len()))
        {
            {
                let pos = loca_loc.offset_from(output_order.as_ptr()) as usize;
                output_order.remove(pos);
                loca_loc
            };
            {
                let pos = {
                    let mut it = output_order.as_mut_ptr();
                    while it != output_order.as_mut_ptr().add(output_order.len())
                        && *it != woff2_kGlyfTableTag
                    {
                        it = it.add(1);
                    }
                    it
                }
                .add(1_i64 as usize)
                .offset_from(output_order.as_ptr()) as usize;
                output_order.insert(pos, woff2_kLocaTableTag);
            };
        }
        return output_order;
    }
}
pub unsafe fn ReadTrueTypeFont_15(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: u64,
    mut font: *mut woff2_Font,
) -> bool {
    if (!(unsafe {
        let _value: *mut u16 = (&mut (*font).num_tables as *mut u16);
        (*file).ReadU16(_value)
    })) || (!(unsafe {
        let _n_bytes: u64 = 6_u64;
        (*file).Skip(_n_bytes)
    })) {
        return false;
    }
    let mut intervals: BTreeMap<u32, Box<u32>> = BTreeMap::new();
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < ((*font).num_tables as i32)) {
        let mut table: woff2_Font_Table = <woff2_Font_Table>::default();
        table.flag_byte = 0_u8;
        table.reuse_of = Default::default();
        if (((!(unsafe {
            let _value: *mut u32 = (&mut table.tag as *mut u32);
            (*file).ReadU32(_value)
        })) || (!(unsafe {
            let _value: *mut u32 = (&mut table.checksum as *mut u32);
            (*file).ReadU32(_value)
        }))) || (!(unsafe {
            let _value: *mut u32 = (&mut table.offset as *mut u32);
            (*file).ReadU32(_value)
        }))) || (!(unsafe {
            let _value: *mut u32 = (&mut table.length as *mut u32);
            (*file).ReadU32(_value)
        })) {
            return false;
        }
        if ((((table.offset) & (3_u32)) != (0_u32)) || ((table.length as u64) > (len)))
            || (((len).wrapping_sub((table.length as u64))) < (table.offset as u64))
        {
            return false;
        }
        (*intervals.entry(table.offset).or_default().as_mut()) = table.length;
        table.data = data.offset((table.offset) as isize);
        if UnsafeMapIterator::find_key(
            &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            &table.tag,
        ) != UnsafeMapIterator::end(
            &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
        ) {
            return false;
        }
        (*(*font).tables.entry(table.tag).or_default().as_mut()) = (table).clone();
        i.prefix_inc();
    }
    let mut last_offset: u32 = (((12_u64 as u64)
        .wrapping_add((16_u64 as u64).wrapping_mul(((*font).num_tables as u64))))
        as u32);
    'loop_: for i in UnsafeMapIterator::begin(&intervals as *const BTreeMap<u32, Box<u32>>) {
        if ((*i.first()) < (last_offset))
            || (((*i.first()).wrapping_add(*i.second())) < (*i.first()))
        {
            return false;
        }
        last_offset = (*i.first()).wrapping_add(*i.second());
    }
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    if ((head_table) != (Default::default())) && (((*head_table).length) < (52_u32)) {
        return false;
    }
    return true;
}
pub unsafe fn ReadCollectionFont_16(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: u64,
    mut font: *mut woff2_Font,
    mut all_tables: *mut BTreeMap<u32, Box<*mut woff2_Font_Table>>,
) -> bool {
    if !(unsafe {
        let _value: *mut u32 = (&mut (*font).flavor as *mut u32);
        (*file).ReadU32(_value)
    }) {
        return false;
    }
    if !(unsafe {
        let _file: *mut woff2_Buffer = file;
        let _data: *const u8 = data;
        let _len: u64 = len;
        let _font: *mut woff2_Font = font;
        ReadTrueTypeFont_15(_file, _data, _len, _font)
    }) {
        return false;
    }
    'loop_: for entry in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let table: *mut woff2_Font_Table = &mut *entry.second() as *mut woff2_Font_Table;
        if UnsafeMapIterator::find_key(
            &(*all_tables) as *const BTreeMap<u32, Box<*mut woff2_Font_Table>>,
            &(*table).offset,
        ) == UnsafeMapIterator::end(
            &(*all_tables) as *const BTreeMap<u32, Box<*mut woff2_Font_Table>>,
        ) {
            (*(*all_tables).entry((*table).offset).or_default().as_mut()) = (unsafe {
                let _tag: u32 = (*table).tag;
                (*font).FindTable_u32(_tag)
            });
        } else {
            (*table).reuse_of = (*(*all_tables).entry((*table).offset).or_default().as_mut());
            if (((*table).tag) != ((*(*table).reuse_of).tag)) {
                return false;
            }
        }
    }
    return true;
}
pub unsafe fn ReadTrueTypeCollection_17(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: u64,
    mut font_collection: *mut woff2_FontCollection,
) -> bool {
    let mut num_fonts: u32 = 0_u32;
    if (!(unsafe {
        let _value: *mut u32 = (&mut (*font_collection).header_version as *mut u32);
        (*file).ReadU32(_value)
    })) || (!(unsafe {
        let _value: *mut u32 = (&mut num_fonts as *mut u32);
        (*file).ReadU32(_value)
    })) {
        return false;
    }
    let mut offsets: Vec<u32> = Vec::new();
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (num_fonts as u64)) {
        let mut offset: u32 = 0_u32;
        if !(unsafe {
            let _value: *mut u32 = (&mut offset as *mut u32);
            (*file).ReadU32(_value)
        }) {
            return false;
        }
        {
            let a0_clone = offset.clone();
            offsets.push(a0_clone)
        };
        i.postfix_inc();
    }
    {
        let __a0 = offsets.len() as u64 as usize;
        (*font_collection)
            .fonts
            .resize_with(__a0, || <woff2_Font>::default())
    };
    let mut font_it: *mut woff2_Font = (*font_collection).fonts.as_mut_ptr();
    let mut all_tables: BTreeMap<u32, Box<*mut woff2_Font_Table>> = BTreeMap::new();
    'loop_: for offset in 0..(offsets.len()) {
        let offset = offsets[offset].clone();
        if !(unsafe {
            let _newoffset: u64 = (offset as u64);
            (*file).set_offset(_newoffset)
        }) {
            return false;
        }
        let font: *mut woff2_Font = &mut (*font_it.postfix_inc()) as *mut woff2_Font;
        if !(unsafe {
            let _file: *mut woff2_Buffer = file;
            let _data: *const u8 = data;
            let _len: u64 = len;
            let _font: *mut woff2_Font = (font);
            let _all_tables: *mut BTreeMap<u32, Box<*mut woff2_Font_Table>> =
                (&mut all_tables as *mut BTreeMap<u32, Box<*mut woff2_Font_Table>>);
            ReadCollectionFont_16(_file, _data, _len, _font, _all_tables)
        }) {
            return false;
        }
    }
    return true;
}
pub unsafe fn ReadFont_18(mut data: *const u8, mut len: u64, mut font: *mut woff2_Font) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    if !(unsafe {
        let _value: *mut u32 = (&mut (*font).flavor as *mut u32);
        file.ReadU32(_value)
    }) {
        return false;
    }
    if (((*font).flavor) == (woff2_kTtcFontFlavor)) {
        return false;
    }
    return (unsafe {
        let _file: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
        let _data: *const u8 = data;
        let _len: u64 = len;
        let _font: *mut woff2_Font = font;
        ReadTrueTypeFont_15(_file, _data, _len, _font)
    });
}
pub unsafe fn ReadFontCollection_19(
    mut data: *const u8,
    mut len: u64,
    mut font_collection: *mut woff2_FontCollection,
) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    if !(unsafe {
        let _value: *mut u32 = (&mut (*font_collection).flavor as *mut u32);
        file.ReadU32(_value)
    }) {
        return false;
    }
    if (((*font_collection).flavor) != (woff2_kTtcFontFlavor)) {
        {
            let __a0 = 1_u64 as usize;
            (*font_collection)
                .fonts
                .resize_with(__a0, || <woff2_Font>::default())
        };
        let font: *mut woff2_Font =
            &mut (&mut (*font_collection)).fonts[(0_u64) as usize] as *mut woff2_Font;
        (*font).flavor = (*font_collection).flavor;
        return (unsafe {
            let _file: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
            let _data: *const u8 = data;
            let _len: u64 = len;
            let _font: *mut woff2_Font = (font);
            ReadTrueTypeFont_15(_file, _data, _len, _font)
        });
    }
    return (unsafe {
        let _file: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
        let _data: *const u8 = data;
        let _len: u64 = len;
        let _font_collection: *mut woff2_FontCollection = font_collection;
        ReadTrueTypeCollection_17(_file, _data, _len, _font_collection)
    });
}
pub unsafe fn FontFileSize_20(font: *const woff2_Font) -> u64 {
    let mut max_offset: u64 =
        (12_u64 as u64).wrapping_add((16_u64 as u64).wrapping_mul(((*font).num_tables as u64)));
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let table: *const woff2_Font_Table = &*i.second() as *const woff2_Font_Table;
        let mut padding_size: u64 =
            ((((4_u32).wrapping_sub((((*table).length) & (3_u32)))) & (3_u32)) as u64);
        let mut end_offset: u64 = ((padding_size).wrapping_add(((*table).offset as u64)))
            .wrapping_add(((*table).length as u64));
        max_offset = (*if *&mut max_offset >= *&mut end_offset {
            (&mut max_offset) as *const _
        } else {
            (&mut end_offset) as *const _
        });
    }
    return max_offset;
}
pub unsafe fn FontCollectionFileSize_21(font_collection: *const woff2_FontCollection) -> u64 {
    let mut max_offset: u64 = 0_u64;
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_ptr().add(font);
        max_offset = {
            let mut __tmp_1 = (unsafe {
                let _font: *const woff2_Font = font;
                FontFileSize_20(_font)
            });
            (*if *&mut max_offset >= *&mut __tmp_1 {
                (&mut max_offset) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        };
    }
    return max_offset;
}
pub unsafe fn WriteFont_22(font: *const woff2_Font, mut dst: *mut u8, mut dst_size: u64) -> bool {
    let mut offset: u64 = 0_u64;
    return (unsafe {
        let _font: *const woff2_Font = font;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = dst;
        let _dst_size: u64 = dst_size;
        WriteFont_23(_font, _offset, _dst, _dst_size)
    });
}
pub unsafe fn WriteTableRecord_24(
    mut table: *const woff2_Font_Table,
    mut offset: *mut u64,
    mut dst: *mut u8,
    mut dst_size: u64,
) -> bool {
    if ((dst_size) < ((*offset).wrapping_add(woff2_kSfntEntrySize))) {
        return false;
    }
    if (unsafe { (*table).IsReused() }) {
        table = (*table).reuse_of.cast_const();
    }
    (unsafe {
        let _val: u32 = (*table).tag;
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = (*table).checksum;
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = (*table).offset;
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = (*table).length;
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    return true;
}
pub unsafe fn WriteTable_25(
    table: *const woff2_Font_Table,
    mut offset: *mut u64,
    mut dst: *mut u8,
    mut dst_size: u64,
) -> bool {
    if !(unsafe {
        let _table: *const woff2_Font_Table = (table);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        let _dst_size: u64 = dst_size;
        WriteTableRecord_24(_table, _offset, _dst, _dst_size)
    }) {
        return false;
    }
    if !(unsafe { (*table).IsReused() }) {
        if ((((*table).offset).wrapping_add((*table).length)) < ((*table).offset))
            || ((dst_size) < ((((*table).offset).wrapping_add((*table).length)) as u64))
        {
            return false;
        }
        {
            if ((*table).length as u64) != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((*table).data as *const u8 as *const ::libc::c_void),
                    (dst.offset(((*table).offset) as isize) as *mut u8 as *mut ::libc::c_void),
                    ((*table).length as u64) as usize,
                )
            }
            (dst.offset(((*table).offset) as isize) as *mut u8 as *mut ::libc::c_void)
        };
        let mut padding_size: u64 =
            ((((4_u32).wrapping_sub((((*table).length) & (3_u32)))) & (3_u32)) as u64);
        if ((((((*table).offset).wrapping_add((*table).length)) as u64).wrapping_add(padding_size))
            < (padding_size))
            || ((dst_size)
                < (((((*table).offset).wrapping_add((*table).length)) as u64)
                    .wrapping_add(padding_size)))
        {
            return false;
        }
        {
            let byte_0 = (dst
                .offset(((*table).offset) as isize)
                .offset(((*table).length) as isize) as *mut u8
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..padding_size {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            (dst.offset(((*table).offset) as isize)
                .offset(((*table).length) as isize) as *mut u8 as *mut ::libc::c_void)
        };
    }
    return true;
}
pub unsafe fn WriteFont_23(
    font: *const woff2_Font,
    mut offset: *mut u64,
    mut dst: *mut u8,
    mut dst_size: u64,
) -> bool {
    if ((dst_size)
        < ((12_u64 as u64).wrapping_add((16_u64 as u64).wrapping_mul(((*font).num_tables as u64)))))
    {
        return false;
    }
    (unsafe {
        let _val: u32 = (*font).flavor;
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = ((*font).num_tables as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    let mut max_pow2: u16 = (if ((*font).num_tables != 0) {
        (unsafe {
            let _n: u32 = ((*font).num_tables as u32);
            Log2Floor_7(_n)
        })
    } else {
        0
    } as u16);
    let mut search_range: u16 = (if (max_pow2 != 0) {
        ((1) << ((max_pow2 as i32) + (4)))
    } else {
        0
    } as u16);
    let mut range_shift: u16 =
        (((((*font).num_tables as i32) << (4)) - (search_range as i32)) as u16);
    (unsafe {
        let _val: i32 = (search_range as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = (max_pow2 as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = (range_shift as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        if !(unsafe {
            let _table: *const woff2_Font_Table = &*i.second() as *const woff2_Font_Table;
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            let _dst_size: u64 = dst_size;
            WriteTable_25(_table, _offset, _dst, _dst_size)
        }) {
            return false;
        }
    }
    return true;
}
pub unsafe fn WriteFontCollection_26(
    font_collection: *const woff2_FontCollection,
    mut dst: *mut u8,
    mut dst_size: u64,
) -> bool {
    let mut offset: u64 = 0_u64;
    if (((*font_collection).flavor) != (woff2_kTtcFontFlavor)) {
        return (unsafe {
            let _font: *const woff2_Font =
                &(&(*font_collection)).fonts[(0_u64) as usize] as *const woff2_Font;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            let _dst_size: u64 = dst_size;
            WriteFont_23(_font, _offset, _dst, _dst_size)
        });
    }
    (unsafe {
        let _val: u32 = woff2_kTtcFontFlavor;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = (*font_collection).header_version;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = ((*font_collection).fonts.len() as u64 as u32);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = dst;
        StoreU32_12(_val, _offset, _dst)
    });
    let mut offset_table: u64 = offset;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*font_collection).fonts.len() as u64)) {
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
        i.postfix_inc();
    }
    if (((*font_collection).header_version) == (131072_u32)) {
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*font_collection).fonts.len() as u64)) {
        let font: *const woff2_Font =
            &(&(*font_collection)).fonts[(i) as usize] as *const woff2_Font;
        (unsafe {
            let _val: u32 = (offset as u32);
            let _offset: *mut u64 = (&mut offset_table as *mut u64);
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
        if !(unsafe {
            let _font: *const woff2_Font = font;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            let _dst_size: u64 = dst_size;
            WriteFont_23(_font, _offset, _dst, _dst_size)
        }) {
            return false;
        }
        i.postfix_inc();
    }
    return true;
}
pub unsafe fn NumGlyphs_27(font: *const woff2_Font) -> i32 {
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    let mut loca_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kLocaTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    if (((head_table) == (Default::default())) || ((loca_table) == (Default::default())))
        || (((*head_table).length) < (52_u32))
    {
        return 0;
    }
    let mut index_fmt: i32 = (unsafe {
        let _font: *const woff2_Font = font;
        IndexFormat_28(_font)
    });
    let mut loca_record_size: i32 = (if ((index_fmt) == (0)) { 2 } else { 4 });
    if (((*loca_table).length) < (loca_record_size as u32)) {
        return 0;
    }
    return (((((*loca_table).length).wrapping_div((loca_record_size as u32))).wrapping_sub(1_u32))
        as i32);
}
pub unsafe fn IndexFormat_28(font: *const woff2_Font) -> i32 {
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    if ((head_table) == (Default::default())) {
        return 0;
    }
    return ((*(*head_table).data.offset((51) as isize)) as i32);
}
impl woff2_Font_Table {
    pub unsafe fn IsReused(&self) -> bool {
        return ((self.reuse_of) != (Default::default()));
    }
}
pub unsafe fn GetGlyphData_29(
    font: *const woff2_Font,
    mut glyph_index: i32,
    mut glyph_data: *mut *const u8,
    mut glyph_size: *mut u64,
) -> bool {
    if ((glyph_index) < (0)) {
        return false;
    }
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    let mut loca_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kLocaTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    let mut glyf_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kGlyfTableTag;
        (*font).FindTable_u32_const(_tag)
    });
    if ((((head_table) == (Default::default())) || ((loca_table) == (Default::default())))
        || ((glyf_table) == (Default::default())))
        || (((*head_table).length) < (52_u32))
    {
        return false;
    }
    let mut index_fmt: i32 = (unsafe {
        let _font: *const woff2_Font = font;
        IndexFormat_28(_font)
    });
    let mut loca_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*loca_table).data, ((*loca_table).length as u64));
    if ((index_fmt) == (0)) {
        let mut offset1: u16 = 0_u16;
        let mut offset2: u16 = 0_u16;
        if ((((!(unsafe {
            let _n_bytes: u64 = (((2) * (glyph_index)) as u64);
            loca_buf.Skip(_n_bytes)
        })) || (!(unsafe {
            let _value: *mut u16 = (&mut offset1 as *mut u16);
            loca_buf.ReadU16(_value)
        }))) || (!(unsafe {
            let _value: *mut u16 = (&mut offset2 as *mut u16);
            loca_buf.ReadU16(_value)
        }))) || ((offset2 as i32) < (offset1 as i32)))
            || ((((2) * (offset2 as i32)) as u32) > ((*glyf_table).length))
        {
            return false;
        }
        (*glyph_data) = (*glyf_table).data.offset(((2) * (offset1 as i32)) as isize);
        (*glyph_size) = (((2) * ((offset2 as i32) - (offset1 as i32))) as u64);
    } else {
        let mut offset1: u32 = 0_u32;
        let mut offset2: u32 = 0_u32;
        if ((((!(unsafe {
            let _n_bytes: u64 = (((4) * (glyph_index)) as u64);
            loca_buf.Skip(_n_bytes)
        })) || (!(unsafe {
            let _value: *mut u32 = (&mut offset1 as *mut u32);
            loca_buf.ReadU32(_value)
        }))) || (!(unsafe {
            let _value: *mut u32 = (&mut offset2 as *mut u32);
            loca_buf.ReadU32(_value)
        }))) || ((offset2) < (offset1)))
            || ((offset2) > ((*glyf_table).length))
        {
            return false;
        }
        (*glyph_data) = (*glyf_table).data.offset((offset1) as isize);
        (*glyph_size) = (((offset2).wrapping_sub(offset1)) as u64);
    }
    return true;
}
pub unsafe fn RemoveDigitalSignature_30(mut font: *mut woff2_Font) -> bool {
    let mut it: UnsafeMapIterator<u32, woff2_Font_Table> = UnsafeMapIterator::find_key(
        &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
        &woff2_kDsigTableTag,
    );
    if it != UnsafeMapIterator::end(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        UnsafeMapIterator::erase(
            &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            &it.clone(),
        );
        (*font).num_tables = ((*font).tables.len() as u64 as u16).clone();
    }
    return true;
}
// glyph.rs
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_Glyph_Point {
    pub x: i32,
    pub y: i32,
    pub on_curve: bool,
}
#[repr(C)]
#[derive(Clone)]
pub struct woff2_Glyph {
    pub x_min: i16,
    pub x_max: i16,
    pub y_min: i16,
    pub y_max: i16,
    pub instructions_size: u16,
    pub instructions_data: *const u8,
    pub overlap_simple_flag_set: bool,
    pub contours: Vec<Vec<woff2_Glyph_Point>>,
    pub composite_data: *const u8,
    pub composite_data_size: u32,
    pub have_instructions: bool,
}
impl woff2_Glyph {
    pub unsafe fn woff2_Glyph() -> Self {
        let mut this = Self {
            x_min: 0_i16,
            x_max: 0_i16,
            y_min: 0_i16,
            y_max: 0_i16,
            instructions_size: 0_u16,
            instructions_data: Default::default(),
            overlap_simple_flag_set: false,
            contours: Vec::new(),
            composite_data: Default::default(),
            composite_data_size: 0_u32,
            have_instructions: false,
        };
        this
    }
}
impl Default for woff2_Glyph {
    fn default() -> Self {
        unsafe { woff2_Glyph::woff2_Glyph() }
    }
}
pub static woff2_kFLAG_ONCURVE: i32 = 1;
pub static woff2_kFLAG_XSHORT: i32 = ((1) << (1));
pub static woff2_kFLAG_YSHORT: i32 = ((1) << (2));
pub static woff2_kFLAG_REPEAT: i32 = ((1) << (3));
pub static woff2_kFLAG_XREPEATSIGN: i32 = ((1) << (4));
pub static woff2_kFLAG_YREPEATSIGN: i32 = ((1) << (5));
pub static woff2_kFLAG_OVERLAP_SIMPLE: i32 = ((1) << (6));
pub static woff2_kFLAG_ARG_1_AND_2_ARE_WORDS: i32 = ((1) << (0));
pub static woff2_kFLAG_WE_HAVE_A_SCALE: i32 = ((1) << (3));
pub static woff2_kFLAG_MORE_COMPONENTS: i32 = ((1) << (5));
pub static woff2_kFLAG_WE_HAVE_AN_X_AND_Y_SCALE: i32 = ((1) << (6));
pub static woff2_kFLAG_WE_HAVE_A_TWO_BY_TWO: i32 = ((1) << (7));
pub static woff2_kFLAG_WE_HAVE_INSTRUCTIONS: i32 = ((1) << (8));
pub unsafe fn ReadCompositeGlyphData_31(
    mut buffer: *mut woff2_Buffer,
    mut glyph: *mut woff2_Glyph,
) -> bool {
    (*glyph).have_instructions = false;
    (*glyph).composite_data = (unsafe { (*buffer.cast_const()).buffer() })
        .offset((unsafe { (*buffer.cast_const()).offset() }) as isize);
    let mut start_offset: u64 = (unsafe { (*buffer.cast_const()).offset() });
    let mut flags: u16 = (woff2_kFLAG_MORE_COMPONENTS as u16);
    'loop_: while (((flags as i32) & (woff2_kFLAG_MORE_COMPONENTS)) != 0) {
        if !(unsafe {
            let _value: *mut u16 = (&mut flags as *mut u16);
            (*buffer).ReadU16(_value)
        }) {
            return false;
        }
        (*glyph).have_instructions = (((*glyph).have_instructions as i32)
            | ((((flags as i32) & (woff2_kFLAG_WE_HAVE_INSTRUCTIONS)) != (0)) as i32))
            != 0;
        let mut arg_size: u64 = 2_u64;
        if (((flags as i32) & (woff2_kFLAG_ARG_1_AND_2_ARE_WORDS)) != 0) {
            arg_size = (arg_size).wrapping_add(4_u64);
        } else {
            arg_size = (arg_size).wrapping_add(2_u64);
        }
        if (((flags as i32) & (woff2_kFLAG_WE_HAVE_A_SCALE)) != 0) {
            arg_size = (arg_size).wrapping_add(2_u64);
        } else if (((flags as i32) & (woff2_kFLAG_WE_HAVE_AN_X_AND_Y_SCALE)) != 0) {
            arg_size = (arg_size).wrapping_add(4_u64);
        } else if (((flags as i32) & (woff2_kFLAG_WE_HAVE_A_TWO_BY_TWO)) != 0) {
            arg_size = (arg_size).wrapping_add(8_u64);
        }
        if !(unsafe {
            let _n_bytes: u64 = arg_size;
            (*buffer).Skip(_n_bytes)
        }) {
            return false;
        }
    }
    if (((unsafe { (*buffer.cast_const()).offset() }).wrapping_sub(start_offset))
        > (<u32>::MAX as u64))
    {
        return false;
    }
    (*glyph).composite_data_size =
        (((unsafe { (*buffer.cast_const()).offset() }).wrapping_sub(start_offset)) as u32);
    return true;
}
pub unsafe fn ReadGlyph_32(mut data: *const u8, mut len: u64, mut glyph: *mut woff2_Glyph) -> bool {
    let mut buffer: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    let mut num_contours: i16 = 0_i16;
    if !(unsafe {
        let _value: *mut i16 = (&mut num_contours as *mut i16);
        buffer.ReadS16(_value)
    }) {
        return false;
    }
    if (((!(unsafe {
        let _value: *mut i16 = (&mut (*glyph).x_min as *mut i16);
        buffer.ReadS16(_value)
    })) || (!(unsafe {
        let _value: *mut i16 = (&mut (*glyph).y_min as *mut i16);
        buffer.ReadS16(_value)
    }))) || (!(unsafe {
        let _value: *mut i16 = (&mut (*glyph).x_max as *mut i16);
        buffer.ReadS16(_value)
    }))) || (!(unsafe {
        let _value: *mut i16 = (&mut (*glyph).y_max as *mut i16);
        buffer.ReadS16(_value)
    })) {
        return false;
    }
    if ((num_contours as i32) == (0)) {
        return true;
    }
    if ((num_contours as i32) > (0)) {
        (*glyph)
            .contours
            .resize_with((num_contours as u64) as usize, || {
                <Vec<woff2_Glyph_Point>>::default()
            });
        let mut last_point_index: u16 = 0_u16;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut point_index: u16 = 0_u16;
            if !(unsafe {
                let _value: *mut u16 = (&mut point_index as *mut u16);
                buffer.ReadU16(_value)
            }) {
                return false;
            }
            let mut num_points: u16 = ((((point_index as i32) - (last_point_index as i32))
                + (if ((i) == (0)) { 1 } else { 0 }))
                as u16);
            {
                let __a0 = (num_points as u64) as usize;
                (&mut (*glyph)).contours[(i as u64) as usize]
                    .resize_with(__a0, || <woff2_Glyph_Point>::default())
            };
            last_point_index = point_index;
            i.prefix_inc();
        }
        if !(unsafe {
            let _value: *mut u16 = (&mut (*glyph).instructions_size as *mut u16);
            buffer.ReadU16(_value)
        }) {
            return false;
        }
        (*glyph).instructions_data = data.offset((unsafe { buffer.offset() }) as isize);
        if !(unsafe {
            let _n_bytes: u64 = ((*glyph).instructions_size as u64);
            buffer.Skip(_n_bytes)
        }) {
            return false;
        }
        let mut flags: Vec<Vec<u8>> = (0..(num_contours as u64) as usize)
            .map(|_| <Vec<u8>>::default())
            .collect::<Vec<_>>();
        let mut flag: u8 = 0_u8;
        let mut flag_repeat: u8 = 0_u8;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            {
                let __a0 = (&mut (*glyph)).contours[(i as u64) as usize].len() as u64 as usize;
                flags[(i as u64) as usize].resize_with(__a0, || <u8>::default())
            };
            let mut j: u64 = 0_u64;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as u64) as usize].len() as u64)) {
                if ((flag_repeat as i32) == (0)) {
                    if !(unsafe {
                        let _value: *mut u8 = (&mut flag as *mut u8);
                        buffer.ReadU8(_value)
                    }) {
                        return false;
                    }
                    if (((flag as i32) & (woff2_kFLAG_REPEAT)) != 0) {
                        if !(unsafe {
                            let _value: *mut u8 = (&mut flag_repeat as *mut u8);
                            buffer.ReadU8(_value)
                        }) {
                            return false;
                        }
                    }
                } else {
                    flag_repeat.postfix_dec();
                }
                flags[(i as u64) as usize][(j) as usize] = flag;
                (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].on_curve =
                    (((flag as i32) & (woff2_kFLAG_ONCURVE)) != 0);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        if (!flags.is_empty()) && (!flags[(0_u64) as usize].is_empty()) {
            (*glyph).overlap_simple_flag_set = (((flags[(0_u64) as usize][(0_u64) as usize]
                as i32)
                & (woff2_kFLAG_OVERLAP_SIMPLE))
                != 0);
        }
        let mut prev_x: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut j: u64 = 0_u64;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as u64) as usize].len() as u64)) {
                let mut flag: u8 = flags[(i as u64) as usize][(j) as usize];
                if (((flag as i32) & (woff2_kFLAG_XSHORT)) != 0) {
                    let mut x_delta: u8 = 0_u8;
                    if !(unsafe {
                        let _value: *mut u8 = (&mut x_delta as *mut u8);
                        buffer.ReadU8(_value)
                    }) {
                        return false;
                    }
                    let mut sign: i32 = if (((flag as i32) & (woff2_kFLAG_XREPEATSIGN)) != 0) {
                        1
                    } else {
                        -1_i32
                    };
                    (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].x =
                        ((prev_x) + ((sign) * (x_delta as i32)));
                } else {
                    let mut x_delta: i16 = 0_i16;
                    if !(((flag as i32) & (woff2_kFLAG_XREPEATSIGN)) != 0) {
                        if !(unsafe {
                            let _value: *mut i16 = (&mut x_delta as *mut i16);
                            buffer.ReadS16(_value)
                        }) {
                            return false;
                        }
                    }
                    (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].x =
                        ((prev_x) + (x_delta as i32));
                }
                prev_x = (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].x;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut prev_y: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut j: u64 = 0_u64;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as u64) as usize].len() as u64)) {
                let mut flag: u8 = flags[(i as u64) as usize][(j) as usize];
                if (((flag as i32) & (woff2_kFLAG_YSHORT)) != 0) {
                    let mut y_delta: u8 = 0_u8;
                    if !(unsafe {
                        let _value: *mut u8 = (&mut y_delta as *mut u8);
                        buffer.ReadU8(_value)
                    }) {
                        return false;
                    }
                    let mut sign: i32 = if (((flag as i32) & (woff2_kFLAG_YREPEATSIGN)) != 0) {
                        1
                    } else {
                        -1_i32
                    };
                    (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].y =
                        ((prev_y) + ((sign) * (y_delta as i32)));
                } else {
                    let mut y_delta: i16 = 0_i16;
                    if !(((flag as i32) & (woff2_kFLAG_YREPEATSIGN)) != 0) {
                        if !(unsafe {
                            let _value: *mut i16 = (&mut y_delta as *mut i16);
                            buffer.ReadS16(_value)
                        }) {
                            return false;
                        }
                    }
                    (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].y =
                        ((prev_y) + (y_delta as i32));
                }
                prev_y = (&mut (*glyph)).contours[(i as u64) as usize][(j) as usize].y;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    } else if ((num_contours as i32) == (-1_i32)) {
        if !(unsafe {
            let _buffer: *mut woff2_Buffer = (&mut buffer as *mut woff2_Buffer);
            let _glyph: *mut woff2_Glyph = glyph;
            ReadCompositeGlyphData_31(_buffer, _glyph)
        }) {
            return false;
        }
        if (*glyph).have_instructions {
            if !(unsafe {
                let _value: *mut u16 = (&mut (*glyph).instructions_size as *mut u16);
                buffer.ReadU16(_value)
            }) {
                return false;
            }
            (*glyph).instructions_data = data.offset((unsafe { buffer.offset() }) as isize);
            if !(unsafe {
                let _n_bytes: u64 = ((*glyph).instructions_size as u64);
                buffer.Skip(_n_bytes)
            }) {
                return false;
            }
        } else {
            (*glyph).instructions_size = 0_u16;
        }
    } else {
        return false;
    }
    return true;
}
pub unsafe fn StoreBbox_33(glyph: *const woff2_Glyph, mut offset: *mut u64, mut dst: *mut u8) {
    (unsafe {
        let _val: i32 = ((*glyph).x_min as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = ((*glyph).y_min as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = ((*glyph).x_max as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = ((*glyph).y_max as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
}
pub unsafe fn StoreInstructions_34(
    glyph: *const woff2_Glyph,
    mut offset: *mut u64,
    mut dst: *mut u8,
) {
    (unsafe {
        let _val: i32 = ((*glyph).instructions_size as i32);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _data: *const u8 = (*glyph).instructions_data;
        let _len: u64 = ((*glyph).instructions_size as u64);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreBytes_14(_data, _len, _offset, _dst)
    });
}
pub unsafe fn StoreEndPtsOfContours_35(
    glyph: *const woff2_Glyph,
    mut offset: *mut u64,
    mut dst: *mut u8,
) -> bool {
    let mut end_point: i32 = -1_i32;
    'loop_: for contour in 0..((*glyph).contours.len()) {
        let mut contour = (*glyph).contours.as_ptr().add(contour);
        end_point = ((end_point as u64).wrapping_add((*contour).len() as u64)) as i32;
        if (((*contour).len() as u64) > (<u16>::MAX as u64)) || ((end_point) > (<u16>::MAX as i32))
        {
            return false;
        }
        (unsafe {
            let _val: i32 = end_point;
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            Store16_13(_val, _offset, _dst)
        });
    }
    return true;
}
pub unsafe fn StorePoints_36(
    glyph: *const woff2_Glyph,
    mut offset: *mut u64,
    mut dst: *mut u8,
    mut dst_size: u64,
) -> bool {
    let mut previous_flag: i32 = -1_i32;
    let mut repeat_count: i32 = 0;
    let mut last_x: i32 = 0;
    let mut last_y: i32 = 0;
    let mut x_bytes: u64 = 0_u64;
    let mut y_bytes: u64 = 0_u64;
    'loop_: for contour in 0..((*glyph).contours.len()) {
        let mut contour = (*glyph).contours.as_ptr().add(contour);
        'loop_: for point in 0..((*contour).len()) {
            let mut point = (*contour).as_ptr().add(point);
            let mut flag: i32 = if (*point).on_curve {
                woff2_kFLAG_ONCURVE
            } else {
                0
            };
            if ((previous_flag) == (-1_i32)) && ((*glyph).overlap_simple_flag_set) {
                flag = ((flag) | (woff2_kFLAG_OVERLAP_SIMPLE));
            }
            let mut dx: i32 = (((*point).x) - (last_x));
            let mut dy: i32 = (((*point).y) - (last_y));
            if ((dx) == (0)) {
                flag |= woff2_kFLAG_XREPEATSIGN;
            } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
                flag |= ((woff2_kFLAG_XSHORT)
                    | (if ((dx) > (0)) {
                        woff2_kFLAG_XREPEATSIGN
                    } else {
                        0
                    }));
                x_bytes = (x_bytes).wrapping_add(1_u64);
            } else {
                x_bytes = (x_bytes).wrapping_add(2_u64);
            }
            if ((dy) == (0)) {
                flag |= woff2_kFLAG_YREPEATSIGN;
            } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
                flag |= ((woff2_kFLAG_YSHORT)
                    | (if ((dy) > (0)) {
                        woff2_kFLAG_YREPEATSIGN
                    } else {
                        0
                    }));
                y_bytes = (y_bytes).wrapping_add(1_u64);
            } else {
                y_bytes = (y_bytes).wrapping_add(2_u64);
            }
            if ((flag) == (previous_flag)) && ((repeat_count) != (255)) {
                (*dst.offset(((*offset).wrapping_sub(1_u64)) as isize)) =
                    (((*dst.offset(((*offset).wrapping_sub(1_u64)) as isize)) as i32)
                        | woff2_kFLAG_REPEAT) as u8;
                repeat_count.postfix_inc();
            } else {
                if ((repeat_count) != (0)) {
                    if ((*offset) >= (dst_size)) {
                        return false;
                    }
                    (*dst.offset(((*offset).postfix_inc()) as isize)) = (repeat_count as u8);
                }
                if ((*offset) >= (dst_size)) {
                    return false;
                }
                (*dst.offset(((*offset).postfix_inc()) as isize)) = (flag as u8);
                repeat_count = 0;
            }
            last_x = (*point).x;
            last_y = (*point).y;
            previous_flag = flag;
        }
    }
    if ((repeat_count) != (0)) {
        if ((*offset) >= (dst_size)) {
            return false;
        }
        (*dst.offset(((*offset).postfix_inc()) as isize)) = (repeat_count as u8);
    }
    if ((((*offset).wrapping_add(x_bytes)).wrapping_add(y_bytes)) > (dst_size)) {
        return false;
    }
    let mut x_offset: u64 = (*offset);
    let mut y_offset: u64 = (*offset).wrapping_add(x_bytes);
    last_x = 0;
    last_y = 0;
    'loop_: for contour in 0..((*glyph).contours.len()) {
        let mut contour = (*glyph).contours.as_ptr().add(contour);
        'loop_: for point in 0..((*contour).len()) {
            let mut point = (*contour).as_ptr().add(point);
            let mut dx: i32 = (((*point).x) - (last_x));
            let mut dy: i32 = (((*point).y) - (last_y));
            if ((dx) == (0)) {
            } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
                (*dst.offset((x_offset.postfix_inc()) as isize)) = (dx.abs() as u8);
            } else {
                (unsafe {
                    let _val: i32 = dx;
                    let _offset: *mut u64 = (&mut x_offset as *mut u64);
                    let _dst: *mut u8 = dst;
                    Store16_13(_val, _offset, _dst)
                });
            }
            if ((dy) == (0)) {
            } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
                (*dst.offset((y_offset.postfix_inc()) as isize)) = (dy.abs() as u8);
            } else {
                (unsafe {
                    let _val: i32 = dy;
                    let _offset: *mut u64 = (&mut y_offset as *mut u64);
                    let _dst: *mut u8 = dst;
                    Store16_13(_val, _offset, _dst)
                });
            }
            last_x += dx;
            last_y += dy;
        }
    }
    (*offset) = y_offset;
    return true;
}
pub unsafe fn StoreGlyph_37(
    glyph: *const woff2_Glyph,
    mut dst: *mut u8,
    mut dst_size: *mut u64,
) -> bool {
    let mut offset: u64 = 0_u64;
    if (((*glyph).composite_data_size) > (0_u32)) {
        if ((*dst_size)
            < (((10_u64 as u64).wrapping_add(((*glyph).composite_data_size as u64))).wrapping_add(
                ((if (*glyph).have_instructions {
                    2_u64
                } else {
                    0_u64
                })
                .wrapping_add(((*glyph).instructions_size as u64))),
            )))
        {
            return false;
        }
        (unsafe {
            let _val: i32 = -1_i32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            Store16_13(_val, _offset, _dst)
        });
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreBbox_33(_glyph, _offset, _dst)
        });
        (unsafe {
            let _data: *const u8 = (*glyph).composite_data;
            let _len: u64 = ((*glyph).composite_data_size as u64);
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreBytes_14(_data, _len, _offset, _dst)
        });
        if (*glyph).have_instructions {
            (unsafe {
                let _glyph: *const woff2_Glyph = glyph;
                let _offset: *mut u64 = (&mut offset as *mut u64);
                let _dst: *mut u8 = dst;
                StoreInstructions_34(_glyph, _offset, _dst)
            });
        }
    } else if (((*glyph).contours.len() as u64) > (0_u64)) {
        if (((*glyph).contours.len() as u64) > (<i16>::MAX as u64)) {
            return false;
        }
        if ((*dst_size)
            < (((12_u64 as u64)
                .wrapping_add((2_u64).wrapping_mul((*glyph).contours.len() as u64)))
            .wrapping_add(((*glyph).instructions_size as u64))))
        {
            return false;
        }
        (unsafe {
            let _val: i32 = ((*glyph).contours.len() as u64 as i32);
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            Store16_13(_val, _offset, _dst)
        });
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreBbox_33(_glyph, _offset, _dst)
        });
        if !(unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreEndPtsOfContours_35(_glyph, _offset, _dst)
        }) {
            return false;
        }
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            StoreInstructions_34(_glyph, _offset, _dst)
        });
        if !(unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = dst;
            let _dst_size: u64 = (*dst_size);
            StorePoints_36(_glyph, _offset, _dst, _dst_size)
        }) {
            return false;
        }
    }
    (*dst_size) = offset;
    return true;
}
// normalize.rs
pub unsafe fn Round4_38(mut value: i32) -> i32 {
    if (((<i32>::MAX) - (value)) < (3)) {
        return value;
    }
    return (((value) + (3)) & (!3));
}
pub unsafe fn Round4_39(mut value: u64) -> u64 {
    if (((<u64>::MAX as u64).wrapping_sub(value)) < (3_u64)) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub unsafe fn Round4_40(mut value: u32) -> u32 {
    if (((<u32>::MAX as u32).wrapping_sub(value)) < (3_u32)) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub unsafe fn StoreLoca_41(
    mut index_fmt: i32,
    mut value: u32,
    mut offset: *mut u64,
    mut dst: *mut u8,
) {
    if ((index_fmt) == (0)) {
        (unsafe {
            let _val: i32 = (((value) >> (1)) as i32);
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            Store16_13(_val, _offset, _dst)
        });
    } else {
        (unsafe {
            let _val: u32 = value;
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
    }
}
pub unsafe fn WriteNormalizedLoca_42(
    mut index_fmt: i32,
    mut num_glyphs: i32,
    mut font: *mut woff2_Font,
) -> bool {
    let mut glyf_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kGlyfTableTag;
        (*font).FindTable_u32(_tag)
    });
    let mut loca_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kLocaTableTag;
        (*font).FindTable_u32(_tag)
    });
    let mut glyph_sz: i32 = if ((index_fmt) == (0)) { 2 } else { 4 };
    {
        let __a0 = (((unsafe {
            let _value: i32 = ((num_glyphs) + (1));
            Round4_38(_value)
        }) * (glyph_sz)) as u64) as usize;
        (*loca_table).buffer.resize_with(__a0, || <u8>::default())
    };
    (*loca_table).length = ((((num_glyphs) + (1)) * (glyph_sz)) as u32);
    let mut glyf_dst: *mut u8 = if (num_glyphs != 0) {
        (&mut (&mut (*glyf_table)).buffer[(0_u64) as usize] as *mut u8)
    } else {
        Default::default()
    };
    let mut loca_dst: *mut u8 = (&mut (&mut (*loca_table)).buffer[(0_u64) as usize] as *mut u8);
    let mut glyf_offset: u32 = 0_u32;
    let mut loca_offset: u64 = 0_u64;
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        (unsafe {
            let _index_fmt: i32 = index_fmt;
            let _value: u32 = glyf_offset;
            let _offset: *mut u64 = (&mut loca_offset as *mut u64);
            let _dst: *mut u8 = loca_dst;
            StoreLoca_41(_index_fmt, _value, _offset, _dst)
        });
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = Default::default();
        let mut glyph_size: u64 = 0_u64;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut u64 = (&mut glyph_size as *mut u64);
            GetGlyphData_29(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_u64))
            && (!(unsafe {
                let _data: *const u8 = glyph_data;
                let _len: u64 = glyph_size;
                let _glyph: *mut woff2_Glyph = (&mut glyph as *mut woff2_Glyph);
                ReadGlyph_32(_data, _len, _glyph)
            })))
        {
            return false;
        }
        let mut glyf_dst_size: u64 =
            ((*glyf_table).buffer.len() as u64).wrapping_sub((glyf_offset as u64));
        if !(unsafe {
            let _glyph: *const woff2_Glyph = &glyph as *const woff2_Glyph;
            let _dst: *mut u8 = glyf_dst.offset((glyf_offset) as isize);
            let _dst_size: *mut u64 = (&mut glyf_dst_size as *mut u64);
            StoreGlyph_37(_glyph, _dst, _dst_size)
        }) {
            return false;
        }
        glyf_dst_size = (unsafe {
            let _value: u64 = glyf_dst_size;
            Round4_39(_value)
        });
        if (((glyf_dst_size) > (<u32>::MAX as u64))
            || (((glyf_offset).wrapping_add((glyf_dst_size as u32))) < (glyf_offset)))
            || (((index_fmt) == (0))
                && (((glyf_offset as u64).wrapping_add(glyf_dst_size)) >= ((1_u64) << (17))))
        {
            return false;
        }
        glyf_offset = ((glyf_offset as u64).wrapping_add(glyf_dst_size)) as u32;
        i.prefix_inc();
    }
    (unsafe {
        let _index_fmt: i32 = index_fmt;
        let _value: u32 = glyf_offset;
        let _offset: *mut u64 = (&mut loca_offset as *mut u64);
        let _dst: *mut u8 = loca_dst;
        StoreLoca_41(_index_fmt, _value, _offset, _dst)
    });
    {
        let __a0 = (glyf_offset as u64) as usize;
        (*glyf_table).buffer.resize_with(__a0, || <u8>::default())
    };
    (*glyf_table).data = if (glyf_offset != 0) {
        (&mut (&mut (*glyf_table)).buffer[(0_u64) as usize] as *mut u8)
    } else {
        Default::default()
    }
    .cast_const();
    (*glyf_table).length = glyf_offset;
    (*loca_table).data = if (loca_offset != 0) {
        (&mut (&mut (*loca_table)).buffer[(0_u64) as usize] as *mut u8)
    } else {
        Default::default()
    }
    .cast_const();
    return true;
}
pub unsafe fn MakeEditableBuffer_43(mut font: *mut woff2_Font, mut tableTag: i32) -> bool {
    let mut table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = (tableTag as u32);
        (*font).FindTable_u32(_tag)
    });
    if ((table) == (Default::default())) {
        return false;
    }
    if (unsafe { (*table.cast_const()).IsReused() }) {
        return true;
    }
    let mut sz: i32 = ((unsafe {
        let _value: u32 = (*table).length;
        Round4_40(_value)
    }) as i32);
    {
        let __a0 = (sz as u64) as usize;
        (*table).buffer.resize_with(__a0, || <u8>::default())
    };
    let mut buf: *mut u8 = (&mut (&mut (*table)).buffer[(0_u64) as usize] as *mut u8);
    {
        if ((*table).length as u64) != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((*table).data as *const u8 as *const ::libc::c_void),
                (buf as *mut u8 as *mut ::libc::c_void),
                ((*table).length as u64) as usize,
            )
        }
        (buf as *mut u8 as *mut ::libc::c_void)
    };
    if ((((sz as u32) > ((*table).length)) as i64) != 0) {
        {
            let byte_0 = (buf.offset(((*table).length) as isize) as *mut u8 as *mut ::libc::c_void)
                as *mut u8;
            for offset in 0..(((sz as u32).wrapping_sub((*table).length)) as u64) {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            (buf.offset(((*table).length) as isize) as *mut u8 as *mut ::libc::c_void)
        };
    }
    (*table).data = buf.cast_const();
    return true;
}
pub unsafe fn NormalizeGlyphs_44(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32(_tag)
    });
    let mut glyf_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kGlyfTableTag;
        (*font).FindTable_u32(_tag)
    });
    let mut loca_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kLocaTableTag;
        (*font).FindTable_u32(_tag)
    });
    if ((head_table) == (Default::default())) {
        return false;
    }
    if ((loca_table) == (Default::default())) && ((glyf_table) == (Default::default())) {
        return true;
    }
    if ((((glyf_table) == (Default::default())) as i32)
        != (((loca_table) == (Default::default())) as i32))
    {
        return false;
    }
    if (((unsafe { (*loca_table.cast_const()).IsReused() }) as i32)
        != ((unsafe { (*glyf_table.cast_const()).IsReused() }) as i32))
    {
        return false;
    }
    if (unsafe { (*loca_table.cast_const()).IsReused() }) {
        return true;
    }
    let mut index_fmt: i32 = ((*(*head_table).data.offset((51) as isize)) as i32);
    let mut num_glyphs: i32 = (unsafe {
        let _font: *const woff2_Font = &(*font) as *const woff2_Font;
        NumGlyphs_27(_font)
    });
    let mut max_normalized_glyf_size: u64 =
        ((((1.1E+0) * ((*glyf_table).length as f64)) + (((2) * (num_glyphs)) as f64)) as u64);
    {
        let __a0 = max_normalized_glyf_size as usize;
        (*glyf_table).buffer.resize_with(__a0, || <u8>::default())
    };
    if !(unsafe {
        let _index_fmt: i32 = index_fmt;
        let _num_glyphs: i32 = num_glyphs;
        let _font: *mut woff2_Font = font;
        WriteNormalizedLoca_42(_index_fmt, _num_glyphs, _font)
    }) {
        if ((index_fmt) != (0)) {
            return false;
        }
        index_fmt = 1;
        if !(unsafe {
            let _index_fmt: i32 = index_fmt;
            let _num_glyphs: i32 = num_glyphs;
            let _font: *mut woff2_Font = font;
            WriteNormalizedLoca_42(_index_fmt, _num_glyphs, _font)
        }) {
            return false;
        }
        (&mut (*head_table)).buffer[(51_u64) as usize] = 1_u8;
    }
    return true;
}
pub unsafe fn NormalizeOffsets_45(mut font: *mut woff2_Font) -> bool {
    let mut offset: u32 = (((12) + ((16) * ((*font).num_tables as i32))) as u32);
    'loop_: for tag in 0..((unsafe { (*font.cast_const()).OutputOrderedTags() }).len()) {
        let mut tag = (unsafe { (&(*font.cast_const())).OutputOrderedTags() })[tag].clone();
        let table: *mut woff2_Font_Table =
            &mut (*(*font).tables.entry(tag).or_default().as_mut()) as *mut woff2_Font_Table;
        (*table).offset = offset;
        offset = ((offset as u32).wrapping_add(
            (unsafe {
                let _value: u32 = (*table).length;
                Round4_40(_value)
            }),
        )) as u32;
    }
    return true;
}
pub unsafe fn ComputeHeaderChecksum_46(font: *const woff2_Font) -> u32 {
    let mut checksum: u32 = (*font).flavor;
    let mut max_pow2: u16 = (if ((*font).num_tables != 0) {
        (unsafe {
            let _n: u32 = ((*font).num_tables as u32);
            Log2Floor_7(_n)
        })
    } else {
        0
    } as u16);
    let mut search_range: u16 = (if (max_pow2 != 0) {
        ((1) << ((max_pow2 as i32) + (4)))
    } else {
        0
    } as u16);
    let mut range_shift: u16 =
        (((((*font).num_tables as i32) << (4)) - (search_range as i32)) as u16);
    checksum = (checksum)
        .wrapping_add((((((*font).num_tables as i32) << (16)) | (search_range as i32)) as u32));
    checksum =
        (checksum).wrapping_add(((((max_pow2 as i32) << (16)) | (range_shift as i32)) as u32));
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let mut table: *const woff2_Font_Table = (&*i.second() as *const woff2_Font_Table);
        if (unsafe { (*table).IsReused() }) {
            table = (*table).reuse_of.cast_const();
        }
        checksum = (checksum).wrapping_add((*table).tag);
        checksum = (checksum).wrapping_add((*table).checksum);
        checksum = (checksum).wrapping_add((*table).offset);
        checksum = (checksum).wrapping_add((*table).length);
    }
    return checksum;
}
pub unsafe fn FixChecksums_47(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32(_tag)
    });
    if ((head_table) == (Default::default())) {
        return false;
    }
    if (((*head_table).reuse_of) != (Default::default())) {
        head_table = (*head_table).reuse_of;
    }
    if (((*head_table).length) < (12_u32)) {
        return false;
    }
    let mut head_buf: *mut u8 = (&mut (&mut (*head_table)).buffer[(0_u64) as usize] as *mut u8);
    let mut offset: u64 = 8_u64;
    (unsafe {
        let _val: u32 = 0_u32;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = head_buf;
        StoreU32_12(_val, _offset, _dst)
    });
    let mut file_checksum: u32 = 0_u32;
    let mut head_checksum: u32 = 0_u32;
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let mut table: *mut woff2_Font_Table = (&mut *i.second() as *mut woff2_Font_Table);
        if (unsafe { (*table.cast_const()).IsReused() }) {
            table = (*table).reuse_of;
        }
        (*table).checksum = (unsafe {
            let _buf: *const u8 = (*table).data;
            let _size: u64 = ((*table).length as u64);
            ComputeULongSum_8(_buf, _size)
        });
        file_checksum = (file_checksum).wrapping_add((*table).checksum);
        if (((*table).tag) == (woff2_kHeadTableTag)) {
            head_checksum = (*table).checksum;
        }
    }
    file_checksum = (file_checksum).wrapping_add(
        (unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            ComputeHeaderChecksum_46(_font)
        }),
    );
    offset = 8_u64;
    (unsafe {
        let _val: u32 = (2981146554_u32 as u32).wrapping_sub(file_checksum);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = head_buf;
        StoreU32_12(_val, _offset, _dst)
    });
    return true;
}
pub unsafe fn MarkTransformed_48(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32(_tag)
    });
    if ((head_table) == (Default::default())) {
        return false;
    }
    if (((*head_table).reuse_of) != (Default::default())) {
        head_table = (*head_table).reuse_of;
    }
    if (((*head_table).length) < (17_u32)) {
        return false;
    }
    let mut head_flags: i32 = ((*(*head_table).data.offset((16) as isize)) as i32);
    (&mut (*head_table)).buffer[(16_u64) as usize] = (((head_flags) | (8)) as u8);
    return true;
}
pub unsafe fn NormalizeWithoutFixingChecksums_49(mut font: *mut woff2_Font) -> bool {
    return (((((unsafe {
        let _font: *mut woff2_Font = font;
        let _tableTag: i32 = (woff2_kHeadTableTag as i32);
        MakeEditableBuffer_43(_font, _tableTag)
    }) && (unsafe {
        let _font: *mut woff2_Font = font;
        RemoveDigitalSignature_30(_font)
    })) && (unsafe {
        let _font: *mut woff2_Font = font;
        MarkTransformed_48(_font)
    })) && (unsafe {
        let _font: *mut woff2_Font = font;
        NormalizeGlyphs_44(_font)
    })) && (unsafe {
        let _font: *mut woff2_Font = font;
        NormalizeOffsets_45(_font)
    }));
}
pub unsafe fn NormalizeFont_50(mut font: *mut woff2_Font) -> bool {
    return ((unsafe {
        let _font: *mut woff2_Font = font;
        NormalizeWithoutFixingChecksums_49(_font)
    }) && (unsafe {
        let _font: *mut woff2_Font = font;
        FixChecksums_47(_font)
    }));
}
pub unsafe fn NormalizeFontCollection_51(mut font_collection: *mut woff2_FontCollection) -> bool {
    if (((*font_collection).fonts.len() as u64) == (1_u64)) {
        return (unsafe {
            let _font: *mut woff2_Font =
                (&mut (&mut (*font_collection)).fonts[(0_u64) as usize] as *mut woff2_Font);
            NormalizeFont_50(_font)
        });
    }
    let mut offset: u32 = ((unsafe {
        let _header_version: u32 = (*font_collection).header_version;
        let _num_fonts: u32 = ((*font_collection).fonts.len() as u64 as u32);
        CollectionHeaderSize_9(_header_version, _num_fonts)
    }) as u32);
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe {
            let _font: *mut woff2_Font = (font);
            NormalizeWithoutFixingChecksums_49(_font)
        }) {
            printf(b"Font normalization failed.\n\0".as_ptr() as *const i8);
            return false;
        }
        offset =
            ((offset as u64)
                .wrapping_add((woff2_kSfntHeaderSize).wrapping_add(
                    (woff2_kSfntEntrySize).wrapping_mul(((*font).num_tables as u64)),
                ))) as u32;
    }
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        'loop_: for tag in 0..((unsafe { (*font).OutputOrderedTags() }).len()) {
            let mut tag = (unsafe { (&(*font)).OutputOrderedTags() })[tag].clone();
            let table: *mut woff2_Font_Table =
                &mut (*(*font).tables.entry(tag).or_default().as_mut()) as *mut woff2_Font_Table;
            if (unsafe { (*table).IsReused() }) {
                (*table).offset = (*(*table).reuse_of).offset;
            } else {
                (*table).offset = offset;
                offset = ((offset as u32).wrapping_add(
                    (unsafe {
                        let _value: u32 = (*table).length;
                        Round4_40(_value)
                    }),
                )) as u32;
            }
        }
    }
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe {
            let _font: *mut woff2_Font = (font);
            FixChecksums_47(_font)
        }) {
            printf(b"Failed to fix checksums\n\0".as_ptr() as *const i8);
            return false;
        }
    }
    return true;
}
// transform.rs
pub static woff2_FLAG_ARG_1_AND_2_ARE_WORDS: i32 = ((1) << (0));
pub static woff2_FLAG_WE_HAVE_INSTRUCTIONS: i32 = ((1) << (8));
pub static woff2_FLAG_OVERLAP_SIMPLE_BITMAP: i32 = ((1) << (0));
pub unsafe fn WriteBytes_52(mut out: *mut Vec<u8>, mut data: *const u8, mut len: u64) {
    if ((len) == (0_u64)) {
        return;
    }
    let mut offset: u64 = (*out.cast_const()).len() as u64;
    {
        let __a0 = (offset).wrapping_add(len) as usize;
        (*out).resize_with(__a0, || <u8>::default())
    };
    {
        if len != 0 {
            ::std::ptr::copy_nonoverlapping(
                (data as *const u8 as *const ::libc::c_void),
                ((&mut (&mut (*out))[(offset) as usize] as *mut u8) as *mut u8
                    as *mut ::libc::c_void),
                len as usize,
            )
        }
        ((&mut (&mut (*out))[(offset) as usize] as *mut u8) as *mut u8 as *mut ::libc::c_void)
    };
}
pub unsafe fn WriteBytes_53(mut out: *mut Vec<u8>, in_: *const Vec<u8>) {
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*in_).len() as u64)) {
        {
            let a0_clone = (&(*in_))[(i) as usize].clone();
            (*out).push(a0_clone)
        };
        i.prefix_inc();
    }
}
pub unsafe fn WriteUShort_54(mut out: *mut Vec<u8>, mut value: i32) {
    (*out).push((((value) >> (8)) as u8));
    (*out).push((((value) & (255)) as u8));
}
pub unsafe fn WriteLong_55(mut out: *mut Vec<u8>, mut value: i32) {
    (*out).push(((((value) >> (24)) & (255)) as u8));
    (*out).push(((((value) >> (16)) & (255)) as u8));
    (*out).push(((((value) >> (8)) & (255)) as u8));
    (*out).push((((value) & (255)) as u8));
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_GlyfEncoder {
    n_contour_stream_: Vec<u8>,
    n_points_stream_: Vec<u8>,
    flag_byte_stream_: Vec<u8>,
    composite_stream_: Vec<u8>,
    bbox_bitmap_: Vec<u8>,
    bbox_stream_: Vec<u8>,
    glyph_stream_: Vec<u8>,
    instruction_stream_: Vec<u8>,
    overlap_bitmap_: Vec<u8>,
    n_glyphs_: i32,
}
impl woff2_GlyfEncoder {
    pub unsafe fn woff2_GlyfEncoder(mut num_glyphs: i32) -> Self {
        let mut this = Self {
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
        };
        {
            let __a0 = (((((num_glyphs) + (31)) >> (5)) << (2)) as u64) as usize;
            this.bbox_bitmap_.resize_with(__a0, || <u8>::default())
        };
        this
    }
    pub unsafe fn Encode(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) -> bool {
        if (((*glyph).composite_data_size) > (0_u32)) {
            (unsafe {
                let _glyph_id: i32 = glyph_id;
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteCompositeGlyph(_glyph_id, _glyph)
            });
        } else if (((*glyph).contours.len() as u64) > (0_u64)) {
            (unsafe {
                let _glyph_id: i32 = glyph_id;
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteSimpleGlyph(_glyph_id, _glyph)
            });
        } else {
            (unsafe {
                let _out: *mut Vec<u8> = (&mut self.n_contour_stream_ as *mut Vec<u8>);
                let _value: i32 = 0;
                WriteUShort_54(_out, _value)
            });
        }
        return true;
    }
    pub unsafe fn GetTransformedGlyfBytes(&mut self, mut result: *mut Vec<u8>) {
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = 0;
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = if self.overlap_bitmap_.is_empty() {
                0
            } else {
                woff2_FLAG_OVERLAP_SIMPLE_BITMAP
            };
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = self.n_glyphs_;
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = 0;
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.n_contour_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.n_points_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.flag_byte_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.glyph_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.composite_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (((self.bbox_bitmap_.len() as u64)
                .wrapping_add(self.bbox_stream_.len() as u64))
                as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _value: i32 = (self.instruction_stream_.len() as u64 as i32);
            WriteLong_55(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.n_contour_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.n_points_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.flag_byte_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.glyph_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.composite_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.bbox_bitmap_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.bbox_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.instruction_stream_ as *const Vec<u8>;
            WriteBytes_53(_out, _in)
        });
        if !self.overlap_bitmap_.is_empty() {
            (unsafe {
                let _out: *mut Vec<u8> = result;
                let _in: *const Vec<u8> = &self.overlap_bitmap_ as *const Vec<u8>;
                WriteBytes_53(_out, _in)
            });
        }
    }
    unsafe fn WriteInstructions(&mut self, glyph: *const woff2_Glyph) {
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.glyph_stream_ as *mut Vec<u8>);
            let _value: i32 = ((*glyph).instructions_size as i32);
            Write255UShort_1(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.instruction_stream_ as *mut Vec<u8>);
            let _data: *const u8 = (*glyph).instructions_data;
            let _len: u64 = ((*glyph).instructions_size as u64);
            WriteBytes_52(_out, _data, _len)
        });
    }
    unsafe fn ShouldWriteSimpleGlyphBbox(&mut self, glyph: *const woff2_Glyph) -> bool {
        if ((*glyph).contours.is_empty()) || ((&(*glyph)).contours[(0_u64) as usize].is_empty()) {
            return ((((*glyph).x_min != 0) || ((*glyph).y_min != 0)) || ((*glyph).x_max != 0))
                || ((*glyph).y_max != 0);
        }
        let mut x_min: i16 = ((&(*glyph)).contours[(0_u64) as usize][(0_u64) as usize].x as i16);
        let mut y_min: i16 = ((&(*glyph)).contours[(0_u64) as usize][(0_u64) as usize].y as i16);
        let mut x_max: i16 = x_min;
        let mut y_max: i16 = y_min;
        'loop_: for contour in 0..((*glyph).contours.len()) {
            let mut contour = (*glyph).contours.as_ptr().add(contour);
            'loop_: for point in 0..((*contour).len()) {
                let mut point = (*contour).as_ptr().add(point);
                if (((*point).x) < (x_min as i32)) {
                    x_min = ((*point).x as i16);
                }
                if (((*point).x) > (x_max as i32)) {
                    x_max = ((*point).x as i16);
                }
                if (((*point).y) < (y_min as i32)) {
                    y_min = ((*point).y as i16);
                }
                if (((*point).y) > (y_max as i32)) {
                    y_max = ((*point).y as i16);
                }
            }
        }
        if (((*glyph).x_min as i32) != (x_min as i32)) {
            return true;
        }
        if (((*glyph).y_min as i32) != (y_min as i32)) {
            return true;
        }
        if (((*glyph).x_max as i32) != (x_max as i32)) {
            return true;
        }
        if (((*glyph).y_max as i32) != (y_max as i32)) {
            return true;
        }
        return false;
    }
    unsafe fn WriteSimpleGlyph(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) {
        if (*glyph).overlap_simple_flag_set {
            (unsafe { self.EnsureOverlapBitmap() });
            self.overlap_bitmap_[(((glyph_id) >> (3)) as u64) as usize] =
                ((self.overlap_bitmap_[(((glyph_id) >> (3)) as u64) as usize] as i32)
                    | ((128) >> ((glyph_id) & (7)))) as u8;
        }
        let mut num_contours: i32 = ((*glyph).contours.len() as u64 as i32);
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.n_contour_stream_ as *mut Vec<u8>);
            let _value: i32 = num_contours;
            WriteUShort_54(_out, _value)
        });
        if (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            self.ShouldWriteSimpleGlyphBbox(_glyph)
        }) {
            (unsafe {
                let _glyph_id: i32 = glyph_id;
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteBbox(_glyph_id, _glyph)
            });
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours)) {
            (unsafe {
                let _out: *mut Vec<u8> = (&mut self.n_points_stream_ as *mut Vec<u8>);
                let _value: i32 = ((&(*glyph)).contours[(i as u64) as usize].len() as u64 as i32);
                Write255UShort_1(_out, _value)
            });
            i.postfix_inc();
        }
        let mut lastX: i32 = 0;
        let mut lastY: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours)) {
            let mut num_points: i32 =
                ((&(*glyph)).contours[(i as u64) as usize].len() as u64 as i32);
            let mut j: i32 = 0;
            'loop_: while ((j) < (num_points)) {
                let mut x: i32 = (&(*glyph)).contours[(i as u64) as usize][(j as u64) as usize].x;
                let mut y: i32 = (&(*glyph)).contours[(i as u64) as usize][(j as u64) as usize].y;
                let mut dx: i32 = ((x) - (lastX));
                let mut dy: i32 = ((y) - (lastY));
                (unsafe {
                    let _on_curve: bool =
                        (&(*glyph)).contours[(i as u64) as usize][(j as u64) as usize].on_curve;
                    let _x: i32 = dx;
                    let _y: i32 = dy;
                    self.WriteTriplet(_on_curve, _x, _y)
                });
                lastX = x;
                lastY = y;
                j.postfix_inc();
            }
            i.postfix_inc();
        }
        if ((num_contours) > (0)) {
            (unsafe {
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteInstructions(_glyph)
            });
        }
    }
    unsafe fn WriteCompositeGlyph(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) {
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.n_contour_stream_ as *mut Vec<u8>);
            let _value: i32 = -1_i32;
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _glyph_id: i32 = glyph_id;
            let _glyph: *const woff2_Glyph = glyph;
            self.WriteBbox(_glyph_id, _glyph)
        });
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.composite_stream_ as *mut Vec<u8>);
            let _data: *const u8 = (*glyph).composite_data;
            let _len: u64 = ((*glyph).composite_data_size as u64);
            WriteBytes_52(_out, _data, _len)
        });
        if (*glyph).have_instructions {
            (unsafe {
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteInstructions(_glyph)
            });
        }
    }
    unsafe fn WriteBbox(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) {
        self.bbox_bitmap_[(((glyph_id) >> (3)) as u64) as usize] =
            ((self.bbox_bitmap_[(((glyph_id) >> (3)) as u64) as usize] as i32)
                | ((128) >> ((glyph_id) & (7)))) as u8;
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.bbox_stream_ as *mut Vec<u8>);
            let _value: i32 = ((*glyph).x_min as i32);
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.bbox_stream_ as *mut Vec<u8>);
            let _value: i32 = ((*glyph).y_min as i32);
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.bbox_stream_ as *mut Vec<u8>);
            let _value: i32 = ((*glyph).x_max as i32);
            WriteUShort_54(_out, _value)
        });
        (unsafe {
            let _out: *mut Vec<u8> = (&mut self.bbox_stream_ as *mut Vec<u8>);
            let _value: i32 = ((*glyph).y_max as i32);
            WriteUShort_54(_out, _value)
        });
    }
    unsafe fn WriteTriplet(&mut self, mut on_curve: bool, mut x: i32, mut y: i32) {
        let mut abs_x: i32 = x.abs();
        let mut abs_y: i32 = y.abs();
        let mut on_curve_bit: i32 = if on_curve { 0 } else { 128 };
        let mut x_sign_bit: i32 = if ((x) < (0)) { 0 } else { 1 };
        let mut y_sign_bit: i32 = if ((y) < (0)) { 0 } else { 1 };
        let mut xy_sign_bits: i32 = ((x_sign_bit) + ((2) * (y_sign_bit)));
        if ((x) == (0)) && ((abs_y) < (1280)) {
            self.flag_byte_stream_
                .push(((((on_curve_bit) + (((abs_y) & (3840)) >> (7))) + (y_sign_bit)) as u8));
            self.glyph_stream_.push((((abs_y) & (255)) as u8));
        } else if ((y) == (0)) && ((abs_x) < (1280)) {
            self.flag_byte_stream_.push(
                (((((on_curve_bit) + (10)) + (((abs_x) & (3840)) >> (7))) + (x_sign_bit)) as u8),
            );
            self.glyph_stream_.push((((abs_x) & (255)) as u8));
        } else if ((abs_x) < (65)) && ((abs_y) < (65)) {
            self.flag_byte_stream_.push(
                ((((((on_curve_bit) + (20)) + (((abs_x) - (1)) & (48)))
                    + ((((abs_y) - (1)) & (48)) >> (2)))
                    + (xy_sign_bits)) as u8),
            );
            self.glyph_stream_
                .push(((((((abs_x) - (1)) & (15)) << (4)) | (((abs_y) - (1)) & (15))) as u8));
        } else if ((abs_x) < (769)) && ((abs_y) < (769)) {
            self.flag_byte_stream_.push(
                ((((((on_curve_bit) + (84)) + ((12) * ((((abs_x) - (1)) & (768)) >> (8))))
                    + ((((abs_y) - (1)) & (768)) >> (6)))
                    + (xy_sign_bits)) as u8),
            );
            self.glyph_stream_.push(((((abs_x) - (1)) & (255)) as u8));
            self.glyph_stream_.push(((((abs_y) - (1)) & (255)) as u8));
        } else if ((abs_x) < (4096)) && ((abs_y) < (4096)) {
            self.flag_byte_stream_
                .push(((((on_curve_bit) + (120)) + (xy_sign_bits)) as u8));
            self.glyph_stream_.push((((abs_x) >> (4)) as u8));
            self.glyph_stream_
                .push((((((abs_x) & (15)) << (4)) | ((abs_y) >> (8))) as u8));
            self.glyph_stream_.push((((abs_y) & (255)) as u8));
        } else {
            self.flag_byte_stream_
                .push(((((on_curve_bit) + (124)) + (xy_sign_bits)) as u8));
            self.glyph_stream_.push((((abs_x) >> (8)) as u8));
            self.glyph_stream_.push((((abs_x) & (255)) as u8));
            self.glyph_stream_.push((((abs_y) >> (8)) as u8));
            self.glyph_stream_.push((((abs_y) & (255)) as u8));
        }
    }
    unsafe fn EnsureOverlapBitmap(&mut self) {
        if self.overlap_bitmap_.is_empty() {
            {
                let __a0 = ((((self.n_glyphs_) + (7)) >> (3)) as u64) as usize;
                self.overlap_bitmap_.resize_with(__a0, || <u8>::default())
            };
        }
    }
}
pub unsafe fn TransformGlyfAndLocaTables_56(mut font: *mut woff2_Font) -> bool {
    let mut glyf_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kGlyfTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    let mut loca_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kLocaTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    if ((loca_table) == (Default::default())) && ((glyf_table) == (Default::default())) {
        return true;
    }
    if ((((glyf_table) == (Default::default())) as i32)
        != (((loca_table) == (Default::default())) as i32))
    {
        return false;
    }
    if (((unsafe { (*loca_table).IsReused() }) as i32)
        != ((unsafe { (*glyf_table).IsReused() }) as i32))
    {
        return false;
    }
    if (unsafe { (*loca_table).IsReused() }) {
        return true;
    }
    let mut transformed_glyf: *mut woff2_Font_Table = (&mut (*(*font)
        .tables
        .entry(((woff2_kGlyfTableTag) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut transformed_loca: *mut woff2_Font_Table = (&mut (*(*font)
        .tables
        .entry(((woff2_kLocaTableTag) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut num_glyphs: i32 = (unsafe {
        let _font: *const woff2_Font = &(*font) as *const woff2_Font;
        NumGlyphs_27(_font)
    });
    let mut encoder: woff2_GlyfEncoder = woff2_GlyfEncoder::woff2_GlyfEncoder(num_glyphs);
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = Default::default();
        let mut glyph_size: u64 = 0_u64;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut u64 = (&mut glyph_size as *mut u64);
            GetGlyphData_29(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_u64))
            && (!(unsafe {
                let _data: *const u8 = glyph_data;
                let _len: u64 = glyph_size;
                let _glyph: *mut woff2_Glyph = (&mut glyph as *mut woff2_Glyph);
                ReadGlyph_32(_data, _len, _glyph)
            })))
        {
            return false;
        }
        (unsafe {
            let _glyph_id: i32 = i;
            let _glyph: *const woff2_Glyph = &glyph as *const woff2_Glyph;
            encoder.Encode(_glyph_id, _glyph)
        });
        i.prefix_inc();
    }
    (unsafe {
        let _result: *mut Vec<u8> = (&mut (*transformed_glyf).buffer as *mut Vec<u8>);
        encoder.GetTransformedGlyfBytes(_result)
    });
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHeadTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    if ((head_table) == (Default::default())) || (((*head_table).length) < (52_u32)) {
        return false;
    }
    (&mut (*transformed_glyf)).buffer[(7_u64) as usize] =
        (*(*head_table).data.offset((51) as isize));
    (*transformed_glyf).tag = ((woff2_kGlyfTableTag) ^ (2155905152_u32));
    (*transformed_glyf).length = ((*transformed_glyf).buffer.len() as u64 as u32).clone();
    (*transformed_glyf).data = (*transformed_glyf).buffer.as_mut_ptr().cast_const();
    (*transformed_loca).tag = ((woff2_kLocaTableTag) ^ (2155905152_u32));
    (*transformed_loca).length = 0_u32;
    (*transformed_loca).data = Default::default();
    return true;
}
pub unsafe fn TransformHmtxTable_57(mut font: *mut woff2_Font) -> bool {
    let mut glyf_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kGlyfTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    let mut hmtx_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHmtxTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    let mut hhea_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = woff2_kHheaTableTag;
        (*font).FindTable_u32(_tag)
    })
    .cast_const();
    if ((hmtx_table) == (Default::default())) || ((glyf_table) == (Default::default())) {
        return true;
    }
    if ((hhea_table) == (Default::default())) {
        return false;
    }
    let mut hhea_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*hhea_table).data, ((*hhea_table).length as u64));
    let mut num_hmetrics: u16 = 0_u16;
    if (!(unsafe {
        let _n_bytes: u64 = 34_u64;
        hhea_buf.Skip(_n_bytes)
    })) || (!(unsafe {
        let _value: *mut u16 = (&mut num_hmetrics as *mut u16);
        hhea_buf.ReadU16(_value)
    })) {
        return false;
    }
    if ((num_hmetrics as i32) < (1)) {
        return false;
    }
    let mut num_glyphs: i32 = (unsafe {
        let _font: *const woff2_Font = &(*font) as *const woff2_Font;
        NumGlyphs_27(_font)
    });
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut proportional_lsbs: Vec<i16> = Vec::new();
    let mut monospace_lsbs: Vec<i16> = Vec::new();
    let mut remove_proportional_lsb: bool = true;
    let mut remove_monospace_lsb: bool = (((num_glyphs) - (num_hmetrics as i32)) > (0));
    let mut hmtx_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*hmtx_table).data, ((*hmtx_table).length as u64));
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = Default::default();
        let mut glyph_size: u64 = 0_u64;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut u64 = (&mut glyph_size as *mut u64);
            GetGlyphData_29(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_u64))
            && (!(unsafe {
                let _data: *const u8 = glyph_data;
                let _len: u64 = glyph_size;
                let _glyph: *mut woff2_Glyph = (&mut glyph as *mut woff2_Glyph);
                ReadGlyph_32(_data, _len, _glyph)
            })))
        {
            return false;
        }
        let mut advance_width: u16 = 0_u16;
        let mut lsb: i16 = 0_i16;
        if ((i) < (num_hmetrics as i32)) {
            if !(unsafe {
                let _value: *mut u16 = (&mut advance_width as *mut u16);
                hmtx_buf.ReadU16(_value)
            }) {
                return false;
            }
            if !(unsafe {
                let _value: *mut i16 = (&mut lsb as *mut i16);
                hmtx_buf.ReadS16(_value)
            }) {
                return false;
            }
            if ((glyph_size) > (0_u64)) && ((glyph.x_min as i32) != (lsb as i32)) {
                remove_proportional_lsb = false;
            }
            {
                let a0_clone = advance_width.clone();
                advance_widths.push(a0_clone)
            };
            {
                let a0_clone = lsb.clone();
                proportional_lsbs.push(a0_clone)
            };
        } else {
            if !(unsafe {
                let _value: *mut i16 = (&mut lsb as *mut i16);
                hmtx_buf.ReadS16(_value)
            }) {
                return false;
            }
            if ((glyph_size) > (0_u64)) && ((glyph.x_min as i32) != (lsb as i32)) {
                remove_monospace_lsb = false;
            }
            {
                let a0_clone = lsb.clone();
                monospace_lsbs.push(a0_clone)
            };
        }
        if (!remove_proportional_lsb) && (!remove_monospace_lsb) {
            return true;
        }
        i.postfix_inc();
    }
    let mut transformed_hmtx: *mut woff2_Font_Table = (&mut (*(*font)
        .tables
        .entry(((woff2_kHmtxTableTag) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut flags: u8 = 0_u8;
    let mut transformed_size: u64 =
        (1_u64).wrapping_add((2_u64).wrapping_mul(advance_widths.len() as u64));
    if remove_proportional_lsb {
        flags = ((flags as i32) | 1) as u8;
    } else {
        transformed_size = ((transformed_size as u64)
            .wrapping_add((2_u64).wrapping_mul(proportional_lsbs.len() as u64)))
            as u64;
    }
    if remove_monospace_lsb {
        flags = ((flags as i32) | ((1) << (1))) as u8;
    } else {
        transformed_size = ((transformed_size as u64)
            .wrapping_add((2_u64).wrapping_mul(monospace_lsbs.len() as u64)))
            as u64;
    }
    if transformed_size as usize > (*transformed_hmtx).buffer.capacity() as usize {
        let len_0 = (*transformed_hmtx).buffer.len();
        (*transformed_hmtx)
            .buffer
            .reserve_exact(transformed_size as usize - len_0 as usize);
    };
    let mut out: *mut Vec<u8> = (&mut (*transformed_hmtx).buffer as *mut Vec<u8>);
    (unsafe {
        let _out: *mut Vec<u8> = out;
        let _data: *const u8 = (&mut flags as *mut u8).cast_const();
        let _len: u64 = 1_u64;
        WriteBytes_52(_out, _data, _len)
    });
    'loop_: for advance_width in 0..(advance_widths.len()) {
        let mut advance_width = advance_widths[advance_width].clone();
        (unsafe {
            let _out: *mut Vec<u8> = out;
            let _value: i32 = (advance_width as i32);
            WriteUShort_54(_out, _value)
        });
    }
    if !remove_proportional_lsb {
        'loop_: for lsb in 0..(proportional_lsbs.len()) {
            let mut lsb = proportional_lsbs[lsb].clone();
            (unsafe {
                let _out: *mut Vec<u8> = out;
                let _value: i32 = (lsb as i32);
                WriteUShort_54(_out, _value)
            });
        }
    }
    if !remove_monospace_lsb {
        'loop_: for lsb in 0..(monospace_lsbs.len()) {
            let mut lsb = monospace_lsbs[lsb].clone();
            (unsafe {
                let _out: *mut Vec<u8> = out;
                let _value: i32 = (lsb as i32);
                WriteUShort_54(_out, _value)
            });
        }
    }
    (*transformed_hmtx).tag = ((woff2_kHmtxTableTag) ^ (2155905152_u32));
    (*transformed_hmtx).flag_byte = (((1) << (6)) as u8);
    (*transformed_hmtx).length = ((*transformed_hmtx).buffer.len() as u64 as u32).clone();
    (*transformed_hmtx).data = (*transformed_hmtx).buffer.as_mut_ptr().cast_const();
    return true;
}
// woff2_enc.rs
#[repr(C)]
#[derive(Clone)]
pub struct woff2_WOFF2Params {
    pub extended_metadata: Vec<u8>,
    pub brotli_quality: i32,
    pub allow_transforms: bool,
}
impl woff2_WOFF2Params {
    pub unsafe fn woff2_WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: {
                let s = b"\0".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            },
            brotli_quality: 11,
            allow_transforms: true,
        };
        this
    }
}
impl Default for woff2_WOFF2Params {
    fn default() -> Self {
        unsafe { woff2_WOFF2Params::woff2_WOFF2Params() }
    }
}
pub static woff2_kWoff2HeaderSize: u64 = 48_u64;
pub static woff2_kWoff2EntrySize: u64 = 20_u64;
pub unsafe fn Compress_58(
    mut data: *const u8,
    len: u64,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut mode: ::brotli_sys::BrotliEncoderMode,
    mut quality: i32,
) -> bool {
    let mut compressed_len: u64 = ((*result_len) as u64);
    if ((::brotli_sys::BrotliEncoderCompress(
        quality,
        22,
        mode,
        len as usize,
        data,
        (&mut compressed_len as *mut u64) as *mut usize,
        result,
    )) == (0))
    {
        return false;
    }
    (*result_len) = (compressed_len as u32);
    return true;
}
pub unsafe fn Woff2Compress_59(
    mut data: *const u8,
    len: u64,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut quality: i32,
) -> bool {
    return (unsafe {
        let _data: *const u8 = data;
        let _len: u64 = len;
        let _result: *mut u8 = result;
        let _result_len: *mut u32 = result_len;
        let _mode: ::brotli_sys::BrotliEncoderMode = ::brotli_sys::BROTLI_MODE_FONT;
        let _quality: i32 = quality;
        Compress_58(_data, _len, _result, _result_len, _mode, _quality)
    });
}
pub unsafe fn TextCompress_60(
    mut data: *const u8,
    len: u64,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut quality: i32,
) -> bool {
    return (unsafe {
        let _data: *const u8 = data;
        let _len: u64 = len;
        let _result: *mut u8 = result;
        let _result_len: *mut u32 = result_len;
        let _mode: ::brotli_sys::BrotliEncoderMode = ::brotli_sys::BROTLI_MODE_TEXT;
        let _quality: i32 = quality;
        Compress_58(_data, _len, _result, _result_len, _mode, _quality)
    });
}
pub unsafe fn KnownTableIndex_61(mut tag: u32) -> i32 {
    let mut i: i32 = 0;
    'loop_: while ((i) < (63)) {
        if ((tag) == (woff2_kKnownTags[(i) as usize])) {
            return i;
        }
        i.prefix_inc();
    }
    return 63;
}
pub unsafe fn StoreTableEntry_62(
    table: *const woff2_Table,
    mut offset: *mut u64,
    mut dst: *mut u8,
) {
    let mut flag_byte: u8 = (((((*table).flags) & (192_u32))
        | ((unsafe {
            let _tag: u32 = (*table).tag;
            KnownTableIndex_61(_tag)
        }) as u32)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = flag_byte;
    if (((flag_byte as i32) & (63)) == (63)) {
        (unsafe {
            let _val: u32 = (*table).tag;
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            StoreU32_12(_val, _offset, _dst)
        });
    }
    (unsafe {
        let _len: u64 = ((*table).src_length as u64);
        let _offset: *mut u64 = offset;
        let _dst: *mut u8 = dst;
        StoreBase128_6(_len, _offset, _dst)
    });
    if ((((*table).flags) & (woff2_kWoff2FlagsTransform)) != (0_u32)) {
        (unsafe {
            let _len: u64 = ((*table).transform_length as u64);
            let _offset: *mut u64 = offset;
            let _dst: *mut u8 = dst;
            StoreBase128_6(_len, _offset, _dst)
        });
    }
}
pub unsafe fn TableEntrySize_63(table: *const woff2_Table) -> u64 {
    let mut flag_byte: u8 = ((unsafe {
        let _tag: u32 = (*table).tag;
        KnownTableIndex_61(_tag)
    }) as u8);
    let mut size: u64 = (if (((flag_byte as i32) & (63)) != (63)) {
        1
    } else {
        5
    } as u64);
    size = (size).wrapping_add(
        (unsafe {
            let _n: u64 = ((*table).src_length as u64);
            Base128Size_5(_n)
        }),
    );
    if ((((*table).flags) & (woff2_kWoff2FlagsTransform)) != (0_u32)) {
        size = (size).wrapping_add(
            (unsafe {
                let _n: u64 = ((*table).transform_length as u64);
                Base128Size_5(_n)
            }),
        );
    }
    return size;
}
pub unsafe fn ComputeWoff2Length_64(
    font_collection: *const woff2_FontCollection,
    tables: *const Vec<woff2_Table>,
    mut index_by_tag_offset: BTreeMap<(u32, u32), Box<u16>>,
    mut compressed_data_length: u64,
    mut extended_metadata_length: u64,
) -> u64 {
    let mut size: u64 = woff2_kWoff2HeaderSize;
    'loop_: for table in 0..((*tables).len()) {
        let mut table = (*tables).as_ptr().add(table);
        size = (size).wrapping_add(
            (unsafe {
                let _table: *const woff2_Table = table;
                TableEntrySize_63(_table)
            }),
        );
    }
    if (((*font_collection).flavor) == (woff2_kTtcFontFlavor)) {
        size = (size).wrapping_add(4_u64);
        size = (size).wrapping_add(
            (unsafe {
                let _value: u16 = ((*font_collection).fonts.len() as u64 as u16);
                Size255UShort_0(_value)
            }),
        );
        size = ((size as u64)
            .wrapping_add((4_u64).wrapping_mul((*font_collection).fonts.len() as u64)))
            as u64;
        'loop_: for font in 0..((*font_collection).fonts.len()) {
            let mut font = (*font_collection).fonts.as_ptr().add(font);
            size = (size).wrapping_add(
                (unsafe {
                    let _value: u16 = ((*font).tables.len() as u64 as u16);
                    Size255UShort_0(_value)
                }),
            );
            'loop_: for entry in UnsafeMapIterator::begin(
                &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            ) {
                let table: *const woff2_Font_Table = &*entry.second() as *const woff2_Font_Table;
                if ((((*table).tag) & (2155905152_u32)) != 0) {
                    continue 'loop_;
                }
                let mut tag_offset: (u32, u32) = ((*table).tag.into(), (*table).offset.into());
                let mut table_index: u16 =
                    (*index_by_tag_offset.entry(tag_offset).or_default().as_mut());
                size = (size).wrapping_add(
                    (unsafe {
                        let _value: u16 = table_index;
                        Size255UShort_0(_value)
                    }),
                );
            }
        }
    }
    size = (size).wrapping_add(compressed_data_length);
    size = (unsafe {
        let _value: u64 = size;
        Round4_39(_value)
    });
    size = (size).wrapping_add(extended_metadata_length);
    return size;
}
pub unsafe fn ComputeUncompressedLength_65(font: *const woff2_Font) -> u64 {
    let mut size: u64 = (((12) + ((16) * ((*font).num_tables as i32))) as u64);
    'loop_: for entry in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let table: *const woff2_Font_Table = &*entry.second() as *const woff2_Font_Table;
        if ((((*table).tag) & (2155905152_u32)) != 0) {
            continue 'loop_;
        }
        if (unsafe { (*table).IsReused() }) {
            continue 'loop_;
        }
        size = (size).wrapping_add(
            ((unsafe {
                let _value: u32 = (*table).length;
                Round4_40(_value)
            }) as u64),
        );
    }
    return size;
}
pub unsafe fn ComputeUncompressedLength_66(font_collection: *const woff2_FontCollection) -> u64 {
    if (((*font_collection).flavor) != (woff2_kTtcFontFlavor)) {
        return (unsafe {
            let _font: *const woff2_Font =
                &(&(*font_collection)).fonts[(0_u64) as usize] as *const woff2_Font;
            ComputeUncompressedLength_65(_font)
        });
    }
    let mut size: u64 = (unsafe {
        let _header_version: u32 = (*font_collection).header_version;
        let _num_fonts: u32 = ((*font_collection).fonts.len() as u64 as u32);
        CollectionHeaderSize_9(_header_version, _num_fonts)
    });
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_ptr().add(font);
        size = (size).wrapping_add(
            (unsafe {
                let _font: *const woff2_Font = font;
                ComputeUncompressedLength_65(_font)
            }),
        );
    }
    return size;
}
pub unsafe fn ComputeTotalTransformLength_67(font: *const woff2_Font) -> u64 {
    let mut total: u64 = 0_u64;
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let table: *const woff2_Font_Table = &*i.second() as *const woff2_Font_Table;
        if (unsafe { (*table).IsReused() }) {
            continue 'loop_;
        }
        if ((((*table).tag) & (2155905152_u32)) != 0)
            || (!!(unsafe {
                let _tag: u32 = (((*table).tag) ^ (2155905152_u32));
                (*font).FindTable_u32_const(_tag)
            })
            .is_null())
        {
            total = (total).wrapping_add(((*table).length as u64));
        }
    }
    return total;
}
pub unsafe fn MaxWOFF2CompressedSize_68(mut data: *const u8, mut length: u64) -> u64 {
    return (unsafe {
        let _data: *const u8 = data;
        let _length: u64 = length;
        let mut _extended_metadata = {
            let s = b"\0".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
        MaxWOFF2CompressedSize_69(_data, _length, &mut _extended_metadata)
    });
}
pub unsafe fn MaxWOFF2CompressedSize_69(
    mut data: *const u8,
    mut length: u64,
    extended_metadata: *const Vec<u8>,
) -> u64 {
    return ((length).wrapping_add(1024_u64)).wrapping_add(((*extended_metadata).len() - 1) as u64);
}
pub unsafe fn CompressedBufferSize_70(mut original_size: u32) -> u32 {
    return ((((1.2E+0) * (original_size as f64)) + (10240_f64)) as u32);
}
pub unsafe fn TransformFontCollection_71(mut font_collection: *mut woff2_FontCollection) -> bool {
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe {
            let _font: *mut woff2_Font = (font);
            TransformGlyfAndLocaTables_56(_font)
        }) {
            printf(b"glyf/loca transformation failed.\n\0".as_ptr() as *const i8);
            return false;
        }
    }
    return true;
}
pub unsafe fn ConvertTTFToWOFF2_72(
    mut data: *const u8,
    mut length: u64,
    mut result: *mut u8,
    mut result_length: *mut u64,
) -> bool {
    let mut params: woff2_WOFF2Params = woff2_WOFF2Params::woff2_WOFF2Params();
    return (unsafe {
        let _data: *const u8 = data;
        let _length: u64 = length;
        let _result: *mut u8 = result;
        let _result_length: *mut u64 = result_length;
        let _params: *const woff2_WOFF2Params = &params as *const woff2_WOFF2Params;
        ConvertTTFToWOFF2_73(_data, _length, _result, _result_length, _params)
    });
}
pub unsafe fn ConvertTTFToWOFF2_73(
    mut data: *const u8,
    mut length: u64,
    mut result: *mut u8,
    mut result_length: *mut u64,
    params: *const woff2_WOFF2Params,
) -> bool {
    let mut font_collection: woff2_FontCollection = <woff2_FontCollection>::default();
    if !(unsafe {
        let _data: *const u8 = data;
        let _len: u64 = length;
        let _fonts: *mut woff2_FontCollection = (&mut font_collection as *mut woff2_FontCollection);
        ReadFontCollection_19(_data, _len, _fonts)
    }) {
        printf(b"Parsing of the input font failed.\n\0".as_ptr() as *const i8);
        return false;
    }
    if !(unsafe {
        let _font_collection: *mut woff2_FontCollection =
            (&mut font_collection as *mut woff2_FontCollection);
        NormalizeFontCollection_51(_font_collection)
    }) {
        return false;
    }
    if ((*params).allow_transforms)
        && (!(unsafe {
            let _font_collection: *mut woff2_FontCollection =
                (&mut font_collection as *mut woff2_FontCollection);
            TransformFontCollection_71(_font_collection)
        }))
    {
        return false;
    } else {
        'loop_: for font in 0..(font_collection.fonts.len()) {
            let mut font = font_collection.fonts.as_mut_ptr().add(font);
            let mut glyf_table: *mut woff2_Font_Table = (unsafe {
                let _tag: u32 = woff2_kGlyfTableTag;
                (*font).FindTable_u32(_tag)
            });
            let mut loca_table: *mut woff2_Font_Table = (unsafe {
                let _tag: u32 = woff2_kLocaTableTag;
                (*font).FindTable_u32(_tag)
            });
            if !(glyf_table).is_null() {
                (*glyf_table).flag_byte = (((*glyf_table).flag_byte as i32) | 192) as u8;
            }
            if !(loca_table).is_null() {
                (*loca_table).flag_byte = (((*loca_table).flag_byte as i32) | 192) as u8;
            }
        }
    }
    let mut total_transform_length: u64 = 0_u64;
    'loop_: for font in 0..(font_collection.fonts.len()) {
        let mut font = font_collection.fonts.as_ptr().add(font);
        total_transform_length = (total_transform_length).wrapping_add(
            (unsafe {
                let _font: *const woff2_Font = font;
                ComputeTotalTransformLength_67(_font)
            }),
        );
    }
    let mut compression_buffer_size: u64 = ((unsafe {
        let _original_size: u32 = (total_transform_length as u32);
        CompressedBufferSize_70(_original_size)
    }) as u64);
    let mut compression_buf: Vec<u8> = (0..(compression_buffer_size) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut total_compressed_length: u32 = (compression_buffer_size as u32);
    let mut transform_buf: Vec<u8> = (0..(total_transform_length) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut transform_offset: u64 = 0_u64;
    'loop_: for font in 0..(font_collection.fonts.len()) {
        let mut font = font_collection.fonts.as_ptr().add(font);
        'loop_: for tag in 0..((unsafe { (*font).OutputOrderedTags() }).len()) {
            let tag = (unsafe { (&(*font)).OutputOrderedTags() })[tag].clone();
            let original: *const woff2_Font_Table =
                ((*font).tables.get(&tag).expect("out of range!").as_ref()
                    as *const woff2_Font_Table);
            if (unsafe { (*original).IsReused() }) {
                continue 'loop_;
            }
            if (((tag) & (2155905152_u32)) != 0) {
                continue 'loop_;
            }
            let mut table_to_store: *const woff2_Font_Table = (unsafe {
                let _tag: u32 = ((tag) ^ (2155905152_u32));
                (*font).FindTable_u32_const(_tag)
            });
            if ((table_to_store) == (Default::default())) {
                table_to_store = (original);
            }
            (unsafe {
                let _data: *const u8 = (*table_to_store).data;
                let _len: u64 = ((*table_to_store).length as u64);
                let _offset: *mut u64 = (&mut transform_offset as *mut u64);
                let _dst: *mut u8 = (&mut transform_buf[(0_u64) as usize] as *mut u8);
                StoreBytes_14(_data, _len, _offset, _dst)
            });
        }
    }
    if !(unsafe {
        let _data: *const u8 = transform_buf.as_mut_ptr().cast_const();
        let _len: u64 = total_transform_length;
        let _result: *mut u8 = (&mut compression_buf[(0_u64) as usize] as *mut u8);
        let _result_len: *mut u32 = (&mut total_compressed_length as *mut u32);
        let _quality: i32 = (*params).brotli_quality;
        Woff2Compress_59(_data, _len, _result, _result_len, _quality)
    }) {
        printf(b"Compression of combined table failed.\n\0".as_ptr() as *const i8);
        return false;
    }
    printf(
        b"Compressed %zu to %u.\n\0".as_ptr() as *const i8,
        total_transform_length,
        total_compressed_length,
    );
    let mut compressed_metadata_buf_length: u32 = (unsafe {
        let _original_size: u32 = (((*params).extended_metadata.len() - 1) as u64 as u32);
        CompressedBufferSize_70(_original_size)
    });
    let mut compressed_metadata_buf: Vec<u8> = (0..(compressed_metadata_buf_length as u64)
        as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    if ((((*params).extended_metadata.len() - 1) as u64) > (0_u64)) {
        if !(unsafe {
            let _data: *const u8 = ((*params).extended_metadata.as_ptr() as *const u8);
            let _len: u64 = ((*params).extended_metadata.len() - 1) as u64;
            let _result: *mut u8 = compressed_metadata_buf.as_mut_ptr();
            let _result_len: *mut u32 = (&mut compressed_metadata_buf_length as *mut u32);
            let _quality: i32 = (*params).brotli_quality;
            TextCompress_60(_data, _len, _result, _result_len, _quality)
        }) {
            printf(b"Compression of extended metadata failed.\n\0".as_ptr() as *const i8);
            return false;
        }
    } else {
        compressed_metadata_buf_length = 0_u32;
    }
    let mut tables: Vec<woff2_Table> = Vec::new();
    let mut index_by_tag_offset: BTreeMap<(u32, u32), Box<u16>> = BTreeMap::new();
    'loop_: for font in 0..(font_collection.fonts.len()) {
        let mut font = font_collection.fonts.as_ptr().add(font);
        'loop_: for tag in 0..((unsafe { (*font).OutputOrderedTags() }).len()) {
            let tag = (unsafe { (&(*font)).OutputOrderedTags() })[tag].clone();
            let src_table: *const woff2_Font_Table =
                ((*font).tables.get(&tag).expect("out of range!").as_ref()
                    as *const woff2_Font_Table);
            if (unsafe { (*src_table).IsReused() }) {
                continue 'loop_;
            }
            let mut tag_offset: (u32, u32) = ((*src_table).tag.into(), (*src_table).offset.into());
            if UnsafeMapIterator::find_key(
                &index_by_tag_offset as *const BTreeMap<(u32, u32), Box<u16>>,
                &tag_offset,
            ) == UnsafeMapIterator::end(
                &index_by_tag_offset as *const BTreeMap<(u32, u32), Box<u16>>,
            ) {
                (*index_by_tag_offset.entry(tag_offset).or_default().as_mut()) =
                    (tables.len() as u64 as u16).clone();
            } else {
                return false;
            }
            let mut table: woff2_Table = <woff2_Table>::default();
            table.tag = (*src_table).tag;
            table.flags = ((*src_table).flag_byte as u32);
            table.src_length = (*src_table).length;
            table.transform_length = (*src_table).length;
            let mut transformed_data: *const u8 = (*src_table).data;
            let mut transformed_table: *const woff2_Font_Table = (unsafe {
                let _tag: u32 = (((*src_table).tag) ^ (2155905152_u32));
                (*font).FindTable_u32_const(_tag)
            });
            if ((transformed_table) != (Default::default())) {
                table.flags = ((*transformed_table).flag_byte as u32);
                table.flags = ((table.flags as u32) | woff2_kWoff2FlagsTransform) as u32;
                table.transform_length = (*transformed_table).length;
                transformed_data = (*transformed_table).data;
            }
            {
                let a0_clone = table.clone();
                tables.push(a0_clone)
            };
        }
    }
    let mut woff2_length: u64 = (unsafe {
        let _font_collection: *const woff2_FontCollection =
            &font_collection as *const woff2_FontCollection;
        let _tables: *const Vec<woff2_Table> = &tables as *const Vec<woff2_Table>;
        let _index_by_tag_offset: BTreeMap<(u32, u32), Box<u16>> = index_by_tag_offset.clone();
        let _compressed_data_length: u64 = (total_compressed_length as u64);
        let _extended_metadata_length: u64 = (compressed_metadata_buf_length as u64);
        ComputeWoff2Length_64(
            _font_collection,
            _tables,
            _index_by_tag_offset,
            _compressed_data_length,
            _extended_metadata_length,
        )
    });
    if ((woff2_length) > (*result_length)) {
        printf(
            b"Result allocation was too small (%zd vs %zd bytes).\n\0".as_ptr() as *const i8,
            (*result_length),
            woff2_length,
        );
        return false;
    }
    (*result_length) = woff2_length;
    let mut offset: u64 = 0_u64;
    (unsafe {
        let _val: u32 = woff2_kWoff2Signature;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    if ((font_collection.flavor) != (woff2_kTtcFontFlavor)) {
        (unsafe {
            let _val: u32 = font_collection.fonts[(0_u64) as usize].flavor;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
    } else {
        (unsafe {
            let _val: u32 = woff2_kTtcFontFlavor;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
    }
    (unsafe {
        let _val: u32 = (woff2_length as u32);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = (tables.len() as u64 as i32);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = 0;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = ((unsafe {
            let _font_collection: *const woff2_FontCollection =
                &font_collection as *const woff2_FontCollection;
            ComputeUncompressedLength_66(_font_collection)
        }) as u32);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = total_compressed_length;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = 1;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        Store16_13(_val, _offset, _dst)
    });
    (unsafe {
        let _val: i32 = 0;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        Store16_13(_val, _offset, _dst)
    });
    if ((compressed_metadata_buf_length) > (0_u32)) {
        (unsafe {
            let _val: u32 =
                (((woff2_length).wrapping_sub((compressed_metadata_buf_length as u64))) as u32);
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = compressed_metadata_buf_length;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = (((*params).extended_metadata.len() - 1) as u64 as u32);
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
    } else {
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: u32 = 0_u32;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
    }
    (unsafe {
        let _val: u32 = 0_u32;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    (unsafe {
        let _val: u32 = 0_u32;
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreU32_12(_val, _offset, _dst)
    });
    'loop_: for table in 0..(tables.len()) {
        let mut table = tables.as_ptr().add(table);
        (unsafe {
            let _table: *const woff2_Table = table;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreTableEntry_62(_table, _offset, _dst)
        });
    }
    if ((font_collection.flavor) == (woff2_kTtcFontFlavor)) {
        (unsafe {
            let _val: u32 = font_collection.header_version;
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            StoreU32_12(_val, _offset, _dst)
        });
        (unsafe {
            let _val: i32 = (font_collection.fonts.len() as u64 as i32);
            let _offset: *mut u64 = (&mut offset as *mut u64);
            let _dst: *mut u8 = result;
            Store255UShort_2(_val, _offset, _dst)
        });
        'loop_: for font in 0..(font_collection.fonts.len()) {
            let mut font = font_collection.fonts.as_ptr().add(font);
            let mut num_tables: u16 = 0_u16;
            'loop_: for entry in UnsafeMapIterator::begin(
                &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            ) {
                let table: *const woff2_Font_Table = &*entry.second() as *const woff2_Font_Table;
                if ((((*table).tag) & (2155905152_u32)) != 0) {
                    continue 'loop_;
                }
                num_tables.postfix_inc();
            }
            (unsafe {
                let _val: i32 = (num_tables as i32);
                let _offset: *mut u64 = (&mut offset as *mut u64);
                let _dst: *mut u8 = result;
                Store255UShort_2(_val, _offset, _dst)
            });
            (unsafe {
                let _val: u32 = (*font).flavor;
                let _offset: *mut u64 = (&mut offset as *mut u64);
                let _dst: *mut u8 = result;
                StoreU32_12(_val, _offset, _dst)
            });
            'loop_: for entry in UnsafeMapIterator::begin(
                &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            ) {
                let table: *const woff2_Font_Table = &*entry.second() as *const woff2_Font_Table;
                if ((((*table).tag) & (2155905152_u32)) != 0) {
                    continue 'loop_;
                }
                let mut table_offset: u32 = if (unsafe { (*table).IsReused() }) {
                    (*(*table).reuse_of).offset
                } else {
                    (*table).offset
                };
                let mut table_length: u32 = if (unsafe { (*table).IsReused() }) {
                    (*(*table).reuse_of).length
                } else {
                    (*table).length
                };
                let mut tag_offset: (u32, u32) = ((*table).tag.into(), table_offset.into());
                if UnsafeMapIterator::find_key(
                    &index_by_tag_offset as *const BTreeMap<(u32, u32), Box<u16>>,
                    &tag_offset,
                ) == UnsafeMapIterator::end(
                    &index_by_tag_offset as *const BTreeMap<(u32, u32), Box<u16>>,
                ) {
                    printf(
                        b"Missing table index for offset 0x%08x\n\0".as_ptr() as *const i8,
                        table_offset,
                    );
                    return false;
                }
                let mut index: u16 = (*index_by_tag_offset.entry(tag_offset).or_default().as_mut());
                (unsafe {
                    let _val: i32 = (index as i32);
                    let _offset: *mut u64 = (&mut offset as *mut u64);
                    let _dst: *mut u8 = result;
                    Store255UShort_2(_val, _offset, _dst)
                });
            }
        }
    }
    (unsafe {
        let _data: *const u8 = (&mut compression_buf[(0_u64) as usize] as *mut u8).cast_const();
        let _len: u64 = (total_compressed_length as u64);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreBytes_14(_data, _len, _offset, _dst)
    });
    offset = (unsafe {
        let _value: u64 = offset;
        Round4_39(_value)
    });
    (unsafe {
        let _data: *const u8 = compressed_metadata_buf.as_mut_ptr().cast_const();
        let _len: u64 = (compressed_metadata_buf_length as u64);
        let _offset: *mut u64 = (&mut offset as *mut u64);
        let _dst: *mut u8 = result;
        StoreBytes_14(_data, _len, _offset, _dst)
    });
    if ((*result_length) != (offset)) {
        printf(
            b"Mismatch between computed and actual length (%zd vs %zd)\n\0".as_ptr() as *const i8,
            (*result_length),
            offset,
        );
        return false;
    }
    return true;
}
// woff2_compress.rs
pub unsafe fn GetFileContent_74(mut filename: Vec<u8>) -> Vec<u8> {
    let mut ifs: ::std::fs::File = ::std::fs::File::open(
        ::std::ffi::CStr::from_ptr(filename.as_ptr() as *const i8)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    return {
        let mut __buf: Vec<u8> = Vec::new();
        let mut __f = &ifs.try_clone().unwrap();
        __f.read_to_end(&mut __buf).expect("couldn't read the file");
        __buf.push(0);
        __buf
    };
}
pub unsafe fn SetFileContents_75(mut filename: Vec<u8>, mut start: *mut u8, mut end: *mut u8) {
    let mut ofs: ::std::fs::File = ::std::fs::File::create(
        ::std::ffi::CStr::from_ptr(filename.as_ptr() as *const i8)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    {
        let __start = start.clone() as *const u8;
        let __end = end.clone() as *const u8;
        let __len = __end.offset_from(__start) as usize;
        ofs.try_clone()
            .unwrap()
            .write_all(::std::slice::from_raw_parts(__start, __len));
        ofs.try_clone().unwrap().try_clone().unwrap()
    };
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut u8> = args.iter().map(|arg| arg.as_ptr() as *mut u8).collect();
    argv.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32) }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    if ((argc) != (2)) {
        printf(b"One argument, the input filename, must be provided.\n\0".as_ptr() as *const i8);
        return 1;
    }
    let mut filename: Vec<u8> = {
        let s = (*argv.offset((1) as isize)).cast_const();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut outfilename: Vec<u8> = {
        let mut __tmp2 = {
            let mut __tmp1 = filename[(0_u64) as usize
                ..::std::cmp::min(
                    (0_u64
                        + match filename.iter().rposition(|&c| {
                            ::std::ffi::CStr::from_ptr(b".\0".as_ptr() as *const i8)
                                .to_str()
                                .unwrap()
                                .contains(c as u8 as char)
                        }) {
                            Some(idx) => idx as u64,
                            None => u64::MAX,
                        }) as usize,
                    filename.len() - 1,
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }
        .clone();
        __tmp2.pop();
        let __from = b".woff2\0".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    printf(
        b"Processing %s => %s\n\0".as_ptr() as *const i8,
        filename.as_ptr(),
        outfilename.as_ptr(),
    );
    let mut input: Vec<u8> = (unsafe {
        let _filename: Vec<u8> = filename.clone();
        GetFileContent_74(_filename)
    });
    let mut input_data: *const u8 = (input.as_ptr() as *const u8);
    let mut output_size: u64 = (unsafe {
        let _data: *const u8 = input_data;
        let _length: u64 = (input.len() - 1) as u64;
        MaxWOFF2CompressedSize_68(_data, _length)
    });
    let mut output: Vec<u8> = vec![0_u8; (output_size) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut output_data: *mut u8 =
        ((&mut output[(0_u64) as usize] as *mut u8) as *mut u8 as *mut u8);
    let mut params: woff2_WOFF2Params = woff2_WOFF2Params::woff2_WOFF2Params();
    if !(unsafe {
        let _data: *const u8 = input_data;
        let _length: u64 = (input.len() - 1) as u64;
        let _result: *mut u8 = output_data;
        let _result_length: *mut u64 = (&mut output_size as *mut u64);
        let _params: *const woff2_WOFF2Params = &params as *const woff2_WOFF2Params;
        ConvertTTFToWOFF2_73(_data, _length, _result, _result_length, _params)
    }) {
        printf(b"Compression failed.\n\0".as_ptr() as *const i8);
        return 1;
    }
    {
        output.pop();
        output.resize((output_size) as usize, 0);
        output.push(0)
    };
    (unsafe {
        let _filename: Vec<u8> = outfilename.clone();
        let _start: *mut u8 = output.as_mut_ptr();
        let _end: *mut u8 = output.as_mut_ptr().add(output.len() - 1);
        SetFileContents_75(_filename, _start, _end)
    });
    return 0;
}
