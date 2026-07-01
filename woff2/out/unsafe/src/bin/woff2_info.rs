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
pub unsafe fn GetFileContent_49(mut filename: Vec<libc::c_char>) -> Vec<libc::c_char> {
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
pub unsafe fn SetFileContents_50(
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
pub unsafe fn PrintTag_51(mut tag: i32) -> Vec<libc::c_char> {
    if (((tag as u32) & (2155905152_u32)) != 0) {
        return {
            let s = c"_xfm".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
    }
    let mut printable: [libc::c_char; 4] = [
        ((((tag) >> (24)) & (255)) as libc::c_char),
        ((((tag) >> (16)) & (255)) as libc::c_char),
        ((((tag) >> (8)) & (255)) as libc::c_char),
        (((tag) & (255)) as libc::c_char),
    ];
    return std::slice::from_raw_parts((printable.as_mut_ptr()).cast_const(), 4_usize as usize)
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
    let mut input: Vec<libc::c_char> = (unsafe { GetFileContent_49(filename.clone()) });
    let mut file: woff2_Buffer =
        woff2_Buffer::woff2_Buffer((input.as_ptr() as *const u8), (input.len() - 1));
    printf(c"WOFF2Header\n".as_ptr() as *const i8);
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
    if !(unsafe { file.ReadU32((&mut signature as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut flavor as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut length as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU16((&mut num_tables as *mut u16)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU16((&mut reserved as *mut u16)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut totalSfntSize as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut totalCompressedSize as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU16((&mut major as *mut u16)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU16((&mut minor as *mut u16)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut metaOffset as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut metaLength as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut metaOrigLength as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut privOffset as *mut u32)) }) {
        return 1;
    }
    if !(unsafe { file.ReadU32((&mut privLength as *mut u32)) }) {
        return 1;
    }
    if ((signature) != (2001684018_u32)) {
        printf(
            c"Invalid signature: %08x\n".as_ptr() as *const i8,
            signature,
        );
        return 1;
    }
    printf(
        c"signature           0x%08x\n".as_ptr() as *const i8,
        signature,
    );
    printf(
        c"flavor              0x%08x\n".as_ptr() as *const i8,
        flavor,
    );
    printf(c"length              %d\n".as_ptr() as *const i8, length);
    printf(
        c"numTables           %d\n".as_ptr() as *const i8,
        (num_tables as i32),
    );
    printf(
        c"reserved            %d\n".as_ptr() as *const i8,
        (reserved as i32),
    );
    printf(
        c"totalSfntSize       %d\n".as_ptr() as *const i8,
        totalSfntSize,
    );
    printf(
        c"totalCompressedSize %d\n".as_ptr() as *const i8,
        totalCompressedSize,
    );
    printf(
        c"majorVersion        %d\n".as_ptr() as *const i8,
        (major as i32),
    );
    printf(
        c"minorVersion        %d\n".as_ptr() as *const i8,
        (minor as i32),
    );
    printf(
        c"metaOffset          %d\n".as_ptr() as *const i8,
        metaOffset,
    );
    printf(
        c"metaLength          %d\n".as_ptr() as *const i8,
        metaLength,
    );
    printf(
        c"metaOrigLength      %d\n".as_ptr() as *const i8,
        metaOrigLength,
    );
    printf(
        c"privOffset          %d\n".as_ptr() as *const i8,
        privOffset,
    );
    printf(
        c"privLength          %d\n".as_ptr() as *const i8,
        privLength,
    );
    let mut table_tags: Vec<u32> = Vec::new();
    printf(
        c"TableDirectory starts at +%zu\n".as_ptr() as *const i8,
        (unsafe { file.offset() }),
    );
    printf(c"Entry offset flags tag  origLength txLength\n".as_ptr() as *const i8);
    let mut i: i32 = 0;
    'loop_: while ((i) < (num_tables as i32)) {
        let mut offset: usize = (unsafe { file.offset() });
        let mut flags: u8 = 0_u8;
        let mut tag: u32 = 0_u32;
        let mut origLength: u32 = 0_u32;
        let mut transformLength: u32 = 0_u32;
        if !(unsafe { file.ReadU8((&mut flags as *mut u8)) }) {
            return 1;
        }
        if (((flags as i32) & (63)) == (63)) {
            if !(unsafe { file.ReadU32((&mut tag as *mut u32)) }) {
                return 1;
            }
        } else {
            tag = kKnownTags_8[((flags as i32) & (63)) as usize];
        }
        {
            let a0_clone = tag.clone();
            table_tags.push(a0_clone)
        };
        if !(unsafe {
            ReadBase128_17(
                (&mut file as *mut woff2_Buffer),
                (&mut origLength as *mut u32),
            )
        }) {
            return 1;
        }
        printf(
            c"%5d %6zu  0x%02x %s %10d".as_ptr() as *const i8,
            i,
            offset,
            (flags as i32),
            (unsafe { PrintTag_51((tag as i32)) }).as_ptr(),
            origLength,
        );
        let mut xform_version: u8 = ((((flags as i32) >> (6)) & (3)) as u8);
        if ((tag) == (kGlyfTableTag_0)) || ((tag) == (kLocaTableTag_2)) {
            if ((xform_version as i32) == (0)) {
                if !(unsafe {
                    ReadBase128_17(
                        (&mut file as *mut woff2_Buffer),
                        (&mut transformLength as *mut u32),
                    )
                }) {
                    return 1;
                }
                printf(c" %8d".as_ptr() as *const i8, transformLength);
            }
        } else if ((xform_version as i32) > (0)) {
            if !(unsafe {
                ReadBase128_17(
                    (&mut file as *mut woff2_Buffer),
                    (&mut transformLength as *mut u32),
                )
            }) {
                return 1;
            }
            printf(c" %8d".as_ptr() as *const i8, transformLength);
        }
        printf(c"\n".as_ptr() as *const i8);
        i.postfix_inc();
    }
    if ((flavor) == (kTtcFontFlavor_22)) {
        let mut version: u32 = 0_u32;
        let mut numFonts: u32 = 0_u32;
        if !(unsafe { file.ReadU32((&mut version as *mut u32)) }) {
            return 1;
        }
        if !(unsafe {
            Read255UShort_12(
                (&mut file as *mut woff2_Buffer),
                (&mut numFonts as *mut u32),
            )
        }) {
            return 1;
        }
        printf(
            c"CollectionHeader 0x%08x %d fonts\n".as_ptr() as *const i8,
            version,
            numFonts,
        );
        let mut i: i32 = 0;
        'loop_: while ((i as u32) < (numFonts)) {
            let mut numTables: u32 = 0_u32;
            let mut flavor: u32 = 0_u32;
            if !(unsafe {
                Read255UShort_12(
                    (&mut file as *mut woff2_Buffer),
                    (&mut numTables as *mut u32),
                )
            }) {
                return 1;
            }
            if !(unsafe { file.ReadU32((&mut flavor as *mut u32)) }) {
                return 1;
            }
            printf(
                c"CollectionFontEntry %d flavor 0x%08x %d tables\n".as_ptr() as *const i8,
                i,
                flavor,
                numTables,
            );
            let mut j: i32 = 0;
            'loop_: while ((j as u32) < (numTables)) {
                let mut table_idx: u32 = 0_u32;
                if !(unsafe {
                    Read255UShort_12(
                        (&mut file as *mut woff2_Buffer),
                        (&mut table_idx as *mut u32),
                    )
                }) {
                    return 1;
                }
                if ((table_idx as usize) >= (table_tags.len())) {
                    return 1;
                }
                printf(
                    c"  %d %s (idx %d)\n".as_ptr() as *const i8,
                    j,
                    (unsafe { PrintTag_51((table_tags[(table_idx as usize)] as i32)) }).as_ptr(),
                    table_idx,
                );
                j.postfix_inc();
            }
            i.postfix_inc();
        }
    }
    printf(
        c"TableDirectory ends at +%zu\n".as_ptr() as *const i8,
        (unsafe { file.offset() }),
    );
    return 0;
}
