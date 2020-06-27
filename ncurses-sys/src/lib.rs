#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod raw;
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
pub type window = raw::WINDOW;

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
    mask << (shift + raw::NCURSES_ATTR_SHIFT)
}


pub fn addch(ch: &str) -> i32 {
    unsafe {
        raw::addch(to_uint(ch))
    }
}

pub fn addchnstr(ch: &str, n: i32) -> i32 {
    unsafe {
        raw::addchnstr(to_const_uint(ch), n)
    }
}

pub fn addchstr(ch: &str) -> i32 {
    unsafe {
        raw::addchstr(to_const_uint(ch))
    }
}
pub fn addnstr(ch: &str, n: i32) -> i32 {
    unsafe {
        raw::addnstr(to_const_int(ch), n)
    }
}
pub fn addstr(ch: &str) -> i32 {
    unsafe {
        raw::addstr(to_const_int(ch))
    }
}

pub fn attroff(attr: i32) -> i32 {
    unsafe {
        raw::attroff(attr)
    }
}

pub fn attron(attr: i32) -> i32 {
    unsafe {
        raw::attron(attr)
    }
}

pub fn attrset(attr: i32) -> i32 {
    unsafe {
        raw::attrset(attr)
    }
}

pub fn attr_get(attr: *mut u32, cpairs: *mut i16) -> i32 {
    unsafe {
        raw::attr_get(attr, cpairs, ptr::null_mut())
    }
}

pub fn attr_off(attr: u32) -> i32 {
    unsafe {
        raw::attr_off(attr, ptr::null_mut())
    }
}

pub fn attr_on(attr: u32) -> i32 {
    unsafe {
        raw::attr_on(attr, ptr::null_mut())
    }
}

pub fn attr_set(attr: u32, cpairs: i16) -> i32 {
    unsafe {
        raw::attr_set(attr, cpairs, ptr::null_mut())
    }
}

pub fn baudrate() -> i32 {
    unsafe {
        raw::baudrate()
    }
}
pub fn beep() -> i32 {
    unsafe {
        raw::beep()
    }
}
pub fn bkgd(ch: &str) -> i32 {
    unsafe {
        raw::bkgd(to_uint(ch))
    }
}
pub fn bkgdset(ch: &str) {
    unsafe {
        raw::bkgdset(to_uint(ch))
    }
}
pub fn border(ls: &str, rs: &str, ts: &str, bs: &str, tl: &str, tr: &str, bl: &str, br: &str) -> i32 {
    unsafe {
        raw::border(to_uint(ls), to_uint(rs), to_uint(ts), to_uint(bs), to_uint(tl), to_uint(tr), to_uint(bl), to_uint(br))
    }
}
pub fn box_(win: *mut window, verch: &str, horch: &str) -> i32 {
    unsafe {
        raw::box_(win, to_uint(verch), to_uint(horch))
    }
}
pub fn can_change_color() -> bool {
    unsafe {
        raw::can_change_color()
    }
}
pub fn cbreak() -> i32 {
    unsafe {
        raw::cbreak()
    }
}
pub fn chgat(n: i32, attr: u32, color: i16) -> i32 {
    unsafe {
        raw::chgat(n, attr, color, ptr::null())
    }
}
pub fn clear() -> i32 {
    unsafe {
        raw::clear()
    }
}
pub fn clearok(win: *mut window, bf: bool) -> i32 {
    unsafe {
        raw::clearok(win, bf)
    }
}
pub fn clrtobot() -> i32 {
    unsafe {
        raw::clrtobot()
    }
}
pub fn clrtoeol() -> i32 {
    unsafe {
        raw::clrtoeol()
    }
}
pub fn color_content(color: i16, r: *mut i16, g: *mut i16, b: *mut i16) -> i32 {
    unsafe {
        raw::color_content(color, r, g, b)
    }
}

pub fn color_set(color: i16) -> i32 {
    unsafe {
        raw::color_set(color, ptr::null_mut())
    }
}
pub fn COLOR_PAIR(n: i32) -> i32 {
    unsafe {
        raw::COLOR_PAIR(n)
    }
}
pub fn copywin(srcwin: *const window, dstwin: *mut window, sminrow: i32, smincol: i32, dminrow: i32, dmincol: i32, dmaxrow: i32, dmaxcol: i32, overlay: i32) -> i32 {
    unsafe {
        raw::copywin(srcwin, dstwin, sminrow, smincol, dminrow, dmincol, dmaxrow, dmaxcol, overlay)
    }
}
pub fn curs_set(n: i32) -> i32 {
    unsafe {
        raw::curs_set(n)
    }
}
pub fn def_prog_mode() -> i32 {
    unsafe {
        raw::def_prog_mode()
    }
}
pub fn def_shell_mode() -> i32 {
    unsafe {
        raw::def_shell_mode()
    }
}