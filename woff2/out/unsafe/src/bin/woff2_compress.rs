extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut kGlyfTableTag_0: u32 = unsafe { 1735162214_u32 };
pub static mut kHeadTableTag_1: u32 = unsafe { 1751474532_u32 };
pub static mut kLocaTableTag_2: u32 = unsafe { 1819239265_u32 };
pub static mut kDsigTableTag_3: u32 = unsafe { 1146308935_u32 };
pub static mut kCffTableTag_4: u32 = unsafe { 1128678944_u32 };
pub static mut kHmtxTableTag_5: u32 = unsafe { 1752003704_u32 };
pub static mut kHheaTableTag_6: u32 = unsafe { 1751672161_u32 };
pub static mut kMaxpTableTag_7: u32 = unsafe { 1835104368_u32 };
pub static mut kKnownTags_8: [u32; 63] = unsafe {
    [
        ((((((('c' as libc::c_char) as i32) << (24)) | ((('m' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('p' as libc::c_char) as i32)) as u32),
        ((((((('h' as libc::c_char) as i32) << (24)) | ((('e' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('d' as libc::c_char) as i32)) as u32),
        ((((((('h' as libc::c_char) as i32) << (24)) | ((('h' as libc::c_char) as i32) << (16)))
            | ((('e' as libc::c_char) as i32) << (8)))
            | (('a' as libc::c_char) as i32)) as u32),
        ((((((('h' as libc::c_char) as i32) << (24)) | ((('m' as libc::c_char) as i32) << (16)))
            | ((('t' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('m' as libc::c_char) as i32) << (24)) | ((('a' as libc::c_char) as i32) << (16)))
            | ((('x' as libc::c_char) as i32) << (8)))
            | (('p' as libc::c_char) as i32)) as u32),
        ((((((('n' as libc::c_char) as i32) << (24)) | ((('a' as libc::c_char) as i32) << (16)))
            | ((('m' as libc::c_char) as i32) << (8)))
            | (('e' as libc::c_char) as i32)) as u32),
        ((((((('O' as libc::c_char) as i32) << (24)) | ((('S' as libc::c_char) as i32) << (16)))
            | ((('/' as libc::c_char) as i32) << (8)))
            | (('2' as libc::c_char) as i32)) as u32),
        ((((((('p' as libc::c_char) as i32) << (24)) | ((('o' as libc::c_char) as i32) << (16)))
            | ((('s' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('c' as libc::c_char) as i32) << (24)) | ((('v' as libc::c_char) as i32) << (16)))
            | ((('t' as libc::c_char) as i32) << (8)))
            | ((' ' as libc::c_char) as i32)) as u32),
        ((((((('f' as libc::c_char) as i32) << (24)) | ((('p' as libc::c_char) as i32) << (16)))
            | ((('g' as libc::c_char) as i32) << (8)))
            | (('m' as libc::c_char) as i32)) as u32),
        ((((((('g' as libc::c_char) as i32) << (24)) | ((('l' as libc::c_char) as i32) << (16)))
            | ((('y' as libc::c_char) as i32) << (8)))
            | (('f' as libc::c_char) as i32)) as u32),
        ((((((('l' as libc::c_char) as i32) << (24)) | ((('o' as libc::c_char) as i32) << (16)))
            | ((('c' as libc::c_char) as i32) << (8)))
            | (('a' as libc::c_char) as i32)) as u32),
        ((((((('p' as libc::c_char) as i32) << (24)) | ((('r' as libc::c_char) as i32) << (16)))
            | ((('e' as libc::c_char) as i32) << (8)))
            | (('p' as libc::c_char) as i32)) as u32),
        ((((((('C' as libc::c_char) as i32) << (24)) | ((('F' as libc::c_char) as i32) << (16)))
            | ((('F' as libc::c_char) as i32) << (8)))
            | ((' ' as libc::c_char) as i32)) as u32),
        ((((((('V' as libc::c_char) as i32) << (24)) | ((('O' as libc::c_char) as i32) << (16)))
            | ((('R' as libc::c_char) as i32) << (8)))
            | (('G' as libc::c_char) as i32)) as u32),
        ((((((('E' as libc::c_char) as i32) << (24)) | ((('B' as libc::c_char) as i32) << (16)))
            | ((('D' as libc::c_char) as i32) << (8)))
            | (('T' as libc::c_char) as i32)) as u32),
        ((((((('E' as libc::c_char) as i32) << (24)) | ((('B' as libc::c_char) as i32) << (16)))
            | ((('L' as libc::c_char) as i32) << (8)))
            | (('C' as libc::c_char) as i32)) as u32),
        ((((((('g' as libc::c_char) as i32) << (24)) | ((('a' as libc::c_char) as i32) << (16)))
            | ((('s' as libc::c_char) as i32) << (8)))
            | (('p' as libc::c_char) as i32)) as u32),
        ((((((('h' as libc::c_char) as i32) << (24)) | ((('d' as libc::c_char) as i32) << (16)))
            | ((('m' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('k' as libc::c_char) as i32) << (24)) | ((('e' as libc::c_char) as i32) << (16)))
            | ((('r' as libc::c_char) as i32) << (8)))
            | (('n' as libc::c_char) as i32)) as u32),
        ((((((('L' as libc::c_char) as i32) << (24)) | ((('T' as libc::c_char) as i32) << (16)))
            | ((('S' as libc::c_char) as i32) << (8)))
            | (('H' as libc::c_char) as i32)) as u32),
        ((((((('P' as libc::c_char) as i32) << (24)) | ((('C' as libc::c_char) as i32) << (16)))
            | ((('L' as libc::c_char) as i32) << (8)))
            | (('T' as libc::c_char) as i32)) as u32),
        ((((((('V' as libc::c_char) as i32) << (24)) | ((('D' as libc::c_char) as i32) << (16)))
            | ((('M' as libc::c_char) as i32) << (8)))
            | (('X' as libc::c_char) as i32)) as u32),
        ((((((('v' as libc::c_char) as i32) << (24)) | ((('h' as libc::c_char) as i32) << (16)))
            | ((('e' as libc::c_char) as i32) << (8)))
            | (('a' as libc::c_char) as i32)) as u32),
        ((((((('v' as libc::c_char) as i32) << (24)) | ((('m' as libc::c_char) as i32) << (16)))
            | ((('t' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('B' as libc::c_char) as i32) << (24)) | ((('A' as libc::c_char) as i32) << (16)))
            | ((('S' as libc::c_char) as i32) << (8)))
            | (('E' as libc::c_char) as i32)) as u32),
        ((((((('G' as libc::c_char) as i32) << (24)) | ((('D' as libc::c_char) as i32) << (16)))
            | ((('E' as libc::c_char) as i32) << (8)))
            | (('F' as libc::c_char) as i32)) as u32),
        ((((((('G' as libc::c_char) as i32) << (24)) | ((('P' as libc::c_char) as i32) << (16)))
            | ((('O' as libc::c_char) as i32) << (8)))
            | (('S' as libc::c_char) as i32)) as u32),
        ((((((('G' as libc::c_char) as i32) << (24)) | ((('S' as libc::c_char) as i32) << (16)))
            | ((('U' as libc::c_char) as i32) << (8)))
            | (('B' as libc::c_char) as i32)) as u32),
        ((((((('E' as libc::c_char) as i32) << (24)) | ((('B' as libc::c_char) as i32) << (16)))
            | ((('S' as libc::c_char) as i32) << (8)))
            | (('C' as libc::c_char) as i32)) as u32),
        ((((((('J' as libc::c_char) as i32) << (24)) | ((('S' as libc::c_char) as i32) << (16)))
            | ((('T' as libc::c_char) as i32) << (8)))
            | (('F' as libc::c_char) as i32)) as u32),
        ((((((('M' as libc::c_char) as i32) << (24)) | ((('A' as libc::c_char) as i32) << (16)))
            | ((('T' as libc::c_char) as i32) << (8)))
            | (('H' as libc::c_char) as i32)) as u32),
        ((((((('C' as libc::c_char) as i32) << (24)) | ((('B' as libc::c_char) as i32) << (16)))
            | ((('D' as libc::c_char) as i32) << (8)))
            | (('T' as libc::c_char) as i32)) as u32),
        ((((((('C' as libc::c_char) as i32) << (24)) | ((('B' as libc::c_char) as i32) << (16)))
            | ((('L' as libc::c_char) as i32) << (8)))
            | (('C' as libc::c_char) as i32)) as u32),
        ((((((('C' as libc::c_char) as i32) << (24)) | ((('O' as libc::c_char) as i32) << (16)))
            | ((('L' as libc::c_char) as i32) << (8)))
            | (('R' as libc::c_char) as i32)) as u32),
        ((((((('C' as libc::c_char) as i32) << (24)) | ((('P' as libc::c_char) as i32) << (16)))
            | ((('A' as libc::c_char) as i32) << (8)))
            | (('L' as libc::c_char) as i32)) as u32),
        ((((((('S' as libc::c_char) as i32) << (24)) | ((('V' as libc::c_char) as i32) << (16)))
            | ((('G' as libc::c_char) as i32) << (8)))
            | ((' ' as libc::c_char) as i32)) as u32),
        ((((((('s' as libc::c_char) as i32) << (24)) | ((('b' as libc::c_char) as i32) << (16)))
            | ((('i' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('a' as libc::c_char) as i32) << (24)) | ((('c' as libc::c_char) as i32) << (16)))
            | ((('n' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('a' as libc::c_char) as i32) << (24)) | ((('v' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('r' as libc::c_char) as i32)) as u32),
        ((((((('b' as libc::c_char) as i32) << (24)) | ((('d' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('b' as libc::c_char) as i32) << (24)) | ((('l' as libc::c_char) as i32) << (16)))
            | ((('o' as libc::c_char) as i32) << (8)))
            | (('c' as libc::c_char) as i32)) as u32),
        ((((((('b' as libc::c_char) as i32) << (24)) | ((('s' as libc::c_char) as i32) << (16)))
            | ((('l' as libc::c_char) as i32) << (8)))
            | (('n' as libc::c_char) as i32)) as u32),
        ((((((('c' as libc::c_char) as i32) << (24)) | ((('v' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('r' as libc::c_char) as i32)) as u32),
        ((((((('f' as libc::c_char) as i32) << (24)) | ((('d' as libc::c_char) as i32) << (16)))
            | ((('s' as libc::c_char) as i32) << (8)))
            | (('c' as libc::c_char) as i32)) as u32),
        ((((((('f' as libc::c_char) as i32) << (24)) | ((('e' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('f' as libc::c_char) as i32) << (24)) | ((('m' as libc::c_char) as i32) << (16)))
            | ((('t' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('f' as libc::c_char) as i32) << (24)) | ((('v' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('r' as libc::c_char) as i32)) as u32),
        ((((((('g' as libc::c_char) as i32) << (24)) | ((('v' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('r' as libc::c_char) as i32)) as u32),
        ((((((('h' as libc::c_char) as i32) << (24)) | ((('s' as libc::c_char) as i32) << (16)))
            | ((('t' as libc::c_char) as i32) << (8)))
            | (('y' as libc::c_char) as i32)) as u32),
        ((((((('j' as libc::c_char) as i32) << (24)) | ((('u' as libc::c_char) as i32) << (16)))
            | ((('s' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('l' as libc::c_char) as i32) << (24)) | ((('c' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('r' as libc::c_char) as i32)) as u32),
        ((((((('m' as libc::c_char) as i32) << (24)) | ((('o' as libc::c_char) as i32) << (16)))
            | ((('r' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('m' as libc::c_char) as i32) << (24)) | ((('o' as libc::c_char) as i32) << (16)))
            | ((('r' as libc::c_char) as i32) << (8)))
            | (('x' as libc::c_char) as i32)) as u32),
        ((((((('o' as libc::c_char) as i32) << (24)) | ((('p' as libc::c_char) as i32) << (16)))
            | ((('b' as libc::c_char) as i32) << (8)))
            | (('d' as libc::c_char) as i32)) as u32),
        ((((((('p' as libc::c_char) as i32) << (24)) | ((('r' as libc::c_char) as i32) << (16)))
            | ((('o' as libc::c_char) as i32) << (8)))
            | (('p' as libc::c_char) as i32)) as u32),
        ((((((('t' as libc::c_char) as i32) << (24)) | ((('r' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('k' as libc::c_char) as i32)) as u32),
        ((((((('Z' as libc::c_char) as i32) << (24)) | ((('a' as libc::c_char) as i32) << (16)))
            | ((('p' as libc::c_char) as i32) << (8)))
            | (('f' as libc::c_char) as i32)) as u32),
        ((((((('S' as libc::c_char) as i32) << (24)) | ((('i' as libc::c_char) as i32) << (16)))
            | ((('l' as libc::c_char) as i32) << (8)))
            | (('f' as libc::c_char) as i32)) as u32),
        ((((((('G' as libc::c_char) as i32) << (24)) | ((('l' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('G' as libc::c_char) as i32) << (24)) | ((('l' as libc::c_char) as i32) << (16)))
            | ((('o' as libc::c_char) as i32) << (8)))
            | (('c' as libc::c_char) as i32)) as u32),
        ((((((('F' as libc::c_char) as i32) << (24)) | ((('e' as libc::c_char) as i32) << (16)))
            | ((('a' as libc::c_char) as i32) << (8)))
            | (('t' as libc::c_char) as i32)) as u32),
        ((((((('S' as libc::c_char) as i32) << (24)) | ((('i' as libc::c_char) as i32) << (16)))
            | ((('l' as libc::c_char) as i32) << (8)))
            | (('l' as libc::c_char) as i32)) as u32),
    ]
};
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_Buffer {
    buffer_: *const u8,
    length_: usize,
    offset_: usize,
}
impl woff2_Buffer {
    pub unsafe fn woff2_Buffer(mut data: *const u8, mut len: usize) -> Self {
        let mut this = Self {
            buffer_: data,
            length_: len,
            offset_: 0_usize,
        };
        this
    }
    pub unsafe fn Skip(&mut self, mut n_bytes: usize) -> bool {
        return (unsafe { self.Read(std::ptr::null_mut(), n_bytes) });
    }
    pub unsafe fn Read(&mut self, mut data: *mut u8, mut n_bytes: usize) -> bool {
        if ((n_bytes) > ((((1024) * (1024)) * (1024)) as usize)) {
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
        if ((self.length_) < (1_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(1_usize)))
        {
            return false;
        }
        (*value) = (*self.buffer_.offset((self.offset_) as isize));
        self.offset_.prefix_inc();
        return true;
    }
    pub unsafe fn ReadU16(&mut self, mut value: *mut u16) -> bool {
        if ((self.length_) < (2_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(2_usize)))
        {
            return false;
        }
        {
            if ::std::mem::size_of::<u16>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u16 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u16>() as usize,
                )
            }
            (value as *mut u16 as *mut ::libc::c_void)
        };
        (*value) = u16::from_be((*value));
        self.offset_ = (self.offset_).wrapping_add(2_usize);
        return true;
    }
    pub unsafe fn ReadS16(&mut self, mut value: *mut i16) -> bool {
        return (unsafe { self.ReadU16((value as *mut u16)) });
    }
    pub unsafe fn ReadU24(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (3_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(3_usize)))
        {
            return false;
        }
        (*value) = (((((*self.buffer_.offset((self.offset_) as isize)) as u32) << (16))
            | (((*self
                .buffer_
                .offset(((self.offset_).wrapping_add(1_usize)) as isize))
                as u32)
                << (8)))
            | ((*self
                .buffer_
                .offset(((self.offset_).wrapping_add(2_usize)) as isize)) as u32));
        self.offset_ = (self.offset_).wrapping_add(3_usize);
        return true;
    }
    pub unsafe fn ReadU32(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (4_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(4_usize)))
        {
            return false;
        }
        {
            if ::std::mem::size_of::<u32>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u32 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u32>() as usize,
                )
            }
            (value as *mut u32 as *mut ::libc::c_void)
        };
        (*value) = u32::from_be((*value));
        self.offset_ = (self.offset_).wrapping_add(4_usize);
        return true;
    }
    pub unsafe fn ReadS32(&mut self, mut value: *mut i32) -> bool {
        return (unsafe { self.ReadU32((value as *mut u32)) });
    }
    pub unsafe fn ReadTag(&mut self, mut value: *mut u32) -> bool {
        if ((self.length_) < (4_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(4_usize)))
        {
            return false;
        }
        {
            if ::std::mem::size_of::<u32>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u32 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u32>() as usize,
                )
            }
            (value as *mut u32 as *mut ::libc::c_void)
        };
        self.offset_ = (self.offset_).wrapping_add(4_usize);
        return true;
    }
    pub unsafe fn ReadR64(&mut self, mut value: *mut u64) -> bool {
        if ((self.length_) < (8_usize)) || ((self.offset_) > ((self.length_).wrapping_sub(8_usize)))
        {
            return false;
        }
        {
            if ::std::mem::size_of::<u64>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (self.buffer_.offset((self.offset_) as isize) as *const u8
                        as *const ::libc::c_void),
                    (value as *mut u64 as *mut ::libc::c_void),
                    ::std::mem::size_of::<u64>() as usize,
                )
            }
            (value as *mut u64 as *mut ::libc::c_void)
        };
        self.offset_ = (self.offset_).wrapping_add(8_usize);
        return true;
    }
    pub unsafe fn buffer(&self) -> *const u8 {
        return self.buffer_;
    }
    pub unsafe fn offset(&self) -> usize {
        return self.offset_;
    }
    pub unsafe fn length(&self) -> usize {
        return self.length_;
    }
    pub unsafe fn set_offset(&mut self, mut newoffset: usize) -> bool {
        if ((newoffset) > (self.length_)) {
            return false;
        }
        self.offset_ = newoffset;
        return true;
    }
}
pub unsafe fn Size255UShort_9(mut value: u16) -> usize {
    let mut result: usize = 3_usize;
    if ((value as i32) < (253)) {
        result = 1_usize;
    } else if ((value as i32) < (762)) {
        result = 2_usize;
    } else {
        result = 3_usize;
    }
    return result;
}
pub unsafe fn Write255UShort_10(mut out: *mut Vec<u8>, mut value: i32) {
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
pub unsafe fn Store255UShort_11(mut val: i32, mut offset: *mut usize, mut dst: *mut u8) {
    let mut packed: Vec<u8> = Vec::new();
    (unsafe { Write255UShort_10((&mut packed as *mut Vec<u8>), val) });
    'loop_: for packed_byte in 0..(packed.len()) {
        let mut packed_byte = packed[packed_byte].clone();
        (*dst.offset(((*offset).postfix_inc()) as isize)) = packed_byte;
    }
}
pub unsafe fn Read255UShort_12(mut buf: *mut woff2_Buffer, mut value: *mut u32) -> bool {
    static mut kWordCode_13: i32 = unsafe { 253 };;
    static mut kOneMoreByteCode2_14: i32 = unsafe { 254 };;
    static mut kOneMoreByteCode1_15: i32 = unsafe { 255 };;
    static mut kLowestUCode_16: i32 = unsafe { 253 };;
    let mut code: u8 = 0_u8;
    if !(unsafe { (*buf).ReadU8((&mut code as *mut u8)) }) {
        return false;
    }
    if ((code as i32) == (kWordCode_13)) {
        let mut result: u16 = 0_u16;
        if !(unsafe { (*buf).ReadU16((&mut result as *mut u16)) }) {
            return false;
        }
        (*value) = (result as u32);
        return true;
    } else if ((code as i32) == (kOneMoreByteCode1_15)) {
        let mut result: u8 = 0_u8;
        if !(unsafe { (*buf).ReadU8((&mut result as *mut u8)) }) {
            return false;
        }
        (*value) = (((result as i32) + (kLowestUCode_16)) as u32);
        return true;
    } else if ((code as i32) == (kOneMoreByteCode2_14)) {
        let mut result: u8 = 0_u8;
        if !(unsafe { (*buf).ReadU8((&mut result as *mut u8)) }) {
            return false;
        }
        (*value) = (((result as i32) + ((kLowestUCode_16) * (2))) as u32);
        return true;
    } else {
        (*value) = (code as u32);
        return true;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn ReadBase128_17(mut buf: *mut woff2_Buffer, mut value: *mut u32) -> bool {
    let mut result: u32 = 0_u32;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (5_usize)) {
        let mut code: u8 = 0_u8;
        if !(unsafe { (*buf).ReadU8((&mut code as *mut u8)) }) {
            return false;
        }
        if ((i) == (0_usize)) && ((code as i32) == (128)) {
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
pub unsafe fn Base128Size_18(mut n: usize) -> usize {
    let mut size: usize = 1_usize;
    'loop_: while ((n) >= (128_usize)) {
        size.prefix_inc();
        n >>= 7;
    }
    return size;
}
pub unsafe fn StoreBase128_19(mut len: usize, mut offset: *mut usize, mut dst: *mut u8) {
    let mut size: usize = (unsafe { Base128Size_18(len) });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (size)) {
        let mut b: i32 = ((((len)
            >> ((7_usize).wrapping_mul((((size).wrapping_sub(i)).wrapping_sub(1_usize)))))
            & (127_usize)) as i32);
        if ((i) < ((size).wrapping_sub(1_usize))) {
            b |= 128;
        }
        (*dst.offset(((*offset).postfix_inc()) as isize)) = (b as u8);
        i.prefix_inc();
    }
}
pub static mut kWoff2Signature_20: u32 = unsafe { 2001684018_u32 };
pub static mut kWoff2FlagsTransform_21: u32 = unsafe { (((1) << (8)) as u32) };
pub static mut kTtcFontFlavor_22: u32 = unsafe { 1953784678_u32 };
pub static mut kSfntHeaderSize_23: usize = unsafe { 12_usize };
pub static mut kSfntEntrySize_24: usize = unsafe { 16_usize };
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
            } else if other.lt(self) {
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
        unsafe { !(self.lt(other)) && !(other.lt(self)) }
    }
}
impl Eq for woff2_Table {}
pub unsafe fn Log2Floor_25(mut n: u32) -> i32 {
    return if ((n) == (0_u32)) {
        -1_i32
    } else {
        ((31) ^ (n.leading_zeros() as i32))
    };
}
pub unsafe fn ComputeULongSum_26(mut buf: *const u8, mut size: usize) -> u32 {
    let mut checksum: u32 = 0_u32;
    let mut aligned_size: usize = ((size) & (!3 as usize));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (aligned_size)) {
        checksum = (checksum).wrapping_add(
            (((((((*buf.offset((i) as isize)) as i32) << (24))
                | (((*buf.offset(((i).wrapping_add(1_usize)) as isize)) as i32) << (16)))
                | (((*buf.offset(((i).wrapping_add(2_usize)) as isize)) as i32) << (8)))
                | ((*buf.offset(((i).wrapping_add(3_usize)) as isize)) as i32))
                as u32),
        );
        i = (i).wrapping_add(4_usize);
    }
    if ((size) != (aligned_size)) {
        let mut v: u32 = 0_u32;
        let mut i: usize = aligned_size;
        'loop_: while ((i) < (size)) {
            v |= ((((*buf.offset((i) as isize)) as i32)
                << ((24_usize).wrapping_sub((8_usize).wrapping_mul(((i) & (3_usize))))))
                as u32);
            i.prefix_inc();
        }
        checksum = (checksum).wrapping_add(v);
    }
    return checksum;
}
pub unsafe fn CollectionHeaderSize_27(mut header_version: u32, mut num_fonts: u32) -> usize {
    let mut size: usize = 0_usize;
    if ((header_version) == (131072_u32)) {
        size = (size).wrapping_add(12_usize);
    }
    if ((header_version) == (65536_u32)) || ((header_version) == (131072_u32)) {
        size = (size)
            .wrapping_add((((12_u32).wrapping_add((4_u32).wrapping_mul(num_fonts))) as usize));
    }
    return size;
}
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
pub unsafe fn StoreU32_28(mut dst: *mut u8, mut offset: usize, mut x: u32) -> usize {
    (*dst.offset((offset) as isize)) = (((x) >> (24)) as u8);
    (*dst.offset(((offset).wrapping_add(1_usize)) as isize)) = (((x) >> (16)) as u8);
    (*dst.offset(((offset).wrapping_add(2_usize)) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(3_usize)) as isize)) = (x as u8);
    return (offset).wrapping_add(4_usize);
}
pub unsafe fn Store16_29(mut dst: *mut u8, mut offset: usize, mut x: i32) -> usize {
    (*dst.offset((offset) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(1_usize)) as isize)) = (x as u8);
    return (offset).wrapping_add(2_usize);
}
pub unsafe fn StoreU32_30(mut val: u32, mut offset: *mut usize, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (24)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (16)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn Store16_31(mut val: i32, mut offset: *mut usize, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn StoreBytes_32(
    mut data: *const u8,
    mut len: usize,
    mut offset: *mut usize,
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
            std::ptr::null_mut()
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
            std::ptr::null()
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
            while it != output_order.as_mut_ptr().add(output_order.len()) && *it != kGlyfTableTag_0
            {
                it = it.add(1);
            }
            it
        };
        let mut loca_loc: *mut u32 = {
            let mut it = output_order.as_mut_ptr();
            while it != output_order.as_mut_ptr().add(output_order.len()) && *it != kLocaTableTag_2
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
                        && *it != kGlyfTableTag_0
                    {
                        it = it.add(1);
                    }
                    it
                }
                .add(1_i64 as usize)
                .offset_from(output_order.as_ptr()) as usize;
                output_order.insert(pos, kLocaTableTag_2);
            };
        }
        return output_order;
    }
}
pub unsafe fn ReadTrueTypeFont_33(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: usize,
    mut font: *mut woff2_Font,
) -> bool {
    if (!(unsafe { (*file).ReadU16((&mut (*font).num_tables as *mut u16)) }))
        || (!(unsafe { (*file).Skip(6_usize) }))
    {
        return false;
    }
    let mut intervals: BTreeMap<u32, Box<u32>> = BTreeMap::new();
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < ((*font).num_tables as i32)) {
        let mut table: woff2_Font_Table = <woff2_Font_Table>::default();
        table.flag_byte = 0_u8;
        table.reuse_of = std::ptr::null_mut();
        if (((!(unsafe { (*file).ReadU32((&mut table.tag as *mut u32)) }))
            || (!(unsafe { (*file).ReadU32((&mut table.checksum as *mut u32)) })))
            || (!(unsafe { (*file).ReadU32((&mut table.offset as *mut u32)) })))
            || (!(unsafe { (*file).ReadU32((&mut table.length as *mut u32)) }))
        {
            return false;
        }
        if ((((table.offset) & (3_u32)) != (0_u32)) || ((table.length as usize) > (len)))
            || (((len).wrapping_sub((table.length as usize))) < (table.offset as usize))
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
    let mut head_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kHeadTableTag_1) }).cast_const();
    if (!((head_table).is_null())) && (((*head_table).length) < (52_u32)) {
        return false;
    }
    return true;
}
pub unsafe fn ReadCollectionFont_34(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: usize,
    mut font: *mut woff2_Font,
    mut all_tables: *mut BTreeMap<u32, Box<*mut woff2_Font_Table>>,
) -> bool {
    if !(unsafe { (*file).ReadU32((&mut (*font).flavor as *mut u32)) }) {
        return false;
    }
    if !(unsafe { ReadTrueTypeFont_33(file, data, len, font) }) {
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
            (*(*all_tables).entry((*table).offset).or_default().as_mut()) =
                (unsafe { (*font).FindTable_u32((*table).tag) });
        } else {
            (*table).reuse_of = (*(*all_tables).entry((*table).offset).or_default().as_mut());
            if (((*table).tag) != ((*(*table).reuse_of).tag)) {
                return false;
            }
        }
    }
    return true;
}
pub unsafe fn ReadTrueTypeCollection_35(
    mut file: *mut woff2_Buffer,
    mut data: *const u8,
    mut len: usize,
    mut font_collection: *mut woff2_FontCollection,
) -> bool {
    let mut num_fonts: u32 = 0_u32;
    if (!(unsafe { (*file).ReadU32((&mut (*font_collection).header_version as *mut u32)) }))
        || (!(unsafe { (*file).ReadU32((&mut num_fonts as *mut u32)) }))
    {
        return false;
    }
    let mut offsets: Vec<u32> = Vec::new();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_fonts as usize)) {
        let mut offset: u32 = 0_u32;
        if !(unsafe { (*file).ReadU32((&mut offset as *mut u32)) }) {
            return false;
        }
        {
            let a0_clone = offset.clone();
            offsets.push(a0_clone)
        };
        i.postfix_inc();
    }
    {
        let __a0 = offsets.len() as usize;
        (*font_collection)
            .fonts
            .resize_with(__a0, || <woff2_Font>::default())
    };
    let mut font_it: *mut woff2_Font = (*font_collection).fonts.as_mut_ptr();
    let mut all_tables: BTreeMap<u32, Box<*mut woff2_Font_Table>> = BTreeMap::new();
    'loop_: for offset in 0..(offsets.len()) {
        let offset = offsets[offset].clone();
        if !(unsafe { (*file).set_offset((offset as usize)) }) {
            return false;
        }
        let font: *mut woff2_Font = &mut (*font_it.postfix_inc()) as *mut woff2_Font;
        if !(unsafe {
            ReadCollectionFont_34(
                file,
                data,
                len,
                (font),
                (&mut all_tables as *mut BTreeMap<u32, Box<*mut woff2_Font_Table>>),
            )
        }) {
            return false;
        }
    }
    return true;
}
pub unsafe fn ReadFont_36(mut data: *const u8, mut len: usize, mut font: *mut woff2_Font) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    if !(unsafe { file.ReadU32((&mut (*font).flavor as *mut u32)) }) {
        return false;
    }
    if (((*font).flavor) == (kTtcFontFlavor_22)) {
        return false;
    }
    return (unsafe { ReadTrueTypeFont_33((&mut file as *mut woff2_Buffer), data, len, font) });
}
pub unsafe fn ReadFontCollection_37(
    mut data: *const u8,
    mut len: usize,
    mut font_collection: *mut woff2_FontCollection,
) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    if !(unsafe { file.ReadU32((&mut (*font_collection).flavor as *mut u32)) }) {
        return false;
    }
    if (((*font_collection).flavor) != (kTtcFontFlavor_22)) {
        {
            let __a0 = 1_usize as usize;
            (*font_collection)
                .fonts
                .resize_with(__a0, || <woff2_Font>::default())
        };
        let font: *mut woff2_Font =
            &mut (&mut (*font_collection)).fonts[(0_usize)] as *mut woff2_Font;
        (*font).flavor = (*font_collection).flavor;
        return (unsafe {
            ReadTrueTypeFont_33((&mut file as *mut woff2_Buffer), data, len, (font))
        });
    }
    return (unsafe {
        ReadTrueTypeCollection_35((&mut file as *mut woff2_Buffer), data, len, font_collection)
    });
}
pub unsafe fn FontFileSize_38(font: *const woff2_Font) -> usize {
    let mut max_offset: usize = (((12_u64 as u64)
        .wrapping_add((16_u64 as u64).wrapping_mul(((*font).num_tables as u64))))
        as usize);
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let table: *const woff2_Font_Table = &*i.second() as *const woff2_Font_Table;
        let mut padding_size: usize =
            ((((4_u32).wrapping_sub((((*table).length) & (3_u32)))) & (3_u32)) as usize);
        let mut end_offset: usize = ((padding_size).wrapping_add(((*table).offset as usize)))
            .wrapping_add(((*table).length as usize));
        max_offset = ({
            let mut __tmp_0 = (max_offset as u64);
            let mut __tmp_1 = (end_offset as u64);
            (*if *&mut __tmp_0 >= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        } as usize);
    }
    return max_offset;
}
pub unsafe fn FontCollectionFileSize_39(font_collection: *const woff2_FontCollection) -> usize {
    let mut max_offset: usize = 0_usize;
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_ptr().add(font);
        max_offset = ({
            let mut __tmp_0 = (max_offset as u64);
            let mut __tmp_1 = ((unsafe { FontFileSize_38(font) }) as u64);
            (*if *&mut __tmp_0 >= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        } as usize);
    }
    return max_offset;
}
pub unsafe fn WriteFont_40(font: *const woff2_Font, mut dst: *mut u8, mut dst_size: usize) -> bool {
    let mut offset: usize = 0_usize;
    return (unsafe {
        let _font: *const woff2_Font = font;
        let _offset: *mut usize = (&mut offset as *mut usize);
        let _dst: *mut u8 = dst;
        let _dst_size: usize = dst_size;
        WriteFont_41(_font, _offset, _dst, _dst_size)
    });
}
pub unsafe fn WriteTableRecord_42(
    mut table: *const woff2_Font_Table,
    mut offset: *mut usize,
    mut dst: *mut u8,
    mut dst_size: usize,
) -> bool {
    if ((dst_size) < ((*offset).wrapping_add(kSfntEntrySize_24))) {
        return false;
    }
    if (unsafe { (*table).IsReused() }) {
        table = ((*table).reuse_of).cast_const();
    }
    (unsafe { StoreU32_30((*table).tag, offset, dst) });
    (unsafe { StoreU32_30((*table).checksum, offset, dst) });
    (unsafe { StoreU32_30((*table).offset, offset, dst) });
    (unsafe { StoreU32_30((*table).length, offset, dst) });
    return true;
}
pub unsafe fn WriteTable_43(
    table: *const woff2_Font_Table,
    mut offset: *mut usize,
    mut dst: *mut u8,
    mut dst_size: usize,
) -> bool {
    if !(unsafe {
        let _offset: *mut usize = offset;
        let _dst_size: usize = dst_size;
        WriteTableRecord_42((table), _offset, dst, _dst_size)
    }) {
        return false;
    }
    if !(unsafe { (*table).IsReused() }) {
        if ((((*table).offset).wrapping_add((*table).length)) < ((*table).offset))
            || ((dst_size) < ((((*table).offset).wrapping_add((*table).length)) as usize))
        {
            return false;
        }
        {
            if ((*table).length as usize) != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((*table).data as *const u8 as *const ::libc::c_void),
                    (dst.offset(((*table).offset) as isize) as *mut u8 as *mut ::libc::c_void),
                    ((*table).length as usize) as usize,
                )
            }
            (dst.offset(((*table).offset) as isize) as *mut u8 as *mut ::libc::c_void)
        };
        let mut padding_size: usize =
            ((((4_u32).wrapping_sub((((*table).length) & (3_u32)))) & (3_u32)) as usize);
        if ((((((*table).offset).wrapping_add((*table).length)) as usize)
            .wrapping_add(padding_size))
            < (padding_size))
            || ((dst_size)
                < (((((*table).offset).wrapping_add((*table).length)) as usize)
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
pub unsafe fn WriteFont_41(
    font: *const woff2_Font,
    mut offset: *mut usize,
    mut dst: *mut u8,
    mut dst_size: usize,
) -> bool {
    if ((dst_size as u64)
        < ((12_u64 as u64).wrapping_add((16_u64 as u64).wrapping_mul(((*font).num_tables as u64)))))
    {
        return false;
    }
    (unsafe { StoreU32_30((*font).flavor, offset, dst) });
    (unsafe { Store16_31(((*font).num_tables as i32), offset, dst) });
    let mut max_pow2: u16 = (if ((*font).num_tables != 0) {
        (unsafe { Log2Floor_25(((*font).num_tables as u32)) })
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
    (unsafe { Store16_31((search_range as i32), offset, dst) });
    (unsafe { Store16_31((max_pow2 as i32), offset, dst) });
    (unsafe { Store16_31((range_shift as i32), offset, dst) });
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        if !(unsafe {
            let _offset: *mut usize = offset;
            let _dst_size: usize = dst_size;
            WriteTable_43(
                &*i.second() as *const woff2_Font_Table,
                _offset,
                dst,
                _dst_size,
            )
        }) {
            return false;
        }
    }
    return true;
}
pub unsafe fn WriteFontCollection_44(
    font_collection: *const woff2_FontCollection,
    mut dst: *mut u8,
    mut dst_size: usize,
) -> bool {
    let mut offset: usize = 0_usize;
    if (((*font_collection).flavor) != (kTtcFontFlavor_22)) {
        return (unsafe {
            WriteFont_41(
                &(&(*font_collection)).fonts[(0_usize)] as *const woff2_Font,
                (&mut offset as *mut usize),
                dst,
                dst_size,
            )
        });
    }
    (unsafe { StoreU32_30(kTtcFontFlavor_22, (&mut offset as *mut usize), dst) });
    (unsafe {
        StoreU32_30(
            (*font_collection).header_version,
            (&mut offset as *mut usize),
            dst,
        )
    });
    (unsafe {
        StoreU32_30(
            ((*font_collection).fonts.len() as u32),
            (&mut offset as *mut usize),
            dst,
        )
    });
    let mut offset_table: usize = offset;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*font_collection).fonts.len())) {
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), dst) });
        i.postfix_inc();
    }
    if (((*font_collection).header_version) == (131072_u32)) {
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), dst) });
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), dst) });
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), dst) });
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*font_collection).fonts.len())) {
        let font: *const woff2_Font = &(&(*font_collection)).fonts[(i)] as *const woff2_Font;
        (unsafe { StoreU32_30((offset as u32), (&mut offset_table as *mut usize), dst) });
        if !(unsafe {
            let _font: *const woff2_Font = font;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            let _dst_size: usize = dst_size;
            WriteFont_41(_font, _offset, _dst, _dst_size)
        }) {
            return false;
        }
        i.postfix_inc();
    }
    return true;
}
pub unsafe fn NumGlyphs_45(font: *const woff2_Font) -> i32 {
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kHeadTableTag_1;
        (*font).FindTable_u32_const(_tag)
    });
    let mut loca_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kLocaTableTag_2;
        (*font).FindTable_u32_const(_tag)
    });
    if (((head_table).is_null()) || ((loca_table).is_null())) || (((*head_table).length) < (52_u32))
    {
        return 0;
    }
    let mut index_fmt: i32 = (unsafe { IndexFormat_46(font) });
    let mut loca_record_size: i32 = (if ((index_fmt) == (0)) { 2 } else { 4 });
    if (((*loca_table).length) < (loca_record_size as u32)) {
        return 0;
    }
    return (((((*loca_table).length).wrapping_div((loca_record_size as u32))).wrapping_sub(1_u32))
        as i32);
}
pub unsafe fn IndexFormat_46(font: *const woff2_Font) -> i32 {
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kHeadTableTag_1;
        (*font).FindTable_u32_const(_tag)
    });
    if (head_table).is_null() {
        return 0;
    }
    return ((*(*head_table).data.offset((51) as isize)) as i32);
}
impl woff2_Font_Table {
    pub unsafe fn IsReused(&self) -> bool {
        return !((self.reuse_of).is_null());
    }
}
pub unsafe fn GetGlyphData_47(
    font: *const woff2_Font,
    mut glyph_index: i32,
    mut glyph_data: *mut *const u8,
    mut glyph_size: *mut usize,
) -> bool {
    if ((glyph_index) < (0)) {
        return false;
    }
    let mut head_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kHeadTableTag_1;
        (*font).FindTable_u32_const(_tag)
    });
    let mut loca_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kLocaTableTag_2;
        (*font).FindTable_u32_const(_tag)
    });
    let mut glyf_table: *const woff2_Font_Table = (unsafe {
        let _tag: u32 = kGlyfTableTag_0;
        (*font).FindTable_u32_const(_tag)
    });
    if ((((head_table).is_null()) || ((loca_table).is_null())) || ((glyf_table).is_null()))
        || (((*head_table).length) < (52_u32))
    {
        return false;
    }
    let mut index_fmt: i32 = (unsafe { IndexFormat_46(font) });
    let mut loca_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*loca_table).data, ((*loca_table).length as usize));
    if ((index_fmt) == (0)) {
        let mut offset1: u16 = 0_u16;
        let mut offset2: u16 = 0_u16;
        if ((((!(unsafe { loca_buf.Skip((((2) * (glyph_index)) as usize)) }))
            || (!(unsafe { loca_buf.ReadU16((&mut offset1 as *mut u16)) })))
            || (!(unsafe { loca_buf.ReadU16((&mut offset2 as *mut u16)) })))
            || ((offset2 as i32) < (offset1 as i32)))
            || ((((2) * (offset2 as i32)) as u32) > ((*glyf_table).length))
        {
            return false;
        }
        (*glyph_data) = (*glyf_table).data.offset(((2) * (offset1 as i32)) as isize);
        (*glyph_size) = (((2) * ((offset2 as i32) - (offset1 as i32))) as usize);
    } else {
        let mut offset1: u32 = 0_u32;
        let mut offset2: u32 = 0_u32;
        if ((((!(unsafe { loca_buf.Skip((((4) * (glyph_index)) as usize)) }))
            || (!(unsafe { loca_buf.ReadU32((&mut offset1 as *mut u32)) })))
            || (!(unsafe { loca_buf.ReadU32((&mut offset2 as *mut u32)) })))
            || ((offset2) < (offset1)))
            || ((offset2) > ((*glyf_table).length))
        {
            return false;
        }
        (*glyph_data) = (*glyf_table).data.offset((offset1) as isize);
        (*glyph_size) = (((offset2).wrapping_sub(offset1)) as usize);
    }
    return true;
}
pub unsafe fn RemoveDigitalSignature_48(mut font: *mut woff2_Font) -> bool {
    let mut it: UnsafeMapIterator<u32, woff2_Font_Table> = UnsafeMapIterator::find_key(
        &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
        &kDsigTableTag_3,
    );
    if it != UnsafeMapIterator::end(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        UnsafeMapIterator::erase(
            &(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>,
            &it.clone(),
        );
        (*font).num_tables = ((*font).tables.len() as u16).clone();
    }
    return true;
}
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
            instructions_data: std::ptr::null(),
            overlap_simple_flag_set: false,
            contours: Vec::new(),
            composite_data: std::ptr::null(),
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
pub static mut kFLAG_ONCURVE_49: i32 = unsafe { 1 };
pub static mut kFLAG_XSHORT_50: i32 = unsafe { ((1) << (1)) };
pub static mut kFLAG_YSHORT_51: i32 = unsafe { ((1) << (2)) };
pub static mut kFLAG_REPEAT_52: i32 = unsafe { ((1) << (3)) };
pub static mut kFLAG_XREPEATSIGN_53: i32 = unsafe { ((1) << (4)) };
pub static mut kFLAG_YREPEATSIGN_54: i32 = unsafe { ((1) << (5)) };
pub static mut kFLAG_OVERLAP_SIMPLE_55: i32 = unsafe { ((1) << (6)) };
pub static mut kFLAG_ARG_1_AND_2_ARE_WORDS_56: i32 = unsafe { ((1) << (0)) };
pub static mut kFLAG_WE_HAVE_A_SCALE_57: i32 = unsafe { ((1) << (3)) };
pub static mut kFLAG_MORE_COMPONENTS_58: i32 = unsafe { ((1) << (5)) };
pub static mut kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59: i32 = unsafe { ((1) << (6)) };
pub static mut kFLAG_WE_HAVE_A_TWO_BY_TWO_60: i32 = unsafe { ((1) << (7)) };
pub static mut kFLAG_WE_HAVE_INSTRUCTIONS_61: i32 = unsafe { ((1) << (8)) };
pub unsafe fn ReadCompositeGlyphData_62(
    mut buffer: *mut woff2_Buffer,
    mut glyph: *mut woff2_Glyph,
) -> bool {
    (*glyph).have_instructions = false;
    (*glyph).composite_data = (unsafe { (*(buffer).cast_const()).buffer() })
        .offset((unsafe { (*(buffer).cast_const()).offset() }) as isize);
    let mut start_offset: usize = (unsafe { (*(buffer).cast_const()).offset() });
    let mut flags: u16 = (kFLAG_MORE_COMPONENTS_58 as u16);
    'loop_: while (((flags as i32) & (kFLAG_MORE_COMPONENTS_58)) != 0) {
        if !(unsafe { (*buffer).ReadU16((&mut flags as *mut u16)) }) {
            return false;
        }
        (*glyph).have_instructions = (((*glyph).have_instructions as i32)
            | ((((flags as i32) & (kFLAG_WE_HAVE_INSTRUCTIONS_61)) != (0)) as i32))
            != 0;
        let mut arg_size: usize = 2_usize;
        if (((flags as i32) & (kFLAG_ARG_1_AND_2_ARE_WORDS_56)) != 0) {
            arg_size = (arg_size).wrapping_add(4_usize);
        } else {
            arg_size = (arg_size).wrapping_add(2_usize);
        }
        if (((flags as i32) & (kFLAG_WE_HAVE_A_SCALE_57)) != 0) {
            arg_size = (arg_size).wrapping_add(2_usize);
        } else if (((flags as i32) & (kFLAG_WE_HAVE_AN_X_AND_Y_SCALE_59)) != 0) {
            arg_size = (arg_size).wrapping_add(4_usize);
        } else if (((flags as i32) & (kFLAG_WE_HAVE_A_TWO_BY_TWO_60)) != 0) {
            arg_size = (arg_size).wrapping_add(8_usize);
        }
        if !(unsafe { (*buffer).Skip(arg_size) }) {
            return false;
        }
    }
    if (((unsafe { (*(buffer).cast_const()).offset() }).wrapping_sub(start_offset))
        > (<u32>::MAX as usize))
    {
        return false;
    }
    (*glyph).composite_data_size =
        (((unsafe { (*(buffer).cast_const()).offset() }).wrapping_sub(start_offset)) as u32);
    return true;
}
pub unsafe fn ReadGlyph_63(
    mut data: *const u8,
    mut len: usize,
    mut glyph: *mut woff2_Glyph,
) -> bool {
    let mut buffer: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, len);
    let mut num_contours: i16 = 0_i16;
    if !(unsafe { buffer.ReadS16((&mut num_contours as *mut i16)) }) {
        return false;
    }
    if (((!(unsafe { buffer.ReadS16((&mut (*glyph).x_min as *mut i16)) }))
        || (!(unsafe { buffer.ReadS16((&mut (*glyph).y_min as *mut i16)) })))
        || (!(unsafe { buffer.ReadS16((&mut (*glyph).x_max as *mut i16)) })))
        || (!(unsafe { buffer.ReadS16((&mut (*glyph).y_max as *mut i16)) }))
    {
        return false;
    }
    if ((num_contours as i32) == (0)) {
        return true;
    }
    if ((num_contours as i32) > (0)) {
        (*glyph)
            .contours
            .resize_with((num_contours as usize) as usize, || {
                <Vec<woff2_Glyph_Point>>::default()
            });
        let mut last_point_index: u16 = 0_u16;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut point_index: u16 = 0_u16;
            if !(unsafe { buffer.ReadU16((&mut point_index as *mut u16)) }) {
                return false;
            }
            let mut num_points: u16 = ((((point_index as i32) - (last_point_index as i32))
                + (if ((i) == (0)) { 1 } else { 0 }))
                as u16);
            {
                let __a0 = (num_points as usize) as usize;
                (&mut (*glyph)).contours[(i as usize)]
                    .resize_with(__a0, || <woff2_Glyph_Point>::default())
            };
            last_point_index = point_index;
            i.prefix_inc();
        }
        if !(unsafe { buffer.ReadU16((&mut (*glyph).instructions_size as *mut u16)) }) {
            return false;
        }
        (*glyph).instructions_data = data.offset((unsafe { buffer.offset() }) as isize);
        if !(unsafe { buffer.Skip(((*glyph).instructions_size as usize)) }) {
            return false;
        }
        let mut flags: Vec<Vec<u8>> = (0..(num_contours as usize) as usize)
            .map(|_| <Vec<u8>>::default())
            .collect::<Vec<_>>();
        let mut flag: u8 = 0_u8;
        let mut flag_repeat: u8 = 0_u8;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            {
                let __a0 = (&mut (*glyph)).contours[(i as usize)].len() as usize;
                flags[(i as usize)].resize_with(__a0, || <u8>::default())
            };
            let mut j: usize = 0_usize;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as usize)].len())) {
                if ((flag_repeat as i32) == (0)) {
                    if !(unsafe { buffer.ReadU8((&mut flag as *mut u8)) }) {
                        return false;
                    }
                    if (((flag as i32) & (kFLAG_REPEAT_52)) != 0) {
                        if !(unsafe { buffer.ReadU8((&mut flag_repeat as *mut u8)) }) {
                            return false;
                        }
                    }
                } else {
                    flag_repeat.postfix_dec();
                }
                flags[(i as usize)][(j)] = flag;
                (&mut (*glyph)).contours[(i as usize)][(j)].on_curve =
                    (((flag as i32) & (kFLAG_ONCURVE_49)) != 0);
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        if (!flags.is_empty()) && (!flags[(0_usize)].is_empty()) {
            (*glyph).overlap_simple_flag_set =
                (((flags[(0_usize)][(0_usize)] as i32) & (kFLAG_OVERLAP_SIMPLE_55)) != 0);
        }
        let mut prev_x: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut j: usize = 0_usize;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as usize)].len())) {
                let mut flag: u8 = flags[(i as usize)][(j)];
                if (((flag as i32) & (kFLAG_XSHORT_50)) != 0) {
                    let mut x_delta: u8 = 0_u8;
                    if !(unsafe { buffer.ReadU8((&mut x_delta as *mut u8)) }) {
                        return false;
                    }
                    let mut sign: i32 = if (((flag as i32) & (kFLAG_XREPEATSIGN_53)) != 0) {
                        1
                    } else {
                        -1_i32
                    };
                    (&mut (*glyph)).contours[(i as usize)][(j)].x =
                        ((prev_x) + ((sign) * (x_delta as i32)));
                } else {
                    let mut x_delta: i16 = 0_i16;
                    if !(((flag as i32) & (kFLAG_XREPEATSIGN_53)) != 0) {
                        if !(unsafe { buffer.ReadS16((&mut x_delta as *mut i16)) }) {
                            return false;
                        }
                    }
                    (&mut (*glyph)).contours[(i as usize)][(j)].x = ((prev_x) + (x_delta as i32));
                }
                prev_x = (&mut (*glyph)).contours[(i as usize)][(j)].x;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        let mut prev_y: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours as i32)) {
            let mut j: usize = 0_usize;
            'loop_: while ((j) < ((&mut (*glyph)).contours[(i as usize)].len())) {
                let mut flag: u8 = flags[(i as usize)][(j)];
                if (((flag as i32) & (kFLAG_YSHORT_51)) != 0) {
                    let mut y_delta: u8 = 0_u8;
                    if !(unsafe { buffer.ReadU8((&mut y_delta as *mut u8)) }) {
                        return false;
                    }
                    let mut sign: i32 = if (((flag as i32) & (kFLAG_YREPEATSIGN_54)) != 0) {
                        1
                    } else {
                        -1_i32
                    };
                    (&mut (*glyph)).contours[(i as usize)][(j)].y =
                        ((prev_y) + ((sign) * (y_delta as i32)));
                } else {
                    let mut y_delta: i16 = 0_i16;
                    if !(((flag as i32) & (kFLAG_YREPEATSIGN_54)) != 0) {
                        if !(unsafe { buffer.ReadS16((&mut y_delta as *mut i16)) }) {
                            return false;
                        }
                    }
                    (&mut (*glyph)).contours[(i as usize)][(j)].y = ((prev_y) + (y_delta as i32));
                }
                prev_y = (&mut (*glyph)).contours[(i as usize)][(j)].y;
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    } else if ((num_contours as i32) == (-1_i32)) {
        if !(unsafe { ReadCompositeGlyphData_62((&mut buffer as *mut woff2_Buffer), glyph) }) {
            return false;
        }
        if (*glyph).have_instructions {
            if !(unsafe { buffer.ReadU16((&mut (*glyph).instructions_size as *mut u16)) }) {
                return false;
            }
            (*glyph).instructions_data = data.offset((unsafe { buffer.offset() }) as isize);
            if !(unsafe { buffer.Skip(((*glyph).instructions_size as usize)) }) {
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
pub unsafe fn StoreBbox_64(glyph: *const woff2_Glyph, mut offset: *mut usize, mut dst: *mut u8) {
    (unsafe { Store16_31(((*glyph).x_min as i32), offset, dst) });
    (unsafe { Store16_31(((*glyph).y_min as i32), offset, dst) });
    (unsafe { Store16_31(((*glyph).x_max as i32), offset, dst) });
    (unsafe { Store16_31(((*glyph).y_max as i32), offset, dst) });
}
pub unsafe fn StoreInstructions_65(
    glyph: *const woff2_Glyph,
    mut offset: *mut usize,
    mut dst: *mut u8,
) {
    (unsafe { Store16_31(((*glyph).instructions_size as i32), offset, dst) });
    (unsafe {
        let _data: *const u8 = (*glyph).instructions_data;
        let _len: usize = ((*glyph).instructions_size as usize);
        let _offset: *mut usize = offset;
        StoreBytes_32(_data, _len, _offset, dst)
    });
}
pub unsafe fn StoreEndPtsOfContours_66(
    glyph: *const woff2_Glyph,
    mut offset: *mut usize,
    mut dst: *mut u8,
) -> bool {
    let mut end_point: i32 = -1_i32;
    'loop_: for contour in 0..((*glyph).contours.len()) {
        let mut contour = (*glyph).contours.as_ptr().add(contour);
        end_point = ((end_point as usize).wrapping_add((*contour).len())) as i32;
        if (((*contour).len()) > (<u16>::MAX as usize)) || ((end_point) > (<u16>::MAX as i32)) {
            return false;
        }
        (unsafe { Store16_31(end_point, offset, dst) });
    }
    return true;
}
pub unsafe fn StorePoints_67(
    glyph: *const woff2_Glyph,
    mut offset: *mut usize,
    mut dst: *mut u8,
    mut dst_size: usize,
) -> bool {
    let mut previous_flag: i32 = -1_i32;
    let mut repeat_count: i32 = 0;
    let mut last_x: i32 = 0;
    let mut last_y: i32 = 0;
    let mut x_bytes: usize = 0_usize;
    let mut y_bytes: usize = 0_usize;
    'loop_: for contour in 0..((*glyph).contours.len()) {
        let mut contour = (*glyph).contours.as_ptr().add(contour);
        'loop_: for point in 0..((*contour).len()) {
            let mut point = (*contour).as_ptr().add(point);
            let mut flag: i32 = if (*point).on_curve {
                kFLAG_ONCURVE_49
            } else {
                0
            };
            if ((previous_flag) == (-1_i32)) && ((*glyph).overlap_simple_flag_set) {
                flag = ((flag) | (kFLAG_OVERLAP_SIMPLE_55));
            }
            let mut dx: i32 = (((*point).x) - (last_x));
            let mut dy: i32 = (((*point).y) - (last_y));
            if ((dx) == (0)) {
                flag |= kFLAG_XREPEATSIGN_53;
            } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
                flag |= ((kFLAG_XSHORT_50)
                    | (if ((dx) > (0)) {
                        kFLAG_XREPEATSIGN_53
                    } else {
                        0
                    }));
                x_bytes = (x_bytes).wrapping_add(1_usize);
            } else {
                x_bytes = (x_bytes).wrapping_add(2_usize);
            }
            if ((dy) == (0)) {
                flag |= kFLAG_YREPEATSIGN_54;
            } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
                flag |= ((kFLAG_YSHORT_51)
                    | (if ((dy) > (0)) {
                        kFLAG_YREPEATSIGN_54
                    } else {
                        0
                    }));
                y_bytes = (y_bytes).wrapping_add(1_usize);
            } else {
                y_bytes = (y_bytes).wrapping_add(2_usize);
            }
            if ((flag) == (previous_flag)) && ((repeat_count) != (255)) {
                (*dst.offset(((*offset).wrapping_sub(1_usize)) as isize)) =
                    (((*dst.offset(((*offset).wrapping_sub(1_usize)) as isize)) as i32)
                        | kFLAG_REPEAT_52) as u8;
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
    let mut x_offset: usize = (*offset);
    let mut y_offset: usize = (*offset).wrapping_add(x_bytes);
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
                (unsafe { Store16_31(dx, (&mut x_offset as *mut usize), dst) });
            }
            if ((dy) == (0)) {
            } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
                (*dst.offset((y_offset.postfix_inc()) as isize)) = (dy.abs() as u8);
            } else {
                (unsafe { Store16_31(dy, (&mut y_offset as *mut usize), dst) });
            }
            last_x += dx;
            last_y += dy;
        }
    }
    (*offset) = y_offset;
    return true;
}
pub unsafe fn StoreGlyph_68(
    glyph: *const woff2_Glyph,
    mut dst: *mut u8,
    mut dst_size: *mut usize,
) -> bool {
    let mut offset: usize = 0_usize;
    if (((*glyph).composite_data_size) > (0_u32)) {
        if (((*dst_size) as u64)
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
        (unsafe { Store16_31(-1_i32, (&mut offset as *mut usize), dst) });
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            StoreBbox_64(_glyph, _offset, _dst)
        });
        (unsafe {
            let _data: *const u8 = (*glyph).composite_data;
            let _len: usize = ((*glyph).composite_data_size as usize);
            StoreBytes_32(_data, _len, (&mut offset as *mut usize), dst)
        });
        if (*glyph).have_instructions {
            (unsafe {
                let _glyph: *const woff2_Glyph = glyph;
                let _offset: *mut usize = (&mut offset as *mut usize);
                let _dst: *mut u8 = dst;
                StoreInstructions_65(_glyph, _offset, _dst)
            });
        }
    } else if (((*glyph).contours.len()) > (0_usize)) {
        if (((*glyph).contours.len()) > (<i16>::MAX as usize)) {
            return false;
        }
        if (((*dst_size) as u64)
            < (((12_u64 as u64)
                .wrapping_add((((2_usize).wrapping_mul((*glyph).contours.len())) as u64)))
            .wrapping_add(((*glyph).instructions_size as u64))))
        {
            return false;
        }
        (unsafe {
            Store16_31(
                ((*glyph).contours.len() as i32),
                (&mut offset as *mut usize),
                dst,
            )
        });
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            StoreBbox_64(_glyph, _offset, _dst)
        });
        if !(unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            StoreEndPtsOfContours_66(_glyph, _offset, _dst)
        }) {
            return false;
        }
        (unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            StoreInstructions_65(_glyph, _offset, _dst)
        });
        if !(unsafe {
            let _glyph: *const woff2_Glyph = glyph;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = dst;
            let _dst_size: usize = (*dst_size);
            StorePoints_67(_glyph, _offset, _dst, _dst_size)
        }) {
            return false;
        }
    }
    (*dst_size) = offset;
    return true;
}
pub unsafe fn Round4_69(mut value: i32) -> i32 {
    if (((<i32>::MAX) - (value)) < (3)) {
        return value;
    }
    return (((value) + (3)) & (!3));
}
pub unsafe fn Round4_70(mut value: u64) -> u64 {
    if (((<u64>::MAX as u64).wrapping_sub(value)) < (3_u64)) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub unsafe fn Round4_71(mut value: u32) -> u32 {
    if (((<u32>::MAX as u32).wrapping_sub(value)) < (3_u32)) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub unsafe fn StoreLoca_72(
    mut index_fmt: i32,
    mut value: u32,
    mut offset: *mut usize,
    mut dst: *mut u8,
) {
    if ((index_fmt) == (0)) {
        (unsafe { Store16_31((((value) >> (1)) as i32), offset, dst) });
    } else {
        (unsafe { StoreU32_30(value, offset, dst) });
    }
}
pub unsafe fn WriteNormalizedLoca_73(
    mut index_fmt: i32,
    mut num_glyphs: i32,
    mut font: *mut woff2_Font,
) -> bool {
    let mut glyf_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kGlyfTableTag_0) });
    let mut loca_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kLocaTableTag_2) });
    let mut glyph_sz: i32 = if ((index_fmt) == (0)) { 2 } else { 4 };
    {
        let __a0 = (((unsafe { Round4_69(((num_glyphs) + (1))) }) * (glyph_sz)) as usize) as usize;
        (*loca_table).buffer.resize_with(__a0, || <u8>::default())
    };
    (*loca_table).length = ((((num_glyphs) + (1)) * (glyph_sz)) as u32);
    let mut glyf_dst: *mut u8 = if (num_glyphs != 0) {
        (&mut (&mut (*glyf_table)).buffer[(0_usize)] as *mut u8)
    } else {
        std::ptr::null_mut()
    };
    let mut loca_dst: *mut u8 = (&mut (&mut (*loca_table)).buffer[(0_usize)] as *mut u8);
    let mut glyf_offset: u32 = 0_u32;
    let mut loca_offset: usize = 0_usize;
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        (unsafe {
            StoreLoca_72(
                index_fmt,
                glyf_offset,
                (&mut loca_offset as *mut usize),
                loca_dst,
            )
        });
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = std::ptr::null();
        let mut glyph_size: usize = 0_usize;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut usize = (&mut glyph_size as *mut usize);
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_usize))
            && (!(unsafe {
                ReadGlyph_63(glyph_data, glyph_size, (&mut glyph as *mut woff2_Glyph))
            })))
        {
            return false;
        }
        let mut glyf_dst_size: usize =
            ((*glyf_table).buffer.len()).wrapping_sub((glyf_offset as usize));
        if !(unsafe {
            StoreGlyph_68(
                &glyph as *const woff2_Glyph,
                glyf_dst.offset((glyf_offset) as isize),
                (&mut glyf_dst_size as *mut usize),
            )
        }) {
            return false;
        }
        glyf_dst_size = ((unsafe { Round4_70((glyf_dst_size as u64)) }) as usize);
        if (((glyf_dst_size) > (<u32>::MAX as usize))
            || (((glyf_offset).wrapping_add((glyf_dst_size as u32))) < (glyf_offset)))
            || (((index_fmt) == (0))
                && (((glyf_offset as usize).wrapping_add(glyf_dst_size))
                    >= (((1_u64) << (17)) as usize)))
        {
            return false;
        }
        glyf_offset = ((glyf_offset as usize).wrapping_add(glyf_dst_size)) as u32;
        i.prefix_inc();
    }
    (unsafe {
        StoreLoca_72(
            index_fmt,
            glyf_offset,
            (&mut loca_offset as *mut usize),
            loca_dst,
        )
    });
    {
        let __a0 = (glyf_offset as usize) as usize;
        (*glyf_table).buffer.resize_with(__a0, || <u8>::default())
    };
    (*glyf_table).data = (if (glyf_offset != 0) {
        (&mut (&mut (*glyf_table)).buffer[(0_usize)] as *mut u8)
    } else {
        std::ptr::null_mut()
    })
    .cast_const();
    (*glyf_table).length = glyf_offset;
    (*loca_table).data = (if (loca_offset != 0) {
        (&mut (&mut (*loca_table)).buffer[(0_usize)] as *mut u8)
    } else {
        std::ptr::null_mut()
    })
    .cast_const();
    return true;
}
pub unsafe fn MakeEditableBuffer_74(mut font: *mut woff2_Font, mut tableTag: i32) -> bool {
    let mut table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32((tableTag as u32)) });
    if (table).is_null() {
        return false;
    }
    if (unsafe { (*(table).cast_const()).IsReused() }) {
        return true;
    }
    let mut sz: i32 = ((unsafe { Round4_71((*table).length) }) as i32);
    {
        let __a0 = (sz as usize) as usize;
        (*table).buffer.resize_with(__a0, || <u8>::default())
    };
    let mut buf: *mut u8 = (&mut (&mut (*table)).buffer[(0_usize)] as *mut u8);
    {
        if ((*table).length as usize) != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((*table).data as *const u8 as *const ::libc::c_void),
                (buf as *mut u8 as *mut ::libc::c_void),
                ((*table).length as usize) as usize,
            )
        }
        (buf as *mut u8 as *mut ::libc::c_void)
    };
    if ((((sz as u32) > ((*table).length)) as i64) != 0) {
        {
            let byte_0 = (buf.offset(((*table).length) as isize) as *mut u8 as *mut ::libc::c_void)
                as *mut u8;
            for offset in 0..(((sz as u32).wrapping_sub((*table).length)) as usize) {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            (buf.offset(((*table).length) as isize) as *mut u8 as *mut ::libc::c_void)
        };
    }
    (*table).data = (buf).cast_const();
    return true;
}
pub unsafe fn NormalizeGlyphs_75(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kHeadTableTag_1) });
    let mut glyf_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kGlyfTableTag_0) });
    let mut loca_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kLocaTableTag_2) });
    if (head_table).is_null() {
        return false;
    }
    if ((loca_table).is_null()) && ((glyf_table).is_null()) {
        return true;
    }
    if ((((glyf_table).is_null()) as i32) != (((loca_table).is_null()) as i32)) {
        return false;
    }
    if (((unsafe { (*(loca_table).cast_const()).IsReused() }) as i32)
        != ((unsafe { (*(glyf_table).cast_const()).IsReused() }) as i32))
    {
        return false;
    }
    if (unsafe { (*(loca_table).cast_const()).IsReused() }) {
        return true;
    }
    let mut index_fmt: i32 = ((*(*head_table).data.offset((51) as isize)) as i32);
    let mut num_glyphs: i32 = (unsafe { NumGlyphs_45(&(*font) as *const woff2_Font) });
    let mut max_normalized_glyf_size: usize =
        ((((1.1E+0) * ((*glyf_table).length as f64)) + (((2) * (num_glyphs)) as f64)) as usize);
    {
        let __a0 = max_normalized_glyf_size as usize;
        (*glyf_table).buffer.resize_with(__a0, || <u8>::default())
    };
    if !(unsafe { WriteNormalizedLoca_73(index_fmt, num_glyphs, font) }) {
        if ((index_fmt) != (0)) {
            return false;
        }
        index_fmt = 1;
        if !(unsafe { WriteNormalizedLoca_73(index_fmt, num_glyphs, font) }) {
            return false;
        }
        (&mut (*head_table)).buffer[(51_usize)] = 1_u8;
    }
    return true;
}
pub unsafe fn NormalizeOffsets_76(mut font: *mut woff2_Font) -> bool {
    let mut offset: u32 = (((12) + ((16) * ((*font).num_tables as i32))) as u32);
    'loop_: for tag in 0..((unsafe { (*(font).cast_const()).OutputOrderedTags() }).len()) {
        let mut tag = (unsafe { (&(*(font).cast_const())).OutputOrderedTags() })[tag].clone();
        let table: *mut woff2_Font_Table =
            &mut (*(*font).tables.entry(tag).or_default().as_mut()) as *mut woff2_Font_Table;
        (*table).offset = offset;
        offset = (offset).wrapping_add((unsafe { Round4_71((*table).length) }));
    }
    return true;
}
pub unsafe fn ComputeHeaderChecksum_77(font: *const woff2_Font) -> u32 {
    let mut checksum: u32 = (*font).flavor;
    let mut max_pow2: u16 = (if ((*font).num_tables != 0) {
        (unsafe { Log2Floor_25(((*font).num_tables as u32)) })
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
            table = ((*table).reuse_of).cast_const();
        }
        checksum = (checksum).wrapping_add((*table).tag);
        checksum = (checksum).wrapping_add((*table).checksum);
        checksum = (checksum).wrapping_add((*table).offset);
        checksum = (checksum).wrapping_add((*table).length);
    }
    return checksum;
}
pub unsafe fn FixChecksums_78(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kHeadTableTag_1) });
    if (head_table).is_null() {
        return false;
    }
    if !(((*head_table).reuse_of).is_null()) {
        head_table = (*head_table).reuse_of;
    }
    if (((*head_table).length) < (12_u32)) {
        return false;
    }
    let mut head_buf: *mut u8 = (&mut (&mut (*head_table)).buffer[(0_usize)] as *mut u8);
    let mut offset: usize = 8_usize;
    (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), head_buf) });
    let mut file_checksum: u32 = 0_u32;
    let mut head_checksum: u32 = 0_u32;
    'loop_: for i in
        UnsafeMapIterator::begin(&(*font).tables as *const BTreeMap<u32, Box<woff2_Font_Table>>)
    {
        let mut table: *mut woff2_Font_Table = (&mut *i.second() as *mut woff2_Font_Table);
        if (unsafe { (*(table).cast_const()).IsReused() }) {
            table = (*table).reuse_of;
        }
        (*table).checksum = (unsafe {
            let _buf: *const u8 = (*table).data;
            let _size: usize = ((*table).length as usize);
            ComputeULongSum_26(_buf, _size)
        });
        file_checksum = (file_checksum).wrapping_add((*table).checksum);
        if (((*table).tag) == (kHeadTableTag_1)) {
            head_checksum = (*table).checksum;
        }
    }
    file_checksum = (file_checksum)
        .wrapping_add((unsafe { ComputeHeaderChecksum_77(&(*font) as *const woff2_Font) }));
    offset = 8_usize;
    (unsafe {
        StoreU32_30(
            (2981146554_u32 as u32).wrapping_sub(file_checksum),
            (&mut offset as *mut usize),
            head_buf,
        )
    });
    return true;
}
pub unsafe fn MarkTransformed_79(mut font: *mut woff2_Font) -> bool {
    let mut head_table: *mut woff2_Font_Table = (unsafe { (*font).FindTable_u32(kHeadTableTag_1) });
    if (head_table).is_null() {
        return false;
    }
    if !(((*head_table).reuse_of).is_null()) {
        head_table = (*head_table).reuse_of;
    }
    if (((*head_table).length) < (17_u32)) {
        return false;
    }
    let mut head_flags: i32 = ((*(*head_table).data.offset((16) as isize)) as i32);
    (&mut (*head_table)).buffer[(16_usize)] = (((head_flags) | (8)) as u8);
    return true;
}
pub unsafe fn NormalizeWithoutFixingChecksums_80(mut font: *mut woff2_Font) -> bool {
    return (((((unsafe { MakeEditableBuffer_74(font, (kHeadTableTag_1 as i32)) })
        && (unsafe { RemoveDigitalSignature_48(font) }))
        && (unsafe { MarkTransformed_79(font) }))
        && (unsafe { NormalizeGlyphs_75(font) }))
        && (unsafe { NormalizeOffsets_76(font) }));
}
pub unsafe fn NormalizeFont_81(mut font: *mut woff2_Font) -> bool {
    return ((unsafe { NormalizeWithoutFixingChecksums_80(font) })
        && (unsafe { FixChecksums_78(font) }));
}
pub unsafe fn NormalizeFontCollection_82(mut font_collection: *mut woff2_FontCollection) -> bool {
    if (((*font_collection).fonts.len()) == (1_usize)) {
        return (unsafe {
            NormalizeFont_81((&mut (&mut (*font_collection)).fonts[(0_usize)] as *mut woff2_Font))
        });
    }
    let mut offset: u32 = ((unsafe {
        let _header_version: u32 = (*font_collection).header_version;
        let _num_fonts: u32 = ((*font_collection).fonts.len() as u32);
        CollectionHeaderSize_27(_header_version, _num_fonts)
    }) as u32);
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe { NormalizeWithoutFixingChecksums_80((font)) }) {
            printf(c"Font normalization failed.\n".as_ptr() as *const i8);
            return false;
        }
        offset = ((offset as usize).wrapping_add(
            (kSfntHeaderSize_23)
                .wrapping_add((kSfntEntrySize_24).wrapping_mul(((*font).num_tables as usize))),
        )) as u32;
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
                offset = (offset).wrapping_add((unsafe { Round4_71((*table).length) }));
            }
        }
    }
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe { FixChecksums_78((font)) }) {
            printf(c"Failed to fix checksums\n".as_ptr() as *const i8);
            return false;
        }
    }
    return true;
}
pub static mut FLAG_ARG_1_AND_2_ARE_WORDS_83: i32 = unsafe { ((1) << (0)) };
pub static mut FLAG_WE_HAVE_INSTRUCTIONS_84: i32 = unsafe { ((1) << (8)) };
pub static mut FLAG_OVERLAP_SIMPLE_BITMAP_85: i32 = unsafe { ((1) << (0)) };
pub unsafe fn WriteBytes_86(mut out: *mut Vec<u8>, mut data: *const u8, mut len: usize) {
    if ((len) == (0_usize)) {
        return;
    }
    let mut offset: usize = (*(out).cast_const()).len();
    {
        let __a0 = (offset).wrapping_add(len) as usize;
        (*out).resize_with(__a0, || <u8>::default())
    };
    {
        if len != 0 {
            ::std::ptr::copy_nonoverlapping(
                (data as *const u8 as *const ::libc::c_void),
                ((&mut (&mut (*out))[(offset)] as *mut u8) as *mut u8 as *mut ::libc::c_void),
                len as usize,
            )
        }
        ((&mut (&mut (*out))[(offset)] as *mut u8) as *mut u8 as *mut ::libc::c_void)
    };
}
pub unsafe fn WriteBytes_87(mut out: *mut Vec<u8>, in_: *const Vec<u8>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*in_).len())) {
        {
            let a0_clone = (&(*in_))[(i)].clone();
            (*out).push(a0_clone)
        };
        i.prefix_inc();
    }
}
pub unsafe fn WriteUShort_88(mut out: *mut Vec<u8>, mut value: i32) {
    (*out).push((((value) >> (8)) as u8));
    (*out).push((((value) & (255)) as u8));
}
pub unsafe fn WriteLong_89(mut out: *mut Vec<u8>, mut value: i32) {
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
            let __a0 = (((((num_glyphs) + (31)) >> (5)) << (2)) as usize) as usize;
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
        } else if (((*glyph).contours.len()) > (0_usize)) {
            (unsafe {
                let _glyph_id: i32 = glyph_id;
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteSimpleGlyph(_glyph_id, _glyph)
            });
        } else {
            (unsafe { WriteUShort_88((&mut self.n_contour_stream_ as *mut Vec<u8>), 0) });
        }
        return true;
    }
    pub unsafe fn GetTransformedGlyfBytes(&mut self, mut result: *mut Vec<u8>) {
        (unsafe { WriteUShort_88(result, 0) });
        (unsafe {
            WriteUShort_88(
                result,
                if self.overlap_bitmap_.is_empty() {
                    0
                } else {
                    FLAG_OVERLAP_SIMPLE_BITMAP_85
                },
            )
        });
        (unsafe { WriteUShort_88(result, self.n_glyphs_) });
        (unsafe { WriteUShort_88(result, 0) });
        (unsafe { WriteLong_89(result, (self.n_contour_stream_.len() as i32)) });
        (unsafe { WriteLong_89(result, (self.n_points_stream_.len() as i32)) });
        (unsafe { WriteLong_89(result, (self.flag_byte_stream_.len() as i32)) });
        (unsafe { WriteLong_89(result, (self.glyph_stream_.len() as i32)) });
        (unsafe { WriteLong_89(result, (self.composite_stream_.len() as i32)) });
        (unsafe {
            WriteLong_89(
                result,
                (((self.bbox_bitmap_.len()).wrapping_add(self.bbox_stream_.len())) as i32),
            )
        });
        (unsafe { WriteLong_89(result, (self.instruction_stream_.len() as i32)) });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.n_contour_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.n_points_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.flag_byte_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.glyph_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.composite_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.bbox_bitmap_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.bbox_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        (unsafe {
            let _out: *mut Vec<u8> = result;
            let _in: *const Vec<u8> = &self.instruction_stream_ as *const Vec<u8>;
            WriteBytes_87(_out, _in)
        });
        if !self.overlap_bitmap_.is_empty() {
            (unsafe {
                let _out: *mut Vec<u8> = result;
                let _in: *const Vec<u8> = &self.overlap_bitmap_ as *const Vec<u8>;
                WriteBytes_87(_out, _in)
            });
        }
    }
    unsafe fn WriteInstructions(&mut self, glyph: *const woff2_Glyph) {
        (unsafe {
            Write255UShort_10(
                (&mut self.glyph_stream_ as *mut Vec<u8>),
                ((*glyph).instructions_size as i32),
            )
        });
        (unsafe {
            let _data: *const u8 = (*glyph).instructions_data;
            let _len: usize = ((*glyph).instructions_size as usize);
            WriteBytes_86((&mut self.instruction_stream_ as *mut Vec<u8>), _data, _len)
        });
    }
    unsafe fn ShouldWriteSimpleGlyphBbox(&mut self, glyph: *const woff2_Glyph) -> bool {
        if ((*glyph).contours.is_empty()) || ((&(*glyph)).contours[(0_usize)].is_empty()) {
            return ((((*glyph).x_min != 0) || ((*glyph).y_min != 0)) || ((*glyph).x_max != 0))
                || ((*glyph).y_max != 0);
        }
        let mut x_min: i16 = ((&(*glyph)).contours[(0_usize)][(0_usize)].x as i16);
        let mut y_min: i16 = ((&(*glyph)).contours[(0_usize)][(0_usize)].y as i16);
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
            self.overlap_bitmap_[(((glyph_id) >> (3)) as usize)] =
                ((self.overlap_bitmap_[(((glyph_id) >> (3)) as usize)] as i32)
                    | ((128) >> ((glyph_id) & (7)))) as u8;
        }
        let mut num_contours: i32 = ((*glyph).contours.len() as i32);
        (unsafe { WriteUShort_88((&mut self.n_contour_stream_ as *mut Vec<u8>), num_contours) });
        if (unsafe { self.ShouldWriteSimpleGlyphBbox(glyph) }) {
            (unsafe {
                let _glyph_id: i32 = glyph_id;
                let _glyph: *const woff2_Glyph = glyph;
                self.WriteBbox(_glyph_id, _glyph)
            });
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours)) {
            (unsafe {
                Write255UShort_10(
                    (&mut self.n_points_stream_ as *mut Vec<u8>),
                    ((&(*glyph)).contours[(i as usize)].len() as i32),
                )
            });
            i.postfix_inc();
        }
        let mut lastX: i32 = 0;
        let mut lastY: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (num_contours)) {
            let mut num_points: i32 = ((&(*glyph)).contours[(i as usize)].len() as i32);
            let mut j: i32 = 0;
            'loop_: while ((j) < (num_points)) {
                let mut x: i32 = (&(*glyph)).contours[(i as usize)][(j as usize)].x;
                let mut y: i32 = (&(*glyph)).contours[(i as usize)][(j as usize)].y;
                let mut dx: i32 = ((x) - (lastX));
                let mut dy: i32 = ((y) - (lastY));
                (unsafe {
                    self.WriteTriplet(
                        (&(*glyph)).contours[(i as usize)][(j as usize)].on_curve,
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
        if ((num_contours) > (0)) {
            (unsafe { self.WriteInstructions(glyph) });
        }
    }
    unsafe fn WriteCompositeGlyph(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) {
        (unsafe { WriteUShort_88((&mut self.n_contour_stream_ as *mut Vec<u8>), -1_i32) });
        (unsafe {
            let _glyph_id: i32 = glyph_id;
            let _glyph: *const woff2_Glyph = glyph;
            self.WriteBbox(_glyph_id, _glyph)
        });
        (unsafe {
            let _data: *const u8 = (*glyph).composite_data;
            let _len: usize = ((*glyph).composite_data_size as usize);
            WriteBytes_86((&mut self.composite_stream_ as *mut Vec<u8>), _data, _len)
        });
        if (*glyph).have_instructions {
            (unsafe { self.WriteInstructions(glyph) });
        }
    }
    unsafe fn WriteBbox(&mut self, mut glyph_id: i32, glyph: *const woff2_Glyph) {
        self.bbox_bitmap_[(((glyph_id) >> (3)) as usize)] =
            ((self.bbox_bitmap_[(((glyph_id) >> (3)) as usize)] as i32)
                | ((128) >> ((glyph_id) & (7)))) as u8;
        (unsafe {
            WriteUShort_88(
                (&mut self.bbox_stream_ as *mut Vec<u8>),
                ((*glyph).x_min as i32),
            )
        });
        (unsafe {
            WriteUShort_88(
                (&mut self.bbox_stream_ as *mut Vec<u8>),
                ((*glyph).y_min as i32),
            )
        });
        (unsafe {
            WriteUShort_88(
                (&mut self.bbox_stream_ as *mut Vec<u8>),
                ((*glyph).x_max as i32),
            )
        });
        (unsafe {
            WriteUShort_88(
                (&mut self.bbox_stream_ as *mut Vec<u8>),
                ((*glyph).y_max as i32),
            )
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
                let __a0 = ((((self.n_glyphs_) + (7)) >> (3)) as usize) as usize;
                self.overlap_bitmap_.resize_with(__a0, || <u8>::default())
            };
        }
    }
}
pub unsafe fn TransformGlyfAndLocaTables_90(mut font: *mut woff2_Font) -> bool {
    let mut glyf_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kGlyfTableTag_0) }).cast_const();
    let mut loca_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kLocaTableTag_2) }).cast_const();
    if ((loca_table).is_null()) && ((glyf_table).is_null()) {
        return true;
    }
    if ((((glyf_table).is_null()) as i32) != (((loca_table).is_null()) as i32)) {
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
        .entry(((kGlyfTableTag_0) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut transformed_loca: *mut woff2_Font_Table = (&mut (*(*font)
        .tables
        .entry(((kLocaTableTag_2) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut num_glyphs: i32 = (unsafe { NumGlyphs_45(&(*font) as *const woff2_Font) });
    let mut encoder: woff2_GlyfEncoder = woff2_GlyfEncoder::woff2_GlyfEncoder(num_glyphs);
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = std::ptr::null();
        let mut glyph_size: usize = 0_usize;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut usize = (&mut glyph_size as *mut usize);
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_usize))
            && (!(unsafe {
                ReadGlyph_63(glyph_data, glyph_size, (&mut glyph as *mut woff2_Glyph))
            })))
        {
            return false;
        }
        (unsafe { encoder.Encode(i, &glyph as *const woff2_Glyph) });
        i.prefix_inc();
    }
    (unsafe { encoder.GetTransformedGlyfBytes((&mut (*transformed_glyf).buffer as *mut Vec<u8>)) });
    let mut head_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kHeadTableTag_1) }).cast_const();
    if ((head_table).is_null()) || (((*head_table).length) < (52_u32)) {
        return false;
    }
    (&mut (*transformed_glyf)).buffer[(7_usize)] = (*(*head_table).data.offset((51) as isize));
    (*transformed_glyf).tag = ((kGlyfTableTag_0) ^ (2155905152_u32));
    (*transformed_glyf).length = ((*transformed_glyf).buffer.len() as u32).clone();
    (*transformed_glyf).data = ((*transformed_glyf).buffer.as_mut_ptr()).cast_const();
    (*transformed_loca).tag = ((kLocaTableTag_2) ^ (2155905152_u32));
    (*transformed_loca).length = 0_u32;
    (*transformed_loca).data = std::ptr::null();
    return true;
}
pub unsafe fn TransformHmtxTable_91(mut font: *mut woff2_Font) -> bool {
    let mut glyf_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kGlyfTableTag_0) }).cast_const();
    let mut hmtx_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kHmtxTableTag_5) }).cast_const();
    let mut hhea_table: *const woff2_Font_Table =
        (unsafe { (*font).FindTable_u32(kHheaTableTag_6) }).cast_const();
    if ((hmtx_table).is_null()) || ((glyf_table).is_null()) {
        return true;
    }
    if (hhea_table).is_null() {
        return false;
    }
    let mut hhea_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*hhea_table).data, ((*hhea_table).length as usize));
    let mut num_hmetrics: u16 = 0_u16;
    if (!(unsafe { hhea_buf.Skip(34_usize) }))
        || (!(unsafe { hhea_buf.ReadU16((&mut num_hmetrics as *mut u16)) }))
    {
        return false;
    }
    if ((num_hmetrics as i32) < (1)) {
        return false;
    }
    let mut num_glyphs: i32 = (unsafe { NumGlyphs_45(&(*font) as *const woff2_Font) });
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut proportional_lsbs: Vec<i16> = Vec::new();
    let mut monospace_lsbs: Vec<i16> = Vec::new();
    let mut remove_proportional_lsb: bool = true;
    let mut remove_monospace_lsb: bool = (((num_glyphs) - (num_hmetrics as i32)) > (0));
    let mut hmtx_buf: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((*hmtx_table).data, ((*hmtx_table).length as usize));
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_glyphs)) {
        let mut glyph: woff2_Glyph = woff2_Glyph::woff2_Glyph();
        let mut glyph_data: *const u8 = std::ptr::null();
        let mut glyph_size: usize = 0_usize;
        if (!(unsafe {
            let _font: *const woff2_Font = &(*font) as *const woff2_Font;
            let _glyph_index: i32 = i;
            let _glyph_data: *mut *const u8 = (&mut glyph_data as *mut *const u8);
            let _glyph_size: *mut usize = (&mut glyph_size as *mut usize);
            GetGlyphData_47(_font, _glyph_index, _glyph_data, _glyph_size)
        })) || (((glyph_size) > (0_usize))
            && (!(unsafe {
                ReadGlyph_63(glyph_data, glyph_size, (&mut glyph as *mut woff2_Glyph))
            })))
        {
            return false;
        }
        let mut advance_width: u16 = 0_u16;
        let mut lsb: i16 = 0_i16;
        if ((i) < (num_hmetrics as i32)) {
            if !(unsafe { hmtx_buf.ReadU16((&mut advance_width as *mut u16)) }) {
                return false;
            }
            if !(unsafe { hmtx_buf.ReadS16((&mut lsb as *mut i16)) }) {
                return false;
            }
            if ((glyph_size) > (0_usize)) && ((glyph.x_min as i32) != (lsb as i32)) {
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
            if !(unsafe { hmtx_buf.ReadS16((&mut lsb as *mut i16)) }) {
                return false;
            }
            if ((glyph_size) > (0_usize)) && ((glyph.x_min as i32) != (lsb as i32)) {
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
        .entry(((kHmtxTableTag_5) ^ (2155905152_u32)))
        .or_default()
        .as_mut()) as *mut woff2_Font_Table);
    let mut flags: u8 = 0_u8;
    let mut transformed_size: usize =
        (1_usize).wrapping_add((2_usize).wrapping_mul(advance_widths.len()));
    if remove_proportional_lsb {
        flags = ((flags as i32) | 1) as u8;
    } else {
        transformed_size = ((transformed_size as u64)
            .wrapping_add(((2_usize).wrapping_mul(proportional_lsbs.len()) as u64)))
            as usize;
    }
    if remove_monospace_lsb {
        flags = ((flags as i32) | ((1) << (1))) as u8;
    } else {
        transformed_size = ((transformed_size as u64)
            .wrapping_add(((2_usize).wrapping_mul(monospace_lsbs.len()) as u64)))
            as usize;
    }
    if transformed_size as usize > (*transformed_hmtx).buffer.capacity() as usize {
        let len_0 = (*transformed_hmtx).buffer.len();
        (*transformed_hmtx)
            .buffer
            .reserve_exact(transformed_size as usize - len_0 as usize);
    };
    let mut out: *mut Vec<u8> = (&mut (*transformed_hmtx).buffer as *mut Vec<u8>);
    (unsafe { WriteBytes_86(out, (&mut flags as *mut u8).cast_const(), 1_usize) });
    'loop_: for advance_width in 0..(advance_widths.len()) {
        let mut advance_width = advance_widths[advance_width].clone();
        (unsafe { WriteUShort_88(out, (advance_width as i32)) });
    }
    if !remove_proportional_lsb {
        'loop_: for lsb in 0..(proportional_lsbs.len()) {
            let mut lsb = proportional_lsbs[lsb].clone();
            (unsafe { WriteUShort_88(out, (lsb as i32)) });
        }
    }
    if !remove_monospace_lsb {
        'loop_: for lsb in 0..(monospace_lsbs.len()) {
            let mut lsb = monospace_lsbs[lsb].clone();
            (unsafe { WriteUShort_88(out, (lsb as i32)) });
        }
    }
    (*transformed_hmtx).tag = ((kHmtxTableTag_5) ^ (2155905152_u32));
    (*transformed_hmtx).flag_byte = (((1) << (6)) as u8);
    (*transformed_hmtx).length = ((*transformed_hmtx).buffer.len() as u32).clone();
    (*transformed_hmtx).data = ((*transformed_hmtx).buffer.as_mut_ptr()).cast_const();
    return true;
}
#[repr(C)]
#[derive(Clone)]
pub struct woff2_WOFF2Params {
    pub extended_metadata: Vec<libc::c_char>,
    pub brotli_quality: i32,
    pub allow_transforms: bool,
}
impl woff2_WOFF2Params {
    pub unsafe fn woff2_WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: {
                let s = c"".as_ptr();
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
pub static mut kWoff2HeaderSize_92: usize = unsafe { 48_usize };
pub static mut kWoff2EntrySize_93: usize = unsafe { 20_usize };
pub unsafe fn Compress_94(
    mut data: *const u8,
    len: usize,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut mode: ::brotli_sys::BrotliEncoderMode,
    mut quality: i32,
) -> bool {
    let mut compressed_len: usize = ((*result_len) as usize);
    if ((::brotli_sys::BrotliEncoderCompress(
        quality,
        22,
        mode,
        len,
        data,
        (&mut compressed_len as *mut usize),
        result,
    )) == (0))
    {
        return false;
    }
    (*result_len) = (compressed_len as u32);
    return true;
}
pub unsafe fn Woff2Compress_95(
    mut data: *const u8,
    len: usize,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut quality: i32,
) -> bool {
    return (unsafe {
        Compress_94(
            data,
            len,
            result,
            result_len,
            ::brotli_sys::BROTLI_MODE_FONT,
            quality,
        )
    });
}
pub unsafe fn TextCompress_96(
    mut data: *const u8,
    len: usize,
    mut result: *mut u8,
    mut result_len: *mut u32,
    mut quality: i32,
) -> bool {
    return (unsafe {
        Compress_94(
            data,
            len,
            result,
            result_len,
            ::brotli_sys::BROTLI_MODE_TEXT,
            quality,
        )
    });
}
pub unsafe fn KnownTableIndex_97(mut tag: u32) -> i32 {
    let mut i: i32 = 0;
    'loop_: while ((i) < (63)) {
        if ((tag) == (kKnownTags_8[(i) as usize])) {
            return i;
        }
        i.prefix_inc();
    }
    return 63;
}
pub unsafe fn StoreTableEntry_98(
    table: *const woff2_Table,
    mut offset: *mut usize,
    mut dst: *mut u8,
) {
    let mut flag_byte: u8 = (((((*table).flags) & (192_u32))
        | ((unsafe { KnownTableIndex_97((*table).tag) }) as u32))
        as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = flag_byte;
    if (((flag_byte as i32) & (63)) == (63)) {
        (unsafe { StoreU32_30((*table).tag, offset, dst) });
    }
    (unsafe {
        let _len: usize = ((*table).src_length as usize);
        let _offset: *mut usize = offset;
        StoreBase128_19(_len, _offset, dst)
    });
    if ((((*table).flags) & (kWoff2FlagsTransform_21)) != (0_u32)) {
        (unsafe {
            let _len: usize = ((*table).transform_length as usize);
            let _offset: *mut usize = offset;
            StoreBase128_19(_len, _offset, dst)
        });
    }
}
pub unsafe fn TableEntrySize_99(table: *const woff2_Table) -> usize {
    let mut flag_byte: u8 = ((unsafe { KnownTableIndex_97((*table).tag) }) as u8);
    let mut size: usize = (if (((flag_byte as i32) & (63)) != (63)) {
        1
    } else {
        5
    } as usize);
    size = (size).wrapping_add((unsafe { Base128Size_18(((*table).src_length as usize)) }));
    if ((((*table).flags) & (kWoff2FlagsTransform_21)) != (0_u32)) {
        size =
            (size).wrapping_add((unsafe { Base128Size_18(((*table).transform_length as usize)) }));
    }
    return size;
}
pub unsafe fn ComputeWoff2Length_100(
    font_collection: *const woff2_FontCollection,
    tables: *const Vec<woff2_Table>,
    mut index_by_tag_offset: BTreeMap<(u32, u32), Box<u16>>,
    mut compressed_data_length: usize,
    mut extended_metadata_length: usize,
) -> usize {
    let mut size: usize = kWoff2HeaderSize_92;
    'loop_: for table in 0..((*tables).len()) {
        let mut table = (*tables).as_ptr().add(table);
        size = (size).wrapping_add((unsafe { TableEntrySize_99(table) }));
    }
    if (((*font_collection).flavor) == (kTtcFontFlavor_22)) {
        size = (size).wrapping_add(4_usize);
        size = (size)
            .wrapping_add((unsafe { Size255UShort_9(((*font_collection).fonts.len() as u16)) }));
        size = ((size as u64)
            .wrapping_add(((4_usize).wrapping_mul((*font_collection).fonts.len()) as u64)))
            as usize;
        'loop_: for font in 0..((*font_collection).fonts.len()) {
            let mut font = (*font_collection).fonts.as_ptr().add(font);
            size = (size).wrapping_add((unsafe { Size255UShort_9(((*font).tables.len() as u16)) }));
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
                size = (size).wrapping_add((unsafe { Size255UShort_9(table_index) }));
            }
        }
    }
    size = (size).wrapping_add(compressed_data_length);
    size = ((unsafe { Round4_70((size as u64)) }) as usize);
    size = (size).wrapping_add(extended_metadata_length);
    return size;
}
pub unsafe fn ComputeUncompressedLength_101(font: *const woff2_Font) -> usize {
    let mut size: usize = (((12) + ((16) * ((*font).num_tables as i32))) as usize);
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
        size = (size).wrapping_add(((unsafe { Round4_71((*table).length) }) as usize));
    }
    return size;
}
pub unsafe fn ComputeUncompressedLength_102(font_collection: *const woff2_FontCollection) -> usize {
    if (((*font_collection).flavor) != (kTtcFontFlavor_22)) {
        return (unsafe {
            ComputeUncompressedLength_101(
                &(&(*font_collection)).fonts[(0_usize)] as *const woff2_Font,
            )
        });
    }
    let mut size: usize = (unsafe {
        let _header_version: u32 = (*font_collection).header_version;
        let _num_fonts: u32 = ((*font_collection).fonts.len() as u32);
        CollectionHeaderSize_27(_header_version, _num_fonts)
    });
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_ptr().add(font);
        size = (size).wrapping_add((unsafe { ComputeUncompressedLength_101(font) }));
    }
    return size;
}
pub unsafe fn ComputeTotalTransformLength_103(font: *const woff2_Font) -> usize {
    let mut total: usize = 0_usize;
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
            total = (total).wrapping_add(((*table).length as usize));
        }
    }
    return total;
}
pub unsafe fn MaxWOFF2CompressedSize_104(mut data: *const u8, mut length: usize) -> usize {
    return (unsafe {
        let mut _extended_metadata = {
            let s = c"".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
        MaxWOFF2CompressedSize_105(data, length, &mut _extended_metadata)
    });
}
pub unsafe fn MaxWOFF2CompressedSize_105(
    mut data: *const u8,
    mut length: usize,
    extended_metadata: *const Vec<libc::c_char>,
) -> usize {
    return (((length).wrapping_add(1024_usize) as u64)
        .wrapping_add((((*extended_metadata).len() - 1) as u64)) as usize);
}
pub unsafe fn CompressedBufferSize_106(mut original_size: u32) -> u32 {
    return ((((1.2E+0) * (original_size as f64)) + (10240_f64)) as u32);
}
pub unsafe fn TransformFontCollection_107(mut font_collection: *mut woff2_FontCollection) -> bool {
    'loop_: for font in 0..((*font_collection).fonts.len()) {
        let mut font = (*font_collection).fonts.as_mut_ptr().add(font);
        if !(unsafe { TransformGlyfAndLocaTables_90((font)) }) {
            printf(c"glyf/loca transformation failed.\n".as_ptr() as *const i8);
            return false;
        }
    }
    return true;
}
pub unsafe fn ConvertTTFToWOFF2_108(
    mut data: *const u8,
    mut length: usize,
    mut result: *mut u8,
    mut result_length: *mut usize,
) -> bool {
    let mut params: woff2_WOFF2Params = woff2_WOFF2Params::woff2_WOFF2Params();
    return (unsafe {
        let _length: usize = length;
        let _result_length: *mut usize = result_length;
        ConvertTTFToWOFF2_109(
            data,
            _length,
            result,
            _result_length,
            &params as *const woff2_WOFF2Params,
        )
    });
}
pub unsafe fn ConvertTTFToWOFF2_109(
    mut data: *const u8,
    mut length: usize,
    mut result: *mut u8,
    mut result_length: *mut usize,
    params: *const woff2_WOFF2Params,
) -> bool {
    let mut font_collection: woff2_FontCollection = <woff2_FontCollection>::default();
    if !(unsafe {
        ReadFontCollection_37(
            data,
            length,
            (&mut font_collection as *mut woff2_FontCollection),
        )
    }) {
        printf(c"Parsing of the input font failed.\n".as_ptr() as *const i8);
        return false;
    }
    if !(unsafe { NormalizeFontCollection_82((&mut font_collection as *mut woff2_FontCollection)) })
    {
        return false;
    }
    if ((*params).allow_transforms)
        && (!(unsafe {
            TransformFontCollection_107((&mut font_collection as *mut woff2_FontCollection))
        }))
    {
        return false;
    } else {
        'loop_: for font in 0..(font_collection.fonts.len()) {
            let mut font = font_collection.fonts.as_mut_ptr().add(font);
            let mut glyf_table: *mut woff2_Font_Table = (unsafe {
                let _tag: u32 = kGlyfTableTag_0;
                (*font).FindTable_u32(_tag)
            });
            let mut loca_table: *mut woff2_Font_Table = (unsafe {
                let _tag: u32 = kLocaTableTag_2;
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
    let mut total_transform_length: usize = 0_usize;
    'loop_: for font in 0..(font_collection.fonts.len()) {
        let mut font = font_collection.fonts.as_ptr().add(font);
        total_transform_length = (total_transform_length)
            .wrapping_add((unsafe { ComputeTotalTransformLength_103(font) }));
    }
    let mut compression_buffer_size: usize =
        ((unsafe { CompressedBufferSize_106((total_transform_length as u32)) }) as usize);
    let mut compression_buf: Vec<u8> = (0..(compression_buffer_size) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut total_compressed_length: u32 = (compression_buffer_size as u32);
    let mut transform_buf: Vec<u8> = (0..(total_transform_length) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut transform_offset: usize = 0_usize;
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
            if (table_to_store).is_null() {
                table_to_store = (original);
            }
            (unsafe {
                let _data: *const u8 = (*table_to_store).data;
                let _len: usize = ((*table_to_store).length as usize);
                StoreBytes_32(
                    _data,
                    _len,
                    (&mut transform_offset as *mut usize),
                    (&mut transform_buf[(0_usize)] as *mut u8),
                )
            });
        }
    }
    if !(unsafe {
        Woff2Compress_95(
            (transform_buf.as_mut_ptr()).cast_const(),
            total_transform_length,
            (&mut compression_buf[(0_usize)] as *mut u8),
            (&mut total_compressed_length as *mut u32),
            (*params).brotli_quality,
        )
    }) {
        printf(c"Compression of combined table failed.\n".as_ptr() as *const i8);
        return false;
    }
    printf(
        c"Compressed %zu to %u.\n".as_ptr() as *const i8,
        total_transform_length,
        total_compressed_length,
    );
    let mut compressed_metadata_buf_length: u32 =
        (unsafe { CompressedBufferSize_106((((*params).extended_metadata.len() - 1) as u32)) });
    let mut compressed_metadata_buf: Vec<u8> = (0..(compressed_metadata_buf_length as usize)
        as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    if (((*params).extended_metadata.len() - 1) > (0_usize)) {
        if !(unsafe {
            let _data: *const u8 = ((*params).extended_metadata.as_ptr() as *const u8);
            let _len: usize = ((*params).extended_metadata.len() - 1);
            let _quality: i32 = (*params).brotli_quality;
            TextCompress_96(
                _data,
                _len,
                compressed_metadata_buf.as_mut_ptr(),
                (&mut compressed_metadata_buf_length as *mut u32),
                _quality,
            )
        }) {
            printf(c"Compression of extended metadata failed.\n".as_ptr() as *const i8);
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
                    (tables.len() as u16).clone();
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
            if !((transformed_table).is_null()) {
                table.flags = ((*transformed_table).flag_byte as u32);
                table.flags |= kWoff2FlagsTransform_21;
                table.transform_length = (*transformed_table).length;
                transformed_data = (*transformed_table).data;
            }
            {
                let a0_clone = table.clone();
                tables.push(a0_clone)
            };
        }
    }
    let mut woff2_length: usize = (unsafe {
        ComputeWoff2Length_100(
            &font_collection as *const woff2_FontCollection,
            &tables as *const Vec<woff2_Table>,
            index_by_tag_offset.clone(),
            (total_compressed_length as usize),
            (compressed_metadata_buf_length as usize),
        )
    });
    if ((woff2_length) > (*result_length)) {
        printf(
            c"Result allocation was too small (%zd vs %zd bytes).\n".as_ptr() as *const i8,
            (*result_length),
            woff2_length,
        );
        return false;
    }
    (*result_length) = woff2_length;
    let mut offset: usize = 0_usize;
    (unsafe { StoreU32_30(kWoff2Signature_20, (&mut offset as *mut usize), result) });
    if ((font_collection.flavor) != (kTtcFontFlavor_22)) {
        (unsafe {
            StoreU32_30(
                font_collection.fonts[(0_usize)].flavor,
                (&mut offset as *mut usize),
                result,
            )
        });
    } else {
        (unsafe { StoreU32_30(kTtcFontFlavor_22, (&mut offset as *mut usize), result) });
    }
    (unsafe { StoreU32_30((woff2_length as u32), (&mut offset as *mut usize), result) });
    (unsafe { Store16_31((tables.len() as i32), (&mut offset as *mut usize), result) });
    (unsafe { Store16_31(0, (&mut offset as *mut usize), result) });
    (unsafe {
        StoreU32_30(
            ((unsafe {
                ComputeUncompressedLength_102(&font_collection as *const woff2_FontCollection)
            }) as u32),
            (&mut offset as *mut usize),
            result,
        )
    });
    (unsafe { StoreU32_30(total_compressed_length, (&mut offset as *mut usize), result) });
    (unsafe { Store16_31(1, (&mut offset as *mut usize), result) });
    (unsafe { Store16_31(0, (&mut offset as *mut usize), result) });
    if ((compressed_metadata_buf_length) > (0_u32)) {
        (unsafe {
            StoreU32_30(
                (((woff2_length).wrapping_sub((compressed_metadata_buf_length as usize))) as u32),
                (&mut offset as *mut usize),
                result,
            )
        });
        (unsafe {
            StoreU32_30(
                compressed_metadata_buf_length,
                (&mut offset as *mut usize),
                result,
            )
        });
        (unsafe {
            StoreU32_30(
                (((*params).extended_metadata.len() - 1) as u32),
                (&mut offset as *mut usize),
                result,
            )
        });
    } else {
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), result) });
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), result) });
        (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), result) });
    }
    (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), result) });
    (unsafe { StoreU32_30(0_u32, (&mut offset as *mut usize), result) });
    'loop_: for table in 0..(tables.len()) {
        let mut table = tables.as_ptr().add(table);
        (unsafe {
            let _table: *const woff2_Table = table;
            let _offset: *mut usize = (&mut offset as *mut usize);
            let _dst: *mut u8 = result;
            StoreTableEntry_98(_table, _offset, _dst)
        });
    }
    if ((font_collection.flavor) == (kTtcFontFlavor_22)) {
        (unsafe {
            StoreU32_30(
                font_collection.header_version,
                (&mut offset as *mut usize),
                result,
            )
        });
        (unsafe {
            Store255UShort_11(
                (font_collection.fonts.len() as i32),
                (&mut offset as *mut usize),
                result,
            )
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
                Store255UShort_11((num_tables as i32), (&mut offset as *mut usize), result)
            });
            (unsafe { StoreU32_30((*font).flavor, (&mut offset as *mut usize), result) });
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
                        c"Missing table index for offset 0x%08x\n".as_ptr() as *const i8,
                        table_offset,
                    );
                    return false;
                }
                let mut index: u16 = (*index_by_tag_offset.entry(tag_offset).or_default().as_mut());
                (unsafe { Store255UShort_11((index as i32), (&mut offset as *mut usize), result) });
            }
        }
    }
    (unsafe {
        StoreBytes_32(
            (&mut compression_buf[(0_usize)] as *mut u8).cast_const(),
            (total_compressed_length as usize),
            (&mut offset as *mut usize),
            result,
        )
    });
    offset = ((unsafe { Round4_70((offset as u64)) }) as usize);
    (unsafe {
        StoreBytes_32(
            (compressed_metadata_buf.as_mut_ptr()).cast_const(),
            (compressed_metadata_buf_length as usize),
            (&mut offset as *mut usize),
            result,
        )
    });
    if ((*result_length) != (offset)) {
        printf(
            c"Mismatch between computed and actual length (%zd vs %zd)\n".as_ptr() as *const i8,
            (*result_length),
            offset,
        );
        return false;
    }
    return true;
}
pub unsafe fn GetFileContent_110(mut filename: Vec<libc::c_char>) -> Vec<libc::c_char> {
    let mut ifs: ::std::fs::File = ::std::fs::File::open(
        ::std::ffi::CStr::from_ptr(filename.as_ptr())
            .to_str()
            .unwrap(),
    )
    .unwrap();
    return {
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &ifs.try_clone().unwrap();
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        let mut __buf: Vec<libc::c_char> = __bytes.iter().map(|&b| b as libc::c_char).collect();
        __buf.push(0);
        __buf
    };
}
pub unsafe fn SetFileContents_111(
    mut filename: Vec<libc::c_char>,
    mut start: *mut libc::c_char,
    mut end: *mut libc::c_char,
) {
    let mut ofs: ::std::fs::File = ::std::fs::File::create(
        ::std::ffi::CStr::from_ptr(filename.as_ptr())
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
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32) }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut libc::c_char) -> i32 {
    if ((argc) != (2)) {
        printf(c"One argument, the input filename, must be provided.\n".as_ptr() as *const i8);
        return 1;
    }
    let mut filename: Vec<libc::c_char> = {
        let s = (*argv.offset((1) as isize)).cast_const();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut outfilename: Vec<libc::c_char> = {
        let mut __tmp2 = {
            let mut __tmp1 = filename[(0_usize) as usize
                ..::std::cmp::min(
                    (0_usize
                        + match filename.iter().rposition(|&c| {
                            ::std::ffi::CStr::from_ptr(c".".as_ptr())
                                .to_str()
                                .unwrap()
                                .contains(c as u8 as char)
                        }) {
                            Some(idx) => idx,
                            None => usize::MAX,
                        }) as usize,
                    filename.len() - 1,
                )]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }
        .clone();
        __tmp2.pop();
        let __from = c".woff2".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    printf(
        c"Processing %s => %s\n".as_ptr() as *const i8,
        filename.as_ptr(),
        outfilename.as_ptr(),
    );
    let mut input: Vec<libc::c_char> = (unsafe { GetFileContent_110(filename.clone()) });
    let mut input_data: *const u8 = (input.as_ptr() as *const u8);
    let mut output_size: usize =
        (unsafe { MaxWOFF2CompressedSize_104(input_data, (input.len() - 1)) });
    let mut output: Vec<libc::c_char> = vec![(0 as libc::c_char); (output_size) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut output_data: *mut u8 =
        ((&mut output[(0_usize)] as *mut libc::c_char) as *mut libc::c_char as *mut u8);
    let mut params: woff2_WOFF2Params = woff2_WOFF2Params::woff2_WOFF2Params();
    if !(unsafe {
        ConvertTTFToWOFF2_109(
            input_data,
            (input.len() - 1),
            output_data,
            (&mut output_size as *mut usize),
            &params as *const woff2_WOFF2Params,
        )
    }) {
        printf(c"Compression failed.\n".as_ptr() as *const i8);
        return 1;
    }
    {
        output.pop();
        output.resize((output_size) as usize, 0);
        output.push(0)
    };
    (unsafe {
        let _start: *mut libc::c_char = output.as_mut_ptr();
        let _end: *mut libc::c_char = output.as_mut_ptr().add(output.len() - 1);
        SetFileContents_111(outfilename.clone(), _start, _end)
    });
    return 0;
}
