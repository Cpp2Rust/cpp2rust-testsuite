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
pub static mut kDefaultMaxSize_28: usize = unsafe { ((((128) * (1024)) * (1024)) as usize) };
pub unsafe trait woff2_WOFF2Out {
    unsafe fn Write_pconstlibcc_void_usize(&mut self, buf: *const ::libc::c_void, n: usize)
    -> bool;
    unsafe fn Write_pconstlibcc_void_usize_usize(
        &mut self,
        buf: *const ::libc::c_void,
        offset: usize,
        n: usize,
    ) -> bool;
    unsafe fn Size(&mut self) -> usize;
}
pub unsafe fn Round4_29(mut value: u64) -> u64 {
    if (((<u64>::MAX as u64).wrapping_sub(value)) < (3_u64)) {
        return value;
    }
    return (((value).wrapping_add(3_u64)) & (!3 as u64));
}
pub unsafe fn Round4_30(mut value: u32) -> u32 {
    if (((<u32>::MAX as u32).wrapping_sub(value)) < (3_u32)) {
        return value;
    }
    return (((value).wrapping_add(3_u32)) & (!3 as u32));
}
pub unsafe fn StoreU32_31(mut dst: *mut u8, mut offset: usize, mut x: u32) -> usize {
    (*dst.offset((offset) as isize)) = (((x) >> (24)) as u8);
    (*dst.offset(((offset).wrapping_add(1_usize)) as isize)) = (((x) >> (16)) as u8);
    (*dst.offset(((offset).wrapping_add(2_usize)) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(3_usize)) as isize)) = (x as u8);
    return (offset).wrapping_add(4_usize);
}
pub unsafe fn Store16_32(mut dst: *mut u8, mut offset: usize, mut x: i32) -> usize {
    (*dst.offset((offset) as isize)) = (((x) >> (8)) as u8);
    (*dst.offset(((offset).wrapping_add(1_usize)) as isize)) = (x as u8);
    return (offset).wrapping_add(2_usize);
}
pub unsafe fn StoreU32_33(mut val: u32, mut offset: *mut usize, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (24)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (16)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn Store16_34(mut val: i32, mut offset: *mut usize, mut dst: *mut u8) {
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (((val) >> (8)) as u8);
    (*dst.offset(((*offset).postfix_inc()) as isize)) = (val as u8);
}
pub unsafe fn StoreBytes_35(
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
pub static mut kGlyfOnCurve_36: i32 = unsafe { ((1) << (0)) };
pub static mut kGlyfXShort_37: i32 = unsafe { ((1) << (1)) };
pub static mut kGlyfYShort_38: i32 = unsafe { ((1) << (2)) };
pub static mut kGlyfRepeat_39: i32 = unsafe { ((1) << (3)) };
pub static mut kGlyfThisXIsSame_40: i32 = unsafe { ((1) << (4)) };
pub static mut kGlyfThisYIsSame_41: i32 = unsafe { ((1) << (5)) };
pub static mut kOverlapSimple_42: i32 = unsafe { ((1) << (6)) };
pub static mut FLAG_ARG_1_AND_2_ARE_WORDS_43: i32 = unsafe { ((1) << (0)) };
pub static mut FLAG_WE_HAVE_A_SCALE_44: i32 = unsafe { ((1) << (3)) };
pub static mut FLAG_MORE_COMPONENTS_45: i32 = unsafe { ((1) << (5)) };
pub static mut FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46: i32 = unsafe { ((1) << (6)) };
pub static mut FLAG_WE_HAVE_A_TWO_BY_TWO_47: i32 = unsafe { ((1) << (7)) };
pub static mut FLAG_WE_HAVE_INSTRUCTIONS_48: i32 = unsafe { ((1) << (8)) };
pub static mut FLAG_OVERLAP_SIMPLE_BITMAP_49: i32 = unsafe { ((1) << (0)) };
pub static mut kCheckSumAdjustmentOffset_50: usize = unsafe { 8_usize };
pub static mut kEndPtsOfContoursOffset_51: usize = unsafe { 10_usize };
pub static mut kCompositeGlyphBegin_52: usize = unsafe { 10_usize };
pub static mut kDefaultGlyphBuf_53: usize = unsafe { 5120_usize };
pub static mut kMaxPlausibleCompressionRatio_54: f32 = unsafe { (1.0E+2 as f32) };
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
pub unsafe fn WithSign_55(mut flag: i32, mut baseval: i32) -> i32 {
    return if (((flag) & (1)) != 0) {
        baseval
    } else {
        -baseval
    };
}
pub unsafe fn _SafeIntAddition_56(mut a: i32, mut b: i32, mut result: *mut i32) -> bool {
    if ((((((a) > (0)) && ((b) > ((<i32>::MAX) - (a))))
        || (((a) < (0)) && ((b) < ((<i32>::MIN) - (a))))) as i64)
        != 0)
    {
        return false;
    }
    (*result) = ((a) + (b));
    return true;
}
pub unsafe fn TripletDecode_57(
    mut flags_in: *const u8,
    mut in_: *const u8,
    mut in_size: usize,
    mut n_points: u32,
    mut result: *mut woff2_Point,
    mut in_bytes_consumed: *mut usize,
) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if ((((n_points as usize) > (in_size)) as i64) != 0) {
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
        if (((((((triplet_index).wrapping_add(n_data_bytes)) as usize) > (in_size))
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
                WithSign_55(_flag, _baseval)
            });
        } else if ((flag as i32) < (20)) {
            dx = (unsafe {
                let _flag: i32 = (flag as i32);
                let _baseval: i32 = (((((flag as i32) - (10)) & (14)) << (7))
                    + ((*in_.offset((triplet_index) as isize)) as i32));
                WithSign_55(_flag, _baseval)
            });
            dy = 0;
        } else if ((flag as i32) < (84)) {
            let mut b0: i32 = ((flag as i32) - (20));
            let mut b1: i32 = ((*in_.offset((triplet_index) as isize)) as i32);
            dx = (unsafe { WithSign_55((flag as i32), (((1) + ((b0) & (48))) + ((b1) >> (4)))) });
            dy = (unsafe {
                WithSign_55(
                    ((flag as i32) >> (1)),
                    (((1) + (((b0) & (12)) << (2))) + ((b1) & (15))),
                )
            });
        } else if ((flag as i32) < (120)) {
            let mut b0: i32 = ((flag as i32) - (84));
            dx = (unsafe {
                WithSign_55(
                    (flag as i32),
                    (((1) + (((b0) / (12)) << (8)))
                        + ((*in_.offset((triplet_index) as isize)) as i32)),
                )
            });
            dy = (unsafe {
                WithSign_55(
                    ((flag as i32) >> (1)),
                    (((1) + ((((b0) % (12)) >> (2)) << (8)))
                        + ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32)),
                )
            });
        } else if ((flag as i32) < (124)) {
            let mut b2: i32 =
                ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32);
            dx = (unsafe {
                WithSign_55(
                    (flag as i32),
                    ((((*in_.offset((triplet_index) as isize)) as i32) << (4)) + ((b2) >> (4))),
                )
            });
            dy = (unsafe {
                WithSign_55(
                    ((flag as i32) >> (1)),
                    ((((b2) & (15)) << (8))
                        + ((*in_.offset(((triplet_index).wrapping_add(2_u32)) as isize)) as i32)),
                )
            });
        } else {
            dx = (unsafe {
                WithSign_55(
                    (flag as i32),
                    ((((*in_.offset((triplet_index) as isize)) as i32) << (8))
                        + ((*in_.offset(((triplet_index).wrapping_add(1_u32)) as isize)) as i32)),
                )
            });
            dy = (unsafe {
                WithSign_55(
                    ((flag as i32) >> (1)),
                    ((((*in_.offset(((triplet_index).wrapping_add(2_u32)) as isize)) as i32)
                        << (8))
                        + ((*in_.offset(((triplet_index).wrapping_add(3_u32)) as isize)) as i32)),
                )
            });
        }
        triplet_index = (triplet_index).wrapping_add(n_data_bytes);
        if !(unsafe {
            let _a: i32 = x;
            let _result: *mut i32 = (&mut x as *mut i32);
            _SafeIntAddition_56(_a, dx, _result)
        }) {
            return false;
        }
        if !(unsafe {
            let _a: i32 = y;
            let _result: *mut i32 = (&mut y as *mut i32);
            _SafeIntAddition_56(_a, dy, _result)
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
    (*in_bytes_consumed) = (triplet_index as usize);
    return true;
}
pub unsafe fn StorePoints_58(
    mut n_points: u32,
    mut points: *const woff2_Point,
    mut n_contours: u32,
    mut instruction_length: u32,
    mut has_overlap_bit: bool,
    mut dst: *mut u8,
    mut dst_size: usize,
    mut glyph_size: *mut usize,
) -> bool {
    let mut flag_offset: u32 = (((((kEndPtsOfContoursOffset_51)
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
    'loop_: while ((i) < (n_points)) {
        let point: *const woff2_Point = &(*points.offset((i) as isize)) as *const woff2_Point;
        let mut flag: i32 = if (*point).on_curve {
            kGlyfOnCurve_36
        } else {
            0
        };
        if (has_overlap_bit) && ((i) == (0_u32)) {
            flag |= kOverlapSimple_42;
        }
        let mut dx: i32 = (((*point).x) - (last_x));
        let mut dy: i32 = (((*point).y) - (last_y));
        if ((dx) == (0)) {
            flag |= kGlyfThisXIsSame_40;
        } else if ((dx) > (-256_i32)) && ((dx) < (256)) {
            flag |= ((kGlyfXShort_37) | (if ((dx) > (0)) { kGlyfThisXIsSame_40 } else { 0 }));
            x_bytes = (x_bytes).wrapping_add(1_u32);
        } else {
            x_bytes = (x_bytes).wrapping_add(2_u32);
        }
        if ((dy) == (0)) {
            flag |= kGlyfThisYIsSame_41;
        } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
            flag |= ((kGlyfYShort_38) | (if ((dy) > (0)) { kGlyfThisYIsSame_41 } else { 0 }));
            y_bytes = (y_bytes).wrapping_add(1_u32);
        } else {
            y_bytes = (y_bytes).wrapping_add(2_u32);
        }
        if ((flag) == (last_flag)) && ((repeat_count) != (255)) {
            (*dst.offset(((flag_offset).wrapping_sub(1_u32)) as isize)) =
                (((*dst.offset(((flag_offset).wrapping_sub(1_u32)) as isize)) as i32)
                    | kGlyfRepeat_39) as u8;
            repeat_count.postfix_inc();
        } else {
            if ((repeat_count) != (0)) {
                if ((((flag_offset as usize) >= (dst_size)) as i64) != 0) {
                    return false;
                }
                (*dst.offset((flag_offset.postfix_inc()) as isize)) = (repeat_count as u8);
            }
            if ((((flag_offset as usize) >= (dst_size)) as i64) != 0) {
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
        if ((((flag_offset as usize) >= (dst_size)) as i64) != 0) {
            return false;
        }
        (*dst.offset((flag_offset.postfix_inc()) as isize)) = (repeat_count as u8);
    }
    let mut xy_bytes: u32 = (x_bytes).wrapping_add(y_bytes);
    if ((((((xy_bytes) < (x_bytes)) || (((flag_offset).wrapping_add(xy_bytes)) < (flag_offset)))
        || ((((flag_offset).wrapping_add(xy_bytes)) as usize) > (dst_size))) as i64)
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
            x_offset = ((unsafe { Store16_32(dst, (x_offset as usize), dx) }) as i32);
        }
        last_x += dx;
        let mut dy: i32 = (((*points.offset((i) as isize)).y) - (last_y));
        if ((dy) == (0)) {
        } else if ((dy) > (-256_i32)) && ((dy) < (256)) {
            (*dst.offset((y_offset.postfix_inc()) as isize)) = (dy.abs() as u8);
        } else {
            y_offset = ((unsafe { Store16_32(dst, (y_offset as usize), dy) }) as i32);
        }
        last_y += dy;
        i.prefix_inc();
    }
    (*glyph_size) = (y_offset as usize);
    return true;
}
pub unsafe fn ComputeBbox_59(mut n_points: u32, mut points: *const woff2_Point, mut dst: *mut u8) {
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
    let mut offset: usize = 2_usize;
    offset = (unsafe { Store16_32(dst, offset, x_min) });
    offset = (unsafe { Store16_32(dst, offset, y_min) });
    offset = (unsafe { Store16_32(dst, offset, x_max) });
    offset = (unsafe { Store16_32(dst, offset, y_max) });
}
pub unsafe fn SizeOfComposite_60(
    mut composite_stream: woff2_Buffer,
    mut size: *mut usize,
    mut have_instructions: *mut bool,
) -> bool {
    let mut start_offset: usize = (unsafe { composite_stream.offset() });
    let mut we_have_instructions: bool = false;
    let mut flags: u16 = (FLAG_MORE_COMPONENTS_45 as u16);
    'loop_: while (((flags as i32) & (FLAG_MORE_COMPONENTS_45)) != 0) {
        if ((!(unsafe { composite_stream.ReadU16((&mut flags as *mut u16)) }) as i64) != 0) {
            return false;
        }
        we_have_instructions = ((we_have_instructions as i32)
            | ((((flags as i32) & (FLAG_WE_HAVE_INSTRUCTIONS_48)) != (0)) as i32))
            != 0;
        let mut arg_size: usize = 2_usize;
        if (((flags as i32) & (FLAG_ARG_1_AND_2_ARE_WORDS_43)) != 0) {
            arg_size = (arg_size).wrapping_add(4_usize);
        } else {
            arg_size = (arg_size).wrapping_add(2_usize);
        }
        if (((flags as i32) & (FLAG_WE_HAVE_A_SCALE_44)) != 0) {
            arg_size = (arg_size).wrapping_add(2_usize);
        } else if (((flags as i32) & (FLAG_WE_HAVE_AN_X_AND_Y_SCALE_46)) != 0) {
            arg_size = (arg_size).wrapping_add(4_usize);
        } else if (((flags as i32) & (FLAG_WE_HAVE_A_TWO_BY_TWO_47)) != 0) {
            arg_size = (arg_size).wrapping_add(8_usize);
        }
        if ((!(unsafe { composite_stream.Skip(arg_size) }) as i64) != 0) {
            return false;
        }
    }
    (*size) = (unsafe { composite_stream.offset() }).wrapping_sub(start_offset);
    (*have_instructions) = we_have_instructions;
    return true;
}
pub unsafe fn Pad4_61(mut out: *mut dyn woff2_WOFF2Out) -> bool {
    let mut zeroes: [u8; 3] = [0_u8, 0_u8, 0_u8];
    if (((((unsafe { (*out).Size() }).wrapping_add(3_usize)) < (unsafe { (*out).Size() })) as i64)
        != 0)
    {
        return false;
    }
    let mut pad_bytes: u32 = (((unsafe { Round4_29(((unsafe { (*out).Size() }) as u64)) })
        .wrapping_sub(((unsafe { (*out).Size() }) as u64))) as u32);
    if ((pad_bytes) > (0_u32)) {
        if ((!(unsafe {
            (*out).Write_pconstlibcc_void_usize(
                ((&mut zeroes as *mut [u8; 3]) as *const [u8; 3] as *const ::libc::c_void),
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
pub unsafe fn StoreLoca_62(
    loca_values: *const Vec<u32>,
    mut index_format: i32,
    mut checksum: *mut u32,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let loca_size: u64 = ((*loca_values).len() as u64);
    let offset_size: u64 = (if (index_format != 0) { 4 } else { 2 } as u64);
    if ((((((loca_size) << (2)) >> (2)) != (loca_size)) as i64) != 0) {
        return false;
    }
    let mut loca_content: Vec<u8> = (0..((loca_size).wrapping_mul(offset_size)) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut dst: *mut u8 = (&mut loca_content[(0_usize)] as *mut u8);
    let mut offset: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*loca_values).len())) {
        let mut value: u32 = (&(*loca_values))[(i)];
        if (index_format != 0) {
            offset = (unsafe { StoreU32_31(dst, offset, value) });
        } else {
            offset = (unsafe { Store16_32(dst, offset, (((value) >> (1)) as i32)) });
        }
        i.prefix_inc();
    }
    (*checksum) = (unsafe {
        let _buf: *const u8 = (&mut loca_content[(0_usize)] as *mut u8).cast_const();
        let _size: usize = loca_content.len();
        ComputeULongSum_26(_buf, _size)
    });
    if ((!(unsafe {
        let _buf: *const ::libc::c_void =
            ((&mut loca_content[(0_usize)] as *mut u8) as *const u8 as *const ::libc::c_void);
        let _n: usize = loca_content.len();
        (*out).Write_pconstlibcc_void_usize(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn ReconstructGlyf_63(
    mut data: *const u8,
    mut glyf_table: *mut woff2_Table,
    mut glyf_checksum: *mut u32,
    mut loca_table: *mut woff2_Table,
    mut loca_checksum: *mut u32,
    mut info: *mut woff2_WOFF2FontInfo,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    static mut kNumSubStreams_64: i32 = unsafe { 7 };;
    let mut file: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(data, ((*glyf_table).transform_length as usize));
    let mut version: u16 = 0_u16;
    let mut substreams: Vec<(*const u8, u64)> = (0..(kNumSubStreams_64 as usize) as usize)
        .map(|_| <(*const u8, u64)>::default())
        .collect::<Vec<_>>();
    let glyf_start: usize = (unsafe { (*out).Size() });
    if ((!(unsafe { file.ReadU16((&mut version as *mut u16)) }) as i64) != 0) {
        return false;
    }
    let mut flags: u16 = 0_u16;
    if ((!(unsafe { file.ReadU16((&mut flags as *mut u16)) }) as i64) != 0) {
        return false;
    }
    let mut has_overlap_bitmap: bool = (((flags as i32) & (FLAG_OVERLAP_SIMPLE_BITMAP_49)) != 0);
    if ((((!(unsafe { file.ReadU16((&mut (*info).num_glyphs as *mut u16)) }))
        || (!(unsafe { file.ReadU16((&mut (*info).index_format as *mut u16)) }))) as i64)
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
    let mut offset: u32 = ((((2) + (kNumSubStreams_64)) * (4)) as u32);
    if ((((offset) > ((*glyf_table).transform_length)) as i64) != 0) {
        return false;
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (kNumSubStreams_64)) {
        let mut substream_size: u32 = 0_u32;
        if ((!(unsafe { file.ReadU32((&mut substream_size as *mut u32)) }) as i64) != 0) {
            return false;
        }
        if ((((substream_size) > (((*glyf_table).transform_length).wrapping_sub(offset))) as i64)
            != 0)
        {
            return false;
        }
        substreams[(i as usize)] = (data.offset((offset) as isize).into(), substream_size.into());
        offset = (offset).wrapping_add(substream_size);
        i.prefix_inc();
    }
    let mut n_contour_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(0_usize)].0, (substreams[(0_usize)].1 as usize));
    let mut n_points_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(1_usize)].0, (substreams[(1_usize)].1 as usize));
    let mut flag_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(2_usize)].0, (substreams[(2_usize)].1 as usize));
    let mut glyph_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(3_usize)].0, (substreams[(3_usize)].1 as usize));
    let mut composite_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(4_usize)].0, (substreams[(4_usize)].1 as usize));
    let mut bbox_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(5_usize)].0, (substreams[(5_usize)].1 as usize));
    let mut instruction_stream: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(substreams[(6_usize)].0, (substreams[(6_usize)].1 as usize));
    let mut overlap_bitmap: *const u8 = std::ptr::null();
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
    let mut loca_values: Vec<u32> = (0..((((*info).num_glyphs as i32) + (1)) as usize) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut n_points_vec: Vec<u32> = Vec::new();
    let mut points: Option<Box<[woff2_Point]>> = None;
    let mut points_size: usize = 0_usize;
    let mut bbox_bitmap: *const u8 = (unsafe { bbox_stream.buffer() });
    let mut bitmap_length: u32 = ((((((*info).num_glyphs as i32) + (31)) >> (5)) << (2)) as u32);
    if !(unsafe { bbox_stream.Skip((bitmap_length as usize)) }) {
        return false;
    }
    let mut glyph_buf_size: usize = kDefaultGlyphBuf_53;
    let mut glyph_buf: Option<Box<[u8]>> = Some(Box::from_raw(Box::leak(
        (0..glyph_buf_size).map(|_| 0_u8).collect::<Box<[u8]>>(),
    )));
    {
        let __a0 = ((*info).num_glyphs as usize) as usize;
        (*info).x_mins.resize_with(__a0, || <i16>::default())
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < ((*info).num_glyphs as u32)) {
        let mut glyph_size: usize = 0_usize;
        let mut n_contours: u16 = 0_u16;
        let mut have_bbox: bool = false;
        if ((((*bbox_bitmap.offset(((i) >> (3)) as isize)) as i32) & ((128) >> ((i) & (7_u32))))
            != 0)
        {
            have_bbox = true;
        }
        if ((!(unsafe { n_contour_stream.ReadU16((&mut n_contours as *mut u16)) }) as i64) != 0) {
            return false;
        }
        if ((n_contours as i32) == (65535)) {
            let mut have_instructions: bool = false;
            let mut instruction_size: u32 = 0_u32;
            if ((!have_bbox as i64) != 0) {
                return false;
            }
            let mut composite_size: usize = 0_usize;
            if ((!(unsafe {
                SizeOfComposite_60(
                    composite_stream.clone(),
                    (&mut composite_size as *mut usize),
                    (&mut have_instructions as *mut bool),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if have_instructions {
                if ((!(unsafe {
                    Read255UShort_12(
                        (&mut glyph_stream as *mut woff2_Buffer),
                        (&mut instruction_size as *mut u32),
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
            }
            let mut size_needed: usize =
                ((12_usize).wrapping_add(composite_size)).wrapping_add((instruction_size as usize));
            if ((((glyph_buf_size) < (size_needed)) as i64) != 0) {
                glyph_buf = Some(Box::from_raw(Box::leak(
                    (0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>(),
                )));
                glyph_buf_size = size_needed;
            }
            glyph_size = (unsafe {
                Store16_32(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    glyph_size,
                    (n_contours as i32),
                )
            });
            if ((!(unsafe {
                bbox_stream.Read(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .offset((glyph_size) as isize),
                    8_usize,
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add(8_usize);
            if ((!(unsafe {
                composite_stream.Read(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .offset((glyph_size) as isize),
                    composite_size,
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add(composite_size);
            if have_instructions {
                glyph_size = (unsafe {
                    Store16_32(
                        glyph_buf
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                        glyph_size,
                        (instruction_size as i32),
                    )
                });
                if ((!(unsafe {
                    instruction_stream.Read(
                        glyph_buf
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                            .offset((glyph_size) as isize),
                        (instruction_size as usize),
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
                glyph_size = (glyph_size).wrapping_add((instruction_size as usize));
            }
        } else if ((n_contours as i32) > (0)) {
            n_points_vec.clear();
            let mut total_n_points: u32 = 0_u32;
            let mut n_points_contour: u32 = 0_u32;
            let mut j: u32 = 0_u32;
            'loop_: while ((j) < (n_contours as u32)) {
                if ((!(unsafe {
                    Read255UShort_12(
                        (&mut n_points_stream as *mut woff2_Buffer),
                        (&mut n_points_contour as *mut u32),
                    )
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
            if ((((flag_size as usize)
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
            let mut triplet_size: usize =
                (unsafe { glyph_stream.length() }).wrapping_sub((unsafe { glyph_stream.offset() }));
            let mut triplet_bytes_consumed: usize = 0_usize;
            if ((points_size) < (total_n_points as usize)) {
                points_size = (total_n_points as usize);
                points = Some(Box::from_raw(Box::leak(
                    (0..points_size)
                        .map(|_| <woff2_Point>::default())
                        .collect::<Box<[woff2_Point]>>(),
                )));
            }
            if ((!(unsafe {
                TripletDecode_57(
                    flags_buf,
                    triplet_buf,
                    triplet_size,
                    total_n_points,
                    points
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    (&mut triplet_bytes_consumed as *mut usize),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            if ((!(unsafe { flag_stream.Skip((flag_size as usize)) }) as i64) != 0) {
                return false;
            }
            if ((!(unsafe { glyph_stream.Skip(triplet_bytes_consumed) }) as i64) != 0) {
                return false;
            }
            let mut instruction_size: u32 = 0_u32;
            if ((!(unsafe {
                Read255UShort_12(
                    (&mut glyph_stream as *mut woff2_Buffer),
                    (&mut instruction_size as *mut u32),
                )
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
            let mut size_needed: usize = ((((((12) + ((2) * (n_contours as i32))) as u32)
                .wrapping_add((5_u32).wrapping_mul(total_n_points)))
            .wrapping_add(instruction_size)) as usize);
            if ((((glyph_buf_size) < (size_needed)) as i64) != 0) {
                glyph_buf = Some(Box::from_raw(Box::leak(
                    (0..size_needed).map(|_| 0_u8).collect::<Box<[u8]>>(),
                )));
                glyph_buf_size = size_needed;
            }
            glyph_size = (unsafe {
                Store16_32(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    glyph_size,
                    (n_contours as i32),
                )
            });
            if have_bbox {
                if ((!(unsafe {
                    bbox_stream.Read(
                        glyph_buf
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                            .offset((glyph_size) as isize),
                        8_usize,
                    )
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                (unsafe {
                    ComputeBbox_59(
                        total_n_points,
                        (points
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                        .cast_const(),
                        glyph_buf
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    )
                });
            }
            glyph_size = kEndPtsOfContoursOffset_51;
            let mut end_point: i32 = -1_i32;
            let mut contour_ix: u32 = 0_u32;
            'loop_: while ((contour_ix) < (n_contours as u32)) {
                end_point =
                    ((end_point as u32).wrapping_add(n_points_vec[(contour_ix as usize)])) as i32;
                if ((((end_point) >= (65536)) as i64) != 0) {
                    return false;
                }
                glyph_size = (unsafe {
                    Store16_32(
                        glyph_buf
                            .as_deref_mut()
                            .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                        glyph_size,
                        end_point,
                    )
                });
                contour_ix.prefix_inc();
            }
            glyph_size = (unsafe {
                Store16_32(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    glyph_size,
                    (instruction_size as i32),
                )
            });
            if ((!(unsafe {
                instruction_stream.Read(
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                        .offset((glyph_size) as isize),
                    (instruction_size as usize),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
            glyph_size = (glyph_size).wrapping_add((instruction_size as usize));
            let mut has_overlap_bit: bool = (has_overlap_bitmap)
                && ((((*overlap_bitmap.offset(((i) >> (3)) as isize)) as i32)
                    & ((128) >> ((i) & (7_u32))))
                    != 0);
            if ((!(unsafe {
                StorePoints_58(
                    total_n_points,
                    (points
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    .cast_const(),
                    (n_contours as u32),
                    instruction_size,
                    has_overlap_bit,
                    glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()),
                    glyph_buf_size,
                    (&mut glyph_size as *mut usize),
                )
            }) as i64)
                != 0)
            {
                return false;
            }
        } else {
            if ((have_bbox as i64) != 0) {
                printf(c"Empty glyph has a bbox\n".as_ptr() as *const i8);
                return false;
            }
        }
        loca_values[(i as usize)] = (((unsafe { (*out).Size() }).wrapping_sub(glyf_start)) as u32);
        if ((!(unsafe {
            (*out).Write_pconstlibcc_void_usize(
                (glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    as *const u8 as *const ::libc::c_void),
                glyph_size,
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        if ((!(unsafe { Pad4_61(out) }) as i64) != 0) {
            return false;
        }
        (*glyf_checksum) = (*glyf_checksum).wrapping_add(
            (unsafe {
                ComputeULongSum_26(
                    (glyph_buf
                        .as_deref_mut()
                        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    .cast_const(),
                    glyph_size,
                )
            }),
        );
        if ((n_contours as i32) > (0)) {
            let mut x_min_buf: woff2_Buffer = woff2_Buffer::woff2_Buffer(
                (glyph_buf
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
                    .offset((2) as isize))
                .cast_const(),
                2_usize,
            );
            if ((!(unsafe {
                x_min_buf.ReadS16((&mut (&mut (*info)).x_mins[(i as usize)] as *mut i16))
            }) as i64)
                != 0)
            {
                return false;
            }
        }
        i.prefix_inc();
    }
    (*glyf_table).dst_length =
        (((unsafe { (*out).Size() }).wrapping_sub(((*glyf_table).dst_offset as usize))) as u32);
    (*loca_table).dst_offset = ((unsafe { (*out).Size() }) as u32).clone();
    loca_values[((*info).num_glyphs as usize)] = (*glyf_table).dst_length;
    if ((!(unsafe {
        StoreLoca_62(
            &loca_values as *const Vec<u32>,
            ((*info).index_format as i32),
            loca_checksum,
            out,
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    (*loca_table).dst_length =
        (((unsafe { (*out).Size() }).wrapping_sub(((*loca_table).dst_offset as usize))) as u32);
    return true;
}
pub unsafe fn FindTable_65(
    mut tables: *mut Vec<*mut woff2_Table>,
    mut tag: u32,
) -> *mut woff2_Table {
    'loop_: for table in 0..((*tables).len()) {
        let mut table = (&(*tables))[table].clone();
        if (((*table).tag) == (tag)) {
            return table;
        }
    }
    return std::ptr::null_mut();
}
pub unsafe fn ReadNumHMetrics_66(
    mut data: *const u8,
    mut data_size: usize,
    mut num_hmetrics: *mut u16,
) -> bool {
    let mut buffer: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, data_size);
    if ((((!(unsafe { buffer.Skip(34_usize) })) || (!(unsafe { buffer.ReadU16(num_hmetrics) })))
        as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn ReconstructTransformedHmtx_67(
    mut transformed_buf: *const u8,
    mut transformed_size: usize,
    mut num_glyphs: u16,
    mut num_hmetrics: u16,
    x_mins: *const Vec<i16>,
    mut checksum: *mut u32,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut hmtx_buff_in: woff2_Buffer =
        woff2_Buffer::woff2_Buffer(transformed_buf, transformed_size);
    let mut hmtx_flags: u8 = 0_u8;
    if ((!(unsafe { hmtx_buff_in.ReadU8((&mut hmtx_flags as *mut u8)) }) as i64) != 0) {
        return false;
    }
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut lsbs: Vec<i16> = Vec::new();
    let mut has_proportional_lsbs: bool = (((hmtx_flags as i32) & (1)) == (0));
    let mut has_monospace_lsbs: bool = (((hmtx_flags as i32) & (2)) == (0));
    if (((hmtx_flags as i32) & (252)) != (0)) {
        printf(c"Illegal hmtx flags; bits 2-7 must be 0\n".as_ptr() as *const i8);
        return false;
    }
    if (has_proportional_lsbs) && (has_monospace_lsbs) {
        return false;
    }
    assert!((((*x_mins).len()) == (num_glyphs as usize)));
    if ((((num_hmetrics as i32) > (num_glyphs as i32)) as i64) != 0) {
        return false;
    }
    if ((((num_hmetrics as i32) < (1)) as i64) != 0) {
        return false;
    }
    let mut i: u16 = 0_u16;
    'loop_: while ((i as i32) < (num_hmetrics as i32)) {
        let mut advance_width: u16 = 0_u16;
        if ((!(unsafe { hmtx_buff_in.ReadU16((&mut advance_width as *mut u16)) }) as i64) != 0) {
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
            if ((!(unsafe { hmtx_buff_in.ReadS16((&mut lsb as *mut i16)) }) as i64) != 0) {
                return false;
            }
        } else {
            lsb = (&(*x_mins))[(i as usize)];
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
            if ((!(unsafe { hmtx_buff_in.ReadS16((&mut lsb as *mut i16)) }) as i64) != 0) {
                return false;
            }
        } else {
            lsb = (&(*x_mins))[(i as usize)];
        }
        {
            let a0_clone = lsb.clone();
            lsbs.push(a0_clone)
        };
        i.postfix_inc();
    }
    let mut hmtx_output_size: u32 =
        ((((2) * (num_glyphs as i32)) + ((2) * (num_hmetrics as i32))) as u32);
    let mut hmtx_table: Vec<u8> = (0..(hmtx_output_size as usize) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    let mut dst: *mut u8 = (&mut hmtx_table[(0_usize)] as *mut u8);
    let mut dst_offset: usize = 0_usize;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (num_glyphs as u32)) {
        if ((i) < (num_hmetrics as u32)) {
            (unsafe {
                Store16_34(
                    (advance_widths[(i as usize)] as i32),
                    (&mut dst_offset as *mut usize),
                    dst,
                )
            });
        }
        (unsafe {
            Store16_34(
                (lsbs[(i as usize)] as i32),
                (&mut dst_offset as *mut usize),
                dst,
            )
        });
        i.postfix_inc();
    }
    (*checksum) = (unsafe {
        ComputeULongSum_26(
            (&mut hmtx_table[(0_usize)] as *mut u8).cast_const(),
            (hmtx_output_size as usize),
        )
    });
    if ((!(unsafe {
        (*out).Write_pconstlibcc_void_usize(
            ((&mut hmtx_table[(0_usize)] as *mut u8) as *const u8 as *const ::libc::c_void),
            (hmtx_output_size as usize),
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    return true;
}
pub unsafe fn Woff2Uncompress_68(
    mut dst_buf: *mut u8,
    mut dst_size: usize,
    mut src_buf: *const u8,
    mut src_size: usize,
) -> bool {
    let mut uncompressed_size: usize = dst_size;
    let mut result: ::brotli_sys::BrotliDecoderResult = ::brotli_sys::BrotliDecoderDecompress(
        src_size,
        src_buf,
        (&mut uncompressed_size as *mut usize),
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
pub unsafe fn ReadTableDirectory_69(
    mut file: *mut woff2_Buffer,
    mut tables: *mut Vec<woff2_Table>,
    mut num_tables: usize,
) -> bool {
    let mut src_offset: u32 = 0_u32;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (num_tables)) {
        let mut table: *mut woff2_Table = (&mut (&mut (*tables))[(i)] as *mut woff2_Table);
        let mut flag_byte: u8 = 0_u8;
        if ((!(unsafe { (*file).ReadU8((&mut flag_byte as *mut u8)) }) as i64) != 0) {
            return false;
        }
        let mut tag: u32 = 0_u32;
        if (((flag_byte as i32) & (63)) == (63)) {
            if ((!(unsafe { (*file).ReadU32((&mut tag as *mut u32)) }) as i64) != 0) {
                return false;
            }
        } else {
            tag = kKnownTags_8[((flag_byte as i32) & (63)) as usize];
        }
        let mut flags: u32 = 0_u32;
        let mut xform_version: u8 = ((((flag_byte as i32) >> (6)) & (3)) as u8);
        if ((tag) == (kGlyfTableTag_0)) || ((tag) == (kLocaTableTag_2)) {
            if ((xform_version as i32) == (0)) {
                flags = ((flags as u32) | kWoff2FlagsTransform_21) as u32;
            }
        } else if ((xform_version as i32) != (0)) {
            flags = ((flags as u32) | kWoff2FlagsTransform_21) as u32;
        }
        flags |= (xform_version as u32);
        let mut dst_length: u32 = 0_u32;
        if ((!(unsafe { ReadBase128_17(file, (&mut dst_length as *mut u32)) }) as i64) != 0) {
            return false;
        }
        let mut transform_length: u32 = dst_length;
        if (((flags) & (kWoff2FlagsTransform_21)) != (0_u32)) {
            if ((!(unsafe { ReadBase128_17(file, (&mut transform_length as *mut u32)) }) as i64)
                != 0)
            {
                return false;
            }
            if (((((tag) == (kLocaTableTag_2)) && (transform_length != 0)) as i64) != 0) {
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
pub unsafe fn StoreOffsetTable_70(
    mut result: *mut u8,
    mut offset: usize,
    mut flavor: u32,
    mut num_tables: u16,
) -> usize {
    offset = (unsafe { StoreU32_31(result, offset, flavor) });
    offset = (unsafe { Store16_32(result, offset, (num_tables as i32)) });
    let mut max_pow2: u32 = 0_u32;
    'loop_: while (((1_u32) << ((max_pow2).wrapping_add(1_u32))) <= (num_tables as u32)) {
        max_pow2.postfix_inc();
    }
    let output_search_range: u16 = ((((1_u32) << (max_pow2)) << (4)) as u16);
    offset = (unsafe { Store16_32(result, offset, (output_search_range as i32)) });
    offset = (unsafe { Store16_32(result, offset, (max_pow2 as i32)) });
    offset = (unsafe {
        Store16_32(
            result,
            offset,
            (((num_tables as i32) << (4)) - (output_search_range as i32)),
        )
    });
    return offset;
}
pub unsafe fn StoreTableEntry_71(mut result: *mut u8, mut offset: u32, mut tag: u32) -> usize {
    offset = ((unsafe { StoreU32_31(result, (offset as usize), tag) }) as u32);
    offset = ((unsafe { StoreU32_31(result, (offset as usize), 0_u32) }) as u32);
    offset = ((unsafe { StoreU32_31(result, (offset as usize), 0_u32) }) as u32);
    offset = ((unsafe { StoreU32_31(result, (offset as usize), 0_u32) }) as u32);
    return (offset as usize);
}
pub unsafe fn ComputeOffsetToFirstTable_72(hdr: *const woff2_WOFF2Header) -> u64 {
    let mut offset: u64 = (kSfntHeaderSize_23 as u64)
        .wrapping_add((kSfntEntrySize_24 as u64).wrapping_mul(((*hdr).num_tables as u64)));
    if ((*hdr).header_version != 0) {
        offset = (((unsafe {
            let _header_version: u32 = (*hdr).header_version;
            let _num_fonts: u32 = ((*hdr).ttc_fonts.len() as u32);
            CollectionHeaderSize_27(_header_version, _num_fonts)
        }) as u64)
            .wrapping_add(
                (kSfntHeaderSize_23 as u64).wrapping_mul(((*hdr).ttc_fonts.len() as u64)),
            ))
        .clone();
        'loop_: for ttc_font in 0..((*hdr).ttc_fonts.len()) {
            let mut ttc_font = (*hdr).ttc_fonts.as_ptr().add(ttc_font);
            offset = ((offset as u64).wrapping_add(
                (kSfntEntrySize_24 as u64).wrapping_mul(((*ttc_font).table_indices.len() as u64)),
            )) as u64;
        }
    }
    return offset;
}
pub unsafe fn Tables_73(
    mut hdr: *mut woff2_WOFF2Header,
    mut font_index: usize,
) -> Vec<*mut woff2_Table> {
    let mut tables: Vec<*mut woff2_Table> = Vec::new();
    if (((*hdr).header_version as i64) != 0) {
        'loop_: for index in 0..((&mut (*hdr)).ttc_fonts[(font_index)].table_indices.len()) {
            let mut index = (&mut (*hdr)).ttc_fonts[(font_index)].table_indices[index].clone();
            tables.push((&mut (&mut (*hdr)).tables[(index as usize)] as *mut woff2_Table));
        }
    } else {
        'loop_: for table in 0..((*hdr).tables.len()) {
            let mut table = (*hdr).tables.as_mut_ptr().add(table);
            tables.push((table));
        }
    }
    return tables;
}
pub unsafe fn ReconstructFont_74(
    mut transformed_buf: *mut u8,
    transformed_buf_size: u32,
    mut metadata: *mut woff2_RebuildMetadata,
    mut hdr: *mut woff2_WOFF2Header,
    mut font_index: usize,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut dest_offset: usize = (unsafe { (*out).Size() });
    let mut table_entry: [u8; 12] = [0_u8; 12];
    let mut info: *mut woff2_WOFF2FontInfo =
        (&mut (&mut (*metadata)).font_infos[(font_index)] as *mut woff2_WOFF2FontInfo);
    let mut tables: Vec<*mut woff2_Table> = (unsafe { Tables_73(hdr, font_index) });
    let mut glyf_table: *const woff2_Table =
        (unsafe { FindTable_65((&mut tables as *mut Vec<*mut woff2_Table>), kGlyfTableTag_0) })
            .cast_const();
    let mut loca_table: *const woff2_Table =
        (unsafe { FindTable_65((&mut tables as *mut Vec<*mut woff2_Table>), kLocaTableTag_2) })
            .cast_const();
    if ((((!(glyf_table).is_null() as i32) != (!(loca_table).is_null() as i32)) as i64) != 0) {
        printf(c"Cannot have just one of glyf/loca\n".as_ptr() as *const i8);
        return false;
    }
    if !((glyf_table).is_null()) {
        if ((((((*glyf_table).flags) & (kWoff2FlagsTransform_21))
            != (((*loca_table).flags) & (kWoff2FlagsTransform_21))) as i64)
            != 0)
        {
            printf(c"Cannot transform just one of glyf/loca\n".as_ptr() as *const i8);
            return false;
        }
    }
    let mut font_checksum: u32 = (*metadata).header_checksum;
    if ((*hdr).header_version != 0) {
        font_checksum = (&mut (*hdr)).ttc_fonts[(font_index)].header_checksum;
    }
    let mut loca_checksum: u32 = 0_u32;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (tables.len())) {
        let table: *mut woff2_Table = &mut (*tables[(i)]) as *mut woff2_Table;
        let mut checksum_key: (u32, u32) = ((*table).tag.into(), (*table).src_offset.into());
        let mut reused: bool = UnsafeMapIterator::find_key(
            &(*metadata).checksums as *const BTreeMap<(u32, u32), Box<u32>>,
            &checksum_key,
        ) != UnsafeMapIterator::end(
            &(*metadata).checksums as *const BTreeMap<(u32, u32), Box<u32>>,
        );
        if (((((font_index) == (0_usize)) && (reused)) as i64) != 0) {
            return false;
        }
        if ((((((*table).src_offset as u64).wrapping_add(((*table).src_length as u64)))
            > (transformed_buf_size as u64)) as i64)
            != 0)
        {
            return false;
        }
        if (((*table).tag) == (kHheaTableTag_6)) {
            if !(unsafe {
                let _data: *const u8 =
                    (transformed_buf.offset(((*table).src_offset) as isize)).cast_const();
                let _data_size: usize = ((*table).src_length as usize);
                ReadNumHMetrics_66(_data, _data_size, (&mut (*info).num_hmetrics as *mut u16))
            }) {
                return false;
            }
        }
        let mut checksum: u32 = 0_u32;
        if !reused {
            if ((((*table).flags) & (kWoff2FlagsTransform_21)) != (kWoff2FlagsTransform_21)) {
                if (((*table).tag) == (kHeadTableTag_1)) {
                    if (((((*table).src_length) < (12_u32)) as i64) != 0) {
                        return false;
                    }
                    (unsafe {
                        StoreU32_31(
                            transformed_buf.offset(((*table).src_offset) as isize),
                            8_usize,
                            0_u32,
                        )
                    });
                }
                (*table).dst_offset = (dest_offset as u32);
                checksum = (unsafe {
                    let _buf: *const u8 =
                        (transformed_buf.offset(((*table).src_offset) as isize)).cast_const();
                    let _size: usize = ((*table).src_length as usize);
                    ComputeULongSum_26(_buf, _size)
                });
                if ((!(unsafe {
                    let _buf: *const ::libc::c_void =
                        (transformed_buf.offset(((*table).src_offset) as isize) as *const u8
                            as *const ::libc::c_void);
                    let _n: usize = ((*table).src_length as usize);
                    (*out).Write_pconstlibcc_void_usize(_buf, _n)
                }) as i64)
                    != 0)
                {
                    return false;
                }
            } else {
                if (((*table).tag) == (kGlyfTableTag_0)) {
                    (*table).dst_offset = (dest_offset as u32);
                    let mut loca_table: *mut woff2_Table = (unsafe {
                        FindTable_65((&mut tables as *mut Vec<*mut woff2_Table>), kLocaTableTag_2)
                    });
                    if ((!(unsafe {
                        let _data: *const u8 =
                            (transformed_buf.offset(((*table).src_offset) as isize)).cast_const();
                        let _glyf_table: *mut woff2_Table = (table);
                        ReconstructGlyf_63(
                            _data,
                            _glyf_table,
                            (&mut checksum as *mut u32),
                            loca_table,
                            (&mut loca_checksum as *mut u32),
                            info,
                            out,
                        )
                    }) as i64)
                        != 0)
                    {
                        return false;
                    }
                } else if (((*table).tag) == (kLocaTableTag_2)) {
                    checksum = loca_checksum;
                } else if (((*table).tag) == (kHmtxTableTag_5)) {
                    (*table).dst_offset = (dest_offset as u32);
                    if ((!(unsafe {
                        let _transformed_buf: *const u8 =
                            (transformed_buf.offset(((*table).src_offset) as isize)).cast_const();
                        let _transformed_size: usize = ((*table).src_length as usize);
                        let _num_glyphs: u16 = (*info).num_glyphs;
                        let _num_hmetrics: u16 = (*info).num_hmetrics;
                        let _x_mins: *const Vec<i16> = &(*info).x_mins as *const Vec<i16>;
                        ReconstructTransformedHmtx_67(
                            _transformed_buf,
                            _transformed_size,
                            _num_glyphs,
                            _num_hmetrics,
                            _x_mins,
                            (&mut checksum as *mut u32),
                            out,
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
        (unsafe { StoreU32_31(table_entry.as_mut_ptr(), 0_usize, checksum) });
        (unsafe { StoreU32_31(table_entry.as_mut_ptr(), 4_usize, (*table).dst_offset) });
        (unsafe { StoreU32_31(table_entry.as_mut_ptr(), 8_usize, (*table).dst_length) });
        if ((!(unsafe {
            (*out).Write_pconstlibcc_void_usize_usize(
                (table_entry.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                (((*(*info)
                    .table_entry_by_tag
                    .entry((*table).tag)
                    .or_default()
                    .as_mut())
                .wrapping_add(4_u32)) as usize),
                12_usize,
            )
        }) as i64)
            != 0)
        {
            return false;
        }
        font_checksum = (font_checksum).wrapping_add(
            (unsafe { ComputeULongSum_26((table_entry.as_mut_ptr()).cast_const(), 12_usize) }),
        );
        if ((!(unsafe { Pad4_61(out) }) as i64) != 0) {
            return false;
        }
        if ((((((((*table).dst_offset).wrapping_add((*table).dst_length)) as u64) as usize)
            > (unsafe { (*out).Size() })) as i64)
            != 0)
        {
            return false;
        }
        dest_offset = (unsafe { (*out).Size() }).clone();
        i.postfix_inc();
    }
    let mut head_table: *mut woff2_Table =
        (unsafe { FindTable_65((&mut tables as *mut Vec<*mut woff2_Table>), kHeadTableTag_1) });
    if !(head_table).is_null() {
        if (((((*head_table).dst_length) < (12_u32)) as i64) != 0) {
            return false;
        }
        let mut checksum_adjustment: [u8; 4] = [0_u8; 4];
        (unsafe {
            StoreU32_31(
                checksum_adjustment.as_mut_ptr(),
                0_usize,
                (2981146554_u32 as u32).wrapping_sub(font_checksum),
            )
        });
        if ((!(unsafe {
            (*out).Write_pconstlibcc_void_usize_usize(
                (checksum_adjustment.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                ((((*head_table).dst_offset).wrapping_add(8_u32)) as usize),
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
pub unsafe fn ReadWOFF2Header_75(
    mut data: *const u8,
    mut length: usize,
    mut hdr: *mut woff2_WOFF2Header,
) -> bool {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, length);
    let mut signature: u32 = 0_u32;
    if (((((!(unsafe { file.ReadU32((&mut signature as *mut u32)) }))
        || ((signature) != (kWoff2Signature_20)))
        || (!(unsafe { file.ReadU32((&mut (*hdr).flavor as *mut u32)) }))) as i64)
        != 0)
    {
        return false;
    }
    let mut reported_length: u32 = 0_u32;
    if ((((!(unsafe { file.ReadU32((&mut reported_length as *mut u32)) }))
        || ((length) != (reported_length as usize))) as i64)
        != 0)
    {
        return false;
    }
    if ((((!(unsafe { file.ReadU16((&mut (*hdr).num_tables as *mut u16)) }))
        || (!((*hdr).num_tables != 0))) as i64)
        != 0)
    {
        return false;
    }
    if ((!(unsafe { file.Skip(6_usize) }) as i64) != 0) {
        return false;
    }
    if ((!(unsafe { file.ReadU32((&mut (*hdr).compressed_length as *mut u32)) }) as i64) != 0) {
        return false;
    }
    if ((!(unsafe { file.Skip((((2) * (2)) as usize)) }) as i64) != 0) {
        return false;
    }
    let mut meta_offset: u32 = 0_u32;
    let mut meta_length: u32 = 0_u32;
    let mut meta_length_orig: u32 = 0_u32;
    if (((((!(unsafe { file.ReadU32((&mut meta_offset as *mut u32)) }))
        || (!(unsafe { file.ReadU32((&mut meta_length as *mut u32)) })))
        || (!(unsafe { file.ReadU32((&mut meta_length_orig as *mut u32)) }))) as i64)
        != 0)
    {
        return false;
    }
    if (meta_offset != 0) {
        if (((((meta_offset as usize) >= (length))
            || (((length).wrapping_sub((meta_offset as usize))) < (meta_length as usize)))
            as i64)
            != 0)
        {
            return false;
        }
    }
    let mut priv_offset: u32 = 0_u32;
    let mut priv_length: u32 = 0_u32;
    if ((((!(unsafe { file.ReadU32((&mut priv_offset as *mut u32)) }))
        || (!(unsafe { file.ReadU32((&mut priv_length as *mut u32)) }))) as i64)
        != 0)
    {
        return false;
    }
    if (priv_offset != 0) {
        if (((((priv_offset as usize) >= (length))
            || (((length).wrapping_sub((priv_offset as usize))) < (priv_length as usize)))
            as i64)
            != 0)
        {
            return false;
        }
    }
    {
        let __a0 = ((*hdr).num_tables as usize) as usize;
        (*hdr).tables.resize_with(__a0, || <woff2_Table>::default())
    };
    if ((!(unsafe {
        let _tables: *mut Vec<woff2_Table> = (&mut (*hdr).tables as *mut Vec<woff2_Table>);
        let _num_tables: usize = ((*hdr).num_tables as usize);
        ReadTableDirectory_69((&mut file as *mut woff2_Buffer), _tables, _num_tables)
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
    if (((*hdr).flavor) == (kTtcFontFlavor_22)) {
        if ((!(unsafe { file.ReadU32((&mut (*hdr).header_version as *mut u32)) }) as i64) != 0) {
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
            Read255UShort_12(
                (&mut file as *mut woff2_Buffer),
                (&mut num_fonts as *mut u32),
            )
        })) || (!(num_fonts != 0))) as i64)
            != 0)
        {
            return false;
        }
        {
            let __a0 = (num_fonts as usize) as usize;
            (*hdr)
                .ttc_fonts
                .resize_with(__a0, || <woff2_TtcFont>::default())
        };
        let mut i: u32 = 0_u32;
        'loop_: while ((i) < (num_fonts)) {
            let ttc_font: *mut woff2_TtcFont =
                &mut (&mut (*hdr)).ttc_fonts[(i as usize)] as *mut woff2_TtcFont;
            let mut num_tables: u32 = 0_u32;
            if ((((!(unsafe {
                Read255UShort_12(
                    (&mut file as *mut woff2_Buffer),
                    (&mut num_tables as *mut u32),
                )
            })) || (!(num_tables != 0))) as i64)
                != 0)
            {
                return false;
            }
            if ((!(unsafe { file.ReadU32((&mut (*ttc_font).flavor as *mut u32)) }) as i64) != 0) {
                return false;
            }
            {
                let __a0 = (num_tables as usize) as usize;
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
                    Read255UShort_12(
                        (&mut file as *mut woff2_Buffer),
                        (&mut table_idx as *mut u32),
                    )
                }) as i64)
                    != 0)
                    || ((table_idx as usize) >= ((*hdr).tables.len()))
                {
                    return false;
                }
                (&mut (*ttc_font)).table_indices[(j as usize)] = (table_idx as u16);
                let table: *const woff2_Table =
                    &(&mut (*hdr)).tables[(table_idx as usize)] as *const woff2_Table;
                if (((*table).tag) == (kLocaTableTag_2)) {
                    loca_idx = table_idx;
                }
                if (((*table).tag) == (kGlyfTableTag_0)) {
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
                        c"TTC font %d has non-consecutive glyf/loca\n".as_ptr() as *const i8,
                        i,
                    );
                    return false;
                }
            }
            i.postfix_inc();
        }
    }
    let first_table_offset: u64 =
        (unsafe { ComputeOffsetToFirstTable_72(&(*hdr) as *const woff2_WOFF2Header) });
    (*hdr).compressed_offset = ((unsafe { file.offset() }) as u64);
    if (((((*hdr).compressed_offset) > (<u32>::MAX as u64)) as i64) != 0) {
        return false;
    }
    let mut src_offset: u64 = (unsafe {
        Round4_29(((*hdr).compressed_offset).wrapping_add(((*hdr).compressed_length as u64)))
    });
    let mut dst_offset: u64 = first_table_offset;
    if ((((src_offset as usize) > (length)) as i64) != 0) {
        printf(
            c"offset fail; src_offset %lu length %lu dst_offset %lu\n".as_ptr() as *const i8,
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
        src_offset = ((unsafe { Round4_30((meta_offset).wrapping_add(meta_length)) }) as u64);
        if ((((src_offset) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if (priv_offset != 0) {
        if ((((src_offset) != (priv_offset as u64)) as i64) != 0) {
            return false;
        }
        src_offset = ((unsafe { Round4_30((priv_offset).wrapping_add(priv_length)) }) as u64);
        if ((((src_offset) > (<u32>::MAX as u64)) as i64) != 0) {
            return false;
        }
    }
    if ((((src_offset) != (unsafe { Round4_29((length as u64)) })) as i64) != 0) {
        return false;
    }
    return true;
}
pub unsafe fn WriteHeaders_76(
    mut data: *const u8,
    mut length: usize,
    mut metadata: *mut woff2_RebuildMetadata,
    mut hdr: *mut woff2_WOFF2Header,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut output: Vec<u8> = vec![
        0_u8;
        (unsafe { ComputeOffsetToFirstTable_72(&(*hdr) as *const woff2_WOFF2Header,) })
            as usize
    ];
    let mut sorted_tables: Vec<woff2_Table> = (*hdr).tables.clone();
    if ((*hdr).header_version != 0) {
        'loop_: for ttc_font in 0..((*hdr).ttc_fonts.len()) {
            let mut ttc_font = (*hdr).ttc_fonts.as_mut_ptr().add(ttc_font);
            let mut sorted_index_by_tag: BTreeMap<u32, Box<u16>> = BTreeMap::new();
            'loop_: for table_index in 0..((*ttc_font).table_indices.len()) {
                let mut table_index = (&(*ttc_font)).table_indices[table_index].clone();
                (*sorted_index_by_tag
                    .entry((&mut (*hdr)).tables[(table_index as usize)].tag)
                    .or_default()
                    .as_mut()) = table_index;
            }
            let mut index: u16 = 0_u16;
            'loop_: for i in
                UnsafeMapIterator::begin(&sorted_index_by_tag as *const BTreeMap<u32, Box<u16>>)
            {
                (&mut (*ttc_font)).table_indices[(index.postfix_inc() as usize)] = *i.second();
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
    let mut result: *mut u8 = (&mut output[(0_usize)] as *mut u8);
    let mut offset: usize = 0_usize;
    if ((*hdr).header_version != 0) {
        offset = (unsafe { StoreU32_31(result, offset, (*hdr).flavor) });
        offset = (unsafe { StoreU32_31(result, offset, (*hdr).header_version) });
        offset = (unsafe { StoreU32_31(result, offset, ((*hdr).ttc_fonts.len() as u32)) }).clone();
        let mut offset_table: usize = offset;
        let mut i: usize = 0_usize;
        'loop_: while ((i) < ((*hdr).ttc_fonts.len())) {
            offset = (unsafe { StoreU32_31(result, offset, 0_u32) });
            i.postfix_inc();
        }
        if (((*hdr).header_version) == (131072_u32)) {
            offset = (unsafe { StoreU32_31(result, offset, 0_u32) });
            offset = (unsafe { StoreU32_31(result, offset, 0_u32) });
            offset = (unsafe { StoreU32_31(result, offset, 0_u32) });
        }
        {
            let __a0 = (*hdr).ttc_fonts.len() as usize;
            (*metadata)
                .font_infos
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        let mut i: usize = 0_usize;
        'loop_: while ((i) < ((*hdr).ttc_fonts.len())) {
            let ttc_font: *mut woff2_TtcFont =
                &mut (&mut (*hdr)).ttc_fonts[(i)] as *mut woff2_TtcFont;
            offset_table = (unsafe { StoreU32_31(result, offset_table, (offset as u32)) });
            (*ttc_font).dst_offset = (offset as u32);
            offset = (unsafe {
                let _flavor: u32 = (*ttc_font).flavor;
                let _num_tables: u16 = ((*ttc_font).table_indices.len() as u16);
                StoreOffsetTable_70(result, offset, _flavor, _num_tables)
            });
            'loop_: for table_index in 0..((*ttc_font).table_indices.len()) {
                let table_index = (&(*ttc_font)).table_indices[table_index].clone();
                let mut tag: u32 = (&mut (*hdr)).tables[(table_index as usize)].tag;
                (*(&mut (*metadata)).font_infos[(i)]
                    .table_entry_by_tag
                    .entry(tag)
                    .or_default()
                    .as_mut()) = (offset as u32);
                offset = (unsafe { StoreTableEntry_71(result, (offset as u32), tag) });
            }
            (*ttc_font).header_checksum = (unsafe {
                let _buf: *const u8 =
                    (&mut output[((*ttc_font).dst_offset as usize)] as *mut u8).cast_const();
                let _size: usize = (offset).wrapping_sub(((*ttc_font).dst_offset as usize));
                ComputeULongSum_26(_buf, _size)
            });
            i.postfix_inc();
        }
    } else {
        {
            let __a0 = 1_usize as usize;
            (*metadata)
                .font_infos
                .resize_with(__a0, || <woff2_WOFF2FontInfo>::default())
        };
        offset = (unsafe {
            let _flavor: u32 = (*hdr).flavor;
            let _num_tables: u16 = (*hdr).num_tables;
            StoreOffsetTable_70(result, offset, _flavor, _num_tables)
        });
        let mut i: u16 = 0_u16;
        'loop_: while ((i as i32) < ((*hdr).num_tables as i32)) {
            (*(&mut (*metadata)).font_infos[(0_usize)]
                .table_entry_by_tag
                .entry(sorted_tables[(i as usize)].tag)
                .or_default()
                .as_mut()) = (offset as u32);
            offset = (unsafe {
                StoreTableEntry_71(result, (offset as u32), sorted_tables[(i as usize)].tag)
            });
            i.prefix_inc();
        }
    }
    if ((!(unsafe {
        let _buf: *const ::libc::c_void =
            ((&mut output[(0_usize)] as *mut u8) as *const u8 as *const ::libc::c_void);
        let _n: usize = output.len();
        (*out).Write_pconstlibcc_void_usize(_buf, _n)
    }) as i64)
        != 0)
    {
        return false;
    }
    (*metadata).header_checksum = (unsafe {
        let _buf: *const u8 = (&mut output[(0_usize)] as *mut u8).cast_const();
        let _size: usize = output.len();
        ComputeULongSum_26(_buf, _size)
    });
    return true;
}
pub unsafe fn ComputeWOFF2FinalSize_77(mut data: *const u8, mut length: usize) -> usize {
    let mut file: woff2_Buffer = woff2_Buffer::woff2_Buffer(data, length);
    let mut total_length: u32 = 0_u32;
    if (!(unsafe { file.Skip(16_usize) }))
        || (!(unsafe { file.ReadU32((&mut total_length as *mut u32)) }))
    {
        return 0_usize;
    }
    return (total_length as usize);
}
pub unsafe fn ConvertWOFF2ToTTF_78(
    mut result: *mut u8,
    mut result_length: usize,
    mut data: *const u8,
    mut length: usize,
) -> bool {
    let mut out: woff2_WOFF2MemoryOut =
        woff2_WOFF2MemoryOut::woff2_WOFF2MemoryOut(result, result_length);
    return (unsafe {
        ConvertWOFF2ToTTF_79(data, length, (&mut out as *mut woff2_WOFF2MemoryOut))
    });
}
pub unsafe fn ConvertWOFF2ToTTF_79(
    mut data: *const u8,
    mut length: usize,
    mut out: *mut dyn woff2_WOFF2Out,
) -> bool {
    let mut metadata: woff2_RebuildMetadata = <woff2_RebuildMetadata>::default();
    let mut hdr: woff2_WOFF2Header = <woff2_WOFF2Header>::default();
    if !(unsafe { ReadWOFF2Header_75(data, length, (&mut hdr as *mut woff2_WOFF2Header)) }) {
        return false;
    }
    if !(unsafe {
        WriteHeaders_76(
            data,
            length,
            (&mut metadata as *mut woff2_RebuildMetadata),
            (&mut hdr as *mut woff2_WOFF2Header),
            out,
        )
    }) {
        return false;
    }
    let compression_ratio: f32 = ((hdr.uncompressed_size as f32) / (length as f32));
    if ((compression_ratio) > (kMaxPlausibleCompressionRatio_54)) {
        printf(
            c"Implausible compression ratio %.01f\n".as_ptr() as *const i8,
            (compression_ratio as f64),
        );
        return false;
    }
    let mut src_buf: *const u8 = data.offset((hdr.compressed_offset) as isize);
    let mut uncompressed_buf: Vec<u8> = (0..(hdr.uncompressed_size as usize) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>();
    if ((((hdr.uncompressed_size) < (1_u32)) as i64) != 0) {
        return false;
    }
    if ((!(unsafe {
        let _dst_size: usize = (hdr.uncompressed_size as usize);
        let _src_size: usize = (hdr.compressed_length as usize);
        Woff2Uncompress_68(
            (&mut uncompressed_buf[(0_usize)] as *mut u8),
            _dst_size,
            src_buf,
            _src_size,
        )
    }) as i64)
        != 0)
    {
        return false;
    }
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (metadata.font_infos.len())) {
        if ((!(unsafe {
            let _transformed_buf_size: u32 = hdr.uncompressed_size;
            let _hdr: *mut woff2_WOFF2Header = (&mut hdr as *mut woff2_WOFF2Header);
            ReconstructFont_74(
                (&mut uncompressed_buf[(0_usize)] as *mut u8),
                _transformed_buf_size,
                (&mut metadata as *mut woff2_RebuildMetadata),
                _hdr,
                i,
                out,
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_WOFF2StringOut {
    buf_: *mut Vec<libc::c_char>,
    max_size_: usize,
    offset_: usize,
}
impl woff2_WOFF2StringOut {
    pub unsafe fn woff2_WOFF2StringOut(mut buf: *mut Vec<libc::c_char>) -> Self {
        let mut this = Self {
            buf_: buf,
            max_size_: kDefaultMaxSize_28,
            offset_: 0_usize,
        };
        this
    }
    pub unsafe fn MaxSize(&mut self) -> usize {
        return self.max_size_;
    }
}
unsafe impl woff2_WOFF2Out for woff2_WOFF2StringOut {
    unsafe fn Write_pconstlibcc_void_usize(
        &mut self,
        buf: *const ::libc::c_void,
        n: usize,
    ) -> bool {
        return (unsafe {
            let _offset: usize = self.offset_;
            self.Write_pconstlibcc_void_usize_usize(buf, _offset, n)
        });
    }
    unsafe fn Write_pconstlibcc_void_usize_usize(
        &mut self,
        buf: *const ::libc::c_void,
        offset: usize,
        n: usize,
    ) -> bool {
        if ((offset) > (self.max_size_)) || ((n) > ((self.max_size_).wrapping_sub(offset))) {
            return false;
        }
        if ((offset) == ((*(self.buf_).cast_const()).len() - 1)) {
            (*self.buf_).splice((*self.buf_).len().saturating_sub(1)..(*self.buf_).len(), {
                let mut v =
                    ::std::slice::from_raw_parts((buf as *const libc::c_char), n as usize).to_vec();
                v.push(0);
                v
            });
        } else {
            if (((offset).wrapping_add(n)) > ((*(self.buf_).cast_const()).len() - 1)) {
                (*self.buf_).splice(
                    (*self.buf_).len() - 1..(*self.buf_).len() - 1,
                    ::std::vec::from_elem(
                        (0 as libc::c_char),
                        (((offset).wrapping_add(n) as u64)
                            .wrapping_sub((((*(self.buf_).cast_const()).len() - 1) as u64))
                            as usize) as usize,
                    ),
                );
            }
            (*self.buf_).splice(
                offset as usize..offset as usize + n as usize,
                ::std::slice::from_raw_parts((buf as *const libc::c_char), n as usize).to_vec(),
            );
        }
        self.offset_ = ({
            let mut __tmp_0 = (self.offset_ as u64);
            let mut __tmp_1 = ((offset).wrapping_add(n) as u64);
            (*if *&mut __tmp_0 >= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        } as usize);
        return true;
    }
    unsafe fn Size(&mut self) -> usize {
        return self.offset_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct woff2_WOFF2MemoryOut {
    buf_: *mut u8,
    buf_size_: usize,
    offset_: usize,
}
impl woff2_WOFF2MemoryOut {
    pub unsafe fn woff2_WOFF2MemoryOut(mut buf: *mut u8, mut buf_size: usize) -> Self {
        let mut this = Self {
            buf_: buf,
            buf_size_: buf_size,
            offset_: 0_usize,
        };
        this
    }
}
unsafe impl woff2_WOFF2Out for woff2_WOFF2MemoryOut {
    unsafe fn Write_pconstlibcc_void_usize(
        &mut self,
        buf: *const ::libc::c_void,
        n: usize,
    ) -> bool {
        return (unsafe {
            let _offset: usize = self.offset_;
            self.Write_pconstlibcc_void_usize_usize(buf, _offset, n)
        });
    }
    unsafe fn Write_pconstlibcc_void_usize_usize(
        &mut self,
        buf: *const ::libc::c_void,
        offset: usize,
        n: usize,
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
        self.offset_ = ({
            let mut __tmp_0 = (self.offset_ as u64);
            let mut __tmp_1 = ((offset).wrapping_add(n) as u64);
            (*if *&mut __tmp_0 >= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        } as usize);
        return true;
    }
    unsafe fn Size(&mut self) -> usize {
        return self.offset_;
    }
}
impl woff2_WOFF2StringOut {}
impl woff2_WOFF2StringOut {
    pub unsafe fn SetMaxSize(&mut self, mut max_size: usize) {
        self.max_size_ = max_size;
        if ((self.offset_) > (self.max_size_)) {
            self.offset_ = self.max_size_;
        }
    }
}
impl woff2_WOFF2MemoryOut {}
pub unsafe fn GetFileContent_80(mut filename: Vec<libc::c_char>) -> Vec<libc::c_char> {
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
pub unsafe fn SetFileContents_81(
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
        let __from = c".ttf".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    let mut input: Vec<libc::c_char> = (unsafe { GetFileContent_80(filename.clone()) });
    let mut raw_input: *const u8 = (input.as_ptr() as *const u8);
    let mut output: Vec<libc::c_char> = vec![
        (0 as libc::c_char);
        ({
            let mut __tmp_0 =
                ((unsafe { ComputeWOFF2FinalSize_77(raw_input, (input.len() - 1)) }) as u64);
            let mut __tmp_1 = (kDefaultMaxSize_28 as u64);
            (*if *&mut __tmp_0 <= *&mut __tmp_1 {
                (&mut __tmp_0) as *const _
            } else {
                (&mut __tmp_1) as *const _
            })
        }) as usize
    ]
    .iter()
    .cloned()
    .chain(std::iter::once(0))
    .collect();
    let mut out: woff2_WOFF2StringOut =
        woff2_WOFF2StringOut::woff2_WOFF2StringOut((&mut output as *mut Vec<libc::c_char>));
    let ok: bool = (unsafe {
        ConvertWOFF2ToTTF_79(
            raw_input,
            (input.len() - 1),
            (&mut out as *mut woff2_WOFF2StringOut),
        )
    });
    if ok {
        (unsafe {
            let _start: *mut libc::c_char = output.as_mut_ptr();
            let _end: *mut libc::c_char = output
                .as_mut_ptr()
                .add(((unsafe { out.Size() }) as i64) as usize);
            SetFileContents_81(outfilename.clone(), _start, _end)
        });
    }
    return if ok { 0 } else { 1 };
}
