use std::path::Path;

use crate::utils;
use crate::LizError;

pub fn has(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_absolute(path: &str) -> bool {
    if path.starts_with("/") || path.starts_with("\\") {
        return true;
    }
    is_absolute_win(path)
}

fn is_absolute_win(path: &str) -> bool {
    let mut first_is_letter = false;
    let mut second_is_colon = false;
    for (i, c) in path.chars().enumerate() {
        if i == 0 {
            first_is_letter = c.is_alphabetic();
        } else if i == 1 {
            second_is_colon = c == ':';
        } else {
            break;
        }
    }
    first_is_letter && second_is_colon
}

pub fn is_relative(path: &str) -> bool {
    !is_absolute(path)
}

pub fn cd(path: &str) -> Result<(), LizError> {
    std::env::set_current_dir(path).map_err(|err| {
        utils::dbg_p("liz_files", "cd", "set_current_dir", &[("path", path)], err)
    })?;
    Ok(())
}

pub fn pwd() -> Result<String, LizError> {
    let result = std::env::current_dir()
        .map_err(|err| utils::dbg("liz_files", "pwd", "current_dir", err))?;
    Ok(utils::display(result))
}

pub fn rn(origin: &str, destiny: &str) -> Result<(), LizError> {
    std::fs::rename(origin, destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "rn",
            "rename",
            &[("origin", origin), ("destiny", destiny)],
            err,
        )
    })?;
    Ok(())
}

pub fn cp(origin: &str, destiny: &str) -> Result<(), LizError> {
    if is_dir(origin) {
        copy_directory(origin, destiny).map_err(|err| {
            utils::dbg_p(
                "liz_files",
                "cp",
                "copy_directory",
                &[("origin", origin), ("destiny", destiny)],
                err,
            )
        })?;
    } else {
        copy_file(origin, destiny).map_err(|err| {
            utils::dbg_p(
                "liz_files",
                "cp",
                "copy_file",
                &[("origin", origin), ("destiny", destiny)],
                err,
            )
        })?;
    }
    Ok(())
}

fn copy_directory(origin: &str, destiny: &str) -> Result<(), LizError> {
    std::fs::create_dir_all(destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "copy_directory",
            "create_dir_all",
            &[("destiny", destiny)],
            err,
        )
    })?;
    for entry in std::fs::read_dir(origin).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "copy_directory",
            "read_dir",
            &[("origin", origin)],
            err,
        )
    })? {
        let entry = entry.map_err(|err| utils::dbg("liz_files", "copy_directory", "entry", err))?;
        let file_type = entry
            .file_type()
            .map_err(|err| utils::dbg("liz_files", "copy_directory", "file_type", err))?;
        let entry_str = utils::display(entry.path());
        let entry_name = path_name(&entry_str);
        let entry_dest = path_join(&destiny, &entry_name).map_err(|err| {
            utils::dbg_p(
                "liz_files",
                "copy_directory",
                "path_join",
                &[("destiny", &destiny), ("entry_name", &entry_name)],
                err,
            )
        })?;
        if file_type.is_dir() {
            copy_directory(&entry_str, &entry_dest).map_err(|err| {
                utils::dbg_p(
                    "liz_files",
                    "copy_directory",
                    "copy_directory",
                    &[("entry_str", &entry_str), ("entry_dest", &entry_dest)],
                    err,
                )
            })?;
        } else {
            std::fs::copy(&entry_str, &entry_dest).map_err(|err| {
                utils::dbg_p(
                    "liz_files",
                    "copy_directory",
                    "copy",
                    &[("entry_str", &entry_str), ("entry_dest", &entry_dest)],
                    err,
                )
            })?;
        }
    }
    Ok(())
}

fn copy_file(origin: &str, destiny: &str) -> Result<(), LizError> {
    let parent = path_parent(destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "copy_file",
            "path_parent",
            &[("destiny", destiny)],
            err,
        )
    })?;
    std::fs::create_dir_all(&parent).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "copy_file",
            "create_dir_all",
            &[("parent", &parent)],
            err,
        )
    })?;
    std::fs::copy(origin, destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "copy_file",
            "copy",
            &[("origin", origin), ("destiny", destiny)],
            err,
        )
    })?;
    Ok(())
}

pub fn cp_tmp(origin: &str, destiny: &str) -> Result<(), LizError> {
    if has(destiny) {
        let file_name = path_name(destiny);
        let mut file_name = String::from(file_name);
        let mut destiny_tmp = std::env::temp_dir().join(&file_name);
        while destiny_tmp.exists() {
            file_name.push_str("_");
            destiny_tmp = std::env::temp_dir().join(&file_name);
        }
        let destiny_tmp = utils::display(destiny_tmp);
        cp(destiny, &destiny_tmp).map_err(|err| {
            utils::dbg_p(
                "liz_files",
                "cp_tmp",
                "cp",
                &[("destiny", destiny), ("destiny_tmp", &destiny_tmp)],
                err,
            )
        })?;
        rm(destiny).map_err(|err| {
            utils::dbg_p("liz_files", "cp_tmp", "rm", &[("destiny", destiny)], err)
        })?;
    }
    cp(origin, destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "cp_tmp",
            "cp",
            &[("origin", origin), ("destiny", destiny)],
            err,
        )
    })?;
    Ok(())
}

