use rlua::{Context, MultiValue, Table, Value as LuaValue};
use serde_json::Value as JsonValue;

use std::path::Path;
use std::path::PathBuf;

use crate::LizError;

pub fn display(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of the path.")?;
    Ok(format!("{}", path_display))
}

pub fn load_parent(path: impl AsRef<Path>) -> Result<PathBuf, LizError> {
    let path = path.as_ref();
    let path = if path.is_relative() {
        std::fs::canonicalize(path)?
    } else {
        path.to_path_buf()
    };
    let parent = path
        .parent()
        .ok_or("Could not get the parent from the path.")?;
    Ok(PathBuf::from(parent))
}

pub fn get_liz(from_ctx: Context) -> Option<Table> {
    let globals = from_ctx.globals();
    match globals.get("liz") {
        Ok(liz) => Some(liz),
        Err(_) => None,
    }
}

pub fn treat_error<T>(ctx: Context, result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => {
            if let Some(liz) = get_liz(ctx) {
                let mut new = true;
                if let Ok(has) = liz.contains_key("err") {
                    new = !has;
                }
                let mut stack_err: String = if !new {
                    match liz.get("err") {
                        Ok(old_stacked) => old_stacked,
                        Err(get_old_err) => {
                            eprintln!("Could not get the stacked errors because: {}", get_old_err);
                            String::new()
                        }
                    }
                } else {
                    String::new()
                };
                stack_err.push_str(&format!("{}\n", error));
                if let Err(not_set_err) = liz.set("err", stack_err) {
                    eprintln!("Could not set the error stack because: {}", not_set_err);
                }
            } else {
                eprintln!("Could not set the error stack because: Could not get the liz.",);
            }
            Err(rlua::Error::external(error))
        }
    }
}

pub fn to_json_multi(values: MultiValue) -> Result<Vec<String>, LizError> {
    let mut result: Vec<String> = Vec::new();
    for value in values {
        result.push(to_json(value)?);
    }
    Ok(result)
}

pub fn to_json(value: LuaValue) -> Result<String, LizError> {
    let result = match value {
        LuaValue::Nil => format!("null"),
        LuaValue::Boolean(data) => format!("{}", data),
        LuaValue::Integer(data) => format!("{}", data),
        LuaValue::Number(data) => format!("{}", data),
        LuaValue::String(data) => format!("\"{}\"", data.to_str()?),
        LuaValue::Table(data) => {
            let mut buffer = String::from("{");
            let mut first = true;
            for pair in data.pairs::<String, LuaValue>() {
                let (key, item_value) = pair?;
                if first {
                    first = false;
                } else {
                    buffer.push(',');
                }
                buffer.push('"');
                buffer.push_str(&key);
                buffer.push('"');
                buffer.push(':');
                buffer.push_str(&to_json(item_value)?);
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

pub fn from_json<'a>(ctx: Context<'a>, source: String) -> Result<LuaValue<'a>, LizError> {
    let json: JsonValue = serde_json::from_str(&source)?;
    from_json_value(ctx, json)
}

fn from_json_value<'a>(ctx: Context<'a>, value: JsonValue) -> Result<LuaValue<'a>, LizError> {
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
            let data = ctx.create_string(&data)?;
            LuaValue::String(data)
        }
        JsonValue::Array(data) => {
            let table = ctx.create_table()?;
            for (index, item) in data.into_iter().enumerate() {
                let item_value = from_json_value(ctx, item)?;
                table.set(index + 1, item_value)?;
            }
            LuaValue::Table(table)
        }
        JsonValue::Object(data) => {
            let table = ctx.create_table()?;
            for (name, item) in data {
                let item_value = from_json_value(ctx, item)?;
                table.set(name, item_value)?;
            }
            LuaValue::Table(table)
        }
    };
    Ok(result)
}
