use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use simple_error::simple_error;

use crate::LizError;

pub fn ask(message: &str) -> Result<String, LizError> {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn ask_int(message: &str) -> Result<i32, LizError> {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let result = buffer.parse::<i32>()?;
    Ok(result)
}

pub fn ask_float(message: &str) -> Result<f64, LizError> {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let result = buffer.parse::<f64>()?;
    Ok(result)
}

pub fn ask_bool(message: &str) -> Result<bool, LizError> {
    let result = ask(message)?;
    let result = result.to_lowercase();
    Ok(result == "t" || result == "true" || result == "y" || result == "yes")
}

pub fn len(text: &str) -> usize {
    text.len()
}

pub fn del(text: &str, start: usize, end: usize) -> String {
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
    String::from(text.trim())
}

pub fn is_empty(text: &str) -> bool {
    text.is_empty()
}

pub fn is_ascii(text: &str) -> bool {
    text.is_ascii()
}

pub fn tolower(text: &str) -> String {
    String::from(text.to_lowercase())
}

pub fn toupper(text: &str) -> String {
    String::from(text.to_uppercase())
}

pub fn contains(text: &str, part: &str) -> bool {
    text.contains(part)
}

pub fn find(text: &str, part: &str) -> Option<usize> {
    text.find(part)
}

pub fn rfind(text: &str, part: &str) -> Option<usize> {
    text.rfind(part)
}

pub fn starts_with(text: &str, prefix: &str) -> bool {
    text.starts_with(prefix)
}

pub fn ends_with(text: &str, suffix: &str) -> bool {
    text.ends_with(suffix)
}

pub fn split(text: &str, pattern: &str) -> Vec<String> {
    text.split(pattern).map(|item| item.to_string()).collect()
}

pub fn split_spaces(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|item| item.to_string())
        .collect()
}

pub fn text_file_find(
    path: impl AsRef<Path>,
    contents: &str,
) -> Result<Option<Vec<String>>, LizError> {
    let mut results: Option<Vec<String>> = None;
    let name = match path.as_ref().to_str() {
        Some(name) => String::from(name),
        None => String::default(),
    };
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 1;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if line.ends_with("\n") {
            line.pop();
        }
        if line.ends_with("\r") {
            line.pop();
        }
        if let Some(col) = line.find(contents) {
            if results.is_none() {
                results = Some(Vec::new());
            }
            let len = contents.len();
            results
                .as_mut()
                .unwrap()
                .push(format!("({})[{},{},{}]{}", name, row, col, len, line));
        }
        row += 1;
    }
    Ok(results)
}

pub fn text_file_find_any<S: AsRef<str>, P: AsRef<Path>>(
    path: P,
    contents: &[S],
) -> Result<Option<Vec<String>>, LizError> {
    let mut results: Option<Vec<String>> = None;
    let name = match path.as_ref().to_str() {
        Some(name) => String::from(name),
        None => String::default(),
    };
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 1;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if line.ends_with("\n") {
            line.pop();
        }
        if line.ends_with("\r") {
            line.pop();
        }
        for searcher in contents {
            let searcher = searcher.as_ref();
            if let Some(col) = line.find(searcher) {
                if results.is_none() {
                    results = Some(Vec::new());
                }
                let len = searcher.len();
                results
                    .as_mut()
                    .unwrap()
                    .push(format!("({})[{},{},{}]{}", name, row, col, len, line));
            }
        }
        row += 1;
    }
    Ok(results)
}

pub fn text_files_find(
    paths: Vec<String>,
    contents: String,
) -> Result<Option<Vec<String>>, LizError> {
    let cpus = num_cpus::get();
    let pool = Arc::new(Mutex::new(paths));
    let mut handles: Vec<JoinHandle<Option<Vec<String>>>> = Vec::with_capacity(cpus);
    let contents = Arc::new(contents);
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
                let path = Path::new(&path);
                let file_founds = text_file_find(path, &link_contents).unwrap();
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
            Err(error) => return Err(Box::new(simple_error!(format!("{:?}", error)))),
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

pub fn text_files_find_any(
    paths: Vec<String>,
    contents: Vec<String>,
) -> Result<Option<Vec<String>>, LizError> {
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
                let path = Path::new(&path);
                let file_founds = text_file_find_any(path, link_contents.as_slice()).unwrap();
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
            Err(error) => return Err(Box::new(simple_error!(format!("{:?}", error)))),
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
        } else if result.len() > 0 && result.len() < 4 {
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

pub fn read(path: impl AsRef<Path>) -> Result<String, LizError> {
    let mut file = fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn write(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
    Ok(fs::write(path, contents)?)
}

pub fn append(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
    Ok(writeln!(file, "{}", contents)?)
}

pub fn write_lines(path: impl AsRef<Path>, lines: Vec<String>) -> Result<(), LizError> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn append_lines(path: impl AsRef<Path>, lines: Vec<String>) -> Result<(), LizError> {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
