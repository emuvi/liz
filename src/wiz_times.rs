use rlua::{Context, Table};
use rubx::rux_times;

use crate::LizError;

pub fn inject_times<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let now = lane.create_function(|_, ()| Ok(rux_times::now()))?;

    let now_ur = lane.create_function(|_, ()| Ok(rux_times::now_ur()))?;

    let now_ul = lane.create_function(|_, ()| Ok(rux_times::now_ul()))?;

    let now_uw = lane.create_function(|_, ()| Ok(rux_times::now_uw()))?;

    let now_ud = lane.create_function(|_, ()| Ok(rux_times::now_ud()))?;

    let now_ut = lane.create_function(|_, ()| Ok(rux_times::now_ut()))?;

    let now_us = lane.create_function(|_, ()| Ok(rux_times::now_us()))?;

    let now_ad = lane.create_function(|_, ()| Ok(rux_times::now_ad()))?;

    let now_at = lane.create_function(|_, ()| Ok(rux_times::now_at()))?;

    let now_as = lane.create_function(|_, ()| Ok(rux_times::now_as()))?;

    let now_ft = lane.create_function(|_, format: String| Ok(rux_times::now_ft(&format)))?;

    liz.set("now", now)?;
    liz.set("now_ur", now_ur)?;
    liz.set("now_ul", now_ul)?;
    liz.set("now_uw", now_uw)?;
    liz.set("now_ud", now_ud)?;
    liz.set("now_ut", now_ut)?;
    liz.set("now_us", now_us)?;
    liz.set("now_ad", now_ad)?;
    liz.set("now_at", now_at)?;
    liz.set("now_as", now_as)?;
    liz.set("now_ft", now_ft)?;

    // [TODO] - How to get a DateTime<Utc> from script to pass to rux_times::fmt_xx functions?

    Ok(())
}
