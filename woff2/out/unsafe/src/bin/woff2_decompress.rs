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
// woff2_dec.rs
pub static woff2_kDefaultMaxSize: u64 = ((((128) * (1024)) * (1024)) as u64);
pub unsafe trait woff2_WOFF2Out {
    unsafe fn Write_pconstlibcc_void_u64(&mut self, buf: *const ::libc::c_void, n: u64) -> bool;
    unsafe fn Write_pconstlibcc_void_u64_u64(
        &mut self,
        buf: *const ::libc::c_void,
        offset: u64,
        n: u64,
    ) -> bool;
    unsafe fn Size(&mut self) -> u64;
}
pub unsafe fn Round4_10(mut value: u64) -> u64 {
    if (((<u64>::MAX as u64).wrapping_sub(value)) < (3_u64)) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub unsafe fn Round4_11(mut value: u32) -> u32 {
    if (((<u32>::MAX as u32).wrapping_sub(value)) < (3_u32)) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub unsafe fn StoreU32_12(mut dst: *mut u8, mut offset: u64, mut x: u32) -> u64 {
    (*dst.offset((offset) as isize)) = (((x) >> (24)) as u8);
    (*dst.offset(((offset).wrapping_add(1_u64)) as isize)) = (((x) >> (16)) as u8);
    (*dst.offset(((offset).wrapping_add(2_u64)) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(3_u64)) as isize)) = (x as u8);
    return (offset).wrapping_add(4_u64);
}
pub unsafe fn Store16_13(mut dst: *mut u8, mut offset: u64, mut x: i32) -> u64 {
    (*dst.offset((offset) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(1_u64)) as isize)) = (x as u8);
    return (offset).wrapping_add(2_u64);
}
pub unsafe fn StoreU32_14(mut val: u32, mut offset: *mut u64, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (24)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (16)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn Store16_15(mut val: i32, mut offset: *mut u64, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn StoreBytes_16(
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
pub static woff2_kGlyfOnCurve: i32 = ((1) << (0));
pub static woff2_kGlyfXShort: i32 = ((1) << (1));
pub static woff2_kGlyfYShort: i32 = ((1) << (2));
pub static woff2_kGlyfRepeat: i32 = ((1) << (3));
pub static woff2_kGlyfThisXIsSame: i32 = ((1) << (4));
pub static woff2_kGlyfThisYIsSame: i32 = ((1) << (5));
pub static woff2_kOverlapSimple: i32 = ((1) << (6));
pub static woff2_FLAG_ARG_1_AND_2_ARE_WORDS: i32 = ((1) << (0));
pub static woff2_FLAG_WE_HAVE_A_SCALE: i32 = ((1) << (3));
pub static woff2_FLAG_MORE_COMPONENTS: i32 = ((1) << (5));
pub static woff2_FLAG_WE_HAVE_AN_X_AND_Y_SCALE: i32 = ((1) << (6));
pub static woff2_FLAG_WE_HAVE_A_TWO_BY_TWO: i32 = ((1) << (7));
pub static woff2_FLAG_WE_HAVE_INSTRUCTIONS: i32 = ((1) << (8));
pub static woff2_FLAG_OVERLAP_SIMPLE_BITMAP: i32 = ((1) << (0));
pub static woff2_kCheckSumAdjustmentOffset: u64 = 8_u64;
pub static woff2_kEndPtsOfContoursOffset: u64 = 10_u64;
pub static woff2_kCompositeGlyphBegin: u64 = 10_u64;
pub static woff2_kDefaultGlyphBuf: u64 = 5120_u64;
pub static woff2_kMaxPlausibleCompressionRatio: f32 = (1.0E+2 as f32);
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_TtcFont {
    pub flavor: u32,
    pub dst_offset: u32,
    pub header_checksum: u32,
    pub table_indices: Vec<u16>,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_WOFF2Header {
    pub flavor: u32,
    pub header_version: u32,
    pub num_tables: u16,
    pub compressed_offset: u64,
    pub compressed_length: u32,
    pub uncompressed_size: u32,
    pub tables: Vec<woff2_Table>,
    pub ttc_fonts: Vec<woff2_TtcFont>,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_WOFF2FontInfo {
    pub num_glyphs: u16,
    pub index_format: u16,
    pub num_hmetrics: u16,
    pub x_mins: Vec<i16>,
    pub table_entry_by_tag: BTreeMap<u32, Box<u32>>,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct woff2_RebuildMetadata {
    pub header_checksum: u32,
    pub font_infos: Vec<woff2_WOFF2FontInfo>,
    pub checksums: BTreeMap<(u32, u32), Box<u32>>,
}
pub unsafe fn WithSign_17(mut flag: i32, mut baseval: i32) -> i32 {
    return if (((flag) & (1)) != 0) {
        baseval
    } else {
        -baseval
    };
}
pub unsafe fn _SafeIntAddition_18(mut a: i32, mut b: i32, mut result: *mut i32) -> bool {
    if ((((((a) > (0)) && ((b) > ((<i32>::MAX) - (a))))
        || (((a) < (0)) && ((b) < ((<i32>::MIN) - (a))))) as i64)
        != 0)
    {
        return false;
    }
    (*result) = ((a) + (b));
    return true;
}
pub unsafe fn TripletDecode_19(
    mut flags_in: *const u8,
    mut in_: *const u8,
    mut in_size: u64,
    mut n_points: u32,
    mut result: *mut woff2_Point,
    mut in_bytes_consumed: *mut u64,
) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if ((((n_points as u64) > (in_size)) as i64) != 0) {
        return false;
    }
    let mut triplet_index: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (n_points)) {
        let mut flag: u8 = (*flags_in.offset((i) as isize));
        let mut on_curve: bool = !(((flag as i32) >> (7)) != 0);
        flag = ((flag as i32) & 127) as u8;
        let mut n_data_bytes: u32 = 0_u32;
        if ((flag as i32) < (84)) {
            n_data_bytes = 1_u32;
        } else if ((flag as i32) < (120)) {
            n_data_bytes = 2_u32;
        } else if ((flag as i32) < (124)) {
            n_data_bytes = 3_u32;
        } else {
            n_data_bytes = 4_u32;
        }
        if (((((((triplet_index).wrapping_add(n_data_bytes)) as u64) > (in_size))
            || (((triplet_index).wrapping_add(n_data_bytes)) < (triplet_index)))
            as i64)
            != 0)
        {
            return false;
        }
        let mut dx: i32 = 0_i32;
        let mut dy: i32 = 0_i32;
        if ((flag as i32) < (10)) {
            dx = 0;
            dy = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = ((((flag as i32) & (14)) << (7))
                    + ((*in_.offset((triplet_index) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
        } else if ((flag as i32) < (20)) {
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = (((((flag as i32) - (10)) & (14)) << (7))
                    + ((*in_.offset((triplet_index) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
            dy = 0;
        } else if ((flag as i32) < (84)) {
            let mut b0: i32 = ((flag as i32) - (20));
            let mut b1: i32 = ((*in_.offset((triplet_index) as isize)) as i32);
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = (((1) + ((b0) & (48))) + ((b1) >> (4)));
                WithSign_17(_flag, _baseval)
            });
            dy = (unsafe {
                let _flag: i32 = ((flag as i32) >> (1));
                let _baseval: i32 = (((1) + (((b0) & (12)) << (2))) + ((b1) & (15)));
                WithSign_17(_flag, _baseval)
            });
        } else if ((flag as i32) < (120)) {
            let mut b0: i32 = ((flag as i32) - (84));
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = (((1) + (((b0) / (12)) << (8)))
                    + ((*in_.offset((triplet_index) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
            dy = (unsafe {
                let _flag: i32 = ((flag as i32) >> (1));
                let _baseval: i32 = (((1) + ((((b0) % (12)) >> (2)) << (8)))
                    + ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
        } else if ((flag as i32) < (124)) {
            let mut b2: i32 =
                ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32);
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 =
                    ((((*in_.offset((triplet_index) as isize)) as i32) << (4)) + ((b2) >> (4)));
                WithSign_17(_flag, _baseval)
            });
            dy = (unsafe {
                let _flag: i32 = ((flag as i32) >> (1));
                let _baseval: i32 = ((((b2) & (15)) << (8))
                    + ((*in_.offset(((triplet_index).wrapping_add(2_u32)) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
        } else {
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = ((((*in_.offset((triplet_index) as isize)) as i32) << (8))
                    + ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
            dy = (unsafe {
                let _flag: i32 = ((flag as i32) >> (1));
                let _baseval: i32 = ((((*in_.offset(((triplet_index).wrapping_add(2_u32)) as isize))
                    as i32)
                    << (8))
                    + ((*in_.offset(((triplet_index).wrapping_add(3_u32)) as isize)) as i32));
                WithSign_17(_flag, _baseval)
            });
        }
        triplet_index = (triplet_index).wrapping_add(n_data_bytes);
        if !(unsafe {
            let _a: i32 = x;
            let _b: i32 = dx;
            let _result: *mut i32 = (&mut x as *mut i32);
            _SafeIntAddition_18(_a, _b, _result)
        }) {
            return false;
        }
        if !(unsafe {
            let _a: i32 = y;
            let _b: i32 = dy;
            let _result: *mut i32 = (&mut y as *mut i32);
            _SafeIntAddition_18(_a, _b, _result)
        }) {
            return false;
        }
        (*result.postfix_inc()) = woff2_Point {
            x: x,
            y: y,
            on_curve: on_curve,
        };
        i.prefix_inc();
    }
    (*in_bytes_consumed) = (triplet_index as u64);
    return true;
}
pub unsafe fn StorePoints_20(
    mut n_points: u32,
    mut points: *const woff2_Point,
    mut n_contours: u32,
    mut instruction_length: u32,
    mut has_overlap_bit: bool,
    mut dst: *mut u8,
    mut dst_size: u64,
    mut glyph_size: *mut u64,
) -> bool {
    let mut flag_offset: u32 = (((((woff2_kEndPtsOfContoursOffset)
        .wrapping_add((((2_u32).wrapping_mul(n_contours)) as u64)))
    .wrapping_add(2_u64))
    .wrapping_add((instruction_length as u64))) as u32);
    let mut last_flag: i32 = -1_i32;
    let mut repeat_count: i32 = 0;
    let mut last_x: i32 = 0;
    let mut last_y: i32 = 0;
    let mut x_bytes: u32 = 0_u32;
    let mut y_bytes: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (n_points)) {
        let point: *const woff2_Point = &(*points.offset((i) as isize)) as *const woff2_Point;
        let mut flag: i32 = if (*point).on_curve {
            woff2_kGlyfOnCurve
        } else {
            0
        };
        if (has_overlap_bit) && ((i) == (0_u32)) {
            flag |= woff2_kOverlapSimple;
        }
        let mut dx: i32 = (((*point).x) - (last_x));
        let mut dy: i32 = (((*point).y) - (last_y));
        if ((dx) == (0)) {
            flag |= woff2_kGlyfThisXIsSame;
        } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
            flag |= ((woff2_kGlyfXShort)
                | (if ((dx) > (0)) {
                    woff2_kGlyfThisXIsSame
                } else {
                    0
                }));
            x_bytes = (x_bytes).wrapping_add(1_u32);
        } else {
            x_bytes = (x_bytes).wrapping_add(2_u32);
        }
        if ((dy) == (0)) {
            flag |= woff2_kGlyfThisYIsSame;
        } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
            flag |= ((woff2_kGlyfYShort)
                | (if ((dy) > (0)) {
                    woff2_kGlyfThisYIsSame
                } else {
                    0
                }));
            y_bytes = (y_bytes).wrapping_add(1_u32);
        } else {
            y_bytes = (y_bytes).wrapping_add(2_u32);
        }
        if ((flag) == (last_flag)) && ((repeat_count) != (255)) {
            (*dst.offset(((flag_offset).wrapping_sub(1_u32)) as isize)) =
                (((*dst.offset(((flag_offset).wrapping_sub(1_u32)) as isize)) as i32)
                    | woff2_kGlyfRepeat) as u8;
            repeat_count.postfix_inc();
        } else {
            if ((repeat_count) != (0)) {
                if ((((flag_offset as u64) >= (dst_size)) as i64) != 0) {
                    return false;
                }
                (*dst.offset((flag_offset.postfix_inc()) as isize)) = (repeat_count as u8);
            }
            if ((((flag_offset as u64) >= (dst_size)) as i64) != 0) {
                return false;
            }
            (*dst.offset((flag_offset.postfix_inc()) as isize)) = (flag as u8);
            repeat_count = 0;
        }
        last_x = (*point).x;
        last_y = (*point).y;
        last_flag = flag;
        i.prefix_inc();
    }
    if ((repeat_count) != (0)) {
        if ((((flag_offset as u64) >= (dst_size)) as i64) != 0) {
            return false;
        }
        (*dst.offset((flag_offset.postfix_inc()) as isize)) = (repeat_count as u8);
    }
    let mut xy_bytes: u32 = (x_bytes).wrapping_add(y_bytes);
    if ((((((xy_bytes) < (x_bytes)) || (((flag_offset).wrapping_add(xy_bytes)) < (flag_offset)))
        || ((((flag_offset).wrapping_add(xy_bytes)) as u64) > (dst_size))) as i64)
        != 0)
    {
        return false;
    }
    let mut x_offset: i32 = (flag_offset as i32);
    let mut y_offset: i32 = (((flag_offset).wrapping_add(x_bytes)) as i32);
    last_x = 0;
    last_y = 0;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (n_points)) {
        let mut dx: i32 = (((*points.offset((i) as isize)).x) - (last_x));
        if ((dx) == (0)) {
        } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
            (*dst.offset((x_offset.postfix_inc()) as isize)) = (dx.abs() as u8);
        } else {
            x_offset = ((unsafe {
                let _dst: *mut u8 = dst;
                let _offset: u64 = (x_offset as u64);
                let _x: i32 = dx;
                Store16_13(_dst, _offset, _x)
            }) as i32);
        }
        last_x += dx;
        let mut dy: i32 = (((*points.offset((i) as isize)).y) - (last_y));
        if ((dy) == (0)) {
        } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
            (*dst.offset((y_offset.postfix_inc()) as isize)) = (dy.abs() as u8);
        } else {
            y_offset = ((unsafe {
                let _dst: *mut u8 = dst;
                let _offset: u64 = (y_offset as u64);
                let _x: i32 = dy;
                Store16_13(_dst, _offset, _x)
            }) as i32);
        }
        last_y += dy;
        i.prefix_inc();
    }
    (*glyph_size) = (y_offset as u64);
    return true;
}
pub unsafe fn ComputeBbox_21(mut n_points: u32, mut points: *const woff2_Point, mut dst: *mut u8) {
    let mut x_min: i32 = 0;
    let mut y_min: i32 = 0;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;
    if ((n_points) > (0_u32)) {
        x_min = (*points.offset((0) as isize)).x;
        x_max = (*points.offset((0) as isize)).x;
        y_min = (*points.offset((0) as isize)).y;
        y_max = (*points.offset((0) as isize)).y;
    }
    let mut i: u32 = 1_u32;
    'loop_: while ((i) < (n_points)) {
        let mut x: i32 = (*points.offset((i) as isize)).x;
        let mut y: i32 = (*points.offset((i) as isize)).y;
        x_min = (*if *&mut x <= *&mut x_min {
            (&mut x) as *const _
        } else {
            (&mut x_min) as *const _
        });
        x_max = (*if *&mut x >= *&mut x_max {
            (&mut x) as *const _
        } else {
            (&mut x_max) as *const _
        });
        y_min = (*if *&mut y <= *&mut y_min {
            (&mut y) as *const _
        } else {
            (&mut y_min) as *const _
        });
        y_max = (*if *&mut y >= *&mut y_max {
            (&mut y) as *const _
        } else {
            (&mut y_max) as *const _
        });
        i.prefix_inc();
    }
    let mut offset: u64 = 2_u64;
    offset = (unsafe {
        let _dst: *mut u8 = dst;
        let _offset: u64 = offset;
        let _x: i32 = x_min;
        Store16_13(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = dst;
        let _offset: u64 = offset;
        let _x: i32 = y_min;
        Store16_13(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = dst;
        let _offset: u64 = offset;
        let _x: i32 = x_max;
        Store16_13(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = dst;
        let _offset: u64 = offset;
        let _x: i32 = y_max;
        Store16_13(_dst, _offset, _x)
    });
}
pub unsafe fn SizeOfComposite_22(
    mut composite_stream: woff2_Buffer,
    mut size: *mut u64,
    mut have_instructions: *mut bool,
) -> bool {
    let mut start_offset: u64 = (unsafe { composite_stream.offset() });
    let mut we_have_instructions: bool = false;
    let mut flags: u16 = (woff2_FLAG_MORE_COMPONENTS as u16);
    'loop_: while (((flags as i32) & (woff2_FLAG_MORE_COMPONENTS)) != 0) {
        if ((!(unsafe {
            let _value: *mut u16 = (&mut flags as *mut u16);
            composite_stream.ReadU16(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        we_have_instructions = ((we_have_instructions as i32)
            | ((((flags as i32) & (woff2_FLAG_WE_HAVE_INSTRUCTIONS)) != (0)) as i32))
            != 0;
        let mut arg_size: u64 = 2_u64;
        if (((flags as i32) & (woff2_FLAG_ARG_1_AND_2_ARE_WORDS)) != 0) {
            arg_size = (arg_size).wrapping_add(4_u64);
        } else {
            arg_size = (arg_size).wrapping_add(2_u64);
        }
        if (((flags as i32) & (woff2_FLAG_WE_HAVE_A_SCALE)) != 0) {
            arg_size = (arg_size).wrapping_add(2_u64);
        } else if (((flags as i32) & (woff2_FLAG_WE_HAVE_AN_X_AND_Y_SCALE)) != 0) {
            arg_size = (arg_size).wrapping_add(4_u64);
        } else if (((flags as i32) & (woff2_FLAG_WE_HAVE_A_TWO_BY_TWO)) != 0) {
            arg_size = (arg_size).wrapping_add(8_u64);
        }
        if ((!(unsafe {
            let _n_bytes: u64 = arg_size;
            composite_stream.Skip(_n_bytes)
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    (*size) = (unsafe { composite_stream.offset() }).wrapping_sub(start_offset);
    (*have_instructions) = we_have_instructions;
    return true;
}
pub unsafe fn Pad4_23(mut out: *mut dyn woff2_WOFF2Out) -> bool {
    let mut zeroes: [u8; 3] = [0_u8, 0_u8, 0_u8];
    if (((((unsafe { (*out).Size() }).wrapping_add(3_u64)) < (unsafe { (*out).Size() })) as i64)
        != 0)
    {
        return false;
    }
    let mut pad_bytes: u32 = (((unsafe {
        let _value: u64 = (unsafe { (*out).Size() });
        Round4_10(_value)
    })
    .wrapping_sub((unsafe { (*out).Size() }))) as u32);
    if ((pad_bytes) > (0_u32)) {
        if ((!(unsafe {
            let _buf: *const ::libc::c_void =
                ((&mut zeroes as *mut [u8; 3]) as *const [u8; 3] as *const ::libc::c_void);
            let _n: u64 = (pad_bytes as u64);
            (*out).Write_pconstlibcc_void_u64(_buf, _n)
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn StoreLoca_24(
    loca_values: *const Vec<u32>,
    mut index_format: i32,
    mut checksum: *mut u32,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let loca_size: u64 = (*loca_values).len() as u64;
    let offset_size: u64 = (if (index_format != 0) { 4 } else { 2 } as u64);
    if ((((((loca_size) << (2)) >> (2)) != (loca_size)) as i64) != 0) {
        return false;
    }
    let mut loca_content: Vec<u8> = (0..((loca_size).wrapping_mul(offset_size)) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut dst: *mut u8 = (&mut loca_content[(0_u64) as usize] as *mut u8);
    let mut offset: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < ((*loca_values).len() as u64)) {
        let mut value: u32 = (&(*loca_values))[(i) as usize];
        if (index_format != 0) {
            offset = (unsafe {
                let _dst: *mut u8 = dst;
                let _offset: u64 = offset;
                let _x: u32 = value;
                StoreU32_12(_dst, _offset, _x)
            });
        } else {
            offset = (unsafe {
                let _dst: *mut u8 = dst;
                let _offset: u64 = offset;
                let _x: i32 = (((value) >> (1)) as i32);
                Store16_13(_dst, _offset, _x)
            });
        }
        i.prefix_inc();
    }
    (*checksum) = (unsafe {
        let _buf: *const u8 = (&mut loca_content[(0_u64) as usize] as *mut u8).cast_const();
        let _size: u64 = loca_content.len() as u64;
        ComputeULongSum_8(_buf, _size)
    });
    if ((!(unsafe {
        let _buf: *const ::libc::c_void = ((&mut loca_content[(0_u64) as usize] as *mut u8)
            as *const u8 as *const ::libc::c_void);
        let _n: u64 = loca_content.len() as u64;
        (*out).Write_pconstlibcc_void_u64(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn ReconstructGlyf_25(
    mut data: *const u8,
    mut glyf_table: *mut woff2_Table,
    mut glyf_checksum: *mut u32,
    mut loca_table: *mut woff2_Table,
    mut loca_checksum: *mut u32,
    mut info: *mut woff2_WOFF2FontInfo,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    static kNumSubStreams: i32 = 7;;
    let mut file: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(data, ((*glyf_table).transform_length as u64));
    let mut version: u16 = 0_u16;
    let mut substreams: Vec<(*const u8, u64)> = (0..(kNumSubStreams as u64) as usize)
        .map(|_| <(*const u8, u64)>::default())
        .collect::<Vec<_>>();
    let glyf_start: u64 = (unsafe { (*out).Size() });
    if ((!(unsafe {
        let _value: *mut u16 = (&mut version as *mut u16);
        file.ReadU16(_value)
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut flags: u16 = 0_u16;
    if ((!(unsafe {
        let _value: *mut u16 = (&mut flags as *mut u16);
        file.ReadU16(_value)
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut has_overlap_bitmap: bool = (((flags as i32) & (woff2_FLAG_OVERLAP_SIMPLE_BITMAP)) != 0);
    if ((((!(unsafe {
        let _value: *mut u16 = (&mut (*info).num_glyphs as *mut u16);
        file.ReadU16(_value)
    })) || (!(unsafe {
        let _value: *mut u16 = (&mut (*info).index_format as *mut u16);
        file.ReadU16(_value)
    }))) as i64)
        != 0)
    {
        return false;
    }
    let mut expected_loca_dst_length: u32 = ((if ((*info).index_format != 0) { 4 } else { 2 })
        as u32)
        .wrapping_mul((((*info).num_glyphs as u32).wrapping_add(1_u32)));
    if (((((*loca_table).dst_length) != (expected_loca_dst_length)) as i64) != 0) {
        return false;
    }
    let mut offset: u32 = ((((2) + (kNumSubStreams)) * (4)) as u32);
    if ((((offset) > ((*glyf_table).transform_length)) as i64) != 0) {
        return false;
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (kNumSubStreams)) {
        let mut substream_size: u32 = 0_u32;
        if ((!(unsafe {
            let _value: *mut u32 = (&mut substream_size as *mut u32);
            file.ReadU32(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((((substream_size) > (((*glyf_table).transform_length).wrapping_sub(offset))) as i64)
            != 0)
        {
            return false;
        }
        substreams[(i as u64) as usize] =
            (data.offset((offset) as isize).into(), substream_size.into());
        offset = (offset).wrapping_add(substream_size);
        i.prefix_inc();
    }
    let mut n_contour_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(0_u64) as usize].0,
        substreams[(0_u64) as usize].1,
    );
    let mut n_points_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(1_u64) as usize].0,
        substreams[(1_u64) as usize].1,
    );
    let mut flag_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(2_u64) as usize].0,
        substreams[(2_u64) as usize].1,
    );
    let mut glyph_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(3_u64) as usize].0,
        substreams[(3_u64) as usize].1,
    );
    let mut composite_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(4_u64) as usize].0,
        substreams[(4_u64) as usize].1,
    );
    let mut bbox_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(5_u64) as usize].0,
        substreams[(5_u64) as usize].1,
    );
    let mut instruction_stream: woff2_Buffer = woff2_Buffer::woff2_Buffer(
        substreams[(6_u64) as usize].0,
        substreams[(6_u64) as usize].1,
    );
    let mut overlap_bitmap: *const u8 = Default::default();
    let mut overlap_bitmap_length: u32 = 0_u32;
    if has_overlap_bitmap {
        overlap_bitmap_length = (((((*info).num_glyphs as i32) + (7)) >> (3)) as u32);
        overlap_bitmap = data.offset((offset) as isize);
        if ((((overlap_bitmap_length) > (((*glyf_table).transform_length).wrapping_sub(offset)))
            as i64)
            != 0)
        {
            return false;
        }
    }
    let mut loca_values: Vec<u32> = (0..((((*info).num_glyphs as i32) + (1)) as u64) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut n_points_vec: Vec<u32> = Vec::new();
    let mut points: Option<Box<[woff2_Point]>> = None;
    let mut points_size: u64 = 0_u64;
    let mut bbox_bitmap: *const u8 = (unsafe { bbox_stream.buffer() });
    let mut bitmap_length: u32 = ((((((*info).num_glyphs as i32) + (31)) >> (5)) << (2)) as u32);
    if !(unsafe {
        let _n_bytes: u64 = (bitmap_length as u64);
        bbox_stream.Skip(_n_bytes)
    }) {
        return false;
    }
    let mut glyph_buf_size: u64 = woff2_kDefaultGlyphBuf;
    let mut glyph_buf: Option<Box<[u8]>> = Some(Box::from_raw(Box::leak(
        (0..glyph_buf_size).map(|_| 0_u8).collect::<Box<[u8]>>(),
    )));
    {
        let __a0 = ((*info).num_glyphs as u64) as usize;
        (*info).x_mins.resize_with(__a0, || <i16>::default())
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < ((*info).num_glyphs as u32)) {
        let mut glyph_size: u64 = 0_u64;
        let mut n_contours: u16 = 0_u16;
        let mut have_bbox: bool = false;
        if ((((*bbox_bitmap.offset(((i) >> (3)) as isize)) as i32) & ((128) >> ((i) & (7_u32))))
            != 0)
        {
            have_bbox = true;
        }
        if ((!(unsafe {
            let _value: *mut u16 = (&mut n_contours as *mut u16);
            n_contour_stream.ReadU16(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((n_contours as i32) == (65535)) {
            let mut have_instructions: bool = false;
            let mut instruction_size: u32 = 0_u32;
            if ((!have_bbox as i64) != 0) {
                return false;
            }
            let mut composite_size: u64 = 0_u64;
            if ((!(unsafe {
                let _composite_stream: woff2_Buffer = composite_stream.clone();
                let _size: *mut u64 = (&mut composite_size as *mut u64);
                let _have_instructions: *mut bool = (&mut have_instructions as *mut bool);
                SizeOfComposite_22(_composite_stream, _size, _have_instructions)
            }) as i64)
                != 0)
            {
                return false;
            }
            if have_instructions {
                if ((!(unsafe {
                    let _buf: *mut woff2_Buffer = (&mut glyph_stream as *mut woff2_Buffer);
                    let _value: *mut u32 = (&mut instruction_size as *mut u32);
                    Read255UShort_3(_buf, _value)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            }
            let mut size_needed: u64 =
                ((12_u64).wrapping_add(composite_size)).wrapping_add((instruction_size as u64));
            if ((((glyph_buf_size) < (size_needed)) as i64) != 0) {
                glyph_buf = Some(Box::from_raw(Box::leak(
                    (0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>(),
                )));
                glyph_buf_size = size_needed;
            }
            glyph_size = (unsafe {
                let _dst: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                let _offset: u64 = glyph_size;
                let _x: i32 = (n_contours as i32);
                Store16_13(_dst, _offset, _x)
            });
            if ((!(unsafe {
                let _data: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .offset((glyph_size) as isize);
                let _n_bytes: u64 = 8_u64;
                bbox_stream.Read(_data, _n_bytes)
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add(8_u64);
            if ((!(unsafe {
                let _data: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .offset((glyph_size) as isize);
                let _n_bytes: u64 = composite_size;
                composite_stream.Read(_data, _n_bytes)
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add(composite_size);
            if have_instructions {
                glyph_size = (unsafe {
                    let _dst: *mut u8 = glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                    let _offset: u64 = glyph_size;
                    let _x: i32 = (instruction_size as i32);
                    Store16_13(_dst, _offset, _x)
                });
                if ((!(unsafe {
                    let _data: *mut u8 = glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .offset((glyph_size) as isize);
                    let _n_bytes: u64 = (instruction_size as u64);
                    instruction_stream.Read(_data, _n_bytes)
                }) as i64)
                    != 0)
                {
                    return false;
                }
                glyph_size = (glyph_size).wrapping_add((instruction_size as u64));
            }
        } else if ((n_contours as i32) > (0)) {
            n_points_vec.clear();
            let mut total_n_points: u32 = 0_u32;
            let mut n_points_contour: u32 = 0_u32;
            let mut j: u32 = 0_u32;
            'loop_: while ((j) < (n_contours as u32)) {
                if ((!(unsafe {
                    let _buf: *mut woff2_Buffer = (&mut n_points_stream as *mut woff2_Buffer);
                    let _value: *mut u32 = (&mut n_points_contour as *mut u32);
                    Read255UShort_3(_buf, _value)
                }) as i64)
                    != 0)
                {
                    return false;
                }
                {
                    let a0_clone = n_points_contour.clone();
                    n_points_vec.push(a0_clone)
                };
                if (((((total_n_points).wrapping_add(n_points_contour)) < (total_n_points)) as i64)
                    != 0)
                {
                    return false;
                }
                total_n_points = (total_n_points).wrapping_add(n_points_contour);
                j.prefix_inc();
            }
            let mut flag_size: u32 = total_n_points;
            if ((((flag_size as u64)
                > ((unsafe { flag_stream.length() })
                    .wrapping_sub((unsafe { flag_stream.offset() })))) as i64)
                != 0)
            {
                return false;
            }
            let mut flags_buf: *const u8 = (unsafe { flag_stream.buffer() })
                .offset((unsafe { flag_stream.offset() }) as isize);
            let mut triplet_buf: *const u8 = (unsafe { glyph_stream.buffer() })
                .offset((unsafe { glyph_stream.offset() }) as isize);
            let mut triplet_size: u64 =
                (unsafe { glyph_stream.length() }).wrapping_sub((unsafe { glyph_stream.offset() }));
            let mut triplet_bytes_consumed: u64 = 0_u64;
            if ((points_size) < (total_n_points as u64)) {
                points_size = (total_n_points as u64);
                points = Some(Box::from_raw(Box::leak(
                    (0..points_size)
                        .map(|_| <woff2_Point>::default())
                        .collect::<Box<[woff2_Point]>>(),
                )));
            }
            if ((!(unsafe {
                let _flags_in: *const u8 = flags_buf;
                let _in: *const u8 = triplet_buf;
                let _in_size: u64 = triplet_size;
                let _n_points: u32 = total_n_points;
                let _result: *mut woff2_Point = points
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                let _in_bytes_consumed: *mut u64 = (&mut triplet_bytes_consumed as *mut u64);
                TripletDecode_19(
                    _flags_in,
                    _in,
                    _in_size,
                    _n_points,
                    _result,
                    _in_bytes_consumed,
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((!(unsafe {
                let _n_bytes: u64 = (flag_size as u64);
                flag_stream.Skip(_n_bytes)
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((!(unsafe {
                let _n_bytes: u64 = triplet_bytes_consumed;
                glyph_stream.Skip(_n_bytes)
            }) as i64)
                != 0)
            {
                return false;
            }
            let mut instruction_size: u32 = 0_u32;
            if ((!(unsafe {
                let _buf: *mut woff2_Buffer = (&mut glyph_stream as *mut woff2_Buffer);
                let _value: *mut u32 = (&mut instruction_size as *mut u32);
                Read255UShort_3(_buf, _value)
            }) as i64)
                != 0)
            {
                return false;
            }
            if (((((total_n_points) >= (((1) << (27)) as u32))
                || ((instruction_size) >= (((1) << (30)) as u32))) as i64)
                != 0)
            {
                return false;
            }
            let mut size_needed: u64 = ((((((12) + ((2) * (n_contours as i32))) as u32)
                .wrapping_add((5_u32).wrapping_mul(total_n_points)))
            .wrapping_add(instruction_size)) as u64);
            if ((((glyph_buf_size) < (size_needed)) as i64) != 0) {
                glyph_buf = Some(Box::from_raw(Box::leak(
                    (0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>(),
                )));
                glyph_buf_size = size_needed;
            }
            glyph_size = (unsafe {
                let _dst: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                let _offset: u64 = glyph_size;
                let _x: i32 = (n_contours as i32);
                Store16_13(_dst, _offset, _x)
            });
            if have_bbox {
                if ((!(unsafe {
                    let _data: *mut u8 = glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .offset((glyph_size) as isize);
                    let _n_bytes: u64 = 8_u64;
                    bbox_stream.Read(_data, _n_bytes)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                (unsafe {
                    let _n_points: u32 = total_n_points;
                    let _points: *const woff2_Point = points
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .cast_const();
                    let _dst: *mut u8 = glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                    ComputeBbox_21(_n_points, _points, _dst)
                });
            }
            glyph_size = woff2_kEndPtsOfContoursOffset;
            let mut end_point: i32 = -1_i32;
            let mut contour_ix: u32 = 0_u32;
            'loop_: while ((contour_ix) < (n_contours as u32)) {
                end_point = ((end_point as u32)
                    .wrapping_add(n_points_vec[(contour_ix as u64) as usize]))
                    as i32;
                if ((((end_point) >= (65536)) as i64) != 0) {
                    return false;
                }
                glyph_size = (unsafe {
                    let _dst: *mut u8 = glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                    let _offset: u64 = glyph_size;
                    let _x: i32 = end_point;
                    Store16_13(_dst, _offset, _x)
                });
                contour_ix.prefix_inc();
            }
            glyph_size = (unsafe {
                let _dst: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                let _offset: u64 = glyph_size;
                let _x: i32 = (instruction_size as i32);
                Store16_13(_dst, _offset, _x)
            });
            if ((!(unsafe {
                let _data: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .offset((glyph_size) as isize);
                let _n_bytes: u64 = (instruction_size as u64);
                instruction_stream.Read(_data, _n_bytes)
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add((instruction_size as u64));
            let mut has_overlap_bit: bool = (has_overlap_bitmap)
                && ((((*overlap_bitmap.offset(((i) >> (3)) as isize)) as i32)
                    & ((128) >> ((i) & (7_u32))))
                    != 0);
            if ((!(unsafe {
                let _n_points: u32 = total_n_points;
                let _points: *const woff2_Point = points
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .cast_const();
                let _n_contours: u32 = (n_contours as u32);
                let _instruction_length: u32 = instruction_size;
                let _has_overlap_bit: bool = has_overlap_bit;
                let _dst: *mut u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
                let _dst_size: u64 = glyph_buf_size;
                let _glyph_size: *mut u64 = (&mut glyph_size as *mut u64);
                StorePoints_20(
                    _n_points,
                    _points,
                    _n_contours,
                    _instruction_length,
                    _has_overlap_bit,
                    _dst,
                    _dst_size,
                    _glyph_size,
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            if ((have_bbox as i64) != 0) {
                printf(b"Empty glyph has a bbox\n\0".as_ptr() as *const i8);
                return false;
            }
        }
        loca_values[(i as u64) as usize] =
            (((unsafe { (*out).Size() }).wrapping_sub(glyf_start)) as u32);
        if ((!(unsafe {
            let _buf: *const ::libc::c_void = (glyph_buf
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                as *const u8
                as *const ::libc::c_void);
            let _n: u64 = glyph_size;
            (*out).Write_pconstlibcc_void_u64(_buf, _n)
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((!(unsafe {
            let _out: *mut dyn woff2_WOFF2Out = out;
            Pad4_23(_out)
        }) as i64)
            != 0)
        {
            return false;
        }
        (*glyf_checksum) = (*glyf_checksum).wrapping_add(
            (unsafe {
                let _buf: *const u8 = glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .cast_const();
                let _size: u64 = glyph_size;
                ComputeULongSum_8(_buf, _size)
            }),
        );
        if ((n_contours as i32) > (0)) {
            let mut x_min_buf: woff2_Buffer = woff2_Buffer::woff2_Buffer(
                glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .offset((2) as isize)
                    .cast_const(),
                2_u64,
            );
            if ((!(unsafe {
                let _value: *mut i16 =
                    (&mut (&mut (*info)).x_mins[(i as u64) as usize] as *mut i16);
                x_min_buf.ReadS16(_value)
            }) as i64)
                != 0)
            {
                return false;
            }
        }
        i.prefix_inc();
    }
    (*glyf_table).dst_length =
        (((unsafe { (*out).Size() }).wrapping_sub(((*glyf_table).dst_offset as u64))) as u32);
    (*loca_table).dst_offset = ((unsafe { (*out).Size() }) as u32).clone();
    loca_values[((*info).num_glyphs as u64) as usize] = (*glyf_table).dst_length;
    if ((!(unsafe {
        let _loca_values: *const Vec<u32> = &loca_values as *const Vec<u32>;
        let _index_format: i32 = ((*info).index_format as i32);
        let _checksum: *mut u32 = loca_checksum;
        let _out: *mut dyn woff2_WOFF2Out = out;
        StoreLoca_24(_loca_values, _index_format, _checksum, _out)
    }) as i64)
        != 0)
    {
        return false;
    }
    (*loca_table).dst_length =
        (((unsafe { (*out).Size() }).wrapping_sub(((*loca_table).dst_offset as u64))) as u32);
    return true;
}
pub unsafe fn FindTable_26(
    mut tables: *mut Vec<*mut woff2_Table>,
    mut tag: u32,
) -> *mut woff2_Table {
    'loop_: for table in 0..((*tables).len()) {
        let mut table = (&(*tables))[table].clone();
        if (((*table).tag) == (tag)) {
            return table;
        }
    }
    return Default::default();
}
pub unsafe fn ReadNumHMetrics_27(
    mut data: *const u8,
    mut data_size: u64,
    mut num_hmetrics: *mut u16,
) -> bool {
    let mut buffer: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, data_size);
    if ((((!(unsafe {
        let _n_bytes: u64 = 34_u64;
        buffer.Skip(_n_bytes)
    })) || (!(unsafe {
        let _value: *mut u16 = num_hmetrics;
        buffer.ReadU16(_value)
    }))) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn ReconstructTransformedHmtx_28(
    mut transformed_buf: *const u8,
    mut transformed_size: u64,
    mut num_glyphs: u16,
    mut num_hmetrics: u16,
    x_mins: *const Vec<i16>,
    mut checksum: *mut u32,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut hmtx_buff_in: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(transformed_buf, transformed_size);
    let mut hmtx_flags: u8 = 0_u8;
    if ((!(unsafe {
        let _value: *mut u8 = (&mut hmtx_flags as *mut u8);
        hmtx_buff_in.ReadU8(_value)
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut lsbs: Vec<i16> = Vec::new();
    let mut has_proportional_lsbs: bool = (((hmtx_flags as i32) & (1)) == (0));
    let mut has_monospace_lsbs: bool = (((hmtx_flags as i32) & (2)) == (0));
    if (((hmtx_flags as i32) & (252)) != (0)) {
        printf(b"Illegal hmtx flags; bits 2-7 must be 0\n\0".as_ptr() as *const i8);
        return false;
    }
    if (has_proportional_lsbs) && (has_monospace_lsbs) {
        return false;
    }
    assert!((((*x_mins).len() as u64) == (num_glyphs as u64)));
    if ((((num_hmetrics as i32) > (num_glyphs as i32)) as i64) != 0) {
        return false;
    }
    if ((((num_hmetrics as i32) < (1)) as i64) != 0) {
        return false;
    }
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < (num_hmetrics as i32)) {
        let mut advance_width: u16 = 0_u16;
        if ((!(unsafe {
            let _value: *mut u16 = (&mut advance_width as *mut u16);
            hmtx_buff_in.ReadU16(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        {
            let a0_clone = advance_width.clone();
            advance_widths.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < (num_hmetrics as i32)) {
        let mut lsb: i16 = 0_i16;
        if has_proportional_lsbs {
            if ((!(unsafe {
                let _value: *mut i16 = (&mut lsb as *mut i16);
                hmtx_buff_in.ReadS16(_value)
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            lsb = (&(*x_mins))[(i as u64) as usize];
        }
        {
            let a0_clone = lsb.clone();
            lsbs.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut i: u16 = num_hmetrics;
    'loop_: while ((i as i32) < (num_glyphs as i32)) {
        let mut lsb: i16 = 0_i16;
        if has_monospace_lsbs {
            if ((!(unsafe {
                let _value: *mut i16 = (&mut lsb as *mut i16);
                hmtx_buff_in.ReadS16(_value)
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            lsb = (&(*x_mins))[(i as u64) as usize];
        }
        {
            let a0_clone = lsb.clone();
            lsbs.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut hmtx_output_size: u32 =
        ((((2) * (num_glyphs as i32)) + ((2) * (num_hmetrics as i32))) as u32);
    let mut hmtx_table: Vec<u8> = (0..(hmtx_output_size as u64) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut dst: *mut u8 = (&mut hmtx_table[(0_u64) as usize] as *mut u8);
    let mut dst_offset: u64 = 0_u64;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (num_glyphs as u32)) {
        if ((i) < (num_hmetrics as u32)) {
            (unsafe {
                let _val: i32 = (advance_widths[(i as u64) as usize] as i32);
                let _offset: *mut u64 = (&mut dst_offset as *mut u64);
                let _dst: *mut u8 = dst;
                Store16_15(_val, _offset, _dst)
            });
        }
        (unsafe {
            let _val: i32 = (lsbs[(i as u64) as usize] as i32);
            let _offset: *mut u64 = (&mut dst_offset as *mut u64);
            let _dst: *mut u8 = dst;
            Store16_15(_val, _offset, _dst)
        });
        i.postfix_inc();
    }
    (*checksum) = (unsafe {
        let _buf: *const u8 = (&mut hmtx_table[(0_u64) as usize] as *mut u8).cast_const();
        let _size: u64 = (hmtx_output_size as u64);
        ComputeULongSum_8(_buf, _size)
    });
    if ((!(unsafe {
        let _buf: *const ::libc::c_void =
            ((&mut hmtx_table[(0_u64) as usize] as *mut u8) as *const u8 as *const ::libc::c_void);
        let _n: u64 = (hmtx_output_size as u64);
        (*out).Write_pconstlibcc_void_u64(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn Woff2Uncompress_29(
    mut dst_buf: *mut u8,
    mut dst_size: u64,
    mut src_buf: *const u8,
    mut src_size: u64,
) -> bool {
    let mut uncompressed_size: u64 = dst_size;
    let mut result: ::brotli_sys::BrotliDecoderResult = ::brotli_sys::BrotliDecoderDecompress(
        src_size as usize,
        src_buf,
        (&mut uncompressed_size as *mut u64) as *mut usize,
        dst_buf,
    );
    if (((((result as i32) != (::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS as i32))
        || ((uncompressed_size) != (dst_size))) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn ReadTableDirectory_30(
    mut file: *mut woff2_Buffer,
    mut tables: *mut Vec<woff2_Table>,
    mut num_tables: u64,
) -> bool {
    let mut src_offset: u32 = 0_u32;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (num_tables)) {
        let mut table: *mut woff2_Table = (&mut (&mut (*tables))[(i) as usize] as *mut woff2_Table);
        let mut flag_byte: u8 = 0_u8;
        if ((!(unsafe {
            let _value: *mut u8 = (&mut flag_byte as *mut u8);
            (*file).ReadU8(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        let mut tag: u32 = 0_u32;
        if (((flag_byte as i32) & (63)) == (63)) {
            if ((!(unsafe {
                let _value: *mut u32 = (&mut tag as *mut u32);
                (*file).ReadU32(_value)
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            tag = woff2_kKnownTags[((flag_byte as i32) & (63)) as usize];
        }
        let mut flags: u32 = 0_u32;
        let mut xform_version: u8 = ((((flag_byte as i32) >> (6)) & (3)) as u8);
        if ((tag) == (woff2_kGlyfTableTag)) || ((tag) == (woff2_kLocaTableTag)) {
            if ((xform_version as i32) == (0)) {
                flags = ((flags as u32) | woff2_kWoff2FlagsTransform) as u32;
            }
        } else if ((xform_version as i32) != (0)) {
            flags = ((flags as u32) | woff2_kWoff2FlagsTransform) as u32;
        }
        flags |= (xform_version as u32);
        let mut dst_length: u32 = 0_u32;
        if ((!(unsafe {
            let _buf: *mut woff2_Buffer = file;
            let _value: *mut u32 = (&mut dst_length as *mut u32);
            ReadBase128_4(_buf, _value)
        }) as i64)
            != 0)
        {
            return false;
        }
        let mut transform_length: u32 = dst_length;
        if (((flags) & (woff2_kWoff2FlagsTransform)) != (0_u32)) {
            if ((!(unsafe {
                let _buf: *mut woff2_Buffer = file;
                let _value: *mut u32 = (&mut transform_length as *mut u32);
                ReadBase128_4(_buf, _value)
            }) as i64)
                != 0)
            {
                return false;
            }
            if (((((tag) == (woff2_kLocaTableTag)) && (transform_length != 0)) as i64) != 0) {
                return false;
            }
        }
        if (((((src_offset).wrapping_add(transform_length)) < (src_offset)) as i64) != 0) {
            return false;
        }
        (*table).src_offset = src_offset;
        (*table).src_length = transform_length;
        src_offset = (src_offset).wrapping_add(transform_length);
        (*table).tag = tag;
        (*table).flags = flags;
        (*table).transform_length = transform_length;
        (*table).dst_length = dst_length;
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn StoreOffsetTable_31(
    mut result: *mut u8,
    mut offset: u64,
    mut flavor: u32,
    mut num_tables: u16,
) -> u64 {
    offset = (unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = offset;
        let _x: u32 = flavor;
        StoreU32_12(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = offset;
        let _x: i32 = (num_tables as i32);
        Store16_13(_dst, _offset, _x)
    });
    let mut max_pow2: u32 = 0_u32;
    'loop_: while (((1_u32) << ((max_pow2).wrapping_add(1_u32))) <= (num_tables as u32)) {
        max_pow2.postfix_inc();
    }
    let output_search_range: u16 = ((((1_u32) << (max_pow2)) << (4)) as u16);
    offset = (unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = offset;
        let _x: i32 = (output_search_range as i32);
        Store16_13(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = offset;
        let _x: i32 = (max_pow2 as i32);
        Store16_13(_dst, _offset, _x)
    });
    offset = (unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = offset;
        let _x: i32 = (((num_tables as i32) << (4)) - (output_search_range as i32));
        Store16_13(_dst, _offset, _x)
    });
    return offset;
}
pub unsafe fn StoreTableEntry_32(mut result: *mut u8, mut offset: u32, mut tag: u32) -> u64 {
    offset = ((unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = (offset as u64);
        let _x: u32 = tag;
        StoreU32_12(_dst, _offset, _x)
    }) as u32);
    offset = ((unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = (offset as u64);
        let _x: u32 = 0_u32;
        StoreU32_12(_dst, _offset, _x)
    }) as u32);
    offset = ((unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = (offset as u64);
        let _x: u32 = 0_u32;
        StoreU32_12(_dst, _offset, _x)
    }) as u32);
    offset = ((unsafe {
        let _dst: *mut u8 = result;
        let _offset: u64 = (offset as u64);
        let _x: u32 = 0_u32;
        StoreU32_12(_dst, _offset, _x)
    }) as u32);
    return (offset as u64);
}
pub unsafe fn ComputeOffsetToFirstTable_33(hdr: *const woff2_WOFF2Header) -> u64 {
    let mut offset: u64 = (woff2_kSfntHeaderSize)
        .wrapping_add((woff2_kSfntEntrySize).wrapping_mul(((*hdr).num_tables as u64)));
    if ((*hdr).header_version != 0) {
        offset = ((unsafe {
            let _header_version: u32 = (*hdr).header_version;
            let _num_fonts: u32 = ((*hdr).ttc_fonts.len() as u64 as u32);
            CollectionHeaderSize_9(_header_version, _num_fonts)
        })
        .wrapping_add((woff2_kSfntHeaderSize).wrapping_mul((*hdr).ttc_fonts.len() as u64)))
        .clone();
        'loop_: for ttc_font in 0..((*hdr).ttc_fonts.len()) {
            let mut ttc_font = (*hdr).ttc_fonts.as_ptr().add(ttc_font);
            offset = ((offset as u64).wrapping_add(
                (woff2_kSfntEntrySize).wrapping_mul((*ttc_font).table_indices.len() as u64),
            )) as u64;
        }
    }
    return offset;
}
pub unsafe fn Tables_34(
    mut hdr: *mut woff2_WOFF2Header,
    mut font_index: u64,
) -> Vec<*mut woff2_Table> {
    let mut tables: Vec<*mut woff2_Table> = Vec::new();
    if (((*hdr).header_version as i64) != 0) {
        'loop_: for index in 0..((&mut (*hdr)).ttc_fonts[(font_index) as usize]
            .table_indices
            .len())
        {
            let mut index =
                (&mut (*hdr)).ttc_fonts[(font_index) as usize].table_indices[index].clone();
            tables.push((&mut (&mut (*hdr)).tables[(index as u64) as usize] as *mut woff2_Table));
        }
    } else {
        'loop_: for table in 0..((*hdr).tables.len()) {
            let mut table = (*hdr).tables.as_mut_ptr().add(table);
            tables.push((table));
        }
    }
    return tables;
}
pub unsafe fn ReconstructFont_35(
    mut transformed_buf: *mut u8,
    transformed_buf_size: u32,
    mut metadata: *mut woff2_RebuildMetadata,
    mut hdr: *mut woff2_WOFF2Header,
    mut font_index: u64,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut dest_offset: u64 = (unsafe { (*out).Size() });
    let mut table_entry: [u8; 12] = [0_u8; 12];
    let mut info: *mut woff2_WOFF2FontInfo =
        (&mut (&mut (*metadata)).font_infos[(font_index) as usize] as *mut woff2_WOFF2FontInfo);
    let mut tables: Vec<*mut woff2_Table> = (unsafe {
        let _hdr: *mut woff2_WOFF2Header = hdr;
        let _font_index: u64 = font_index;
        Tables_34(_hdr, _font_index)
    });
    let mut glyf_table: *const woff2_Table = (unsafe {
        let _tables: *mut Vec<*mut woff2_Table> = (&mut tables as *mut Vec<*mut woff2_Table>);
        let _tag: u32 = woff2_kGlyfTableTag;
        FindTable_26(_tables, _tag)
    })
    .cast_const();
    let mut loca_table: *const woff2_Table = (unsafe {
        let _tables: *mut Vec<*mut woff2_Table> = (&mut tables as *mut Vec<*mut woff2_Table>);
        let _tag: u32 = woff2_kLocaTableTag;
        FindTable_26(_tables, _tag)
    })
    .cast_const();
    if ((((!(glyf_table).is_null() as i32) != (!(loca_table).is_null() as i32)) as i64) != 0) {
        printf(b"Cannot have just one of glyf/loca\n\0".as_ptr() as *const i8);
        return false;
    }
    if ((glyf_table) != (Default::default())) {
        if ((((((*glyf_table).flags) & (woff2_kWoff2FlagsTransform))
            != (((*loca_table).flags) & (woff2_kWoff2FlagsTransform))) as i64)
            != 0)
        {
            printf(b"Cannot transform just one of glyf/loca\n\0".as_ptr() as *const i8);
            return false;
        }
    }
    let mut font_checksum: u32 = (*metadata).header_checksum;
    if ((*hdr).header_version != 0) {
        font_checksum = (&mut (*hdr)).ttc_fonts[(font_index) as usize].header_checksum;
    }
    let mut loca_checksum: u32 = 0_u32;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (tables.len() as u64)) {
        let table: *mut woff2_Table = &mut (*tables[(i) as usize]) as *mut woff2_Table;
        let mut checksum_key: (u32, u32) = ((*table).tag.into(), (*table).src_offset.into());
        let mut reused: bool = UnsafeMapIterator::find_key(
            &(*metadata).checksums as *const BTreeMap<(u32, u32), Box<u32>>,
            &checksum_key,
        ) != UnsafeMapIterator::end(
            &(*metadata).checksums as *const BTreeMap<(u32, u32), Box<u32>>,
        );
        if (((((font_index) == (0_u64)) && (reused)) as i64) != 0) {
            return false;
        }
        if ((((((*table).src_offset as u64).wrapping_add(((*table).src_length as u64)))
            > (transformed_buf_size as u64)) as i64)
            != 0)
        {
            return false;
        }
        if (((*table).tag) == (woff2_kHheaTableTag)) {
            if !(unsafe {
                let _data: *const u8 = transformed_buf
                    .offset(((*table).src_offset) as isize)
                    .cast_const();
                let _data_size: u64 = ((*table).src_length as u64);
                let _num_hmetrics: *mut u16 = (&mut (*info).num_hmetrics as *mut u16);
                ReadNumHMetrics_27(_data, _data_size, _num_hmetrics)
            }) {
                return false;
            }
        }
        let mut checksum: u32 = 0_u32;
        if !reused {
            if ((((*table).flags) & (woff2_kWoff2FlagsTransform)) != (woff2_kWoff2FlagsTransform)) {
                if (((*table).tag) == (woff2_kHeadTableTag)) {
                    if (((((*table).src_length) < (12_u32)) as i64) != 0) {
                        return false;
                    }
                    (unsafe {
                        let _dst: *mut u8 = transformed_buf.offset(((*table).src_offset) as isize);
                        let _offset: u64 = 8_u64;
                        let _x: u32 = 0_u32;
                        StoreU32_12(_dst, _offset, _x)
                    });
                }
                (*table).dst_offset = (dest_offset as u32);
                checksum = (unsafe {
                    let _buf: *const u8 = transformed_buf
                        .offset(((*table).src_offset) as isize)
                        .cast_const();
                    let _size: u64 = ((*table).src_length as u64);
                    ComputeULongSum_8(_buf, _size)
                });
                if ((!(unsafe {
                    let _buf: *const ::libc::c_void =
                        (transformed_buf.offset(((*table).src_offset) as isize) as *const u8
                            as *const ::libc::c_void);
                    let _n: u64 = ((*table).src_length as u64);
                    (*out).Write_pconstlibcc_void_u64(_buf, _n)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                if (((*table).tag) == (woff2_kGlyfTableTag)) {
                    (*table).dst_offset = (dest_offset as u32);
                    let mut loca_table: *mut woff2_Table = (unsafe {
                        let _tables: *mut Vec<*mut woff2_Table> =
                            (&mut tables as *mut Vec<*mut woff2_Table>);
                        let _tag: u32 = woff2_kLocaTableTag;
                        FindTable_26(_tables, _tag)
                    });
                    if ((!(unsafe {
                        let _data: *const u8 = transformed_buf
                            .offset(((*table).src_offset) as isize)
                            .cast_const();
                        let _glyf_table: *mut woff2_Table = (table);
                        let _glyf_checksum: *mut u32 = (&mut checksum as *mut u32);
                        let _loca_table: *mut woff2_Table = loca_table;
                        let _loca_checksum: *mut u32 = (&mut loca_checksum as *mut u32);
                        let _info: *mut woff2_WOFF2FontInfo = info;
                        let _out: *mut dyn woff2_WOFF2Out = out;
                        ReconstructGlyf_25(
                            _data,
                            _glyf_table,
                            _glyf_checksum,
                            _loca_table,
                            _loca_checksum,
                            _info,
                            _out,
                        )
                    }) as i64)
                        != 0)
                    {
                        return false;
                    }
                } else if (((*table).tag) == (woff2_kLocaTableTag)) {
                    checksum = loca_checksum;
                } else if (((*table).tag) == (woff2_kHmtxTableTag)) {
                    (*table).dst_offset = (dest_offset as u32);
                    if ((!(unsafe {
                        let _transformed_buf: *const u8 = transformed_buf
                            .offset(((*table).src_offset) as isize)
                            .cast_const();
                        let _transformed_size: u64 = ((*table).src_length as u64);
                        let _num_glyphs: u16 = (*info).num_glyphs;
                        let _num_hmetrics: u16 = (*info).num_hmetrics;
                        let _x_mins: *const Vec<i16> = &(*info).x_mins as *const Vec<i16>;
                        let _checksum: *mut u32 = (&mut checksum as *mut u32);
                        let _out: *mut dyn woff2_WOFF2Out = out;
                        ReconstructTransformedHmtx_28(
                            _transformed_buf,
                            _transformed_size,
                            _num_glyphs,
                            _num_hmetrics,
                            _x_mins,
                            _checksum,
                            _out,
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
            (*(*metadata)
                .checksums
                .entry(checksum_key)
                .or_default()
                .as_mut()) = checksum;
        } else {
            checksum = (*(*metadata)
                .checksums
                .entry(checksum_key)
                .or_default()
                .as_mut());
        }
        font_checksum = (font_checksum).wrapping_add(checksum);
        (unsafe {
            let _dst: *mut u8 = table_entry.as_mut_ptr();
            let _offset: u64 = 0_u64;
            let _x: u32 = checksum;
            StoreU32_12(_dst, _offset, _x)
        });
        (unsafe {
            let _dst: *mut u8 = table_entry.as_mut_ptr();
            let _offset: u64 = 4_u64;
            let _x: u32 = (*table).dst_offset;
            StoreU32_12(_dst, _offset, _x)
        });
        (unsafe {
            let _dst: *mut u8 = table_entry.as_mut_ptr();
            let _offset: u64 = 8_u64;
            let _x: u32 = (*table).dst_length;
            StoreU32_12(_dst, _offset, _x)
        });
        if ((!(unsafe {
            let _buf: *const ::libc::c_void =
                (table_entry.as_mut_ptr() as *const u8 as *const ::libc::c_void);
            let _offset: u64 = (((*(*info)
                .table_entry_by_tag
                .entry((*table).tag)
                .or_default()
                .as_mut())
            .wrapping_add(4_u32)) as u64);
            let _n: u64 = 12_u64;
            (*out).Write_pconstlibcc_void_u64_u64(_buf, _offset, _n)
        }) as i64)
            != 0)
        {
            return false;
        }
        font_checksum = (font_checksum).wrapping_add(
            (unsafe {
                let _buf: *const u8 = table_entry.as_mut_ptr().cast_const();
                let _size: u64 = 12_u64;
                ComputeULongSum_8(_buf, _size)
            }),
        );
        if ((!(unsafe {
            let _out: *mut dyn woff2_WOFF2Out = out;
            Pad4_23(_out)
        }) as i64)
            != 0)
        {
            return false;
        }
        if (((((((*table).dst_offset).wrapping_add((*table).dst_length)) as u64)
            > (unsafe { (*out).Size() })) as i64)
            != 0)
        {
            return false;
        }
        dest_offset = (unsafe { (*out).Size() }).clone();
        i.postfix_inc();
    }
    let mut head_table: *mut woff2_Table = (unsafe {
        let _tables: *mut Vec<*mut woff2_Table> = (&mut tables as *mut Vec<*mut woff2_Table>);
        let _tag: u32 = woff2_kHeadTableTag;
        FindTable_26(_tables, _tag)
    });
    if !(head_table).is_null() {
        if (((((*head_table).dst_length) < (12_u32)) as i64) != 0) {
            return false;
        }
        let mut checksum_adjustment: [u8; 4] = [0_u8; 4];
        (unsafe {
            let _dst: *mut u8 = checksum_adjustment.as_mut_ptr();
            let _offset: u64 = 0_u64;
            let _x: u32 = (2981146554_u32 as u32).wrapping_sub(font_checksum);
            StoreU32_12(_dst, _offset, _x)
        });
        if ((!(unsafe {
            let _buf: *const ::libc::c_void =
                (checksum_adjustment.as_mut_ptr() as *const u8 as *const ::libc::c_void);
            let _offset: u64 = ((((*head_table).dst_offset).wrapping_add(8_u32)) as u64);
            let _n: u64 = 4_u64;
            (*out).Write_pconstlibcc_void_u64_u64(_buf, _offset, _n)
        }) as i64)
            != 0)
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn ReadWOFF2Header_36(
    mut data: *const u8,
    mut length: u64,
    mut hdr: *mut woff2_WOFF2Header,
) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, length);
    let mut signature: u32 = 0_u32;
    if (((((!(unsafe {
        let _value: *mut u32 = (&mut signature as *mut u32);
        file.ReadU32(_value)
    })) || ((signature) != (woff2_kWoff2Signature)))
        || (!(unsafe {
            let _value: *mut u32 = (&mut (*hdr).flavor as *mut u32);
            file.ReadU32(_value)
        }))) as i64)
        != 0)
    {
        return false;
    }
    let mut reported_length: u32 = 0_u32;
    if ((((!(unsafe {
        let _value: *mut u32 = (&mut reported_length as *mut u32);
        file.ReadU32(_value)
    })) || ((length) != (reported_length as u64))) as i64)
        != 0)
    {
        return false;
    }
    if ((((!(unsafe {
        let _value: *mut u16 = (&mut (*hdr).num_tables as *mut u16);
        file.ReadU16(_value)
    })) || (!((*hdr).num_tables != 0))) as i64)
        != 0)
    {
        return false;
    }
    if ((!(unsafe {
        let _n_bytes: u64 = 6_u64;
        file.Skip(_n_bytes)
    }) as i64)
        != 0)
    {
        return false;
    }
    if ((!(unsafe {
        let _value: *mut u32 = (&mut (*hdr).compressed_length as *mut u32);
        file.ReadU32(_value)
    }) as i64)
        != 0)
    {
        return false;
    }
    if ((!(unsafe {
        let _n_bytes: u64 = (((2) * (2)) as u64);
        file.Skip(_n_bytes)
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut meta_offset: u32 = 0_u32;
    let mut meta_length: u32 = 0_u32;
    let mut meta_length_orig: u32 = 0_u32;
    if (((((!(unsafe {
        let _value: *mut u32 = (&mut meta_offset as *mut u32);
        file.ReadU32(_value)
    })) || (!(unsafe {
        let _value: *mut u32 = (&mut meta_length as *mut u32);
        file.ReadU32(_value)
    }))) || (!(unsafe {
        let _value: *mut u32 = (&mut meta_length_orig as *mut u32);
        file.ReadU32(_value)
    }))) as i64)
        != 0)
    {
        return false;
    }
    if (meta_offset != 0) {
        if (((((meta_offset as u64) >= (length))
            || (((length).wrapping_sub((meta_offset as u64))) < (meta_length as u64)))
            as i64)
            != 0)
        {
            return false;
        }
    }
    let mut priv_offset: u32 = 0_u32;
    let mut priv_length: u32 = 0_u32;
    if ((((!(unsafe {
        let _value: *mut u32 = (&mut priv_offset as *mut u32);
        file.ReadU32(_value)
    })) || (!(unsafe {
        let _value: *mut u32 = (&mut priv_length as *mut u32);
        file.ReadU32(_value)
    }))) as i64)
        != 0)
    {
        return false;
    }
    if (priv_offset != 0) {
        if (((((priv_offset as u64) >= (length))
            || (((length).wrapping_sub((priv_offset as u64))) < (priv_length as u64)))
            as i64)
            != 0)
        {
            return false;
        }
    }
    {
        let __a0 = ((*hdr).num_tables as u64) as usize;
        (*hdr).tables.resize_with(__a0, || <woff2_Table>::default())
    };
    if ((!(unsafe {
        let _file: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
        let _tables: *mut Vec<woff2_Table> = (&mut (*hdr).tables as *mut Vec<woff2_Table>);
        let _num_tables: u64 = ((*hdr).num_tables as u64);
        ReadTableDirectory_30(_file, _tables, _num_tables)
    }) as i64)
        != 0)
    {
        return false;
    }
    let last_table: *mut woff2_Table = (((*hdr).tables).last_mut().unwrap());
    (*hdr).uncompressed_size = ((*last_table).src_offset).wrapping_add((*last_table).src_length);
    if (((((*hdr).uncompressed_size) < ((*last_table).src_offset)) as i64) != 0) {
        return false;
    }
    (*hdr).header_version = 0_u32;
    if (((*hdr).flavor) == (woff2_kTtcFontFlavor)) {
        if ((!(unsafe {
            let _value: *mut u32 = (&mut (*hdr).header_version as *mut u32);
            file.ReadU32(_value)
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((((((*hdr).header_version) != (65536_u32)) && (((*hdr).header_version) != (131072_u32)))
            as i64)
            != 0)
        {
            return false;
        }
        let mut num_fonts: u32 = 0_u32;
        if ((((!(unsafe {
            let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
            let _value: *mut u32 = (&mut num_fonts as *mut u32);
            Read255UShort_3(_buf, _value)
        })) || (!(num_fonts != 0))) as i64)
            != 0)
        {
            return false;
        }
        {
            let __a0 = (num_fonts as u64) as usize;
            (*hdr)
                .ttc_fonts
                .resize_with(__a0, || <woff2_TtcFont>::default())
        };
        let mut i: u32 = 0_u32;
        'loop_: while ((i) < (num_fonts)) {
            let ttc_font: *mut woff2_TtcFont =
                &mut (&mut (*hdr)).ttc_fonts[(i as u64) as usize] as *mut woff2_TtcFont;
            let mut num_tables: u32 = 0_u32;
            if ((((!(unsafe {
                let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                let _value: *mut u32 = (&mut num_tables as *mut u32);
                Read255UShort_3(_buf, _value)
            })) || (!(num_tables != 0))) as i64)
                != 0)
            {
                return false;
            }
            if ((!(unsafe {
                let _value: *mut u32 = (&mut (*ttc_font).flavor as *mut u32);
                file.ReadU32(_value)
            }) as i64)
                != 0)
            {
                return false;
            }
            {
                let __a0 = (num_tables as u64) as usize;
                (*ttc_font)
                    .table_indices
                    .resize_with(__a0, || <u16>::default())
            };
            let mut glyf_idx: u32 = 0_u32;
            let mut loca_idx: u32 = 0_u32;
            let mut j: u32 = 0_u32;
            'loop_: while ((j) < (num_tables)) {
                let mut table_idx: u32 = 0_u32;
                if ((!(unsafe {
                    let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                    let _value: *mut u32 = (&mut table_idx as *mut u32);
                    Read255UShort_3(_buf, _value)
                }) as i64)
                    != 0)
                    || ((table_idx as u64) >= ((*hdr).tables.len() as u64))
                {
                    return false;
                }
                (&mut (*ttc_font)).table_indices[(j as u64) as usize] = (table_idx as u16);
                let table: *const woff2_Table =
                    &(&mut (*hdr)).tables[(table_idx as u64) as usize] as *const woff2_Table;
                if (((*table).tag) == (woff2_kLocaTableTag)) {
                    loca_idx = table_idx;
                }
                if (((*table).tag) == (woff2_kGlyfTableTag)) {
                    glyf_idx = table_idx;
                }
                j.postfix_inc();
            }
            if ((glyf_idx) > (0_u32)) || ((loca_idx) > (0_u32)) {
                if (((((glyf_idx) > (loca_idx)) || (((loca_idx).wrapping_sub(glyf_idx)) != (1_u32)))
                    as i64)
                    != 0)
                {
                    printf(
                        b"TTC font %d has non-consecutive glyf/loca\n\0".as_ptr() as *const i8,
                        i,
                    );
                    return false;
                }
            }
            i.postfix_inc();
        }
    }
    let first_table_offset: u64 = (unsafe {
        let _hdr: *const woff2_WOFF2Header = &(*hdr) as *const woff2_WOFF2Header;
        ComputeOffsetToFirstTable_33(_hdr)
    });
    (*hdr).compressed_offset = (unsafe { file.offset() }).clone();
    if (((((*hdr).compressed_offset) > (<u32>::MAX as u64)) as i64) != 0) {
        return false;
    }
    let mut src_offset: u64 = (unsafe {
        let _value: u64 =
            ((*hdr).compressed_offset).wrapping_add(((*hdr).compressed_length as u64));
        Round4_10(_value)
    });
    let mut dst_offset: u64 = first_table_offset;
    if ((((src_offset) > (length)) as i64) != 0) {
        printf(
            b"offset fail; src_offset %lu length %lu dst_offset %lu\n\0".as_ptr() as *const i8,
            src_offset,
            length,
            dst_offset,
        );
        return false;
    }
    if (meta_offset != 0) {
        if ((((src_offset) != (meta_offset as u64)) as i64) != 0) {
            return false;
        }
        src_offset = ((unsafe {
            let _value: u32 = (meta_offset).wrapping_add(meta_length);
            Round4_11(_value)
        }) as u64);
        if ((((src_offset) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if (priv_offset != 0) {
        if ((((src_offset) != (priv_offset as u64)) as i64) != 0) {
            return false;
        }
        src_offset = ((unsafe {
            let _value: u32 = (priv_offset).wrapping_add(priv_length);
            Round4_11(_value)
        }) as u64);
        if ((((src_offset) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if ((((src_offset)
        != (unsafe {
            let _value: u64 = length;
            Round4_10(_value)
        })) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn WriteHeaders_37(
    mut data: *const u8,
    mut length: u64,
    mut metadata: *mut woff2_RebuildMetadata,
    mut hdr: *mut woff2_WOFF2Header,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut output: Vec<u8> = vec![
        0_u8;
        (unsafe {
            let _hdr: *const woff2_WOFF2Header = &(*hdr) as *const woff2_WOFF2Header;
            ComputeOffsetToFirstTable_33(_hdr)
        }) as usize
    ];
    let mut sorted_tables: Vec<woff2_Table> = (*hdr).tables.clone();
    if ((*hdr).header_version != 0) {
        'loop_: for ttc_font in 0..((*hdr).ttc_fonts.len()) {
            let mut ttc_font = (*hdr).ttc_fonts.as_mut_ptr().add(ttc_font);
            let mut sorted_index_by_tag: BTreeMap<u32, Box<u16>> = BTreeMap::new();
            'loop_: for table_index in 0..((*ttc_font).table_indices.len()) {
                let mut table_index = (&(*ttc_font)).table_indices[table_index].clone();
                (*sorted_index_by_tag
                    .entry((&mut (*hdr)).tables[(table_index as u64) as usize].tag)
                    .or_default()
                    .as_mut()) = table_index;
            }
            let mut index: u16 = 0_u16;
            'loop_: for i in
                UnsafeMapIterator::begin(&sorted_index_by_tag as *const BTreeMap<u32, Box<u16>>)
            {
                (&mut (*ttc_font)).table_indices[(index.postfix_inc() as u64) as usize] =
                    *i.second();
            }
        }
    } else {
        {
            let len = sorted_tables
                .as_mut_ptr()
                .add(sorted_tables.len())
                .offset_from(sorted_tables.as_mut_ptr()) as usize;
            ::std::slice::from_raw_parts_mut(sorted_tables.as_mut_ptr(), len).sort()
        };
    }
    let mut result: *mut u8 = (&mut output[(0_u64) as usize] as *mut u8);
    let mut offset: u64 = 0_u64;
    if ((*hdr).header_version != 0) {
        offset = (unsafe {
            let _dst: *mut u8 = result;
            let _offset: u64 = offset;
            let _x: u32 = (*hdr).flavor;
            StoreU32_12(_dst, _offset, _x)
        });
        offset = (unsafe {
            let _dst: *mut u8 = result;
            let _offset: u64 = offset;
            let _x: u32 = (*hdr).header_version;
            StoreU32_12(_dst, _offset, _x)
        });
        offset = (unsafe {
            let _dst: *mut u8 = result;
            let _offset: u64 = offset;
            let _x: u32 = ((*hdr).ttc_fonts.len() as u64 as u32);
            StoreU32_12(_dst, _offset, _x)
        })
        .clone();
        let mut offset_table: u64 = offset;
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < ((*hdr).ttc_fonts.len() as u64)) {
            offset = (unsafe {
                let _dst: *mut u8 = result;
                let _offset: u64 = offset;
                let _x: u32 = 0_u32;
                StoreU32_12(_dst, _offset, _x)
            });
            i.postfix_inc();
        }
        if (((*hdr).header_version) == (131072_u32)) {
            offset = (unsafe {
                let _dst: *mut u8 = result;
                let _offset: u64 = offset;
                let _x: u32 = 0_u32;
                StoreU32_12(_dst, _offset, _x)
            });
            offset = (unsafe {
                let _dst: *mut u8 = result;
                let _offset: u64 = offset;
                let _x: u32 = 0_u32;
                StoreU32_12(_dst, _offset, _x)
            });
            offset = (unsafe {
                let _dst: *mut u8 = result;
                let _offset: u64 = offset;
                let _x: u32 = 0_u32;
                StoreU32_12(_dst, _offset, _x)
            });
        }
        {
            let __a0 = (*hdr).ttc_fonts.len() as u64 as usize;
            (*metadata)
                .font_infos
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let mut i: u64 = 0_u64;
        'loop_: while ((i) < ((*hdr).ttc_fonts.len() as u64)) {
            let ttc_font: *mut woff2_TtcFont =
                &mut (&mut (*hdr)).ttc_fonts[(i) as usize] as *mut woff2_TtcFont;
            offset_table = (unsafe {
                let _dst: *mut u8 = result;
                let _offset: u64 = offset_table;
                let _x: u32 = (offset as u32);
                StoreU32_12(_dst, _offset, _x)
            });
            (*ttc_font).dst_offset = (offset as u32);
            offset = (unsafe {
                let _result: *mut u8 = result;
                let _offset: u64 = offset;
                let _flavor: u32 = (*ttc_font).flavor;
                let _num_tables: u16 = ((*ttc_font).table_indices.len() as u64 as u16);
                StoreOffsetTable_31(_result, _offset, _flavor, _num_tables)
            })
            .clone();
            'loop_: for table_index in 0..((*ttc_font).table_indices.len()) {
                let table_index = (&(*ttc_font)).table_indices[table_index].clone();
                let mut tag: u32 = (&mut (*hdr)).tables[(table_index as u64) as usize].tag;
                (*(&mut (*metadata)).font_infos[(i) as usize]
                    .table_entry_by_tag
                    .entry(tag)
                    .or_default()
                    .as_mut()) = (offset as u32);
                offset = (unsafe {
                    let _result: *mut u8 = result;
                    let _offset: u32 = (offset as u32);
                    let _tag: u32 = tag;
                    StoreTableEntry_32(_result, _offset, _tag)
                });
            }
            (*ttc_font).header_checksum = (unsafe {
                let _buf: *const u8 =
                    (&mut output[((*ttc_font).dst_offset as u64) as usize] as *mut u8).cast_const();
                let _size: u64 = (offset).wrapping_sub(((*ttc_font).dst_offset as u64));
                ComputeULongSum_8(_buf, _size)
            });
            i.postfix_inc();
        }
    } else {
        {
            let __a0 = 1_u64 as usize;
            (*metadata)
                .font_infos
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        offset = (unsafe {
            let _result: *mut u8 = result;
            let _offset: u64 = offset;
            let _flavor: u32 = (*hdr).flavor;
            let _num_tables: u16 = (*hdr).num_tables;
            StoreOffsetTable_31(_result, _offset, _flavor, _num_tables)
        });
        let mut i: u16 = 0_u16;
        'loop_: while ((i as i32) < ((*hdr).num_tables as i32)) {
            (*(&mut (*metadata)).font_infos[(0_u64) as usize]
                .table_entry_by_tag
                .entry(sorted_tables[(i as u64) as usize].tag)
                .or_default()
                .as_mut()) = (offset as u32);
            offset = (unsafe {
                let _result: *mut u8 = result;
                let _offset: u32 = (offset as u32);
                let _tag: u32 = sorted_tables[(i as u64) as usize].tag;
                StoreTableEntry_32(_result, _offset, _tag)
            });
            i.prefix_inc();
        }
    }
    if ((!(unsafe {
        let _buf: *const ::libc::c_void =
            ((&mut output[(0_u64) as usize] as *mut u8) as *const u8 as *const ::libc::c_void);
        let _n: u64 = output.len() as u64;
        (*out).Write_pconstlibcc_void_u64(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    (*metadata).header_checksum = (unsafe {
        let _buf: *const u8 = (&mut output[(0_u64) as usize] as *mut u8).cast_const();
        let _size: u64 = output.len() as u64;
        ComputeULongSum_8(_buf, _size)
    });
    return true;
}
pub unsafe fn ComputeWOFF2FinalSize_38(mut data: *const u8, mut length: u64) -> u64 {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, length);
    let mut total_length: u32 = 0_u32;
    if (!(unsafe {
        let _n_bytes: u64 = 16_u64;
        file.Skip(_n_bytes)
    })) || (!(unsafe {
        let _value: *mut u32 = (&mut total_length as *mut u32);
        file.ReadU32(_value)
    })) {
        return 0_u64;
    }
    return (total_length as u64);
}
pub unsafe fn ConvertWOFF2ToTTF_39(
    mut result: *mut u8,
    mut result_length: u64,
    mut data: *const u8,
    mut length: u64,
) -> bool {
    let mut out: woff2_WOFF2MemoryOut =
        woff2_WOFF2MemoryOut::woff2_WOFF2MemoryOut(result, result_length);
    return (unsafe {
        let _data: *const u8 = data;
        let _length: u64 = length;
        let _out: *mut dyn woff2_WOFF2Out = (&mut out as *mut woff2_WOFF2MemoryOut);
        ConvertWOFF2ToTTF_40(_data, _length, _out)
    });
}
pub unsafe fn ConvertWOFF2ToTTF_40(
    mut data: *const u8,
    mut length: u64,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut metadata: woff2_RebuildMetadata = <woff2_RebuildMetadata>::default();
    let mut hdr: woff2_WOFF2Header = <woff2_WOFF2Header>::default();
    if !(unsafe {
        let _data: *const u8 = data;
        let _length: u64 = length;
        let _hdr: *mut woff2_WOFF2Header = (&mut hdr as *mut woff2_WOFF2Header);
        ReadWOFF2Header_36(_data, _length, _hdr)
    }) {
        return false;
    }
    if !(unsafe {
        let _data: *const u8 = data;
        let _length: u64 = length;
        let _metadata: *mut woff2_RebuildMetadata = (&mut metadata as *mut woff2_RebuildMetadata);
        let _hdr: *mut woff2_WOFF2Header = (&mut hdr as *mut woff2_WOFF2Header);
        let _out: *mut dyn woff2_WOFF2Out = out;
        WriteHeaders_37(_data, _length, _metadata, _hdr, _out)
    }) {
        return false;
    }
    let compression_ratio: f32 = ((hdr.uncompressed_size as f32) / (length as f32));
    if ((compression_ratio) > (woff2_kMaxPlausibleCompressionRatio)) {
        printf(
            b"Implausible compression ratio %.01f\n\0".as_ptr() as *const i8,
            (compression_ratio as f64),
        );
        return false;
    }
    let mut src_buf: *const u8 = data.offset((hdr.compressed_offset) as isize);
    let mut uncompressed_buf: Vec<u8> = (0..(hdr.uncompressed_size as u64) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    if ((((hdr.uncompressed_size) < (1_u32)) as i64) != 0) {
        return false;
    }
    if ((!(unsafe {
        let _dst_buf: *mut u8 = (&mut uncompressed_buf[(0_u64) as usize] as *mut u8);
        let _dst_size: u64 = (hdr.uncompressed_size as u64);
        let _src_buf: *const u8 = src_buf;
        let _src_size: u64 = (hdr.compressed_length as u64);
        Woff2Uncompress_29(_dst_buf, _dst_size, _src_buf, _src_size)
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (metadata.font_infos.len() as u64)) {
        if ((!(unsafe {
            let _transformed_buf: *mut u8 = (&mut uncompressed_buf[(0_u64) as usize] as *mut u8);
            let _transformed_buf_size: u32 = hdr.uncompressed_size;
            let _metadata: *mut woff2_RebuildMetadata =
                (&mut metadata as *mut woff2_RebuildMetadata);
            let _hdr: *mut woff2_WOFF2Header = (&mut hdr as *mut woff2_WOFF2Header);
            let _font_index: u64 = i;
            let _out: *mut dyn woff2_WOFF2Out = out;
            ReconstructFont_35(
                _transformed_buf,
                _transformed_buf_size,
                _metadata,
                _hdr,
                _font_index,
                _out,
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
// woff2_out.rs
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_WOFF2StringOut {
    buf_: *mut Vec<u8>,
    max_size_: u64,
    offset_: u64,
}
impl woff2_WOFF2StringOut {
    pub unsafe fn woff2_WOFF2StringOut(mut buf: *mut Vec<u8>) -> Self {
        let mut this = Self {
            buf_: buf,
            max_size_: woff2_kDefaultMaxSize,
            offset_: 0_u64,
        };
        this
    }
    pub unsafe fn MaxSize(&mut self) -> u64 {
        return self.max_size_;
    }
}
unsafe impl woff2_WOFF2Out for woff2_WOFF2StringOut {
    unsafe fn Write_pconstlibcc_void_u64(&mut self, buf: *const ::libc::c_void, n: u64) -> bool {
        return (unsafe {
            let _buf: *const ::libc::c_void = buf;
            let _offset: u64 = self.offset_;
            let _n: u64 = n;
            self.Write_pconstlibcc_void_u64_u64(_buf, _offset, _n)
        });
    }
    unsafe fn Write_pconstlibcc_void_u64_u64(
        &mut self,
        buf: *const ::libc::c_void,
        offset: u64,
        n: u64,
    ) -> bool {
        if ((offset) > (self.max_size_)) || ((n) > ((self.max_size_).wrapping_sub(offset))) {
            return false;
        }
        if ((offset) == (((*self.buf_.cast_const()).len() - 1) as u64)) {
            (*self.buf_).splice((*self.buf_).len().saturating_sub(1)..(*self.buf_).len(), {
                let mut v = ::std::slice::from_raw_parts((buf as *const u8), n as usize).to_vec();
                v.push(0);
                v
            });
        } else {
            if (((offset).wrapping_add(n)) > (((*self.buf_.cast_const()).len() - 1) as u64)) {
                (*self.buf_).splice(
                    (*self.buf_).len() - 1..(*self.buf_).len() - 1,
                    ::std::vec::from_elem(
                        0_u8,
                        ((offset).wrapping_add(n))
                            .wrapping_sub(((*self.buf_.cast_const()).len() - 1) as u64)
                            as usize,
                    ),
                );
            }
            (*self.buf_).splice(
                offset as usize..offset as usize + n as usize,
                ::std::slice::from_raw_parts((buf as *const u8), n as usize).to_vec(),
            );
        }
        self.offset_ = {
            let mut __tmp_1 = (offset).wrapping_add(n);
            (*if *&self.offset_ >= *&mut __tmp_1 {
                (&self.offset_) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        };
        return true;
    }
    unsafe fn Size(&mut self) -> u64 {
        return self.offset_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_WOFF2MemoryOut {
    buf_: *mut u8,
    buf_size_: u64,
    offset_: u64,
}
impl woff2_WOFF2MemoryOut {
    pub unsafe fn woff2_WOFF2MemoryOut(mut buf: *mut u8, mut buf_size: u64) -> Self {
        let mut this = Self {
            buf_: buf,
            buf_size_: buf_size,
            offset_: 0_u64,
        };
        this
    }
}
unsafe impl woff2_WOFF2Out for woff2_WOFF2MemoryOut {
    unsafe fn Write_pconstlibcc_void_u64(&mut self, buf: *const ::libc::c_void, n: u64) -> bool {
        return (unsafe {
            let _buf: *const ::libc::c_void = buf;
            let _offset: u64 = self.offset_;
            let _n: u64 = n;
            self.Write_pconstlibcc_void_u64_u64(_buf, _offset, _n)
        });
    }
    unsafe fn Write_pconstlibcc_void_u64_u64(
        &mut self,
        buf: *const ::libc::c_void,
        offset: u64,
        n: u64,
    ) -> bool {
        if ((offset) > (self.buf_size_)) || ((n) > ((self.buf_size_).wrapping_sub(offset))) {
            return false;
        }
        {
            if n != 0 {
                ::std::ptr::copy_nonoverlapping(
                    buf,
                    (self.buf_.offset((offset) as isize) as *mut u8 as *mut ::libc::c_void),
                    n as usize,
                )
            }
            (self.buf_.offset((offset) as isize) as *mut u8 as *mut ::libc::c_void)
        };
        self.offset_ = {
            let mut __tmp_1 = (offset).wrapping_add(n);
            (*if *&self.offset_ >= *&mut __tmp_1 {
                (&self.offset_) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        };
        return true;
    }
    unsafe fn Size(&mut self) -> u64 {
        return self.offset_;
    }
}
impl woff2_WOFF2StringOut {}
impl woff2_WOFF2StringOut {
    pub unsafe fn SetMaxSize(&mut self, mut max_size: u64) {
        self.max_size_ = max_size;
        if ((self.offset_) > (self.max_size_)) {
            self.offset_ = self.max_size_;
        }
    }
}
impl woff2_WOFF2MemoryOut {}
// woff2_decompress.rs
pub unsafe fn GetFileContent_41(mut filename: Vec<u8>) -> Vec<u8> {
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
pub unsafe fn SetFileContents_42(mut filename: Vec<u8>, mut start: *mut u8, mut end: *mut u8) {
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
        let __from = b".ttf\0".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    let mut input: Vec<u8> = (unsafe {
        let _filename: Vec<u8> = filename.clone();
        GetFileContent_41(_filename)
    });
    let mut raw_input: *const u8 = (input.as_ptr() as *const u8);
    let mut output: Vec<u8> = vec![
        0_u8;
        ({
            let mut __tmp_0 = (unsafe {
                let _data: *const u8 = raw_input;
                let _length: u64 = (input.len() - 1) as u64;
                ComputeWOFF2FinalSize_38(_data, _length)
            });
            (*if *&mut __tmp_0 <= *&woff2_kDefaultMaxSize {
                (&mut __tmp_0) as *const _
            } else {
                (&woff2_kDefaultMaxSize) as *const _
            })
        }) as usize
    ]
    .iter()
    .cloned()
    .chain(std::iter::once(0))
    .collect();
    let mut out: woff2_WOFF2StringOut =
        woff2_WOFF2StringOut::woff2_WOFF2StringOut((&mut output as *mut Vec<u8>));
    let ok: bool = (unsafe {
        let _data: *const u8 = raw_input;
        let _length: u64 = (input.len() - 1) as u64;
        let _out: *mut dyn woff2_WOFF2Out = (&mut out as *mut woff2_WOFF2StringOut);
        ConvertWOFF2ToTTF_40(_data, _length, _out)
    });
    if ok {
        (unsafe {
            let _filename: Vec<u8> = outfilename.clone();
            let _start: *mut u8 = output.as_mut_ptr();
            let _end: *mut u8 = output
                .as_mut_ptr()
                .add(((unsafe { out.Size() }) as i64) as usize);
            SetFileContents_42(_filename, _start, _end)
        });
    }
    return if ok { 0 } else { 1 };
}
