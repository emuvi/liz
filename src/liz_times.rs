use chrono::offset::Utc;
use once_cell::sync::Lazy;

use std::sync::RwLock;

use crate::liz_debug::dbg_stp;

static UNIQUE_REAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";
static UNIQUE_LAST_FORMAT: &str = "%H:%M:%S%.3f";
static UNIQUE_WHIM_FORMAT: &str = "%H:%M:%S%.6f";

static UNIQUE_DATE_FORMAT: &str = "%Y-%m-%d";
static UNIQUE_TIME_FORMAT: &str = "%H:%M:%S";
static UNIQUE_SEAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

static AREALY_DATE_FORMAT: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from("%d/%m/%Y")));
static AREALY_TIME_FORMAT: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from("%H:%M:%S")));
static AREALY_SEAL_FORMAT: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from("%d/%m/%Y %H:%M:%S")));

pub fn now() -> String {
    dbg_stp!();
    now_ur()
}

pub fn now_ur() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_REAL_FORMAT))
}

pub fn now_ul() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_LAST_FORMAT))
}

pub fn now_uw() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_WHIM_FORMAT))
}

pub fn now_ud() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_DATE_FORMAT))
}

pub fn now_ut() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_TIME_FORMAT))
}

pub fn now_us() -> String {
    dbg_stp!();
    format!("{}", Utc::now().format(UNIQUE_SEAL_FORMAT))
}

pub fn now_ad() -> String {
    dbg_stp!();
    format!(
        "{}",
        Utc::now().format(&*AREALY_DATE_FORMAT.read().unwrap())
    )
}

pub fn now_at() -> String {
    dbg_stp!();
    format!(
        "{}",
        Utc::now().format(&*AREALY_TIME_FORMAT.read().unwrap())
    )
}

pub fn now_as() -> String {
    dbg_stp!();
    format!(
        "{}",
        Utc::now().format(&*AREALY_SEAL_FORMAT.read().unwrap())
    )
}

pub fn now_ft(format: &str) -> String {
    dbg_stp!(format);
    format!("{}", Utc::now().format(format))
}
