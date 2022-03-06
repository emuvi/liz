use rlua::{Context, MultiValue, Table, Value as LuaValue};
use serde_json::Value as JsonValue;

use crate::liz_debug::{dbg_erro, dbg_seal};
use crate::LizError;

pub fn print_stack_dir(lane: Context) -> Result<(), LizError> {
    dbg_seal!();
    let liz = get_liz(&lane).map_err(|err| dbg_erro!(err))?;
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_erro!(err))?;
    let size = stack.raw_len();
    for index in 1..size + 1 {
        let dir: String = stack.get(index).map_err(|err| dbg_erro!(err))?;
        println!("{}", dir);
    }
    Ok(())
}

pub fn put_stack_dir<'a>(lane: &Context<'a>, liz: &Table<'a>, dir: String) -> Result<(), LizError> {
    dbg_seal!(dir);
    let contains = liz.contains_key("stack_dir").map_err(|err| dbg_erro!(err))?;
    if !contains {
        let stack = lane.create_table().map_err(|err| dbg_erro!(err))?;
        liz.set("stack_dir", stack).map_err(|err| dbg_erro!(err))?;
    }
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_erro!(err))?;
    let next = stack.raw_len() + 1;
    stack.set(next, dir).map_err(|err| dbg_erro!(err))?;
    Ok(())
}

pub fn get_stack_dir(liz: &Table) -> Result<String, LizError> {
    dbg_seal!();
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_erro!(err))?;
    let last = stack.raw_len();
    let result: String = stack.get(last).map_err(|err| dbg_erro!(err))?;
    Ok(result)
}

pub fn last_stack_dir(lane: Context) -> Result<String, LizError> {
    dbg_seal!();
    let liz = get_liz(&lane).map_err(|err| dbg_erro!(err))?;
    Ok(get_stack_dir(&liz).map_err(|err| dbg_erro!(err))?)
}

pub fn pop_stack_dir(liz: &Table) -> Result<(), LizError> {
    dbg_seal!();
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_erro!(err))?;
    let last = stack.raw_len();
    stack.set(last, rlua::Nil).map_err(|err| dbg_erro!(err))?;
    Ok(())
}

fn get_liz<'a>(lane: &Context<'a>) -> Result<Table<'a>, LizError> {
    dbg_seal!();
    let globals = lane.globals();
    let liz: Table = globals.get("Liz").map_err(|err| dbg_erro!(err))?;
    Ok(liz)
}

pub fn treat_error<T>(result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => Err(rlua::Error::external(error)),
    }
}

pub fn to_json_multi(values: MultiValue) -> Result<Vec<String>, LizError> {
    dbg_seal!(values);
    let mut result: Vec<String> = Vec::new();
    for value in values {
        result.push(to_json(value).map_err(|err| dbg_erro!(err))?);
    }
    Ok(result)
}

pub fn to_json(value: LuaValue) -> Result<String, LizError> {
    dbg_seal!(value);
    let result = match value {
        LuaValue::Nil => format!("null"),
        LuaValue::Boolean(data) => format!("{}", data),
        LuaValue::Integer(data) => format!("{}", data),
        LuaValue::Number(data) => format!("{}", data),
        LuaValue::String(data) => format!(
            "\"{}\"",
            data.to_str()
                .map_err(|err| dbg_erro!(err))?
                .replace("\\", "\\\\")
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t")
                .replace("\"", "\\\"")
        ),
        LuaValue::Table(data) => {
            let mut buffer = String::from("{");
            let mut first = true;
            for pair in data.pairs::<String, LuaValue>() {
                let (key, item_value) = pair.map_err(|err| dbg_erro!(err))?;
                if first {
                    first = false;
                } else {
                    buffer.push(',');
                }
                buffer.push('"');
                buffer.push_str(&key);
                buffer.push('"');
                buffer.push(':');
                buffer.push_str(&to_json(item_value).map_err(|err| dbg_erro!(err))?);
            }
            buffer.push_str("}");
            buffer
        }
        LuaValue::Function(data) => format!("\"|LizFunction|[{:?}]\"", data),
        LuaValue::LightUserData(data) => format!("\"|LizLightUserData|[{:?}]\"", data),
        LuaValue::UserData(data) => format!("\"|LizUserData|[{:?}]\"", data),
        LuaValue::Thread(data) => format!("\"|LizThread|[{:?}]\"", data),
        LuaValue::Error(data) => format!("\"|LizError|[{:?}]\"", data),
    };
    Ok(result)
}

pub fn from_json<'a>(lane: Context<'a>, source: String) -> Result<LuaValue<'a>, LizError> {
    dbg_seal!(source);
    if source.trim().is_empty() {
        return Ok(LuaValue::Nil);
    }
    let json: JsonValue = serde_json::from_str(&source).map_err(|err| dbg_erro!(err))?;
    from_json_value(lane, json)
}

fn from_json_value<'a>(lane: Context<'a>, value: JsonValue) -> Result<LuaValue<'a>, LizError> {
    dbg_seal!(value);
    let result = match value {
        JsonValue::Null => LuaValue::Nil,
        JsonValue::Bool(data) => LuaValue::Boolean(data),
        JsonValue::Number(data) => {
            if data.is_i64() {
                LuaValue::Integer(data.as_i64().unwrap())
            } else if data.is_u64() {
                LuaValue::Integer(data.as_u64().unwrap() as i64)
            } else {
                LuaValue::Number(data.as_f64().unwrap())
            }
        }
        JsonValue::String(data) => {
            let data = lane.create_string(&data).map_err(|err| dbg_erro!(err))?;
            LuaValue::String(data)
        }
        JsonValue::Array(data) => {
            let table = lane.create_table().map_err(|err| dbg_erro!(err))?;
            for (index, item) in data.into_iter().enumerate() {
                let item_value = from_json_value(lane, item).map_err(|err| dbg_erro!(err))?;
                table
                    .set(index + 1, item_value)
                    .map_err(|err| dbg_erro!(err))?;
            }
            LuaValue::Table(table)
        }
        JsonValue::Object(data) => {
            let table = lane.create_table().map_err(|err| dbg_erro!(err))?;
            for (name, item) in data {
                let item_value = from_json_value(lane, item).map_err(|err| dbg_erro!(err))?;
                table.set(name, item_value).map_err(|err| dbg_erro!(err))?;
            }
            LuaValue::Table(table)
        }
    };
    Ok(result)
}
