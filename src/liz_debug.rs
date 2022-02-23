use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::LizError;

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Acquire)
}

pub fn set_verbose(to: bool) {
    VERBOSE.store(to, Ordering::Release)
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

pub fn throw(message: String) -> Box<MessageErr> {
    if is_verbose() {
        println!("[ERR] {}", &message);
    }
    Box::new(MessageErr::of(message))
}

pub fn debug_err(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> LizError {
    throw(debug_msg(file, line, func, vals, err))
}

pub fn debug_msg(file: &str, line: u32, func: &str, vals: String, msg: impl Display) -> String {
    if vals.is_empty() {
        format!("{} on ({}) in {}[{}]", msg, func, file, line)
    } else {
        format!("{} of {} on ({}) in {}[{}]", msg, vals, func, file, line)
    }
}

macro_rules! dbg_fnc {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

macro_rules! dbg_fmt {
    () => (String::default());
    ($v:expr) => (format!("{} = {:?}", stringify!($v), $v));
    ($v:expr, $($n:expr),+) => (format!("{} = {:?}, {}", stringify!($v), $v, crate::liz_debug::dbg_fmt!($($n),+)));
}

macro_rules! dbg_err {
    ($err:expr) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_err(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $err)
    );
}

macro_rules! dbg_knd {
    ($kind:expr, $msg:expr) => (
        if crate::liz_debug::is_verbose() {
            println!("[{}] {}", $kind, crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg))
        }
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        if crate::liz_debug::is_verbose() {
            println!("[{}] {}", $kind, crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg))
        }
    );
}

macro_rules! dbg_inf {
    ($msg:expr) => (
        if crate::liz_debug::is_verbose() {
            println!("[INF] {}", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg))
        }
    );
    ($msg:expr, $($v:expr),+) => (
        if crate::liz_debug::is_verbose() {
            println!("[INF] {}", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg))
        }
    );
}

macro_rules! dbg_cfg {
    ($msg:expr) => (
        #[cfg(debug_assertions)]
        println!("[CFG] {}", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!(), $msg))
    );
    ($msg:expr, $($v:expr),+) => (
        #[cfg(debug_assertions)]
        println!("[CFG] {}", crate::liz_debug::debug_msg(file!(), line!(), crate::liz_debug::dbg_fnc!(), crate::liz_debug::dbg_fmt!($($v),+), $msg))
    );
}

pub(crate) use dbg_cfg;
pub(crate) use dbg_err;
pub(crate) use dbg_fmt;
pub(crate) use dbg_fnc;
pub(crate) use dbg_inf;
pub(crate) use dbg_knd;

#[macro_export]
macro_rules! liz_dbg_fnc {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

#[macro_export]
macro_rules! liz_dbg_vls {
    () => (String::default());
    ($v:expr) => (format!("{} = {:?}", stringify!($v), $v));
    ($v:expr, $($n:expr),+) => (format!("{} = {:?}, {}", stringify!($v), $v, liz::liz_dbg_vls!($($n),+)));
}

#[macro_export]
macro_rules! liz_dbg_trw {
    ($err:expr) => (
        liz::liz_debug::throw(liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!(), $err))
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::throw(liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!($($v),+), $err))
    );
}

#[macro_export]
macro_rules! liz_dbg_err {
    ($err:expr) => (
        liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! liz_dbg_knd {
    ($kind:expr, $msg:expr) => (
        if liz::liz_debug::is_verbose() {
            println!("[{}] {}", $kind, liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!(), $msg))
        }
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        if liz::liz_debug::is_verbose() {
            println!("[{}] {}", $kind, liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!($($v),+), $msg))
        }
    );
}

#[macro_export]
macro_rules! liz_dbg_inf {
    ($msg:expr) => (
        if liz::liz_debug::is_verbose() {
            println!("[INF] {}", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!(), $msg))
        }
    );
    ($msg:expr, $($v:expr),+) => (
        if liz::liz_debug::is_verbose() {
            println!("[INF] {}", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!($($v),+), $msg))
        }
    );
}

#[macro_export]
macro_rules! liz_dbg_cfg {
    ($msg:expr) => (
        #[cfg(debug_assertions)]
        println!("[CFG] {}", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!(), $msg))
    );
    ($msg:expr, $($v:expr),+) => (
        #[cfg(debug_assertions)]
        println!("[CFG] {}", liz::liz_debug::debug_msg(file!(), line!(), liz::liz_dbg_fnc!(), liz::liz_dbg_vls!($($v),+), $msg))
    );
}
