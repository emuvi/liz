use std::path::Path;

use crate::liz_debug::{dbg_call, dbg_erro, dbg_reav, dbg_step};
use crate::LizError;

pub fn has(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).exists());
}

pub fn is_dir(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).is_dir());
}

pub fn is_file(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).is_file());
}

pub fn is_absolute(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).is_absolute());
}

pub fn is_relative(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).is_relative());
}

pub fn is_symlink(path: &str) -> bool {
    dbg_call!(path);
    dbg_reav!(Path::new(path).is_symlink());
}

pub fn cd(path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    std::env::set_current_dir(path).map_err(|err| dbg_erro!(err, path))?;
    Ok(())
}

pub fn wd() -> Result<String, LizError> {
    dbg_call!();
    dbg_reav!(Ok(format!(
        "{}",
        std::env::current_dir()
            .map_err(|err| dbg_erro!(err))?
            .display()
    )));
}

pub fn rn(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    std::fs::rename(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    Ok(())
}

pub fn cp(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    if is_dir(origin) {
        copy_directory(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    } else {
        copy_file(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    }
    Ok(())
}

fn copy_directory(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    std::fs::create_dir_all(destiny).map_err(|err| dbg_erro!(err, destiny))?;
    for entry in std::fs::read_dir(origin).map_err(|err| dbg_erro!(err, origin))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        let entry_str = format!("{}", entry.path().display());
        let entry_name = path_name(&entry_str);
        let entry_dest =
            path_join(&destiny, &entry_name).map_err(|err| dbg_erro!(err, destiny, entry_name))?;
        if file_type.is_dir() {
            copy_directory(&entry_str, &entry_dest)
                .map_err(|err| dbg_erro!(err, entry_str, entry_dest))?;
        } else {
            std::fs::copy(&entry_str, &entry_dest)
                .map_err(|err| dbg_erro!(err, entry_str, entry_dest))?;
        }
    }
    Ok(())
}

fn copy_file(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    let parent = path_parent(destiny).map_err(|err| dbg_erro!(err, destiny))?;
    std::fs::create_dir_all(&parent).map_err(|err| dbg_erro!(err, parent))?;
    std::fs::copy(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    Ok(())
}

pub fn cp_tmp(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    if has(destiny) {
        let file_name = path_name(destiny);
        let mut file_name = String::from(file_name);
        let mut destiny_tmp = std::env::temp_dir().join(&file_name);
        while destiny_tmp.exists() {
            file_name.push_str("_");
            destiny_tmp = std::env::temp_dir().join(&file_name);
        }
        let destiny_tmp = format!("{}", destiny_tmp.display());
        cp(destiny, &destiny_tmp).map_err(|err| dbg_erro!(err, destiny, destiny_tmp))?;
        rm(destiny).map_err(|err| dbg_erro!(err, destiny))?;
    }
    cp(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    Ok(())
}

pub fn mv(origin: &str, destiny: &str) -> Result<(), LizError> {
    dbg_call!(origin, destiny);
    cp(origin, destiny).map_err(|err| dbg_erro!(err, origin, destiny))?;
    rm(origin).map_err(|err| dbg_erro!(err, origin))?;
    Ok(())
}

pub fn rm(path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    if !has(path) {
        return Ok(());
    }
    if is_dir(path) {
        std::fs::remove_dir_all(path).map_err(|err| dbg_erro!(err))?;
    } else {
        std::fs::remove_file(path).map_err(|err| dbg_erro!(err))?;
    }
    Ok(())
}

pub fn mkdir(path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    std::fs::create_dir_all(path).map_err(|err| dbg_erro!(err))?;
    Ok(())
}

pub fn touch(path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .map_err(|err| dbg_erro!(err))?;
    Ok(())
}

pub fn os_sep() -> &'static char {
    dbg_call!();
    &std::path::MAIN_SEPARATOR
}

pub fn path_sep(path: &str) -> &'static str {
    dbg_call!(path);
    if path.contains("\\") {
        "\\"
    } else {
        "/"
    }
}

pub fn path_parts(path: &str) -> Vec<&str> {
    dbg_call!(path);
    let sep = path_sep(path);
    let mut result: Vec<&str> = path.split(sep).collect();
    if !result.is_empty() {
        if result[0].is_empty() {
            result[0] = sep;
        }
    }
    result
}

pub fn path_parts_join(parts: &[&str]) -> String {
    dbg_call!(parts);
    if parts.is_empty() {
        return String::default();
    }
    let mut result = String::new();
    let mut start = 1;
    let end = parts.len();
    if parts[0] == "/" {
        result.push('/');
        if end > 1 {
            result.push_str(parts[1].as_ref());
            start = 2;
        }
    } else {
        result.push_str(parts[0].as_ref());
    }
    let os_sep = if parts[0].contains(":") {
        '\\'
    } else if parts[0] == "/" {
        '/'
    } else {
        *os_sep()
    };
    for index in start..end {
        result.push(os_sep);
        result.push_str(parts[index].as_ref());
    }
    result
}

pub fn path_name(path: &str) -> &str {
    dbg_call!(path);
    let parts = path_parts(path);
    if parts.len() > 0 {
        let last_part = parts[parts.len() - 1];
        return last_part;
    }
    ""
}

pub fn path_stem(path: &str) -> &str {
    dbg_call!(path);
    let parts = path_parts(path);
    if parts.len() > 0 {
        let last_part = parts[parts.len() - 1];
        if let Some(last_dot) = last_part.rfind(".") {
            return &last_part[0..last_dot];
        }
    }
    ""
}

pub fn path_ext(path: &str) -> &str {
    dbg_call!(path);
    let parts = path_parts(path);
    if parts.len() > 0 {
        let last_part = parts[parts.len() - 1];
        if let Some(last_dot) = last_part.rfind(".") {
            return &last_part[last_dot..];
        }
    }
    ""
}

pub fn path_ext_is(path: &str, ext: &str) -> bool {
    dbg_call!(path, ext);
    path_ext(path).to_lowercase() == ext.to_lowercase()
}

pub fn path_ext_is_on(path: &str, exts: Vec<String>) -> bool {
    dbg_step!(path, exts);
    let ext = path_ext(path).to_lowercase();
    for case in exts {
        if ext == case.to_lowercase() {
            return true;
        }
    }
    false
}

pub fn path_absolute(path: &str) -> Result<String, LizError> {
    dbg_call!(path);
    if is_absolute(path) {
        return Ok(String::from(path));
    }
    let working_dir = wd().map_err(|err| dbg_erro!(err))?;
    let mut base_parts = path_parts(&working_dir);
    let path_parts = path_parts(&path);
    for path_part in path_parts {
        if path_part == "." {
            continue;
        } else if path_part == ".." {
            if base_parts.pop().is_none() {
                return Err(dbg_erro!("The base path went empty", path));
            }
        } else {
            base_parts.push(path_part);
        }
    }
    Ok(path_parts_join(base_parts.as_slice()))
}

pub fn path_relative(path: &str, base: &str) -> Result<String, LizError> {
    dbg_call!(path, base);
    if !is_absolute(path) {
        return Ok(String::from(path));
    }
    let base = if !is_absolute(base) {
        path_absolute(base).map_err(|err| dbg_erro!(err))?
    } else {
        String::from(base)
    };
    if !path.starts_with(&base) {
        return Err(dbg_erro!("The path must starts with the base", path, base));
    }
    let sep = path_sep(path);
    let result = &path[base.len()..];
    if result.starts_with(sep) {
        Ok(format!(".{}", result))
    } else {
        Ok(String::from(result))
    }
}

pub fn path_walk(path: &str) -> Result<String, LizError> {
    dbg_call!(path);
    Ok(format!(
        "{}",
        std::fs::read_link(path)
            .map_err(|err| dbg_erro!(err, path))?
            .display(),
    ))
}

pub fn path_parent(path: &str) -> Result<String, LizError> {
    dbg_call!(path);
    let path = path_absolute(path).map_err(|err| dbg_erro!(err))?;
    let mut parts = path_parts(&path);
    if parts.pop().is_none() {
        return Err(dbg_erro!("The path parts went empty", path));
    }
    Ok(path_parts_join(parts.as_slice()))
}

pub fn path_parent_find(path: &str, with_name: &str) -> Result<String, LizError> {
    dbg_call!(path, with_name);
    let path = path_absolute(path).map_err(|err| dbg_erro!(err))?;
    let mut parts = path_parts(&path);
    loop {
        if let Some(part) = parts.pop() {
            if part == with_name {
                parts.push(part);
                return Ok(path_parts_join(parts.as_slice()));
            }
        } else {
            return Err(dbg_erro!("The path parts went empty", path));
        }
    }
}

pub fn path_join(path: &str, child: &str) -> Result<String, LizError> {
    dbg_call!(path, child);
    if is_absolute(child) {
        return Err(dbg_erro!("The child must be relative", child));
    }
    let mut base_parts = path_parts(path)
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let inital_size = base_parts.len();
    let has_more = !is_absolute(path);
    let mut take_more = false;
    let child_parts = path_parts(child);
    let mut child_index = 0;
    loop {
        if child_index >= child_parts.len() {
            break;
        }
        let part = child_parts[child_index];
        child_index = child_index + 1;
        if part == "." {
            continue;
        } else if part == ".." {
            if base_parts.pop().is_none() {
                if !take_more && has_more {
                    let abs_path = path_absolute(path).map_err(|err| dbg_erro!(err))?;
                    let abs_parts = path_parts(&abs_path);
                    let dif_parts = abs_parts.len() - inital_size;
                    for new_index in 0..dif_parts {
                        base_parts.insert(new_index, abs_parts[new_index].into());
                    }
                    take_more = true;
                    child_index = child_index - 1;
                } else {
                    return Err(dbg_erro!("The path parts went empty", path, child));
                }
            }
        } else {
            base_parts.push(part.into());
        }
    }
    Ok(path_parts_join(
        base_parts
            .iter()
            .map(|p| p.as_ref())
            .collect::<Vec<&str>>()
            .as_slice(),
    ))
}

pub fn path_join_if_relative(base: &str, path: &str) -> Result<String, LizError> {
    dbg_call!(base, path);
    if is_relative(path) {
        path_join(base, path)
    } else {
        Ok(path.into())
    }
}

pub fn path_list(path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path);
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|err| dbg_erro!(err))?;
    for entry in entries {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        results.push(format!("{}", entry.path().display()));
    }
    Ok(results)
}

