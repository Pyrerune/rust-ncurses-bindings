use libc::FILE;
use core::ffi::{c_void};
type c_uint = u32;
type c_uchar = u8;
type c_short = i16;
type c_int = i32;
type c_char = i8;
pub const CURSES: u32 = 1;
pub const CURSES_H: u32 = 1;
pub const NCURSES_VERSION_MAJOR: u32 = 6;
pub const NCURSES_VERSION_MINOR: u32 = 2;
pub const NCURSES_VERSION_PATCH: u32 = 20200212;
pub const NCURSES_VERSION: &'static [u8; 4usize] = b"6.2\0";
pub const NCURSES_MOUSE_VERSION: u32 = 2;
pub const NCURSES_ENABLE_STDBOOL_H: u32 = 1;
pub const NCURSES_OPAQUE: u32 = 0;
pub const NCURSES_OPAQUE_FORM: u32 = 0;
pub const NCURSES_OPAQUE_MENU: u32 = 0;
pub const NCURSES_OPAQUE_PANEL: u32 = 0;
pub const NCURSES_WATTR_MACROS: u32 = 1;
pub const NCURSES_REENTRANT: u32 = 0;
pub const NCURSES_INTEROP_FUNCS: u32 = 1;
pub const NCURSES_TPARM_VARARGS: u32 = 1;
pub const NCURSES_WCWIDTH_GRAPHICS: u32 = 1;
pub const NCURSES_WIDECHAR: u32 = 0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const COLOR_BLACK: u32 = 0;
pub const COLOR_RED: u32 = 1;
pub const COLOR_GREEN: u32 = 2;
pub const COLOR_YELLOW: u32 = 3;
pub const COLOR_BLUE: u32 = 4;
pub const COLOR_MAGENTA: u32 = 5;
pub const COLOR_CYAN: u32 = 6;
pub const COLOR_WHITE: u32 = 7;
pub const ERR: i32 = -1;
pub const OK: u32 = 0;
pub const _SUBWIN: u32 = 1;
pub const _ENDLINE: u32 = 2;
pub const _FULLWIN: u32 = 4;
pub const _SCROLLWIN: u32 = 8;
pub const _ISPAD: u32 = 16;
pub const _HASMOVED: u32 = 32;
pub const _WRAPPED: u32 = 64;
pub const _NOCHANGE: i32 = -1;
pub const _NEWINDEX: i32 = -1;
pub const NCURSES_EXT_FUNCS: u32 = 20200212;
pub const NCURSES_SP_FUNCS: u32 = 20200212;
pub const NCURSES_ATTR_SHIFT: u32 = 8;
pub const A_NORMAL: u32 = 0;
pub const KEY_CODE_YES: u32 = 256;
pub const KEY_MIN: u32 = 257;
pub const KEY_BREAK: u32 = 257;
pub const KEY_SRESET: u32 = 344;
pub const KEY_RESET: u32 = 345;
pub const KEY_DOWN: u32 = 258;
pub const KEY_UP: u32 = 259;
pub const KEY_LEFT: u32 = 260;
pub const KEY_RIGHT: u32 = 261;
pub const KEY_HOME: u32 = 262;
pub const KEY_BACKSPACE: u32 = 263;
pub const KEY_F0: u32 = 264;
pub const KEY_DL: u32 = 328;
pub const KEY_IL: u32 = 329;
pub const KEY_DC: u32 = 330;
pub const KEY_IC: u32 = 331;
pub const KEY_EIC: u32 = 332;
pub const KEY_CLEAR: u32 = 333;
pub const KEY_EOS: u32 = 334;
pub const KEY_EOL: u32 = 335;
pub const KEY_SF: u32 = 336;
pub const KEY_SR: u32 = 337;
pub const KEY_NPAGE: u32 = 338;
pub const KEY_PPAGE: u32 = 339;
pub const KEY_STAB: u32 = 340;
pub const KEY_CTAB: u32 = 341;
pub const KEY_CATAB: u32 = 342;
pub const KEY_ENTER: u32 = 343;
pub const KEY_PRINT: u32 = 346;
pub const KEY_LL: u32 = 347;
pub const KEY_A1: u32 = 348;
pub const KEY_A3: u32 = 349;
pub const KEY_B2: u32 = 350;
pub const KEY_C1: u32 = 351;
pub const KEY_C3: u32 = 352;
pub const KEY_BTAB: u32 = 353;
pub const KEY_BEG: u32 = 354;
pub const KEY_CANCEL: u32 = 355;
pub const KEY_CLOSE: u32 = 356;
pub const KEY_COMMAND: u32 = 357;
pub const KEY_COPY: u32 = 358;
pub const KEY_CREATE: u32 = 359;
pub const KEY_END: u32 = 360;
pub const KEY_EXIT: u32 = 361;
pub const KEY_FIND: u32 = 362;
pub const KEY_HELP: u32 = 363;
pub const KEY_MARK: u32 = 364;
pub const KEY_MESSAGE: u32 = 365;
pub const KEY_MOVE: u32 = 366;
pub const KEY_NEXT: u32 = 367;
pub const KEY_OPEN: u32 = 368;
pub const KEY_OPTIONS: u32 = 369;
pub const KEY_PREVIOUS: u32 = 370;
pub const KEY_REDO: u32 = 371;
pub const KEY_REFERENCE: u32 = 372;
pub const KEY_REFRESH: u32 = 373;
pub const KEY_REPLACE: u32 = 374;
pub const KEY_RESTART: u32 = 375;
pub const KEY_RESUME: u32 = 376;
pub const KEY_SAVE: u32 = 377;
pub const KEY_SBEG: u32 = 378;
pub const KEY_SCANCEL: u32 = 379;
pub const KEY_SCOMMAND: u32 = 380;
pub const KEY_SCOPY: u32 = 381;
pub const KEY_SCREATE: u32 = 382;
pub const KEY_SDC: u32 = 383;
pub const KEY_SDL: u32 = 384;
pub const KEY_SELECT: u32 = 385;
pub const KEY_SEND: u32 = 386;
pub const KEY_SEOL: u32 = 387;
pub const KEY_SEXIT: u32 = 388;
pub const KEY_SFIND: u32 = 389;
pub const KEY_SHELP: u32 = 390;
pub const KEY_SHOME: u32 = 391;
pub const KEY_SIC: u32 = 392;
pub const KEY_SLEFT: u32 = 393;
pub const KEY_SMESSAGE: u32 = 394;
pub const KEY_SMOVE: u32 = 395;
pub const KEY_SNEXT: u32 = 396;
pub const KEY_SOPTIONS: u32 = 397;
pub const KEY_SPREVIOUS: u32 = 398;
pub const KEY_SPRINT: u32 = 399;
pub const KEY_SREDO: u32 = 400;
pub const KEY_SREPLACE: u32 = 401;
pub const KEY_SRIGHT: u32 = 402;
pub const KEY_SRSUME: u32 = 403;
pub const KEY_SSAVE: u32 = 404;
pub const KEY_SSUSPEND: u32 = 405;
pub const KEY_SUNDO: u32 = 406;
pub const KEY_SUSPEND: u32 = 407;
pub const KEY_UNDO: u32 = 408;
pub const KEY_MOUSE: u32 = 409;
pub const KEY_RESIZE: u32 = 410;
pub const KEY_EVENT: u32 = 411;
pub const KEY_MAX: u32 = 511;
pub const _XOPEN_CURSES: u32 = 1;
pub const NCURSES_BUTTON_RELEASED: u32 = 1;
pub const NCURSES_BUTTON_PRESSED: u32 = 2;
pub const NCURSES_BUTTON_CLICKED: u32 = 4;
pub const NCURSES_DOUBLE_CLICKED: u32 = 8;
pub const NCURSES_TRIPLE_CLICKED: u32 = 16;
pub const NCURSES_RESERVED_EVENT: u32 = 32;
pub const TRACE_DISABLE: u32 = 0;
pub const TRACE_TIMES: u32 = 1;
pub const TRACE_TPUTS: u32 = 2;
pub const TRACE_UPDATE: u32 = 4;
pub const TRACE_MOVE: u32 = 8;
pub const TRACE_CHARPUT: u32 = 16;
pub const TRACE_ORDINARY: u32 = 31;
pub const TRACE_CALLS: u32 = 32;
pub const TRACE_VIRTPUT: u32 = 64;
pub const TRACE_IEVENT: u32 = 128;
pub const TRACE_BITS: u32 = 256;
pub const TRACE_ICALLS: u32 = 512;
pub const TRACE_CCALLS: u32 = 1024;
pub const TRACE_DATABASE: u32 = 2048;
pub const TRACE_ATTRS: u32 = 4096;
pub const TRACE_SHIFT: u32 = 13;
pub const TRACE_MAXIMUM: u32 = 8191;
pub const NCURSES_UNCTRL_H_incl: u32 = 1;


