use chrono::offset::Utc;
use once_cell::sync::Lazy;

use std::sync::RwLock;

static UNIQUE_REAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";
static UNIQUE_LAST_FORMAT: &str = "%H:%M:%S%.3f";
static UNIQUE_WHIM_FORMAT: &str = "%H:%M:%S%.6f";

static UNIQUE_DATE_FORMAT: &str = "%Y-%m-%d";
static UNIQUE_TIME_FORMAT: &str = "%H:%M:%S";
static UNIQUE_SEAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

static AREALY_DATE_FORMAT: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("%d/%m/%Y")));
static AREALY_TIME_FORMAT: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("%H:%M:%S")));
static AREALY_SEAL_FORMAT: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("%d/%m/%Y %H:%M:%S")));

pub fn pnow() -> String {
    pnow_ur()
}

pub fn pnow_ur() -> String {
    format!("{}", Utc::now().format(UNIQUE_REAL_FORMAT))
}

pub fn pnow_ul() -> String {
    format!("{}", Utc::now().format(UNIQUE_LAST_FORMAT))
}

pub fn pnow_uw() -> String {
    format!("{}", Utc::now().format(UNIQUE_WHIM_FORMAT))
}

pub fn pnow_ud() -> String {
    format!("{}", Utc::now().format(UNIQUE_DATE_FORMAT))
}

pub fn pnow_ut() -> String {
    format!("{}", Utc::now().format(UNIQUE_TIME_FORMAT))
}

pub fn pnow_us() -> String {
    format!("{}", Utc::now().format(UNIQUE_SEAL_FORMAT))
}

pub fn pnow_ad() -> String {
    format!("{}", Utc::now().format(&*AREALY_DATE_FORMAT.read().unwrap()))
}

pub fn pnow_at() -> String {
    format!("{}", Utc::now().format(&*AREALY_TIME_FORMAT.read().unwrap()))
}

pub fn pnow_as() -> String {
    format!("{}", Utc::now().format(&*AREALY_SEAL_FORMAT.read().unwrap()))
}