pub fn path_list_in(path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path);
    let mut results = Vec::new();
    path_list_in_make(path, &mut results).map_err(|err| dbg_erro!(err))?;
    Ok(results)
}

fn path_list_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path).map_err(|err| dbg_erro!(err))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        let inside = format!("{}", entry.path().display());
        if file_type.is_dir() {
            path_list_in_make(&inside, results).map_err(|err| dbg_erro!(err))?;
        }
        results.push(inside);
    }
    Ok(())
}

pub fn path_list_dirs(path: &str) -> Result<Vec<String>, LizError> {
    dbg_step!(path);
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|err| dbg_erro!(err))?;
    for entry in entries {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        if file_type.is_dir() {
            results.push(format!("{}", entry.path().display()));
        }
    }
    Ok(results)
}

pub fn path_list_dirs_in(path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path);
    let mut results = Vec::new();
    path_list_dirs_in_make(path, &mut results).map_err(|err| dbg_erro!(err))?;
    return Ok(results);
}

fn path_list_dirs_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path).map_err(|err| dbg_erro!(err))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        if file_type.is_dir() {
            let inside = format!("{}", entry.path().display());
            path_list_dirs_in_make(&inside, results).map_err(|err| dbg_erro!(err))?;
            results.push(inside);
        }
    }
    Ok(())
}

