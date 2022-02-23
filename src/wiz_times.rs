use rlua::{Context, Table};

use crate::liz_times;
use crate::LizError;

pub fn inject_times<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let pnow = lane.create_function(|_, (): ()| Ok(liz_times::pnow()))?;
    
    let pnow_ur = lane.create_function(|_, (): ()| Ok(liz_times::pnow_ur()))?;

    let pnow_ul = lane.create_function(|_, (): ()| Ok(liz_times::pnow_ul()))?;
    
    let pnow_uw = lane.create_function(|_, (): ()| Ok(liz_times::pnow_uw()))?;

    let pnow_ud = lane.create_function(|_, (): ()| Ok(liz_times::pnow_ud()))?;

    let pnow_ut = lane.create_function(|_, (): ()| Ok(liz_times::pnow_ut()))?;
    
    let pnow_us = lane.create_function(|_, (): ()| Ok(liz_times::pnow_us()))?;

    let pnow_ad = lane.create_function(|_, (): ()| Ok(liz_times::pnow_ad()))?;

    let pnow_at = lane.create_function(|_, (): ()| Ok(liz_times::pnow_at()))?;
    
    let pnow_as = lane.create_function(|_, (): ()| Ok(liz_times::pnow_as()))?;

    liz.set("pnow", pnow)?;
    liz.set("pnow_ur", pnow_ur)?;
    liz.set("pnow_ul", pnow_ul)?;
    liz.set("pnow_uw", pnow_uw)?;
    liz.set("pnow_ud", pnow_ud)?;
    liz.set("pnow_ut", pnow_ut)?;
    liz.set("pnow_us", pnow_us)?;
    liz.set("pnow_ad", pnow_ad)?;
    liz.set("pnow_at", pnow_at)?;
    liz.set("pnow_as", pnow_as)?;

    Ok(())
}
