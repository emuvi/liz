use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use simple_error::simple_error;

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
    let mut partial: Option<Vec<String>> = None;
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
                    if partial.is_none() {
                        partial = Some(Vec::new());
                    }
                    partial.as_mut().unwrap().push(format!(
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
    Ok(partial)
}

pub fn text_file_find(
    path: impl AsRef<Path>,
    contents: &str,
) -> Result<Option<Vec<String>>, LizError> {
    let mut partial: Option<Vec<String>> = None;
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
            if partial.is_none() {
                partial = Some(Vec::new());
            }
            partial
                .as_mut()
                .unwrap()
                .push(format!("[{},{}] {}", row, col, line.trim()));
        }
        row += 1;
    }
    Ok(partial)
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
                let name = match path.file_name() {
                    Some(name) => match name.to_str() {
                        Some(name) => String::from(name),
                        None => String::default(),
                    },
                    None => String::default(),
                };
                let file = File::open(path).unwrap();
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                let mut row = 1;
                loop {
                    line.clear();
                    if reader.read_line(&mut line).unwrap() == 0 {
                        break;
                    }
                    if let Some(col) = line.find(&*link_contents) {
                        if partial.is_none() {
                            partial = Some(Vec::new());
                        }
                        partial.as_mut().unwrap().push(format!(
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
            partial
        });
		handles.push(handle);
    }
	let mut results: Option<Vec<String>> = None;
	for handle in handles {
		let partial = match handle.join() {
			Ok(partial) => partial,
			Err(error) => return Err(Box::new(simple_error!(format!("{:?}", error))))
		};
		if let Some(partial) = partial {
			if results.is_none() {
				results = Some(Vec::new());
			}
			let inserter = results.as_mut().unwrap();
			for found in partial {
				inserter.push(found);
			}
		}
	}
    Ok(results)
}
