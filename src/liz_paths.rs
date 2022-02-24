use std::path::Path;

use crate::liz_debug::dbg_err;
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
    Path::new(path).is_absolute()
}

pub fn is_relative(path: &str) -> bool {
    Path::new(path).is_relative()
}

pub fn is_symlink(path: &str) -> bool {
    Path::new(path).is_symlink()
}

pub fn cd(path: &str) -> Result<(), LizError> {
    std::env::set_current_dir(path).map_err(|err| dbg_err!(err, path))?;
    Ok(())
}

pub fn pwd() -> Result<String, LizError> {
    let result = std::env::current_dir().map_err(|err| dbg_err!(err))?;
    Ok(utils::display(result))
}

pub fn rn(origin: &str, destiny: &str) -> Result<(), LizError> {
    std::fs::rename(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
    Ok(())
}

pub fn cp(origin: &str, destiny: &str) -> Result<(), LizError> {
    if is_dir(origin) {
        copy_directory(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
    } else {
        copy_file(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
    }
    Ok(())
}

fn copy_directory(origin: &str, destiny: &str) -> Result<(), LizError> {
    std::fs::create_dir_all(destiny).map_err(|err| dbg_err!(err, destiny))?;
    for entry in std::fs::read_dir(origin).map_err(|err| dbg_err!(err, origin))? {
        let entry = entry.map_err(|err| dbg_err!(err))?;
        let file_type = entry.file_type().map_err(|err| dbg_err!(err))?;
        let entry_str = utils::display(entry.path());
        let entry_name = path_name(&entry_str);
        let entry_dest =
            path_join(&destiny, &entry_name).map_err(|err| dbg_err!(err, destiny, entry_name))?;
        if file_type.is_dir() {
            copy_directory(&entry_str, &entry_dest)
                .map_err(|err| dbg_err!(err, entry_str, entry_dest))?;
        } else {
            std::fs::copy(&entry_str, &entry_dest)
                .map_err(|err| dbg_err!(err, entry_str, entry_dest))?;
        }
    }
    Ok(())
}

fn copy_file(origin: &str, destiny: &str) -> Result<(), LizError> {
    let parent = path_parent(destiny).map_err(|err| dbg_err!(err, destiny))?;
    std::fs::create_dir_all(&parent).map_err(|err| dbg_err!(err, parent))?;
    std::fs::copy(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
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
        cp(destiny, &destiny_tmp).map_err(|err| dbg_err!(err, destiny, destiny_tmp))?;
        rm(destiny).map_err(|err| dbg_err!(err, destiny))?;
    }
    cp(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
    Ok(())
}

pub fn mv(origin: &str, destiny: &str) -> Result<(), LizError> {
    cp(origin, destiny).map_err(|err| dbg_err!(err, origin, destiny))?;
    rm(origin).map_err(|err| dbg_err!(err, origin))?;
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
    let tester = path_parts("/home/pointel/test");
    assert_eq!(tester.len(), 4);
    assert_eq!(tester[0], "/");
    assert_eq!(tester[1], "home");
    assert_eq!(tester[2], "pointel");
    assert_eq!(tester[3], "test");
    let tester = path_parts("pointel/test");
    assert_eq!(tester.len(), 2);
    assert_eq!(tester[0], "pointel");
    assert_eq!(tester[1], "test");
    let tester = path_parts("./pointel/test");
    assert_eq!(tester.len(), 3);
    assert_eq!(tester[0], ".");
    assert_eq!(tester[1], "pointel");
    assert_eq!(tester[2], "test");
    let tester = path_parts("C:\\pointel\\test");
    assert_eq!(tester.len(), 3);
    assert_eq!(tester[0], "C:");
    assert_eq!(tester[1], "pointel");
    assert_eq!(tester[2], "test");
}

pub fn path_parts_join(parts: &[impl AsRef<str>]) -> String {
    if parts.is_empty() {
        return String::default();
    }
    let mut result = String::new();
    let mut start = 1;
    let end = parts.len();
    if parts[0].as_ref() == "/" {
        result.push('/');
        if end > 1 {
            result.push_str(parts[1].as_ref());
            start = 2;
        }
    } else {
        result.push_str(parts[0].as_ref());
    }
    let os_sep = if parts[0].as_ref().contains(":") {
        '\\'
    } else if parts[0].as_ref() == "/" {
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

#[test]
fn path_parts_join_test() {
    let tester = path_parts("/home/pointel/test");
    let expect = "/home/pointel/test";
    let result = path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = path_parts("C:\\pointel\\test");
    let expect = "C:\\pointel\\test";
    let result = path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = path_parts("pointel/test");
    let expect = format!("pointel{}test", os_sep());
    let result = path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = path_parts("./pointel/test");
    let expect = format!(".{}pointel{}test", os_sep(), os_sep());
    let result = path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = path_parts("../../pointel/test");
    let expect = format!("..{}..{}pointel{}test", os_sep(), os_sep(), os_sep());
    let result = path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
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

pub fn path_ext_is(path: &str, ext: &str) -> bool {
    path_ext(path).to_lowercase() == ext.to_lowercase()
}

pub fn path_ext_is_on(path: &str, exts: &[impl AsRef<str>]) -> bool {
    let ext = path_ext(path).to_lowercase();
    for case in exts {
        if ext == case.as_ref().to_lowercase() {
            return true;
        }
    }
    false
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
                return Err(dbg_err!("The base path went empty", path));
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
    let tester = "test";
    let expect = format!("{}{}test", wd, os_sep());
    let result = path_absolute(tester).unwrap();
    assert_eq!(result, expect);
    let tester = "./test";
    let expect = format!("{}{}test", wd, os_sep());
    let result = path_absolute(tester).unwrap();
    assert_eq!(result, expect);
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
        return Err(dbg_err!("The path must starts with the base", path, base));
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
    Ok(utils::display(
        std::fs::read_link(path).map_err(|err| dbg_err!(err, path))?,
    ))
}

pub fn path_parent(path: &str) -> Result<String, LizError> {
    let path = path_absolute(path)?;
    let mut parts = path_parts(&path);
    if parts.pop().is_none() {
        return Err(dbg_err!("The path parts went empty", path));
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
            return Err(dbg_err!("The path parts went empty", path));
        }
    }
}

pub fn path_join(path: &str, child: &str) -> Result<String, LizError> {
    if is_absolute(child) {
        return Err(dbg_err!("The child must be relative", child));
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
                    return Err(dbg_err!("The path parts went empty", path, child));
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
    if is_relative(path) {
        path_join(base, path)
    } else {
        Ok(path.into())
    }
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
        let file_type = entry.file_type()?;
        let inside = utils::display(entry.path());
        if file_type.is_dir() {
            path_list_in_make(&inside, results)?;
        }
        results.push(inside);
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
            let inside = utils::display(entry.path());
            path_list_dirs_in_make(&inside, results)?;
            results.push(inside);
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
        let inside = utils::display(entry.path());
        if file_type.is_dir() {
            path_list_files_in_make(&inside, results)?;
        }
        if file_type.is_file() {
            results.push(inside);
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
        let inside = utils::display(entry.path());
        if file_type.is_dir() {
            path_list_files_ext_in_make(&inside, ext, results)?;
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
        let inside = utils::display(entry.path());
        if file_type.is_dir() {
            path_list_files_exts_in_make(&inside, exts, results)?;
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