pub fn path_list_files(path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path);
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|err| dbg_erro!(err))?;
    for entry in entries {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        if file_type.is_file() {
            results.push(format!("{}", entry.path().display()));
        }
    }
    Ok(results)
}

pub fn path_list_files_in(path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path);
    let mut results = Vec::new();
    path_list_files_in_make(path, &mut results).map_err(|err| dbg_erro!(err))?;
    Ok(results)
}

fn path_list_files_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path).map_err(|err| dbg_erro!(err))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        let inside = format!("{}", entry.path().display());
        if file_type.is_dir() {
            path_list_files_in_make(&inside, results).map_err(|err| dbg_erro!(err))?;
        }
        if file_type.is_file() {
            results.push(inside);
        }
    }
    Ok(())
}

pub fn path_list_files_ext(path: &str, ext: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path, ext);
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|err| dbg_erro!(err))?;
    for entry in entries {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        if file_type.is_file() {
            let name = format!("{}", entry.path().display());
            if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                results.push(name);
            }
        }
    }
    Ok(results)
}

pub fn path_list_files_ext_in(path: &str, ext: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(path, ext);
    let mut results = Vec::new();
    path_list_files_ext_in_make(path, ext, &mut results).map_err(|err| dbg_erro!(err))?;
    Ok(results)
}

fn path_list_files_ext_in_make(
    path: &str,
    ext: &str,
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path).map_err(|err| dbg_erro!(err))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        let inside = format!("{}", entry.path().display());
        if file_type.is_dir() {
            path_list_files_ext_in_make(&inside, ext, results).map_err(|err| dbg_erro!(err))?;
        }
        if file_type.is_file() {
            if inside.to_lowercase().ends_with(&ext.to_lowercase()) {
                results.push(inside);
            }
        }
    }
    Ok(())
}

pub fn path_list_files_exts(path: &str, exts: &[&str]) -> Result<Vec<String>, LizError> {
    dbg_call!(path, exts);
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|err| dbg_erro!(err))?;
    for entry in entries {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        if file_type.is_file() {
            let name = format!("{}", entry.path().display());
            for ext in exts {
                if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                    results.push(name);
                    break;
                }
            }
        }
    }
    Ok(results)
}

pub fn path_list_files_exts_in(path: &str, exts: &[&str]) -> Result<Vec<String>, LizError> {
    dbg_call!(path, exts);
    let mut results = Vec::new();
    path_list_files_exts_in_make(path, exts, &mut results).map_err(|err| dbg_erro!(err))?;
    Ok(results)
}

fn path_list_files_exts_in_make(
    path: &str,
    exts: &[&str],
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path).map_err(|err| dbg_erro!(err))? {
        let entry = entry.map_err(|err| dbg_erro!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_erro!(err))?;
        let inside = format!("{}", entry.path().display());
        if file_type.is_dir() {
            path_list_files_exts_in_make(&inside, exts, results).map_err(|err| dbg_erro!(err))?;
        }
        if file_type.is_file() {
            let name = inside.to_lowercase();
            for ext in exts {
                if name.ends_with(&ext.to_lowercase()) {
                    results.push(inside);
                    break;
                }
            }
        }
    }
    Ok(())
}