pub fn mv(origin: &str, destiny: &str) -> Result<(), LizError> {
    cp(origin, destiny).map_err(|err| {
        utils::dbg_p(
            "liz_files",
            "mv",
            "cp",
            &[("origin", origin), ("destiny", destiny)],
            err,
        )
    })?;
    rm(origin).map_err(|err| utils::dbg_p("liz_files", "mv", "rm", &[("origin", origin)], err))?;
    Ok(())
}

pub fn rm(path: &str) -> Result<(), LizError> {
    if !has(path) {
        return Ok(());
    }
    if is_dir(path) {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn mkdir(path: &str) -> Result<(), LizError> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn touch(path: &str) -> Result<(), LizError> {
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    Ok(())
}

pub fn exe_ext() -> &'static str {
    std::env::consts::EXE_EXTENSION
}

pub fn os_sep() -> &'static char {
    &std::path::MAIN_SEPARATOR
}

pub fn path_sep(path: &str) -> &'static str {
    if path.contains("\\") {
        "\\"
    } else {
        "/"
    }
}

pub fn path_parts(path: &str) -> Vec<&str> {
    let sep = path_sep(path);
    let mut result: Vec<&str> = path.split(sep).collect();
    if !result.is_empty() {
        if result[0].is_empty() {
            result[0] = sep;
        }
    }
    result
}

#[test]
fn path_parts_test() {
    let parts = path_parts("/home/pointel/test");
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "/");
    assert_eq!(parts[1], "home");
    assert_eq!(parts[2], "pointel");
    assert_eq!(parts[3], "test");
    let parts = path_parts("pointel/test");
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "pointel");
    assert_eq!(parts[1], "test");
    let parts = path_parts("./pointel/test");
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], ".");
    assert_eq!(parts[1], "pointel");
    assert_eq!(parts[2], "test");
    let parts = path_parts("C:\\pointel\\test");
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "C:");
    assert_eq!(parts[1], "pointel");
    assert_eq!(parts[2], "test");
}

pub fn path_parts_join(parts: &[&str]) -> String {
    if parts.is_empty() {
        return String::default();
    }
    let mut result = String::new();
    let mut start = 1;
    let end = parts.len();
    if parts[0] == "/" {
        result.push('/');
        if end > 1 {
            result.push_str(parts[1]);
            start = 2;
        }
    } else {
        result.push_str(parts[0]);
    }
    let os_sep = if parts[0].contains(":") {
        '\\'
    } else {
        *os_sep()
    };
    for index in start..end {
        result.push(os_sep);
        result.push_str(parts[index]);
    }
    result
}

#[test]
fn path_parts_join_test() {
    let parts = path_parts("/home/pointel/test");
    assert_eq!(path_parts_join(parts.as_slice()), "/home/pointel/test");
    let parts = path_parts("pointel/test");
    assert_eq!(path_parts_join(parts.as_slice()), "pointel/test");
    let parts = path_parts("./pointel/test");
    assert_eq!(path_parts_join(parts.as_slice()), "./pointel/test");
    let parts = path_parts("../../pointel/test");
    assert_eq!(path_parts_join(parts.as_slice()), "../../pointel/test");
    let parts = path_parts("C:\\pointel\\test");
    assert_eq!(path_parts_join(parts.as_slice()), "C:\\pointel\\test");
}

pub fn path_name(path: &str) -> &str {
    let parts = path_parts(path);
    if parts.len() > 0 {
        let last_part = parts[parts.len() - 1];
        return last_part;
    }
    ""
}

pub fn path_stem(path: &str) -> &str {
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
    let parts = path_parts(path);
    if parts.len() > 0 {
        let last_part = parts[parts.len() - 1];
        if let Some(last_dot) = last_part.rfind(".") {
            return &last_part[last_dot..];
        }
    }
    ""
}

pub fn path_absolute(path: &str) -> Result<String, LizError> {
    if is_absolute(path) {
        return Ok(String::from(path));
    }
    let wd = pwd()?;
    let mut base_parts = path_parts(&wd);
    let path_parts = path_parts(&path);
    for path_part in path_parts {
        if path_part == "." {
            continue;
        } else if path_part == ".." {
            if base_parts.pop().is_none() {
                return Err(utils::dbg_p(
                    "liz_files",
                    "path_absolute",
                    "pop",
                    &[("path", path)],
                    "the base path went empty",
                ));
            }
        } else {
            base_parts.push(path_part);
        }
    }
    Ok(path_parts_join(base_parts.as_slice()))
}

