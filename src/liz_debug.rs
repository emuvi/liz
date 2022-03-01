use once_cell::sync::Lazy;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Mutex,
};

use crate::liz_times;
use crate::LizError;

static VERBOSE: AtomicBool = AtomicBool::new(false);
static ARCHIVE: AtomicBool = AtomicBool::new(false);
static ARCFILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(File::create("archive.log").unwrap()));
static DBGSIZE: AtomicUsize = AtomicUsize::new(1);

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Acquire)
}

pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::Release);
    if is_verbose() {
        dbg_inf!("Verbose started");
    }
}

pub fn is_archive() -> bool {
    ARCHIVE.load(Ordering::Acquire)
}

pub fn set_archive(archive: bool) {
    ARCHIVE.store(archive, Ordering::Release);
    if is_archive() {
        dbg_inf!("Archive started");
    }
}

pub fn get_dbg_size() -> usize {
    DBGSIZE.load(Ordering::Acquire)
}

pub fn set_dbg_size(size: usize) {
    DBGSIZE.store(size, Ordering::Release)
}

pub fn debug(message: impl AsRef<str>) {
    if is_verbose() {
        println!(
            "({}) {}",
            std::thread::current().name().unwrap_or(""),
            message.as_ref()
        );
    }
    if is_archive() {
        let mut file = ARCFILE.lock().unwrap();
        writeln!(
            file,
            "{} ({}) {}",
            liz_times::now(),
            std::thread::current().name().unwrap_or(""),
            message.as_ref()
        )
        .unwrap();
    }
}

pub fn wrong(message: String) -> Box<MessageErr> {
    Box::new(MessageErr::of(message))
}

pub fn throw(message: String) -> Box<MessageErr> {
    Box::new(MessageErr::of(message))
}

