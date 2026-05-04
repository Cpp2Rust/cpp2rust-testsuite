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
// woff2_info.rs
pub unsafe fn GetFileContent_31(mut filename: Vec<u8>) -> Vec<u8> {
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
pub unsafe fn SetFileContents_32(mut filename: Vec<u8>, mut start: *mut u8, mut end: *mut u8) {
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
pub unsafe fn PrintTag_33(mut tag: i32) -> Vec<u8> {
    if (((tag as u32) & (2155905152_u32)) != 0) {
        return {
            let s = b"_xfm\0".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
    }
    let mut printable: [u8; 4] = [
        ((((tag) >> (24)) & (255)) as u8),
        ((((tag) >> (16)) & (255)) as u8),
        ((((tag) >> (8)) & (255)) as u8),
        (((tag) & (255)) as u8),
    ];
    return std::slice::from_raw_parts(printable.as_mut_ptr().cast_const(), 4_u64 as usize)
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .collect();
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
        GetFileContent_31(_filename)
    });
    let mut file: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((input.as_ptr() as *const u8), (input.len() - 1) as u64);
    printf(b"WOFF2Header\n\0".as_ptr() as *const i8);
    let mut signature: u32 = 0_u32;
    let mut flavor: u32 = 0_u32;
    let mut length: u32 = 0_u32;
    let mut totalSfntSize: u32 = 0_u32;
    let mut totalCompressedSize: u32 = 0_u32;
    let mut metaOffset: u32 = 0_u32;
    let mut metaLength: u32 = 0_u32;
    let mut metaOrigLength: u32 = 0_u32;
    let mut privOffset: u32 = 0_u32;
    let mut privLength: u32 = 0_u32;
    let mut num_tables: u16 = 0_u16;
    let mut reserved: u16 = 0_u16;
    let mut major: u16 = 0_u16;
    let mut minor: u16 = 0_u16;
    if !(unsafe {
        let _value: *mut u32 = (&mut signature as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut flavor as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut length as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u16 = (&mut num_tables as *mut u16);
        file.ReadU16(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u16 = (&mut reserved as *mut u16);
        file.ReadU16(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut totalSfntSize as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut totalCompressedSize as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u16 = (&mut major as *mut u16);
        file.ReadU16(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u16 = (&mut minor as *mut u16);
        file.ReadU16(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut metaOffset as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut metaLength as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut metaOrigLength as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut privOffset as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if !(unsafe {
        let _value: *mut u32 = (&mut privLength as *mut u32);
        file.ReadU32(_value)
    }) {
        return 1;
    }
    if ((signature) != (2001684018_u32)) {
        printf(
            b"Invalid signature: %08x\n\0".as_ptr() as *const i8,
            signature,
        );
        return 1;
    }
    printf(
        b"signature           0x%08x\n\0".as_ptr() as *const i8,
        signature,
    );
    printf(
        b"flavor              0x%08x\n\0".as_ptr() as *const i8,
        flavor,
    );
    printf(b"length              %d\n\0".as_ptr() as *const i8, length);
    printf(
        b"numTables           %d\n\0".as_ptr() as *const i8,
        (num_tables as i32),
    );
    printf(
        b"reserved            %d\n\0".as_ptr() as *const i8,
        (reserved as i32),
    );
    printf(
        b"totalSfntSize       %d\n\0".as_ptr() as *const i8,
        totalSfntSize,
    );
    printf(
        b"totalCompressedSize %d\n\0".as_ptr() as *const i8,
        totalCompressedSize,
    );
    printf(
        b"majorVersion        %d\n\0".as_ptr() as *const i8,
        (major as i32),
    );
    printf(
        b"minorVersion        %d\n\0".as_ptr() as *const i8,
        (minor as i32),
    );
    printf(
        b"metaOffset          %d\n\0".as_ptr() as *const i8,
        metaOffset,
    );
    printf(
        b"metaLength          %d\n\0".as_ptr() as *const i8,
        metaLength,
    );
    printf(
        b"metaOrigLength      %d\n\0".as_ptr() as *const i8,
        metaOrigLength,
    );
    printf(
        b"privOffset          %d\n\0".as_ptr() as *const i8,
        privOffset,
    );
    printf(
        b"privLength          %d\n\0".as_ptr() as *const i8,
        privLength,
    );
    let mut table_tags: Vec<u32> = Vec::new();
    printf(
        b"TableDirectory starts at +%zu\n\0".as_ptr() as *const i8,
        (unsafe { file.offset() }),
    );
    printf(b"Entry offset flags tag  origLength txLength\n\0".as_ptr() as *const i8);
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_tables as i32)) {
        let mut offset: u64 = (unsafe { file.offset() });
        let mut flags: u8 = 0_u8;
        let mut tag: u32 = 0_u32;
        let mut origLength: u32 = 0_u32;
        let mut transformLength: u32 = 0_u32;
        if !(unsafe {
            let _value: *mut u8 = (&mut flags as *mut u8);
            file.ReadU8(_value)
        }) {
            return 1;
        }
        if (((flags as i32) & (63)) == (63)) {
            if !(unsafe {
                let _value: *mut u32 = (&mut tag as *mut u32);
                file.ReadU32(_value)
            }) {
                return 1;
            }
        } else {
            tag = woff2_kKnownTags[((flags as i32) & (63)) as usize];
        }
        {
            let a0_clone = tag.clone();
            table_tags.push(a0_clone)
        };
        if !(unsafe {
            let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
            let _value: *mut u32 = (&mut origLength as *mut u32);
            ReadBase128_4(_buf, _value)
        }) {
            return 1;
        }
        printf(
            b"%5d %6zu  0x%02x %s %10d\0".as_ptr() as *const i8,
            i,
            offset,
            (flags as i32),
            (unsafe {
                let _tag: i32 = (tag as i32);
                PrintTag_33(_tag)
            })
            .as_ptr(),
            origLength,
        );
        let mut xform_version: u8 = ((((flags as i32) >> (6)) & (3)) as u8);
        if ((tag) == (woff2_kGlyfTableTag)) || ((tag) == (woff2_kLocaTableTag)) {
            if ((xform_version as i32) == (0)) {
                if !(unsafe {
                    let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                    let _value: *mut u32 = (&mut transformLength as *mut u32);
                    ReadBase128_4(_buf, _value)
                }) {
                    return 1;
                }
                printf(b" %8d\0".as_ptr() as *const i8, transformLength);
            }
        } else if ((xform_version as i32) > (0)) {
            if !(unsafe {
                let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                let _value: *mut u32 = (&mut transformLength as *mut u32);
                ReadBase128_4(_buf, _value)
            }) {
                return 1;
            }
            printf(b" %8d\0".as_ptr() as *const i8, transformLength);
        }
        printf(b"\n\0".as_ptr() as *const i8);
        i.postfix_inc();
    }
    if ((flavor) == (woff2_kTtcFontFlavor)) {
        let mut version: u32 = 0_u32;
        let mut numFonts: u32 = 0_u32;
        if !(unsafe {
            let _value: *mut u32 = (&mut version as *mut u32);
            file.ReadU32(_value)
        }) {
            return 1;
        }
        if !(unsafe {
            let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
            let _value: *mut u32 = (&mut numFonts as *mut u32);
            Read255UShort_3(_buf, _value)
        }) {
            return 1;
        }
        printf(
            b"CollectionHeader 0x%08x %d fonts\n\0".as_ptr() as *const i8,
            version,
            numFonts,
        );
        let mut i: i32 = 0;
        'loop_: while ((i as u32) < (numFonts)) {
            let mut numTables: u32 = 0_u32;
            let mut flavor: u32 = 0_u32;
            if !(unsafe {
                let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                let _value: *mut u32 = (&mut numTables as *mut u32);
                Read255UShort_3(_buf, _value)
            }) {
                return 1;
            }
            if !(unsafe {
                let _value: *mut u32 = (&mut flavor as *mut u32);
                file.ReadU32(_value)
            }) {
                return 1;
            }
            printf(
                b"CollectionFontEntry %d flavor 0x%08x %d tables\n\0".as_ptr() as *const i8,
                i,
                flavor,
                numTables,
            );
            let mut j: i32 = 0;
            'loop_: while ((j as u32) < (numTables)) {
                let mut table_idx: u32 = 0_u32;
                if !(unsafe {
                    let _buf: *mut woff2_Buffer = (&mut file as *mut woff2_Buffer);
                    let _value: *mut u32 = (&mut table_idx as *mut u32);
                    Read255UShort_3(_buf, _value)
                }) {
                    return 1;
                }
                if ((table_idx as u64) >= (table_tags.len() as u64)) {
                    return 1;
                }
                printf(
                    b"  %d %s (idx %d)\n\0".as_ptr() as *const i8,
                    j,
                    (unsafe {
                        let _tag: i32 = (table_tags[(table_idx as u64) as usize] as i32);
                        PrintTag_33(_tag)
                    })
                    .as_ptr(),
                    table_idx,
                );
                j.postfix_inc();
            }
            i.postfix_inc();
        }
    }
    printf(
        b"TableDirectory ends at +%zu\n\0".as_ptr() as *const i8,
        (unsafe { file.offset() }),
    );
    return 0;
}
