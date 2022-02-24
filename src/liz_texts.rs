use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::liz_debug::{dbg_err, dbg_stp};
use crate::liz_forms::Forms;
use crate::liz_parse::{Parser, TEXT_PARSER};
use crate::LizError;

pub fn text(source: &str) -> Forms {
    dbg_stp!(source);
    TEXT_PARSER.parse(source)
}

pub fn ask(message: &str) -> Result<String, LizError> {
    dbg_stp!(message);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|err| dbg_err!(err))?;
    Ok(buffer)
}

pub fn ask_int(message: &str) -> Result<i32, LizError> {
    dbg_stp!(message);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|err| dbg_err!(err))?;
    let result = buffer.parse::<i32>().map_err(|err| dbg_err!(err))?;
    Ok(result)
}

pub fn ask_float(message: &str) -> Result<f64, LizError> {
    dbg_stp!(message);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let result = buffer.parse::<f64>()?;
    Ok(result)
}

pub fn ask_bool(message: &str) -> Result<bool, LizError> {
    dbg_stp!(message);
    let result = ask(message)?;
    let result = result.to_lowercase();
    Ok(result == "t" || result == "true" || result == "y" || result == "yes")
}

pub fn len(text: &str) -> usize {
    dbg_stp!(text);
    text.len()
}

pub fn del(text: &str, start: usize, end: usize) -> String {
    dbg_stp!(text, start, end);
    let mut start = start;
    let mut end = end;
    if start > text.len() {
        start = text.len();
    }
    if end < start {
        end = start;
    }
    let mut result = String::new();
    for (i, c) in text.chars().enumerate() {
        if i < start || i >= end {
            result.push(c);
        }
    }
    result
}

pub fn trim(text: &str) -> String {
    dbg_stp!(text);
    String::from(text.trim())
}

pub fn is_empty(text: &str) -> bool {
    dbg_stp!(text);
    text.is_empty()
}

pub fn is_ascii(text: &str) -> bool {
    dbg_stp!(text);
    text.is_ascii()
}

pub fn tolower(text: &str) -> String {
    dbg_stp!(text);
    String::from(text.to_lowercase())
}

pub fn toupper(text: &str) -> String {
    dbg_stp!(text);
    String::from(text.to_uppercase())
}

pub fn tocapital(text: &str) -> String {
    dbg_stp!(text);
    if text.is_empty() {
        return String::default();
    }
    let mut result = text[0..1].to_uppercase();
    if text.len() > 1 {
        result.push_str(text[1..].to_lowercase().as_ref());
    }
    result
}

pub fn contains(text: &str, part: &str) -> bool {
    dbg_stp!(text, part);
    text.contains(part)
}

pub fn find(text: &str, part: &str) -> Option<usize> {
    dbg_stp!(text, part);
    text.find(part)
}

pub fn rfind(text: &str, part: &str) -> Option<usize> {
    dbg_stp!(text, part);
    text.rfind(part)
}

pub fn starts_with(text: &str, prefix: &str) -> bool {
    dbg_stp!(text, prefix);
    text.starts_with(prefix)
}

pub fn ends_with(text: &str, suffix: &str) -> bool {
    dbg_stp!(text, suffix);
    text.ends_with(suffix)
}

pub fn split(text: &str, pattern: &str) -> Vec<String> {
    dbg_stp!(text, pattern);
    text.split(pattern).map(|item| item.to_string()).collect()
}

pub fn split_spaces(text: &str) -> Vec<String> {
    dbg_stp!(text);
    text.split_whitespace()
        .map(|item| item.to_string())
        .collect()
}

pub fn text_file_find(path: &str, content: String) -> Result<Option<Vec<String>>, LizError> {
    dbg_stp!(path, content);
    text_file_find_any(path, vec![content])
}

pub fn text_file_find_any(
    path: &str,
    contents: Vec<String>,
) -> Result<Option<Vec<String>>, LizError> {
    dbg_stp!(path, contents);
    let mut results: Option<Vec<String>> = None;
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 1;
    let mut done = 0;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        for content in &contents {
            if let Some(col) = line.find(content) {
                if results.is_none() {
                    results = Some(Vec::new());
                }
                let pos = done + col;
                let len = content.len();
                results.as_mut().unwrap().push(format!(
                    "({})[{},{},{},{}]{}",
                    path,
                    row,
                    col,
                    pos,
                    len,
                    line.trim()
                ));
            }
        }
        done = done + line.len();
        row += 1;
    }
    Ok(results)
}

