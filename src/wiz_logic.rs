use rlua::{Context, Table};

use crate::liz_logic::{self, Sense};
use crate::LizError;

pub fn inject_logic<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let sense_same = lane.create_function(|_, ()| Ok(liz_logic::sense_same()))?;

    let sense_swap = lane.create_function(|_, ()| Ok(liz_logic::sense_swap()))?;

    let sense_apply = lane.create_function(|_, (sense, apply_to): (Sense, bool)| {
        Ok(liz_logic::sense_apply(sense, apply_to))
    })?;

    liz.set("sense_same", sense_same)?;
    liz.set("sense_swap", sense_swap)?;
    liz.set("sense_apply", sense_apply)?;

    Ok(())
}