#[test]
fn path_absolute_test() {
    let wd = pwd().unwrap();
    assert_eq!(path_absolute("test").unwrap(), format!("{}/test", wd));
    assert_eq!(path_absolute("./test").unwrap(), format!("{}/test", wd));
}

pub fn path_relative(path: &str, base: &str) -> Result<String, LizError> {
    if !is_absolute(path) {
        return Ok(String::from(path));
    }
    let base = if !is_absolute(base) {
        path_absolute(base)?
    } else {
        String::from(base)
    };
    if !path.starts_with(&base) {
        return Err(utils::dbg_p(
            "liz_files",
            "path_relative",
            "starts",
            &[("path", path), ("base", &base)],
            "path does not starts with the base",
        ));
    }
    let sep = path_sep(path);
    let result = &path[base.len()..];
    if result.starts_with(sep) {
        Ok(format!(".{}", result))
    } else {
        Ok(String::from(result))
    }
}

pub fn path_parent(path: &str) -> Result<String, LizError> {
    let path = path_absolute(path)?;
    let mut parts = path_parts(&path);
    if parts.pop().is_none() {
        return Err(utils::dbg_p(
            "liz_files",
            "path_parent",
            "pop",
            &[("path", &path)],
            "the path parts went empty",
        ));
    }
    Ok(path_parts_join(parts.as_slice()))
}

pub fn path_parent_find(path: &str, with_name: &str) -> Result<String, LizError> {
    let path = path_absolute(path)?;
    let mut parts = path_parts(&path);
    loop {
        if let Some(part) = parts.pop() {
            if part == with_name {
                parts.push(part);
                return Ok(path_parts_join(parts.as_slice()));
            }
        } else {
            return Err(utils::dbg_p(
                "liz_files",
                "path_parent_find",
                "pop",
                &[("path", &path)],
                "the path parts went empty",
            ));
        }
    }
}

pub fn path_join(path: &str, child: &str) -> Result<String, LizError> {
    if is_absolute(child) {
        return Err(utils::dbg_p(
            "liz_files",
            "path_join",
            "is_absolute",
            &[("child", &child)],
            "the child is absolute",
        ));
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
                    let abs_path = path_absolute(path)?;
                    let abs_parts = path_parts(&abs_path);
                    let dif_parts = abs_parts.len() - inital_size;
                    for new_index in 0..dif_parts {
                        base_parts.insert(new_index, abs_parts[new_index].into());
                    }
                    take_more = true;
                    child_index = child_index - 1;
                } else {
                    return Err(utils::dbg_p(
                        "liz_files",
                        "path_join",
                        "pop",
                        &[("path", &path), ("child", &child)],
                        "the path parts went empty",
                    ));
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

pub fn path_list(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        results.push(utils::display(entry.path()));
    }
    Ok(results)
}

pub fn path_list_in(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_in_make(path, &mut results)?;
    Ok(results)
}

fn path_list_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        results.push(utils::display(entry.path()));
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            path_list_in_make(path, results)?;
        }
    }
    Ok(())
}

pub fn path_list_dirs(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            results.push(utils::display(entry.path()));
        }
    }
    Ok(results)
}

pub fn path_list_dirs_in(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_dirs_in_make(path, &mut results)?;
    return Ok(results);
}

fn path_list_dirs_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            results.push(utils::display(entry.path()));
            path_list_dirs_in_make(path, results)?;
        }
    }
    Ok(())
}

pub fn path_list_files(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            results.push(utils::display(entry.path()));
        }
    }
    Ok(results)
}

pub fn path_list_files_in(path: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_files_in_make(path, &mut results)?;
    Ok(results)
}

fn path_list_files_in_make(path: &str, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            path_list_files_in_make(path, results)?;
        }
        if file_type.is_file() {
            results.push(utils::display(entry.path()));
        }
    }
    Ok(())
}

pub fn path_list_files_ext(path: &str, ext: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let name = utils::display(entry.path());
            if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                results.push(name);
            }
        }
    }
    Ok(results)
}

pub fn path_list_files_ext_in(path: &str, ext: &str) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_files_ext_in_make(path, ext, &mut results)?;
    Ok(results)
}

fn path_list_files_ext_in_make(
    path: &str,
    ext: &str,
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            path_list_files_ext_in_make(path, ext, results)?;
        }
        if file_type.is_file() {
            let name = utils::display(entry.path());
            if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                results.push(name);
            }
        }
    }
    Ok(())
}

pub fn path_list_files_exts(path: &str, exts: &[&str]) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let name = utils::display(entry.path());
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
    let mut results = Vec::new();
    path_list_files_exts_in_make(path, exts, &mut results)?;
    Ok(results)
}

fn path_list_files_exts_in_make(
    path: &str,
    exts: &[&str],
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            path_list_files_exts_in_make(path, exts, results)?;
        }
        if file_type.is_file() {
            let name = utils::display(entry.path());
            for ext in exts {
                if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                    results.push(name);
                    break;
                }
            }
        }
    }
    Ok(())
}