pub fn text_files_find(
    paths: Vec<String>,
    content: String,
) -> Result<Option<Vec<String>>, LizError> {
    dbg_stp!(paths, content);
    text_files_find_any(paths, vec![content])
}

pub fn text_files_find_any(
    paths: Vec<String>,
    contents: Vec<String>,
) -> Result<Option<Vec<String>>, LizError> {
    dbg_stp!(paths, contents);
    let cpus = num_cpus::get();
    let pool = Arc::new(Mutex::new(paths));
    let mut handles: Vec<JoinHandle<Option<Vec<String>>>> = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        let link_pool = pool.clone();
        let link_contents = contents.clone();
        let handle = std::thread::spawn(move || -> Option<Vec<String>> {
            let mut partial: Option<Vec<String>> = None;
            loop {
                let path = {
                    let mut lock_pool = link_pool.lock().unwrap();
                    lock_pool.pop()
                };
                if path.is_none() {
                    break;
                }
                let path = path.unwrap();
                let file_founds = text_file_find_any(&path, link_contents.clone()).unwrap();
                if let Some(file_founds) = file_founds {
                    if partial.is_none() {
                        partial = Some(Vec::new());
                    }
                    let edit_partial = partial.as_mut().unwrap();
                    for found in file_founds {
                        edit_partial.push(found);
                    }
                }
            }
            partial
        });
        handles.push(handle);
    }
    let mut results: Option<Vec<String>> = None;
    for handle in handles {
        let partial = match handle.join() {
            Ok(partial) => partial,
            Err(error) => return Err(dbg_err!(format!("{:?}", error))),
        };
        if let Some(partial) = partial {
            if results.is_none() {
                results = Some(Vec::new());
            }
            let editor = results.as_mut().unwrap();
            for found in partial {
                editor.push(found);
            }
        }
    }
    Ok(results)
}

pub fn text_file_founds(found: &str) -> Vec<String> {
    dbg_stp!(found);
    let mut result: Vec<String> = Vec::new();
    let mut actual = String::new();
    let mut first = true;
    for ch in found.chars() {
        if first {
            first = false;
            continue;
        }
        if result.len() == 0 {
            if ch == ')' {
                result.push(actual.clone());
                actual.clear();
            } else {
                actual.push(ch);
            }
        } else if result.len() > 0 && result.len() < 5 {
            if ch == ',' || ch == ']' {
                result.push(actual.clone());
                actual.clear();
            }
            if ch.is_numeric() {
                actual.push(ch);
            }
        } else {
            actual.push(ch);
        }
    }
    result.push(actual);
    result
}

pub fn read(path: &str) -> Result<String, LizError> {
    dbg_stp!(path);
    let mut file = std::fs::OpenOptions::new()
        .create(false)
        .write(false)
        .read(true)
        .open(path)
        .map_err(|err| dbg_err!(err, path))?;
    let mut result = String::new();
    file.read_to_string(&mut result)
        .map_err(|err| dbg_err!(err, path))?;
    Ok(result)
}

pub fn write(path: &str, contents: String) -> Result<(), LizError> {
    dbg_stp!(path, contents);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(path)
        .map_err(|err| dbg_err!(err, path))?;
    Ok(write!(file, "{}", contents).map_err(|err| dbg_err!(err, path))?)
}

pub fn append(path: &str, contents: String) -> Result<(), LizError> {
    dbg_stp!(path, contents);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .append(true)
        .open(path)
        .map_err(|err| dbg_err!(err, path))?;
    Ok(writeln!(file, "{}", contents).map_err(|err| dbg_err!(err, path))?)
}

pub fn write_lines(path: &str, lines: Vec<String>) -> Result<(), LizError> {
    dbg_stp!(path, lines);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(path)
        .map_err(|err| dbg_err!(err, path))?;
    for line in lines {
        writeln!(file, "{}", line).map_err(|err| dbg_err!(err, path, line))?;
    }
    Ok(())
}

pub fn append_lines(path: &str, lines: Vec<String>) -> Result<(), LizError> {
    dbg_stp!(path, lines);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .append(true)
        .open(path)
        .map_err(|err| dbg_err!(err, path))?;
    for line in lines {
        writeln!(file, "{}", line).map_err(|err| dbg_err!(err, path, line))?;
    }
    Ok(())
}
