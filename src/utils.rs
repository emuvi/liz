use rlua::{Context, MultiValue, Table, Value as LuaValue};
use serde_json::Value as JsonValue;

use crate::liz_debug::{dbg_err, dbg_stp};
use crate::liz_fires;
use crate::liz_paths;
use crate::liz_winds;
use crate::LizError;

pub fn liz_suit_path(path: &str) -> Result<String, LizError> {
    dbg_stp!(path);
    let os_sep = liz_paths::os_sep().to_string();
    let path = if path.contains("\\") && os_sep != "\\" {
        path.replace("\\", &os_sep)
    } else {
        String::from(path)
    };
    let path = if path.contains("/") && os_sep != "/" {
        path.replace("/", &os_sep)
    } else {
        path
    };
    let check_ext = path.to_lowercase();
    let path = if !(check_ext.ends_with(".liz") || check_ext.ends_with(".lua")) {
        format!("{}.liz", path)
    } else {
        path
    };
    let path = if path.contains("$pwd") {
        path.replace(
            "$pwd",
            liz_paths::wd().map_err(|err| dbg_err!(err))?.as_ref(),
        )
    } else {
        path
    };
    let path = if path.contains("$liz") {
        path.replace(
            "$liz",
            liz_fires::liz_dir().map_err(|err| dbg_err!(err))?.as_ref(),
        )
    } else {
        path
    };
    Ok(path)
}

pub fn gotta_lizs(path: &str) -> Result<(), LizError> {
    dbg_stp!(path);
    if let Some(lizs_pos) = get_lizs_pos(path) {
        if !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path).map_err(|err| dbg_err!(err))?;
            std::fs::create_dir_all(path_dir).map_err(|err| dbg_err!(err))?;
            let net_path = (&path[lizs_pos + 8..]).replace("\\", "/");
            get_lizs_file(&net_path, path).map_err(|err| dbg_err!(err))?;
        }
    }
    Ok(())
}

pub fn get_lizs_pos(path: &str) -> Option<usize> {
    dbg_stp!(path);
    let sep = if path.contains("\\") { "\\" } else { "/" };
    let lizs_dir = format!("{}(lizs){}", sep, sep);
    path.rfind(&lizs_dir)
}

pub fn get_lizs_file(net_path: &str, local_path: &str) -> Result<(), LizError> {
    dbg_stp!(net_path, local_path);
    let origin = format!(
        "https://raw.githubusercontent.com/emuvi/lizs/main/{}",
        &net_path
    );
    liz_winds::download(&origin, local_path, None).map_err(|err| dbg_err!(err))
}

pub fn print_stack_dir(lane: Context) -> Result<(), LizError> {
    dbg_stp!();
    let liz = get_liz(&lane).map_err(|err| dbg_err!(err))?;
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_err!(err))?;
    let size = stack.raw_len();
    for index in 1..size + 1 {
        let dir: String = stack.get(index).map_err(|err| dbg_err!(err))?;
        println!("{}", dir);
    }
    Ok(())
}

pub fn put_stack_dir<'a>(lane: &Context<'a>, liz: &Table<'a>, dir: String) -> Result<(), LizError> {
    dbg_stp!(dir);
    let contains = liz.contains_key("stack_dir").map_err(|err| dbg_err!(err))?;
    if !contains {
        let stack = lane.create_table().map_err(|err| dbg_err!(err))?;
        liz.set("stack_dir", stack).map_err(|err| dbg_err!(err))?;
    }
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_err!(err))?;
    let next = stack.raw_len() + 1;
    stack.set(next, dir).map_err(|err| dbg_err!(err))?;
    Ok(())
}

pub fn get_stack_dir(liz: &Table) -> Result<String, LizError> {
    dbg_stp!();
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_err!(err))?;
    let last = stack.raw_len();
    let result: String = stack.get(last).map_err(|err| dbg_err!(err))?;
    Ok(result)
}

pub fn last_stack_dir(lane: Context) -> Result<String, LizError> {
    dbg_stp!();
    let liz = get_liz(&lane).map_err(|err| dbg_err!(err))?;
    Ok(get_stack_dir(&liz).map_err(|err| dbg_err!(err))?)
}

pub fn pop_stack_dir(liz: &Table) -> Result<(), LizError> {
    dbg_stp!();
    let stack: Table = liz.get("stack_dir").map_err(|err| dbg_err!(err))?;
    let last = stack.raw_len();
    stack.set(last, rlua::Nil).map_err(|err| dbg_err!(err))?;
    Ok(())
}

fn get_liz<'a>(lane: &Context<'a>) -> Result<Table<'a>, LizError> {
    dbg_stp!();
    let globals = lane.globals();
    let liz: Table = globals.get("liz").map_err(|err| dbg_err!(err))?;
    Ok(liz)
}

pub fn treat_error<T>(result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => Err(rlua::Error::external(error)),
    }
}

pub fn to_json_multi(values: MultiValue) -> Result<Vec<String>, LizError> {
    dbg_stp!(values);
    let mut result: Vec<String> = Vec::new();
    for value in values {
        result.push(to_json(value).map_err(|err| dbg_err!(err))?);
    }
    Ok(result)
}

pub fn to_json(value: LuaValue) -> Result<String, LizError> {
    dbg_stp!(value);
    let result = match value {
        LuaValue::Nil => format!("null"),
        LuaValue::Boolean(data) => format!("{}", data),
        LuaValue::Integer(data) => format!("{}", data),
        LuaValue::Number(data) => format!("{}", data),
        LuaValue::String(data) => format!(
            "\"{}\"",
            data.to_str()
                .map_err(|err| dbg_err!(err))?
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
                let (key, item_value) = pair.map_err(|err| dbg_err!(err))?;
                if first {
                    first = false;
                } else {
                    buffer.push(',');
                }
                buffer.push('"');
                buffer.push_str(&key);
                buffer.push('"');
                buffer.push(':');
                buffer.push_str(&to_json(item_value).map_err(|err| dbg_err!(err))?);
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
    dbg_stp!(source);
    if source.trim().is_empty() {
        return Ok(LuaValue::Nil);
    }
    let json: JsonValue = serde_json::from_str(&source).map_err(|err| dbg_err!(err))?;
    from_json_value(lane, json)
}

fn from_json_value<'a>(lane: Context<'a>, value: JsonValue) -> Result<LuaValue<'a>, LizError> {
    dbg_stp!(value);
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
            let data = lane.create_string(&data).map_err(|err| dbg_err!(err))?;
            LuaValue::String(data)
        }
        JsonValue::Array(data) => {
            let table = lane.create_table().map_err(|err| dbg_err!(err))?;
            for (index, item) in data.into_iter().enumerate() {
                let item_value = from_json_value(lane, item).map_err(|err| dbg_err!(err))?;
                table
                    .set(index + 1, item_value)
                    .map_err(|err| dbg_err!(err))?;
            }
            LuaValue::Table(table)
        }
        JsonValue::Object(data) => {
            let table = lane.create_table().map_err(|err| dbg_err!(err))?;
            for (name, item) in data {
                let item_value = from_json_value(lane, item).map_err(|err| dbg_err!(err))?;
                table.set(name, item_value).map_err(|err| dbg_err!(err))?;
            }
            LuaValue::Table(table)
        }
    };
    Ok(result)
}
