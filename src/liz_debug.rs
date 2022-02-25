use once_cell::sync::Lazy;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use crate::liz_times;
use crate::LizError;

static VERBOSE: AtomicBool = AtomicBool::new(false);
static ARCHIVE: AtomicBool = AtomicBool::new(false);
static ARCFILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(File::create("archive.log").unwrap()));

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Acquire)
}

pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::Release);
    if is_verbose() {
        debug("INFO", "Verbose started");
    }
}

pub fn is_archive() -> bool {
    ARCHIVE.load(Ordering::Acquire)
}

pub fn set_archive(archive: bool) {
    ARCHIVE.store(archive, Ordering::Release);
    if is_archive() {
        debug("INFO", "Archive started");
    }
}

pub fn debug(kind: impl AsRef<str>, message: impl AsRef<str>) {
    if is_verbose() {
        println!(
            "[{}] ({}) {}",
            kind.as_ref(),
            std::thread::current().name().unwrap_or(""),
            message.as_ref()
        );
    }
    if is_archive() {
        let mut file = ARCFILE.lock().unwrap();
        writeln!(
            file,
            "{} [{}] ({}) {}",
            liz_times::now(),
            kind.as_ref(),
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
    debug("ERRO", &message);
    Box::new(MessageErr::of(message))
}

pub fn debug_err(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> LizError {
    throw(debug_msg(file, line, func, vals, err))
}

pub fn debug_bub(file: &str, line: u32, func: &str, vals: String, err: LizError) -> LizError {
    let from = format!("{}", err);
    let from = if let Some(pos) = from.rfind(" on (") {
        &from[pos + 4..]
    } else {
        ""
    };
    let from = format!("Bubbled from {}", from);
    debug("ERRO", debug_msg(file, line, func, vals, from));
    err
}

pub fn debug_msg(file: &str, line: u32, func: &str, vals: String, msg: impl Display) -> String {
    if vals.is_empty() {
        format!("{} on ({}) in {}[{}]", msg, func, file, line)
    } else {
        format!(
            "{} as {{{}}} on ({}) in {}[{}]",
            msg, vals, func, file, line
        )
    }
}

pub fn debug_stp(file: &str, line: u32, func: &str, vals: String) -> String {
    if vals.is_empty() {
        format!("[STEP] on ({}) in {}[{}]", func, file, line)
    } else {
        format!("[STEP] on ({}) in {}[{}] as {{{}}}", func, file, line, vals)
    }
}

macro_rules! dbg_fnm {
    ($val:expr) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($val)
    }};
}

macro_rules! dbg_fnc {
    () => {{
        fn f() {}
        let name = crate::liz_debug::dbg_fnm!(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
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

macro_rules! dbg_fmt {
    () => (String::default());
    ($v:expr) => (format!("{} = {}", stringify!($v), crate::liz_debug::dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {} , {}", stringify!($v), crate::liz_debug::dbg_fvl!(&$v), crate::liz_debug::dbg_fmt!($($n),+)));
}

macro_rules! dbg_err {
    ($err:expr) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_bub {
    ($err:expr) => (
        crate::liz_debug::debug_bub(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_bub(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_knd {
    ($kind:expr, $msg:expr) => (
        crate::liz_debug::debug($kind, crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg))
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug($kind, crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg))
    );
}

macro_rules! dbg_inf {
    ($msg:expr) => (
        crate::liz_debug::debug("INFO", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg))
    );
    ($msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug("INFO", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg))
    );
}

macro_rules! dbg_stp {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug("DBUG", crate::liz_debug::debug_stp(file!(), line!(), crate::liz_debug::dbg_fnc!(), String::default()))
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug("DBUG", crate::liz_debug::debug_stp(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+)))
    );
}

pub(crate) use dbg_bub;
pub(crate) use dbg_err;
pub(crate) use dbg_fmt;
pub(crate) use dbg_fnc;
pub(crate) use dbg_fnm;
pub(crate) use dbg_fvl;
pub(crate) use dbg_inf;
pub(crate) use dbg_knd;
pub(crate) use dbg_stp;

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
macro_rules! liz_dbg_fmt {
    () => (String::default());
    ($v:expr) => (format!("{} = {}", stringify!($v), liz::liz_dbg_fvl!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {} , {}", stringify!($v), liz::liz_dbg_fvl!(&$v), liz::liz_dbg_fmt!($($n),+)));
}

#[macro_export]
macro_rules! liz_dbg_trw {
    ($err:expr) => (
        liz::liz_debug::throw(liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $err))
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::throw(liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $err))
    );
}

#[macro_export]
macro_rules! liz_dbg_err {
    ($err:expr) => (
        liz::liz_debug::debug_err(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::debug_err(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! liz_dbg_bub {
    ($err:expr) => (
        liz::liz_debug::debug_bub(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::debug_bub(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! liz_dbg_knd {
    ($kind:expr, $msg:expr) => (
        liz::liz_debug::debug($kind, liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $msg))
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug($kind, liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $msg))
    );
}

#[macro_export]
macro_rules! liz_dbg_inf {
    ($msg:expr) => (
        liz::liz_debug::debug("INFO", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!(), $msg))
    );
    ($msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug("INFO", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+), $msg))
    );
}

#[macro_export]
macro_rules! liz_dbg_stp {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug("DBUG", liz::liz_debug::debug_stp(file!(), line!(), liz::liz_dbg_fnc!(), String::default()))
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug("DBUG", liz::liz_debug::debug_stp(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_fmt!($($v),+)))
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
