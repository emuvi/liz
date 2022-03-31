use chrono::offset::Utc;
use chrono::DateTime;
use once_cell::sync::Lazy;

use crate::liz_debug::{dbg_call, dbg_reav};

pub static UNIQUE_REAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";
pub static UNIQUE_LAST_FORMAT: &str = "%H:%M:%S%.3f";
pub static UNIQUE_WHIM_FORMAT: &str = "%H:%M:%S%.6f";

pub static UNIQUE_DATE_FORMAT: &str = "%Y-%m-%d";
pub static UNIQUE_TIME_FORMAT: &str = "%H:%M:%S";
pub static UNIQUE_SEAL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

pub static AREALY_DATE_FORMAT: Lazy<String> = Lazy::new(|| String::from("%d/%m/%Y"));
pub static AREALY_TIME_FORMAT: Lazy<String> = Lazy::new(|| String::from("%H:%M:%S"));
pub static AREALY_SEAL_FORMAT: Lazy<String> = Lazy::new(|| String::from("%d/%m/%Y %H:%M:%S"));

pub fn now() -> String {
    dbg_call!();
    dbg_reav!(now_ur());
}

pub fn now_ur() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_REAL_FORMAT)));
}

pub fn now_ul() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_LAST_FORMAT)));
}

pub fn now_uw() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_WHIM_FORMAT)));
}

pub fn now_ud() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_DATE_FORMAT)));
}

pub fn now_ut() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_TIME_FORMAT)));
}

pub fn now_us() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(UNIQUE_SEAL_FORMAT)));
}

pub fn now_ad() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(&*AREALY_DATE_FORMAT)));
}

pub fn now_at() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(&*AREALY_TIME_FORMAT)));
}

pub fn now_as() -> String {
    dbg_call!();
    dbg_reav!(format!("{}", Utc::now().format(&*AREALY_SEAL_FORMAT)));
}

pub fn now_ft(format: &str) -> String {
    dbg_call!(format);
    dbg_reav!(format!("{}", Utc::now().format(format)));
}

pub fn fmt(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(fmt_ur(time));
}

pub fn fmt_ur(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_REAL_FORMAT)));
}

pub fn fmt_ul(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_LAST_FORMAT)));
}

pub fn fmt_uw(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_WHIM_FORMAT)));
}

pub fn fmt_ud(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_DATE_FORMAT)));
}

pub fn fmt_ut(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_TIME_FORMAT)));
}

pub fn fmt_us(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(UNIQUE_SEAL_FORMAT)));
}

pub fn fmt_ad(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(&*AREALY_DATE_FORMAT)));
}

pub fn fmt_at(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(&*AREALY_TIME_FORMAT)));
}

pub fn fmt_as(time: &DateTime<Utc>) -> String {
    dbg_call!(time);
    dbg_reav!(format!("{}", time.format(&*AREALY_SEAL_FORMAT)));
}

pub fn fmt_ft(time: &DateTime<Utc>, format: &str) -> String {
    dbg_call!(time, format);
    dbg_reav!(format!("{}", time.format(format)));
}
