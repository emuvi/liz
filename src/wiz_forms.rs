use rlua::{Context, Table};

use crate::liz_forms;
use crate::LizError;
use crate::utils;

pub fn inject_forms<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let kit_new = lane.create_function(|_, ()| Ok(liz_forms::kit_new()))?;

    let kit_from =
        lane.create_function(|_, from: Vec<String>| Ok(liz_forms::kit_from(from.as_slice())))?;

    let kit_len = lane.create_function(|_, forms: Vec<String>| Ok(liz_forms::kit_len(&forms)))?;

    let kit_get = lane.create_function(|_, (forms, index): (Vec<String>, usize)| {
        Ok(liz_forms::kit_get(&forms, index).to_string())
    })?;

    let kit_set = lane.create_function_mut(
        |_, (mut forms, index, form): (Vec<String>, usize, String)| {
            Ok(liz_forms::kit_set(&mut forms, index, form))
        },
    )?;

    let kit_add = lane.create_function_mut(
        |_, (mut forms, index, form): (Vec<String>, usize, String)| {
            Ok(liz_forms::kit_add(&mut forms, index, form))
        },
    )?;

    let kit_add_range = lane.create_function_mut(
        |_, (mut forms, on, range): (Vec<String>, usize, Vec<String>)| {
            Ok(liz_forms::kit_add_range(&mut forms, on, range))
        },
    )?;

    let kit_put = lane.create_function_mut(|_, (mut forms, form): (Vec<String>, String)| {
        Ok(liz_forms::kit_put(&mut forms, form))
    })?;

    let kit_del = lane.create_function_mut(|_, (mut forms, index): (Vec<String>, usize)| {
        Ok(liz_forms::kit_del(&mut forms, index))
    })?;

    let kit_del_range =
        lane.create_function_mut(|_, (mut forms, from, till): (Vec<String>, usize, usize)| {
            Ok(liz_forms::kit_del_range(&mut forms, from, till))
        })?;

    let kit_pop =
        lane.create_function_mut(|_, mut forms: Vec<String>| Ok(liz_forms::kit_pop(&mut forms)))?;

    let kit_find_all = lane.create_function(|_, (forms, part): (Vec<String>, String)| {
        Ok(liz_forms::kit_find_all(&forms, &part))
    })?;

    let kit_find_all_like = lane.create_function(|_, (forms, part): (Vec<String>, String)| {
        Ok(liz_forms::kit_find_all_like(&forms, &part))
    })?;

    let kit_first_some = lane.create_function(|_, forms: Vec<String>| {
        Ok(liz_forms::kit_first_some(&forms))
    })?;

    let kit_prior_some = lane.create_function(|_, (forms, of): (Vec<String>, usize)| {
        Ok(liz_forms::kit_prior_some(&forms, of))
    })?;

    let kit_next_some = lane.create_function(|_, (forms, of): (Vec<String>, usize)| {
        Ok(liz_forms::kit_next_some(&forms, of))
    })?;

    let kit_last_some = lane.create_function(|_, forms: Vec<String>| {
        Ok(liz_forms::kit_last_some(&forms))
    })?;

    let kit_change_all = lane.create_function_mut(
        |_, (mut forms, of, to): (Vec<String>, String, String)| {
            Ok(liz_forms::kit_change_all(&mut forms, &of, &to))
        },
    )?;

    let kit_change_all_like = lane.create_function_mut(
        |_, (mut forms, of, to): (Vec<String>, String, String)| {
            Ok(liz_forms::kit_change_all_like(&mut forms, &of, &to))
        },
    )?;

    let kit_print_all = lane.create_function(
        |_, forms: Vec<String>| {
            Ok(liz_forms::kit_print_all(&forms))
        },
    )?;

    let kit_build = lane.create_function(
        |_, forms: Vec<String>| {
            Ok(liz_forms::kit_build(&forms))
        },
    )?;

    let kit_write = lane.create_function(
        |_, (forms, path): (Vec<String>, String)| {
            utils::treat_error(liz_forms::kit_write(&forms, &path))
        },
    )?;

    liz.set("kit_new", kit_new)?;
    liz.set("kit_from", kit_from)?;
    liz.set("kit_len", kit_len)?;
    liz.set("kit_get", kit_get)?;
    liz.set("kit_set", kit_set)?;
    liz.set("kit_add", kit_add)?;
    liz.set("kit_add_range", kit_add_range)?;
    liz.set("kit_put", kit_put)?;
    liz.set("kit_del", kit_del)?;
    liz.set("kit_del_range", kit_del_range)?;
    liz.set("kit_pop", kit_pop)?;
    liz.set("kit_find_all", kit_find_all)?;
    liz.set("kit_find_all_like", kit_find_all_like)?;
    liz.set("kit_first_some", kit_first_some)?;
    liz.set("kit_prior_some", kit_prior_some)?;
    liz.set("kit_next_some", kit_next_some)?;
    liz.set("kit_last_some", kit_last_some)?;
    liz.set("kit_change_all", kit_change_all)?;
    liz.set("kit_change_all_like", kit_change_all_like)?;
    liz.set("kit_print_all", kit_print_all)?;
    liz.set("kit_build", kit_build)?;
    liz.set("kit_write", kit_write)?;

    Ok(())
}
