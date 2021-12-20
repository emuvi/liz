use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;

use crate::files;
use crate::LizError;

pub fn text_trim(text: &str) -> String {
    String::from(text.trim())
}

pub fn text_path_find(
    path: impl AsRef<Path>,
    contents: &str,
) -> Result<Option<Vec<String>>, LizError> {
    if files::is_dir(&path) {
        text_dir_find(&path, contents)
    } else {
        text_file_find(&path, contents)
    }
}

pub fn text_dir_find(
    path: impl AsRef<Path>,
    contents: &str,
) -> Result<Option<Vec<String>>, LizError> {
    let mut founds: Option<Vec<String>> = None;
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = match path.file_name() {
                Some(name) => match name.to_str() {
                    Some(name) => String::from(name),
                    None => String::default(),
                },
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
                if let Some(col) = line.find(contents) {
                    if founds.is_none() {
                        founds = Some(Vec::new());
                    }
                    founds.as_mut().unwrap().push(format!(
                        "({})[{},{}] {}",
                        name,
                        row,
                        col,
                        line.trim()
                    ));
                }
                row += 1;
            }
        }
    }
    Ok(founds)
}

pub fn text_file_find(
    path: impl AsRef<Path>,
    contents: &str,
) -> Result<Option<Vec<String>>, LizError> {
    let mut founds: Option<Vec<String>> = None;
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 1;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if let Some(col) = line.find(contents) {
            if founds.is_none() {
                founds = Some(Vec::new());
            }
            founds
                .as_mut()
                .unwrap()
                .push(format!("[{},{}] {}", row, col, line.trim()));
        }
        row += 1;
    }
    Ok(founds)
}

pub fn text_files_find(paths: &[&str], contents: &str) -> Result<Option<Vec<String>>, LizError> {
    let mut founds: Option<Vec<String>> = None;
    for path in paths {
        let path = Path::new(path);
        let name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => String::from(name),
                None => String::default(),
            },
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
            if let Some(col) = line.find(contents) {
                if founds.is_none() {
                    founds = Some(Vec::new());
                }
                founds.as_mut().unwrap().push(format!(
                    "({})[{},{}] {}",
                    name,
                    row,
                    col,
                    line.trim()
                ));
            }
            row += 1;
        }
    }
    Ok(founds)
}
