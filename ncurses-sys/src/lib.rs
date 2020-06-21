#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod ffi;
use std::ptr;
pub const A_ATTRIBUTES: u32 = NCURSES_BITS(!((1 as u32)-(1 as u32)), 0);
pub const A_CHARTEXT:   u32 = NCURSES_BITS(1 as u32, 0)-(1 as u32);
pub const A_COLOR:      u32 = NCURSES_BITS(((1 as u32) << 8 as u32) - 1 as u32, 0);
pub const A_STANDOUT:   u32 = NCURSES_BITS(1, 8);
pub const A_UNDERLINE:  u32 = NCURSES_BITS(1, 9);
pub const A_REVERSE:    u32 = NCURSES_BITS(1 as u32,10);
pub const A_BLINK:      u32 = NCURSES_BITS(1 as u32,11);
pub const A_DIM:        u32 = NCURSES_BITS(1 as u32,12);
pub const A_BOLD:       u32 = NCURSES_BITS(1 as u32,13);
pub const A_ALTCHARSET: u32 = NCURSES_BITS(1 as u32,14);
pub const A_INVIS:      u32 = NCURSES_BITS(1 as u32,15);
pub const A_PROTECT:    u32 = NCURSES_BITS(1 as u32,16);
pub const A_HORIZONTAL: u32 = NCURSES_BITS(1 as u32,17);
pub const A_LEFT:       u32 = NCURSES_BITS(1 as u32,18);
pub const A_LOW:        u32 = NCURSES_BITS(1 as u32,19);
pub const A_RIGHT:      u32 = NCURSES_BITS(1 as u32,20);
pub const A_TOP:        u32 = NCURSES_BITS(1 as u32,21);
pub const A_VERTICAL:   u32 = NCURSES_BITS(1 as u32,22);


fn to_const_uint(n: &str) -> *const u32 {
    n.as_ptr() as *const u32
}

fn to_uint(n: &str) -> u32 {
    n.as_ptr() as u32
}

fn to_const_int(n: &str) -> *const i8 {
    n.as_ptr() as *const i8
}

fn to_int(n: &str) -> i8 {
    n.as_ptr() as i8
}

const fn NCURSES_BITS(mask: u32, shift: u32) -> u32 {
    mask << (shift + ffi::NCURSES_ATTR_SHIFT)
}


pub fn addch(ch: &str) -> i32 {
    unsafe {
        ffi::addch(to_uint(ch))
    }
}

pub fn addchnstr(ch: &str, n: i32) -> i32 {
    unsafe {
        ffi::addchnstr(to_const_uint(ch), n)
    }
}

pub fn addchstr(ch: &str) -> i32 {
    unsafe {
        ffi::addchstr(to_const_uint(ch))
    }
}
pub fn addnstr(ch: &str, n: i32) -> i32 {
    unsafe {
        ffi::addnstr(to_const_int(ch), n)
    }
}
pub fn addstr(ch: &str) -> i32 {
    unsafe {
        ffi::addstr(to_const_int(ch))
    }
}

pub fn attroff(attr: i32) -> i32 {
    unsafe {
        ffi::attroff(attr)
    }
}

pub fn attron(attr: i32) -> i32 {
    unsafe {
        ffi::attron(attr)
    }
}

pub fn attrset(attr: i32) -> i32 {
    unsafe {
        ffi::attrset(attr)
    }
}

pub fn attr_get(attr: *mut u32, cpairs: *mut i16) -> i32 {
    unsafe {
        ffi::attr_get(attr, cpairs, ptr::null_mut())
    }
}

pub fn attr_off(attr: u32) -> i32 {
    unsafe {
        ffi::attr_off(attr, ptr::null_mut())
    }
}

pub fn attr_on(attr: u32) -> i32 {
    unsafe {
        ffi::attr_on(attr, ptr::null_mut())
    }
}

pub fn attr_set(attr: u32, cpairs: i16) -> i32 {
    unsafe {
        ffi::attr_set(attr, cpairs, ptr::null_mut())
    }
}

