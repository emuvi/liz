use rlua::{Context, MultiValue, Table, Value as LuaValue};
use serde_json::Value as JsonValue;

use std::path::Path;

use crate::liz_fires;
use crate::liz_paths;
use crate::liz_winds;
use crate::LizError;

pub fn display(path: impl AsRef<Path>) -> String {
    format!("{}", path.as_ref().display())
}

pub fn liz_suit_path(path: &str) -> Result<String, LizError> {
    let check_ext = path.to_lowercase();
    let result = if check_ext.ends_with(".liz") || check_ext.ends_with(".lua") {
        String::from(path)
    } else {
        format!("{}.liz", path)
    };
    let result = if result.contains("$pwd") {
        result.replace("$pwd", liz_paths::pwd()?.as_ref())
    } else {
        result
    };
    let result = if result.contains("$liz") {
        result.replace("$liz", liz_fires::liz_dir()?.as_ref())
    } else {
        result
    };
    let result = if result.contains("$lizs") {
        result.replace("$lizs", "lizs")
    } else {
        result
    };
    Ok(result)
}

pub fn gotta_lizs(path: &str) -> Result<(), LizError> {
    let sep = liz_paths::os_sep();
    let lizs_dir = format!("{}lizs{}", sep, sep);
    let lizs_pos = path.rfind(&lizs_dir);
    if let Some(lizs_pos) = lizs_pos {
        if !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path)?;
            std::fs::create_dir_all(path_dir)?;
            let lizs_path = &path[lizs_pos + lizs_dir.len()..];
            let lizs_path = lizs_path.replace("\\", "/");
            let origin = format!(
                "https://raw.githubusercontent.com/emuvi/lizs/main/{}",
                &lizs_path
            );
            liz_winds::download(&origin, path, None)?;
        }
    }
    Ok(())
}

pub fn print_stack_dir(lane: Context) -> Result<(), LizError> {
    let liz = get_liz(&lane)?;
    let stack: Table = liz.get("stack_dir")?;
    let size = stack.raw_len();
    for index in 1..size + 1 {
        let dir: String = stack.get(index)?;
        println!("{}", dir);
    }
    Ok(())
}

pub fn put_stack_dir<'a>(lane: &Context<'a>, liz: &Table<'a>, dir: String) -> Result<(), LizError> {
    let contains = liz.contains_key("stack_dir")?;
    if !contains {
        let stack = lane.create_table()?;
        liz.set("stack_dir", stack)?;
    }
    let stack: Table = liz.get("stack_dir")?;
    let next = stack.raw_len() + 1;
    stack.set(next, dir)?;
    Ok(())
}

pub fn get_stack_dir(liz: &Table) -> Result<String, LizError> {
    let stack: Table = liz.get("stack_dir")?;
    let last = stack.raw_len();
    let result: String = stack.get(last)?;
    Ok(result)
}

pub fn last_stack_dir(lane: Context) -> Result<String, LizError> {
    let liz = get_liz(&lane)?;
    Ok(get_stack_dir(&liz)?)
}

pub fn pop_stack_dir(liz: &Table) -> Result<(), LizError> {
    let stack: Table = liz.get("stack_dir")?;
    let last = stack.raw_len();
    stack.set(last, rlua::Nil)?;
    Ok(())
}

fn get_liz<'a>(lane: &Context<'a>) -> Result<Table<'a>, LizError> {
    let globals = lane.globals();
    let liz: Table = globals.get("liz")?;
    Ok(liz)
}

pub fn treat_error<T>(result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => Err(rlua::Error::external(error)),
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
        LuaValue::String(data) => format!(
            "\"{}\"",
            data.to_str()?
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

pub fn from_json<'a>(lane: Context<'a>, source: String) -> Result<LuaValue<'a>, LizError> {
    if source.trim().is_empty() {
        return Ok(LuaValue::Nil);
    }
    let json: JsonValue = serde_json::from_str(&source)?;
    from_json_value(lane, json)
}

fn from_json_value<'a>(lane: Context<'a>, value: JsonValue) -> Result<LuaValue<'a>, LizError> {
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
            let data = lane.create_string(&data)?;
            LuaValue::String(data)
        }
        JsonValue::Array(data) => {
            let table = lane.create_table()?;
            for (index, item) in data.into_iter().enumerate() {
                let item_value = from_json_value(lane, item)?;
                table.set(index + 1, item_value)?;
            }
            LuaValue::Table(table)
        }
        JsonValue::Object(data) => {
            let table = lane.create_table()?;
            for (name, item) in data {
                let item_value = from_json_value(lane, item)?;
                table.set(name, item_value)?;
            }
            LuaValue::Table(table)
        }
    };
    Ok(result)
}
