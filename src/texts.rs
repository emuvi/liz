use std::fs;
use std::io::Read;
use std::path::Path;

use crate::LizError;

pub fn search(path: impl AsRef<Path>, contents: &str) -> Result<Option<Vec<String>>, LizError> {
    let mut file = fs::File::open(path)?;
    let mut read = String::new();
    file.read_to_string(&mut read)?;
	let mut founds: Option<Vec<String>> = None;
	for line in read.lines() {
		if line.find(contents).is_some() {
			if founds.is_none() {
				founds = Some(Vec::new());
			}
			founds.as_mut().unwrap().push(String::from(line));
		}
	}
    Ok(founds)
}
