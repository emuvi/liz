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
        dbg_info!("Verbose started");
    }
}

pub fn is_archive() -> bool {
    ARCHIVE.load(Ordering::Acquire)
}

pub fn set_archive(archive: bool) {
    ARCHIVE.store(archive, Ordering::Release);
    if is_archive() {
        dbg_info!("Archive started");
    }
}

pub fn get_dbg_size() -> usize {
    DBGSIZE.load(Ordering::Acquire)
}

pub fn set_dbg_size(size: usize) {
    DBGSIZE.store(size, Ordering::Release)
}

pub fn set_dbg_calls() {
    set_dbg_size(1)
}

pub fn set_dbg_reavs() {
    set_dbg_size(2)
}

pub fn set_dbg_steps() {
    set_dbg_size(3)
}

pub fn set_dbg_tells() {
    set_dbg_size(4)
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

pub fn debug_info(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> String {
    debug_make("INFO", file, line, func, vals, err)
}

pub fn debug_bleb(file: &str, line: u32, func: &str, vals: String, err: LizError) -> LizError {
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

pub fn debug_erro(file: &str, line: u32, func: &str, vals: String, err: impl Display) -> LizError {
    throw(debug_make("ERRO", file, line, func, vals, err))
}

pub fn debug_jolt(
    kind: &str,
    file: &str,
    line: u32,
    func: &str,
    vals: String,
    err: impl Display,
) -> LizError {
    throw(debug_make(kind, file, line, func, vals, err))
}

pub fn debug_kind(
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

pub fn debug_seal(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 3 {
        debug_make("DBUG", file, line, func, vals, "[SEAL]");
    }
}

pub fn debug_step(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 3 {
        debug_make("DBUG", file, line, func, vals, "[STEP]");
    }
}

pub fn debug_tell(file: &str, line: u32, func: &str, vals: String) {
    if get_dbg_size() >= 4 {
        debug_make("DBUG", file, line, func, vals, "[TELL]");
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

macro_rules! dbg_func {
    () => {{
        fn f() {}
        let name = crate::liz_debug::dbg_fnam!(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

macro_rules! dbg_fnam {
    ($val:expr) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($val)
    }};
}

macro_rules! dbg_fval {
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

macro_rules! dbg_fmts {
    () => (String::default());
    ($v:expr) => (format!("{} = {{{}}}", stringify!($v), crate::liz_debug::dbg_fval!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {{{}}} , {}", stringify!($v), crate::liz_debug::dbg_fval!(&$v), crate::liz_debug::dbg_fmts!($($n),+)));
}

macro_rules! dbg_fmsn {
    () => {
        String::default()
    };
    ($v:expr) => {
        format!("{{{}}}", crate::liz_debug::dbg_fval!(&$v))
    };
}

macro_rules! dbg_info {
    ($err:expr) => (
        crate::liz_debug::debug_info(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_info(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+), $err)
    );
}

macro_rules! dbg_erro {
    ($err:expr) => (
        crate::liz_debug::debug_erro(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_erro(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+), $err)
    );
}

macro_rules! dbg_bleb {
    ($err:expr) => (
        crate::liz_debug::debug_bleb(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::liz_debug::debug_bleb(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+), $err)
    );
}

macro_rules! dbg_jolt {
    ($kind:expr, $msg:expr) => (
        crate::liz_debug::debug_jolt($kind, file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug_jolt($kind, file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+), $msg)
    );
}

macro_rules! dbg_kind {
    ($kind:expr, $msg:expr) => (
        crate::liz_debug::debug_kind($kind, file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::liz_debug::debug_kind($kind, file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+), $msg)
    );
}

macro_rules! dbg_call {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_call(file!(), line!(), crate::liz_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_call(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+))
    );
}

macro_rules! dbg_reav {
    ($xp:expr) => {{
        let result = $xp;
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_reav(
            file!(),
            line!(),
            crate::liz_debug::dbg_func!(),
            crate::liz_debug::dbg_fmsn!(result),
        );
        return result;
    }};
}

macro_rules! dbg_seal {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_seal(file!(), line!(), crate::liz_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_seal(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+))
    );
}

macro_rules! dbg_step {
    ($xp:expr) => {{
        let step = $xp;
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_step(
            file!(),
            line!(),
            crate::liz_debug::dbg_func!(),
            crate::liz_debug::dbg_fmsn!(step),
        );
        step
    }};
}

macro_rules! dbg_tell {
    () => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_tell(file!(), line!(), crate::liz_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::liz_debug::debug_tell(file!(), line!(), crate::liz_debug::dbg_func!(), crate::liz_debug::dbg_fmts!($($v),+))
    );
}

pub(crate) use {dbg_bleb, dbg_erro, dbg_info, dbg_jolt, dbg_kind};
pub(crate) use {dbg_call, dbg_reav, dbg_seal, dbg_step, dbg_tell};
pub(crate) use {dbg_fmsn, dbg_fmts, dbg_fnam, dbg_func, dbg_fval};

#[macro_export]
macro_rules! liz_dbg_func {
    () => {{
        fn f() {}
        let name = liz::liz_dbg_fnam!(f);
        &name[..name.len() - 3].trim_end_matches("::{{closure}}")
    }};
}

#[macro_export]
macro_rules! liz_dbg_fnam {
    ($val:expr) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($val)
    }};
}

#[macro_export]
macro_rules! liz_dbg_fval {
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
macro_rules! liz_dbg_fmts {
    () => (String::default());
    ($v:expr) => (format!("{} = {{{}}}", stringify!($v), liz::liz_dbg_fval!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{} = {{{}}} , {}", stringify!($v), liz::liz_dbg_fval!(&$v), liz::liz_dbg_fmts!($($n),+)));
}

#[macro_export]
macro_rules! liz_dbg_fmsn {
    () => {
        String::default()
    };
    ($v:expr) => {
        format!("{{{}}}", liz::liz_dbg_fval!(&$v))
    };
}

#[macro_export]
macro_rules! liz_dbg_info {
    ($msg:expr) => (
        liz::liz_debug::debug_info(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_info(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_bleb {
    ($err:expr) => (
        liz::liz_debug::debug_bleb(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        liz::liz_debug::debug_bleb(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! liz_dbg_erro {
    ($msg:expr) => (
        liz::liz_debug::debug_erro(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_erro(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_jolt {
    ($kind:expr, $msg:expr) => (
        liz::liz_debug::debug_jolt($kind, file!(), line!(), liz::liz_debug::dbg_func!(), liz::liz_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_jolt($kind, file!(), line!(), liz::liz_debug::dbg_func!(), liz::liz_debug::dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_kind {
    ($kind:expr, $msg:expr) => (
        liz::liz_debug::debug_kind($kind, file!(), line!(), liz::liz_debug::dbg_func!(), liz::liz_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        liz::liz_debug::debug_kind($kind, file!(), line!(), liz::liz_debug::dbg_func!(), liz::liz_debug::dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! liz_dbg_call {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_call(file!(), line!(), liz::liz_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_call(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+))
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
            liz::liz_dbg_func!(),
            liz::liz_dbg_fmsn!(result),
        );
        return result;
    }};
}

#[macro_export]
macro_rules! liz_dbg_seal {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_seal(file!(), line!(), liz::liz_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_seal(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+))
    );
}

#[macro_export]
macro_rules! liz_dbg_step {
    ($xp:expr) => {{
        let result = $xp;
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_step(
            file!(),
            line!(),
            liz::liz_dbg_func!(),
            liz::liz_dbg_fmsn!(result),
        );
        return result;
    }};
}

#[macro_export]
macro_rules! liz_dbg_tell {
    () => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_tell(file!(), line!(), liz::liz_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        liz::liz_debug::debug_tell(file!(), line!(), liz::liz_dbg_func!(), liz::liz_dbg_fmts!($($v),+))
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