pub type chtype = c_uint;
pub type mmask_t = c_uint;

pub type NCURSES_BOOL = c_uchar;
type __va_list_tag = *mut u8;


extern "C" {
    pub static mut acs_map: [chtype; 0usize];
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct screen {
    _unused: [u8; 0],
}
pub type SCREEN = screen;
pub type WINDOW = _win_st;
pub type attr_t = chtype;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _win_st {
    pub _cury: c_short,
    pub _curx: c_short,
    pub _maxy: c_short,
    pub _maxx: c_short,
    pub _begy: c_short,
    pub _begx: c_short,
    pub _flags: c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: c_int,
    pub _line: *mut ldat,
    pub _regtop: c_short,
    pub _regbottom: c_short,
    pub _parx: c_int,
    pub _pary: c_int,
    pub _parent: *mut WINDOW,
    pub _pad: _win_st_pdat,
    pub _yoffset: c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _win_st_pdat {
    pub _pad_y: c_short,
    pub _pad_x: c_short,
    pub _pad_top: c_short,
    pub _pad_left: c_short,
    pub _pad_bottom: c_short,
    pub _pad_right: c_short,
}
pub type NCURSES_OUTC = ::std::option::Option<
    unsafe extern "C" fn(arg1: c_int) -> c_int,
>;
extern "C" {
    pub fn addch(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn addchnstr(arg1: *const chtype, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn addchstr(arg1: *const chtype) -> c_int;
}
extern "C" {
    pub fn addnstr(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn addstr(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn attroff(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn attron(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn attrset(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn attr_get(
        arg1: *mut attr_t,
        arg2: *mut c_short,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn attr_off(arg1: attr_t, arg2: *mut c_void) -> c_int;
}
extern "C" {
    pub fn attr_on(arg1: attr_t, arg2: *mut c_void) -> c_int;
}
extern "C" {
    pub fn attr_set(
        arg1: attr_t,
        arg2: c_short,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn baudrate() -> c_int;
}
extern "C" {
    pub fn beep() -> c_int;
}
extern "C" {
    pub fn bkgd(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn bkgdset(arg1: chtype);
}
extern "C" {
    pub fn border(
        arg1: chtype,
        arg2: chtype,
        arg3: chtype,
        arg4: chtype,
        arg5: chtype,
        arg6: chtype,
        arg7: chtype,
        arg8: chtype,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}box"]
    pub fn box_(arg1: *mut WINDOW, arg2: chtype, arg3: chtype) -> c_int;
}
extern "C" {
    pub fn can_change_color() -> bool;
}
extern "C" {
    pub fn cbreak() -> c_int;
}
extern "C" {
    pub fn chgat(
        arg1: c_int,
        arg2: attr_t,
        arg3: c_short,
        arg4: *const c_void,
    ) -> c_int;
}
extern "C" {
    pub fn clear() -> c_int;
}
extern "C" {
    pub fn clearok(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn clrtobot() -> c_int;
}
extern "C" {
    pub fn clrtoeol() -> c_int;
}
extern "C" {
    pub fn color_content(
        arg1: c_short,
        arg2: *mut c_short,
        arg3: *mut c_short,
        arg4: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn color_set(
        arg1: c_short,
        arg2: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn COLOR_PAIR(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn copywin(
        arg1: *const WINDOW,
        arg2: *mut WINDOW,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
        arg8: c_int,
        arg9: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn curs_set(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn def_prog_mode() -> c_int;
}
extern "C" {
    pub fn def_shell_mode() -> c_int;
}
extern "C" {
    pub fn delay_output(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn delch() -> c_int;
}
extern "C" {
    pub fn delscreen(arg1: *mut SCREEN);
}
extern "C" {
    pub fn delwin(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn deleteln() -> c_int;
}
extern "C" {
    pub fn derwin(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn doupdate() -> c_int;
}
extern "C" {
    pub fn dupwin(arg1: *mut WINDOW) -> *mut WINDOW;
}
extern "C" {
    pub fn echo() -> c_int;
}
extern "C" {
    pub fn echochar(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn erase() -> c_int;
}
extern "C" {
    pub fn endwin() -> c_int;
}
extern "C" {
    pub fn erasechar() -> c_char;
}
extern "C" {
    pub fn filter();
}
extern "C" {
    pub fn flash() -> c_int;
}
extern "C" {
    pub fn flushinp() -> c_int;
}
extern "C" {
    pub fn getbkgd(arg1: *mut WINDOW) -> chtype;
}
extern "C" {
    pub fn getch() -> c_int;
}
extern "C" {
    pub fn getnstr(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn getstr(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn getwin(arg1: *mut FILE) -> *mut WINDOW;
}
extern "C" {
    pub fn halfdelay(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn has_colors() -> bool;
}
extern "C" {
    pub fn has_ic() -> bool;
}
extern "C" {
    pub fn has_il() -> bool;
}
extern "C" {
    pub fn hline(arg1: chtype, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn idcok(arg1: *mut WINDOW, arg2: bool);
}
extern "C" {
    pub fn idlok(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn immedok(arg1: *mut WINDOW, arg2: bool);
}
extern "C" {
    pub fn inch() -> chtype;
}
extern "C" {
    pub fn inchnstr(arg1: *mut chtype, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn inchstr(arg1: *mut chtype) -> c_int;
}
extern "C" {
    pub fn initscr() -> *mut WINDOW;
}
extern "C" {
    pub fn init_color(
        arg1: c_short,
        arg2: c_short,
        arg3: c_short,
        arg4: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn init_pair(
        arg1: c_short,
        arg2: c_short,
        arg3: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn innstr(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn insch(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn insdelln(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn insertln() -> c_int;
}
extern "C" {
    pub fn insnstr(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn insstr(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn instr(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn intrflush(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn isendwin() -> bool;
}
extern "C" {
    pub fn is_linetouched(arg1: *mut WINDOW, arg2: c_int) -> bool;
}
extern "C" {
    pub fn is_wintouched(arg1: *mut WINDOW) -> bool;
}
extern "C" {
    pub fn keyname(arg1: c_int) -> *const c_char;
}
extern "C" {
    pub fn keypad(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn killchar() -> c_char;
}
extern "C" {
    pub fn leaveok(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn longname() -> *mut c_char;
}
extern "C" {
    pub fn meta(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}move"]
    pub fn move_(arg1: c_int, arg2: c_int)
        -> c_int;
}
extern "C" {
    pub fn mvaddch(
        arg1: c_int,
        arg2: c_int,
        arg3: chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvaddchnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const chtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvaddchstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvaddnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvaddstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvchgat(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: attr_t,
        arg5: c_short,
        arg6: *const c_void,
    ) -> c_int;
}
extern "C" {
    pub fn mvcur(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvdelch(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvderwin(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvgetch(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvgetnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_char,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvgetstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvhline(
        arg1: c_int,
        arg2: c_int,
        arg3: chtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvinch(arg1: c_int, arg2: c_int) -> chtype;
}
extern "C" {
    pub fn mvinchnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut chtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvinchstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvinnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_char,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvinsch(
        arg1: c_int,
        arg2: c_int,
        arg3: chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvinsnstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvinsstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvinstr(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvprintw(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn mvscanw(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn mvvline(
        arg1: c_int,
        arg2: c_int,
        arg3: chtype,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwaddch(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvwaddchnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const chtype,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwaddchstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvwaddnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwaddstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvwchgat(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: attr_t,
        arg6: c_short,
        arg7: *const c_void,
    ) -> c_int;
}
extern "C" {
    pub fn mvwdelch(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwgetch(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwgetnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut c_char,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwgetstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvwhline(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: chtype,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwin(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinch(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> chtype;
}
extern "C" {
    pub fn mvwinchnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut chtype,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinchstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut c_char,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinsch(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: chtype,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinsnstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinsstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvwinstr(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn mvwprintw(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn mvwscanw(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn mvwvline(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: chtype,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn napms(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn newpad(arg1: c_int, arg2: c_int) -> *mut WINDOW;
}
extern "C" {
    pub fn newterm(
        arg1: *const c_char,
        arg2: *mut FILE,
        arg3: *mut FILE,
    ) -> *mut SCREEN;
}
extern "C" {
    pub fn newwin(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn nl() -> c_int;
}
extern "C" {
    pub fn nocbreak() -> c_int;
}
extern "C" {
    pub fn nodelay(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn noecho() -> c_int;
}
extern "C" {
    pub fn nonl() -> c_int;
}
extern "C" {
    pub fn noqiflush();
}
extern "C" {
    pub fn noraw() -> c_int;
}
extern "C" {
    pub fn notimeout(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn overlay(arg1: *const WINDOW, arg2: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn overwrite(arg1: *const WINDOW, arg2: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn pair_content(
        arg1: c_short,
        arg2: *mut c_short,
        arg3: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn PAIR_NUMBER(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn pechochar(arg1: *mut WINDOW, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn pnoutrefresh(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn prefresh(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn printw(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn putwin(arg1: *mut WINDOW, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn qiflush();
}
extern "C" {
    pub fn raw() -> c_int;
}
extern "C" {
    pub fn redrawwin(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn refresh() -> c_int;
}
extern "C" {
    pub fn resetty() -> c_int;
}
extern "C" {
    pub fn reset_prog_mode() -> c_int;
}
extern "C" {
    pub fn reset_shell_mode() -> c_int;
}
extern "C" {
    pub fn ripoffline(
        arg1: c_int,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut WINDOW,
                arg2: c_int,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn savetty() -> c_int;
}
extern "C" {
    pub fn scanw(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn scr_dump(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn scr_init(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn scrl(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn scroll(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn scrollok(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn scr_restore(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn scr_set(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn setscrreg(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn set_term(arg1: *mut SCREEN) -> *mut SCREEN;
}
extern "C" {
    pub fn slk_attroff(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attr_off(arg1: attr_t, arg2: *mut c_void) -> c_int;
}
extern "C" {
    pub fn slk_attron(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attr_on(arg1: attr_t, arg2: *mut c_void) -> c_int;
}
extern "C" {
    pub fn slk_attrset(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attr() -> attr_t;
}
extern "C" {
    pub fn slk_attr_set(
        arg1: attr_t,
        arg2: c_short,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn slk_clear() -> c_int;
}
extern "C" {
    pub fn slk_color(arg1: c_short) -> c_int;
}
extern "C" {
    pub fn slk_init(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn slk_label(arg1: c_int) -> *mut c_char;
}
extern "C" {
    pub fn slk_noutrefresh() -> c_int;
}
extern "C" {
    pub fn slk_refresh() -> c_int;
}
extern "C" {
    pub fn slk_restore() -> c_int;
}
extern "C" {
    pub fn slk_set(
        arg1: c_int,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn slk_touch() -> c_int;
}
extern "C" {
    pub fn standout() -> c_int;
}
extern "C" {
    pub fn standend() -> c_int;
}
extern "C" {
    pub fn start_color() -> c_int;
}
extern "C" {
    pub fn subpad(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn subwin(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn syncok(arg1: *mut WINDOW, arg2: bool) -> c_int;
}
extern "C" {
    pub fn termattrs() -> chtype;
}
extern "C" {
    pub fn termname() -> *mut c_char;
}
extern "C" {
    pub fn timeout(arg1: c_int);
}
extern "C" {
    pub fn touchline(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn touchwin(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn typeahead(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn ungetch(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn untouchwin(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn use_env(arg1: bool);
}
extern "C" {
    pub fn use_tioctl(arg1: bool);
}
extern "C" {
    pub fn vidattr(arg1: chtype) -> c_int;
}
extern "C" {
    pub fn vidputs(arg1: chtype, arg2: NCURSES_OUTC) -> c_int;
}
extern "C" {
    pub fn vline(arg1: chtype, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn vwprintw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vw_printw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vwscanw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vw_scanw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn waddch(arg1: *mut WINDOW, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn waddchnstr(
        arg1: *mut WINDOW,
        arg2: *const chtype,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn waddchstr(arg1: *mut WINDOW, arg2: *const chtype) -> c_int;
}
extern "C" {
    pub fn waddnstr(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn waddstr(arg1: *mut WINDOW, arg2: *const c_char)
        -> c_int;
}
extern "C" {
    pub fn wattron(arg1: *mut WINDOW, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn wattroff(arg1: *mut WINDOW, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn wattrset(arg1: *mut WINDOW, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn wattr_get(
        arg1: *mut WINDOW,
        arg2: *mut attr_t,
        arg3: *mut c_short,
        arg4: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wattr_on(
        arg1: *mut WINDOW,
        arg2: attr_t,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wattr_off(
        arg1: *mut WINDOW,
        arg2: attr_t,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wattr_set(
        arg1: *mut WINDOW,
        arg2: attr_t,
        arg3: c_short,
        arg4: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wbkgd(arg1: *mut WINDOW, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn wbkgdset(arg1: *mut WINDOW, arg2: chtype);
}
extern "C" {
    pub fn wborder(
        arg1: *mut WINDOW,
        arg2: chtype,
        arg3: chtype,
        arg4: chtype,
        arg5: chtype,
        arg6: chtype,
        arg7: chtype,
        arg8: chtype,
        arg9: chtype,
    ) -> c_int;
}
extern "C" {
    pub fn wchgat(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: attr_t,
        arg4: c_short,
        arg5: *const c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wclear(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wclrtobot(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wclrtoeol(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wcolor_set(
        arg1: *mut WINDOW,
        arg2: c_short,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wcursyncup(arg1: *mut WINDOW);
}
extern "C" {
    pub fn wdelch(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wdeleteln(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wechochar(arg1: *mut WINDOW, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn werase(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wgetch(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wgetnstr(
        arg1: *mut WINDOW,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wgetstr(arg1: *mut WINDOW, arg2: *mut c_char) -> c_int;
}
extern "C" {
    pub fn whline(
        arg1: *mut WINDOW,
        arg2: chtype,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn winch(arg1: *mut WINDOW) -> chtype;
}
extern "C" {
    pub fn winchnstr(
        arg1: *mut WINDOW,
        arg2: *mut chtype,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn winchstr(arg1: *mut WINDOW, arg2: *mut chtype) -> c_int;
}
extern "C" {
    pub fn winnstr(
        arg1: *mut WINDOW,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn winsch(arg1: *mut WINDOW, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn winsdelln(arg1: *mut WINDOW, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn winsertln(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn winsnstr(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn winsstr(arg1: *mut WINDOW, arg2: *const c_char)
        -> c_int;
}
extern "C" {
    pub fn winstr(arg1: *mut WINDOW, arg2: *mut c_char) -> c_int;
}
extern "C" {
    pub fn wmove(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wnoutrefresh(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wprintw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn wredrawln(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wrefresh(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wscanw(
        arg1: *mut WINDOW,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn wscrl(arg1: *mut WINDOW, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn wsetscrreg(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wstandout(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wstandend(arg1: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn wsyncdown(arg1: *mut WINDOW);
}
extern "C" {
    pub fn wsyncup(arg1: *mut WINDOW);
}
extern "C" {
    pub fn wtimeout(arg1: *mut WINDOW, arg2: c_int);
}
extern "C" {
    pub fn wtouchln(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wvline(
        arg1: *mut WINDOW,
        arg2: chtype,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tigetflag(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn tigetnum(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn tigetstr(arg1: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn putp(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn tparm(arg1: *const c_char, ...) -> *mut c_char;
}
extern "C" {
    pub fn tiparm(arg1: *const c_char, ...) -> *mut c_char;
}
extern "C" {
    pub fn getattrs(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getcurx(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getcury(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getbegx(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getbegy(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getmaxx(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getmaxy(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getparx(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn getpary(arg1: *const WINDOW) -> c_int;
}
pub type NCURSES_WINDOW_CB = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut WINDOW,
        arg2: *mut c_void,
    ) -> c_int,
>;
pub type NCURSES_SCREEN_CB = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut SCREEN,
        arg2: *mut c_void,
    ) -> c_int,
>;
extern "C" {
    pub fn is_term_resized(arg1: c_int, arg2: c_int) -> bool;
}
extern "C" {
    pub fn keybound(
        arg1: c_int,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn curses_version() -> *const c_char;
}
extern "C" {
    pub fn alloc_pair(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn assume_default_colors(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn define_key(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_color_content(
        arg1: c_int,
        arg2: *mut c_int,
        arg3: *mut c_int,
        arg4: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_pair_content(
        arg1: c_int,
        arg2: *mut c_int,
        arg3: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_slk_color(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn find_pair(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn free_pair(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn get_escdelay() -> c_int;
}
extern "C" {
    pub fn init_extended_color(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn init_extended_pair(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn key_defined(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn keyok(arg1: c_int, arg2: bool) -> c_int;
}
extern "C" {
    pub fn reset_color_pairs();
}
extern "C" {
    pub fn resize_term(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn resizeterm(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn set_escdelay(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn set_tabsize(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn use_default_colors() -> c_int;
}
extern "C" {
    pub fn use_extended_names(arg1: bool) -> c_int;
}
extern "C" {
    pub fn use_legacy_coding(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn use_screen(
        arg1: *mut SCREEN,
        arg2: NCURSES_SCREEN_CB,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn use_window(
        arg1: *mut WINDOW,
        arg2: NCURSES_WINDOW_CB,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn wresize(
        arg1: *mut WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn nofilter();
}
extern "C" {
    pub fn wgetparent(arg1: *const WINDOW) -> *mut WINDOW;
}
extern "C" {
    pub fn is_cleared(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_idcok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_idlok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_immedok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_keypad(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_leaveok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_nodelay(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_notimeout(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_pad(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_scrollok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_subwin(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn is_syncok(arg1: *const WINDOW) -> bool;
}
extern "C" {
    pub fn wgetdelay(arg1: *const WINDOW) -> c_int;
}
extern "C" {
    pub fn wgetscrreg(
        arg1: *const WINDOW,
        arg2: *mut c_int,
        arg3: *mut c_int,
    ) -> c_int;
}
pub type NCURSES_OUTC_sp = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SCREEN, arg2: c_int) -> c_int,
>;
extern "C" {
    pub fn new_prescr() -> *mut SCREEN;
}
extern "C" {
    pub fn baudrate_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn beep_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn can_change_color_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn cbreak_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn curs_set_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn color_content_sp(
        arg1: *mut SCREEN,
        arg2: c_short,
        arg3: *mut c_short,
        arg4: *mut c_short,
        arg5: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn def_prog_mode_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn def_shell_mode_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn delay_output_sp(arg1: *mut SCREEN, arg2: c_int)
        -> c_int;
}
extern "C" {
    pub fn doupdate_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn echo_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn endwin_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn erasechar_sp(arg1: *mut SCREEN) -> c_char;
}
extern "C" {
    pub fn filter_sp(arg1: *mut SCREEN);
}
extern "C" {
    pub fn flash_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn flushinp_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn getwin_sp(arg1: *mut SCREEN, arg2: *mut FILE) -> *mut WINDOW;
}
extern "C" {
    pub fn halfdelay_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn has_colors_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn has_ic_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn has_il_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn init_color_sp(
        arg1: *mut SCREEN,
        arg2: c_short,
        arg3: c_short,
        arg4: c_short,
        arg5: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn init_pair_sp(
        arg1: *mut SCREEN,
        arg2: c_short,
        arg3: c_short,
        arg4: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn intrflush_sp(arg1: *mut SCREEN, arg2: *mut WINDOW, arg3: bool) -> c_int;
}
extern "C" {
    pub fn isendwin_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn keyname_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
    ) -> *const c_char;
}
extern "C" {
    pub fn killchar_sp(arg1: *mut SCREEN) -> c_char;
}
extern "C" {
    pub fn longname_sp(arg1: *mut SCREEN) -> *mut c_char;
}
extern "C" {
    pub fn mvcur_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn napms_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn newpad_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn newterm_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
        arg3: *mut FILE,
        arg4: *mut FILE,
    ) -> *mut SCREEN;
}
extern "C" {
    pub fn newwin_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> *mut WINDOW;
}
extern "C" {
    pub fn nl_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn nocbreak_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn noecho_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn nonl_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn noqiflush_sp(arg1: *mut SCREEN);
}
extern "C" {
    pub fn noraw_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn pair_content_sp(
        arg1: *mut SCREEN,
        arg2: c_short,
        arg3: *mut c_short,
        arg4: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn qiflush_sp(arg1: *mut SCREEN);
}
extern "C" {
    pub fn raw_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn reset_prog_mode_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn reset_shell_mode_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn resetty_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn ripoffline_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut WINDOW,
                arg2: c_int,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn savetty_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn scr_init_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn scr_restore_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn scr_set_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn slk_attroff_sp(arg1: *mut SCREEN, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attron_sp(arg1: *mut SCREEN, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attrset_sp(arg1: *mut SCREEN, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn slk_attr_sp(arg1: *mut SCREEN) -> attr_t;
}
extern "C" {
    pub fn slk_attr_set_sp(
        arg1: *mut SCREEN,
        arg2: attr_t,
        arg3: c_short,
        arg4: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn slk_clear_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn slk_color_sp(arg1: *mut SCREEN, arg2: c_short) -> c_int;
}
extern "C" {
    pub fn slk_init_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn slk_label_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn slk_noutrefresh_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn slk_refresh_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn slk_restore_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn slk_set_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn slk_touch_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn start_color_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn termattrs_sp(arg1: *mut SCREEN) -> chtype;
}
extern "C" {
    pub fn termname_sp(arg1: *mut SCREEN) -> *mut c_char;
}
extern "C" {
    pub fn typeahead_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn ungetch_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn use_env_sp(arg1: *mut SCREEN, arg2: bool);
}
extern "C" {
    pub fn use_tioctl_sp(arg1: *mut SCREEN, arg2: bool);
}
extern "C" {
    pub fn vidattr_sp(arg1: *mut SCREEN, arg2: chtype) -> c_int;
}
extern "C" {
    pub fn vidputs_sp(
        arg1: *mut SCREEN,
        arg2: chtype,
        arg3: NCURSES_OUTC_sp,
    ) -> c_int;
}
extern "C" {
    pub fn keybound_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn alloc_pair_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn assume_default_colors_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn define_key_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_color_content_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: *mut c_int,
        arg4: *mut c_int,
        arg5: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_pair_content_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: *mut c_int,
        arg4: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn extended_slk_color_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn get_escdelay_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn find_pair_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn free_pair_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn init_extended_color_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn init_extended_pair_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn is_term_resized_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> bool;
}
extern "C" {
    pub fn key_defined_sp(
        arg1: *mut SCREEN,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn keyok_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: bool,
    ) -> c_int;
}
extern "C" {
    pub fn nofilter_sp(arg1: *mut SCREEN);
}
extern "C" {
    pub fn reset_color_pairs_sp(arg1: *mut SCREEN);
}
extern "C" {
    pub fn resize_term_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn resizeterm_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn set_escdelay_sp(arg1: *mut SCREEN, arg2: c_int)
        -> c_int;
}
extern "C" {
    pub fn set_tabsize_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn use_default_colors_sp(arg1: *mut SCREEN) -> c_int;
}
extern "C" {
    pub fn use_legacy_coding_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub static mut curscr: *mut WINDOW;
}
extern "C" {
    pub static mut newscr: *mut WINDOW;
}
extern "C" {
    pub static mut stdscr: *mut WINDOW;
}
extern "C" {
    pub static mut ttytype: [c_char; 0usize];
}
extern "C" {
    pub static mut COLORS: c_int;
}
extern "C" {
    pub static mut COLOR_PAIRS: c_int;
}
extern "C" {
    pub static mut COLS: c_int;
}
extern "C" {
    pub static mut ESCDELAY: c_int;
}
extern "C" {
    pub static mut LINES: c_int;
}
extern "C" {
    pub static mut TABSIZE: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MEVENT {
    pub id: c_short,
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub bstate: mmask_t,
}
extern "C" {
    pub fn has_mouse() -> bool;
}
extern "C" {
    pub fn getmouse(arg1: *mut MEVENT) -> c_int;
}
extern "C" {
    pub fn ungetmouse(arg1: *mut MEVENT) -> c_int;
}
extern "C" {
    pub fn mousemask(arg1: mmask_t, arg2: *mut mmask_t) -> mmask_t;
}
extern "C" {
    pub fn wenclose(
        arg1: *const WINDOW,
        arg2: c_int,
        arg3: c_int,
    ) -> bool;
}
extern "C" {
    pub fn mouseinterval(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn wmouse_trafo(
        arg1: *const WINDOW,
        arg2: *mut c_int,
        arg3: *mut c_int,
        arg4: bool,
    ) -> bool;
}
extern "C" {
    pub fn mouse_trafo(
        arg1: *mut c_int,
        arg2: *mut c_int,
        arg3: bool,
    ) -> bool;
}
extern "C" {
    pub fn has_mouse_sp(arg1: *mut SCREEN) -> bool;
}
extern "C" {
    pub fn getmouse_sp(arg1: *mut SCREEN, arg2: *mut MEVENT) -> c_int;
}
extern "C" {
    pub fn ungetmouse_sp(arg1: *mut SCREEN, arg2: *mut MEVENT) -> c_int;
}
extern "C" {
    pub fn mousemask_sp(arg1: *mut SCREEN, arg2: mmask_t, arg3: *mut mmask_t) -> mmask_t;
}
extern "C" {
    pub fn mouseinterval_sp(
        arg1: *mut SCREEN,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mcprint(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn has_key(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn has_key_sp(arg1: *mut SCREEN, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn mcprint_sp(
        arg1: *mut SCREEN,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn _tracef(arg1: *const c_char, ...);
}
extern "C" {
    pub fn _traceattr(arg1: attr_t) -> *mut c_char;
}
extern "C" {
    pub fn _traceattr2(arg1: c_int, arg2: chtype) -> *mut c_char;
}
extern "C" {
    pub fn _tracechar(arg1: c_int) -> *mut c_char;
}
extern "C" {
    pub fn _tracechtype(arg1: chtype) -> *mut c_char;
}
extern "C" {
    pub fn _tracechtype2(arg1: c_int, arg2: chtype) -> *mut c_char;
}
extern "C" {
    pub fn trace(arg1: c_uint);
}
extern "C" {
    pub fn curses_trace(arg1: c_uint) -> c_uint;
}
extern "C" {
    pub fn exit_curses(arg1: c_int);
}
extern "C" {
    pub fn unctrl(arg1: chtype) -> *const c_char;
}
extern "C" {
    pub fn unctrl_sp(arg1: *mut SCREEN, arg2: chtype) -> *const c_char;
}