pub fn debug_inf(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> String {
    debug_make("INFO", file, line, func, vals, err)
}

pub fn debug_ebb(file: &str, line: u32, func: &str, vals: String, err: LizError) -> LizError {
    let from = format!("{}", err);
    let from = if let Some(pos) = from.rfind(" on (") {
        &from[pos + 4..]
    } else {
        ""
    };
    let from = format!("[BLEB] from {}", from);
    debug_make("ERRO", file, line, func, vals, from);
    err
}

pub fn debug_err(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> LizError {
    throw(debug_make("ERRO", file, line, func, vals, err))
}

pub fn debug_trw(
    kind: &str,
    file: &str,
    line: u32,
    func: &str,
    vals: String,
    err: impl Display,
) -> LizError {
    throw(debug_make(kind, file, line, func, vals, err))
}

pub fn debug_knd(
    kind: &str,
    file: &str,
    line: u32,
    func: &str,
    vals: String,
    err: impl Display,
) -> String {
    debug_make(kind, file, line, func, vals, err)
}

pub fn debug_call(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 1 {
        debug_make("DBUG", file, line, func, vals, "[CALL]");
    }
}

pub fn debug_reav(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 2 {
        debug_make("DBUG", file, line, func, vals, "[REAV]");
    }
}

pub fn debug_step(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 3 {
        debug_make("DBUG", file, line, func, vals, "[STEP]");
    }
}

pub fn debug_make(
    kind: &str,
    file: &str,
    line: u32,
    func: &str,
    vals: String,
    msg: impl Display,
) -> String {
    let message = if vals.is_empty() {
        format!("[{}] {} on ({}) in {}[{}]", kind, msg, func, file, line)
    } else {
        format!(
            "[{}] {} as {{{}}} on ({}) in {}[{}]",
            kind, msg, vals, func, file, line
        )
    };
    debug(&message);
    message
}

macro_rules! dbg_fnc {
    () => {{
        fn f() {}
        let name = crate::liz_debug::dbg_fnm!(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

macro_rules! dbg_fnm {
    ($val:expr) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($val)
    }};
}

macro_rules! dbg_fvl {
    ($v:expr) => {{
        let mut value = format!("{:?}", $v);
        if value.len() > 300 {
            let mut end = 300;
            while !value.is_char_boundary(end) {
                end += 1;
            }
            value.truncate(end);
        }
        value
    }};
}

macro_rules! dbg_fmr {
    () => (String::default());
    ($v:expr) => (format!("result = {}", crate::liz_debug::dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("result = {} , {}", crate::liz_debug::dbg_fvl!(&$v), crate::liz_debug::dbg_fmr!($($n),+)));
}

macro_rules! dbg_fmt {
    () => (String::default());
    ($v:expr) => (format!("{} = {}", stringify!($v), crate::liz_debug::dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {} , {}", stringify!($v), crate::liz_debug::dbg_fvl!(&$v), crate::liz_debug::dbg_fmt!($($n),+)));
}

macro_rules! dbg_inf {
    ($err:expr) => (
        crate::liz_debug::debug_inf(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_inf(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_err {
    ($err:expr) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_ebb {
    ($err:expr) => (
        crate::liz_debug::debug_ebb(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_ebb(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_trw {
    ($kind:expr, $msg:expr) => (
        crate::liz_debug::debug_trw($kind, file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug_trw($kind, file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg)
    );
}

macro_rules! dbg_knd {
    ($kind:expr, $msg:expr) => (
        crate::liz_debug::debug_knd($kind, file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug_knd($kind, file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg)
    );
}

macro_rules! dbg_call {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_call(file!(), line!(), crate::liz_debug::dbg_fnc!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_call(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+))
    );
}

macro_rules! dbg_reav {
    ($xp:expr) => {{
        let result = $xp;
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_reav(
            file!(),
            line!(),
            crate::liz_debug::dbg_fnc!(),
            crate::liz_debug::dbg_fmr!(result),
        );
        return result;
    }};
}

macro_rules! dbg_step {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_step(file!(), line!(), crate::liz_debug::dbg_fnc!(), String::default())
    );
    ($($v:ident),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_step(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+))
    );
}

pub(crate) use {dbg_call, dbg_reav, dbg_step};
pub(crate) use {dbg_ebb, dbg_err, dbg_inf, dbg_knd, dbg_trw};
pub(crate) use {dbg_fmr, dbg_fmt, dbg_fnc, dbg_fnm, dbg_fvl};

#[macro_export]
macro_rules! liz_dbg_fnc {
    () => {{
        fn f() {}
        let name = liz::liz_dbg_fnm!(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

#[macro_export]
macro_rules! liz_dbg_fnm {
    ($val:expr) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($val)
    }};
}

#[macro_export]
macro_rules! liz_dbg_fvl {
    ($v:expr) => {{
        let mut value = format!("{:?}", $v);
        if value.len() > 300 {
            let mut end = 300;
            while !value.is_char_boundary(end) {
                end += 1;
            }
            value.truncate(end);
            value.push_str("...");
        }
        value
    }};
}

#[macro_export]
macro_rules! liz_dbg_fmr {
    () => (String::default());
    ($v:expr) => (format!("result = {}", liz::liz_dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("result = {} , {}", liz::liz_dbg_fvl!(&$v), liz::liz_dbg_fmr!($($n),+)));
}

#[macro_export]
macro_rules! liz_dbg_fmt {
    () => (String::default());
    ($v:expr) => (format!("{} = {}", stringify!($v), liz::liz_dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {} , {}", stringify!($v), liz::liz_dbg_fvl!(&$v), liz::liz_dbg_fmt!($($n),+)));
}

#[macro_export]
macro_rules! liz_dbg_inf {
    ($msg:expr) => (
        liz::liz_debug::debug_inf(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_inf(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_ebb {
    ($err:expr) => (
        liz::liz_debug::debug_ebb(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::debug_ebb(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! liz_dbg_err {
    ($msg:expr) => (
        liz::liz_debug::debug_err(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_err(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_trw {
    ($kind:expr, $msg:expr) => (
        liz::liz_debug::debug_thw($kind, file!(), line!(), liz::liz_debug::dbg_fnc!(), liz::liz_debug::dbg_fmt!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_thw($kind, file!(), line!(), liz::liz_debug::dbg_fnc!(), liz::liz_debug::dbg_fmt!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_knd {
    ($kind:expr, $msg:expr) => (
        liz::liz_debug::debug_knd($kind, file!(), line!(), liz::liz_debug::dbg_fnc!(), liz::liz_debug::dbg_fmt!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_knd($kind, file!(), line!(), liz::liz_debug::dbg_fnc!(), liz::liz_debug::dbg_fmt!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_call {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_call(file!(), line!(), liz::liz_dbg_fnc!(), String::default())
    );
    ($($v:ident),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_call(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+))
    );
}

#[macro_export]
macro_rules! liz_dbg_reav {
    ($xp:expr) => {{
        let result = $xp;
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_reav(
            file!(),
            line!(),
            liz::liz_dbg_fnc!(),
            liz::liz_dbg_fmr!(result),
        );
        return result;
    }};
}

#[macro_export]
macro_rules! liz_dbg_step {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_step(file!(), line!(), liz::liz_dbg_fnc!(), String::default())
    );
    ($($v:ident),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_step(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+))
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageErr {
    body: String,
}

impl MessageErr {
    #[inline]
    pub fn of(s: String) -> MessageErr {
        MessageErr { body: s }
    }

    #[inline]
    pub fn new<S: Into<String>>(s: S) -> MessageErr {
        MessageErr { body: s.into() }
    }

    #[inline]
    pub fn from<E: Error>(e: E) -> MessageErr {
        MessageErr {
            body: format!("{}", e),
        }
    }

    #[inline]
    pub fn with<E: Error>(s: &str, e: E) -> MessageErr {
        MessageErr {
            body: format!("{}, {}", s, e),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.body
    }
}

impl Display for MessageErr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.body.fmt(f)
    }
}

impl Error for MessageErr {
    #[inline]
    fn description(&self) -> &str {
        &self.body
    }
}
