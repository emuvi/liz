use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::LizError;

pub fn has(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

pub fn cd(path: impl AsRef<Path>) -> Result<(), LizError> {
    Ok(std::env::set_current_dir(path)?)
}

pub fn pwd() -> Result<String, LizError> {
    Ok(format!("{}", std::env::current_dir()?.display()))
}

pub fn rn(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    Ok(fs::rename(origin, destiny)?)
}

pub fn cp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    if is_dir(&origin) {
        copy_directory(origin, destiny)?;
    } else {
        copy_file(origin, destiny)?;
    }
    Ok(())
}

pub fn cp_tmp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    if has(&destiny) {
        let unknown = "unknown";
        let file_name = match destiny.as_ref().file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_stem) => file_stem,
                None => unknown,
            },
            None => unknown,
        };
        let mut temp_name = String::from(file_name);
        let mut destiny_tmp = std::env::temp_dir().join(&temp_name);
        while destiny_tmp.exists() {
            temp_name.push_str("_");
            destiny_tmp = std::env::temp_dir().join(&temp_name);
        }
        cp(&destiny, &destiny_tmp)?;
        rm(&destiny)?;
    }
    cp(origin, destiny)
}

fn copy_directory(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    fs::create_dir_all(&destiny)?;
    for entry in fs::read_dir(origin)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(entry.path(), destiny.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destiny.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn copy_file(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    if let Some(parent) = destiny.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(origin, destiny)?;
    Ok(())
}

pub fn mv(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
    cp(&origin, &destiny)?;
    rm(origin)?;
    Ok(())
}

pub fn rm(path: impl AsRef<Path>) -> Result<(), LizError> {
    Ok(if has(&path) {
        if is_dir(&path) {
            fs::remove_dir_all(path)?
        } else {
            fs::remove_file(path)?
        }
    })
}

pub fn read(path: impl AsRef<Path>) -> Result<String, LizError> {
    let mut file = fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn mkdir(path: impl AsRef<Path>) -> Result<(), LizError> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn touch(path: impl AsRef<Path>) -> Result<(), LizError> {
    fs::OpenOptions::new().create(true).write(true).open(path)?;
    Ok(())
}

pub fn write(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
    Ok(fs::write(path, contents)?)
}

pub fn append(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
    Ok(writeln!(file, "{}", contents)?)
}

pub fn exe_ext() -> &'static str {
    std::env::consts::EXE_EXTENSION
}

pub fn path_sep() -> String {
    String::from(std::path::MAIN_SEPARATOR)
}

pub fn path_ext(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path.as_ref();
    if let Some(path) = path.extension() {
        if let Some(path_str) = path.to_str() {
            return Ok(String::from(path_str));
        }
    }
    Ok(String::new())
}

pub fn path_name(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path.as_ref();
    if let Some(path) = path.file_name() {
        if let Some(path_str) = path.to_str() {
            return Ok(String::from(path_str));
        }
    }
    Ok(String::new())
}

pub fn path_stem(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path.as_ref();
    if let Some(path) = path.file_stem() {
        if let Some(path_str) = path.to_str() {
            return Ok(String::from(path_str));
        }
    }
    Ok(String::new())
}

pub fn path_absolute(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path.as_ref();
    let path = if path.exists() && path.is_relative() {
        std::fs::canonicalize(path)?
    } else {
        path.to_path_buf()
    };
    let path_str = path
        .to_str()
        .ok_or("Could not convert the path to String.")?;
    let path_str = if path_str.starts_with("\\\\?\\") || path_str.starts_with("//?/") {
        &path_str[4..]
    } else {
        &path_str
    };
    Ok(String::from(path_str))
}

pub fn path_relative(path: impl AsRef<Path>, base: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path_absolute(path)?;
    let base = path_absolute(base)?;
    let result = path_relative_from(&path, &base).ok_or("Could not make relative.")?;
    Ok(format!("{}", result.display()))
}

fn path_relative_from(path: impl AsRef<Path>, base: impl AsRef<Path>) -> Option<PathBuf> {
    use std::path::Component;
    let path = path.as_ref();
    let base = base.as_ref();
    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}

pub fn path_parent(path: impl AsRef<Path>) -> Result<String, LizError> {
    let path = path_absolute(path)?;
    let path = Path::new(&path);
    if let Some(path) = path.parent() {
        if let Some(path_str) = path.to_str() {
            return Ok(String::from(path_str));
        }
    }
    Ok(String::new())
}

pub fn path_parent_find(path: impl AsRef<Path>, with_name: &str) -> Result<String, LizError> {
    let mut path = format!("{}", PathBuf::from(path.as_ref()).display());
    loop {
        let parent = path_parent(&path)?;
        if parent.is_empty() {
            break;
        } else {
            let name = path_stem(&parent)?;
            if name == with_name {
                return Ok(parent);
            } else {
                path = parent;
            }
        }
    }
    Ok(String::new())
}

pub fn path_join(path: impl AsRef<Path>, child: &str) -> Result<String, LizError> {
    let path = path.as_ref().join(child);
    if let Some(path_str) = path.to_str() {
        return Ok(String::from(path_str));
    }
    Ok(String::new())
}

pub fn path_list(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                result.push(String::from(path));
            }
        }
    }
    return Ok(result);
}

pub fn path_list_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_subs_make(path, &mut results)?;
    return Ok(results);
}

fn path_list_subs_make(path: impl AsRef<Path>, results: &mut Vec<String>) -> Result<(), LizError> {
    for entry in fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                results.push(String::from(path));
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    path_list_subs_make(&path, results)?;
                }
            }
        }
    }
    return Ok(());
}

pub fn path_list_dirs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                if let Some(path) = entry.path().to_str() {
                    result.push(String::from(path));
                }
            }
        }
    }
    return Ok(result);
}

pub fn path_list_dirs_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_dirs_subs_make(path, &mut results)?;
    return Ok(results);
}

fn path_list_dirs_subs_make(
    path: impl AsRef<Path>,
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    results.push(String::from(path));
                    path_list_dirs_subs_make(&path, results)?;
                }
            }
        }
    }
    return Ok(());
}

pub fn path_list_files(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                if let Some(path) = entry.path().to_str() {
                    result.push(String::from(path));
                }
            }
        }
    }
    return Ok(result);
}

pub fn path_list_files_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_files_subs_make(path, &mut results)?;
    return Ok(results);
}

fn path_list_files_subs_make(
    path: impl AsRef<Path>,
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    results.push(String::from(path));
                }
                if file_type.is_dir() {
                    path_list_files_subs_make(&path, results)?;
                }
            }
        }
    }
    return Ok(());
}

pub fn path_list_files_ext(path: impl AsRef<Path>, ext: &str) -> Result<Vec<String>, LizError> {
    let mut result = Vec::new();
    let ext = ext.to_lowercase();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                if let Some(path) = entry.path().to_str() {
                    if path.to_lowercase().ends_with(&ext) {
                        result.push(String::from(path));
                    }
                }
            }
        }
    }
    return Ok(result);
}

pub fn path_list_files_exts(
    path: impl AsRef<Path>,
    exts: &[&str],
) -> Result<Vec<String>, LizError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                if let Some(path) = entry.path().to_str() {
                    for ext in exts {
                        if path.to_lowercase().ends_with(&ext.to_lowercase()) {
                            result.push(String::from(path));
                        }
                    }
                }
            }
        }
    }
    return Ok(result);
}

pub fn path_list_files_ext_subs(
    path: impl AsRef<Path>,
    ext: &str,
) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    let ext = ext.to_lowercase();
    path_list_files_ext_subs_make(path, &ext, &mut results)?;
    return Ok(results);
}

fn path_list_files_ext_subs_make(
    path: impl AsRef<Path>,
    ext: &str,
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    if path.to_lowercase().ends_with(&ext) {
                        results.push(String::from(path));
                    }
                }
                if file_type.is_dir() {
                    path_list_files_ext_subs_make(&path, ext, results)?;
                }
            }
        }
    }
    return Ok(());
}

pub fn path_list_files_exts_subs(
    path: impl AsRef<Path>,
    exts: &[&str],
) -> Result<Vec<String>, LizError> {
    let mut results = Vec::new();
    path_list_files_exts_subs_make(path, &exts, &mut results)?;
    return Ok(results);
}

fn path_list_files_exts_subs_make(
    path: impl AsRef<Path>,
    exts: &[&str],
    results: &mut Vec<String>,
) -> Result<(), LizError> {
    for entry in fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    for ext in exts {
                        if path.to_lowercase().ends_with(&ext.to_lowercase()) {
                            results.push(String::from(path));
                        }
                    }
                }
                if file_type.is_dir() {
                    path_list_files_exts_subs_make(&path, exts, results)?;
                }
            }
        }
    }
    return Ok(());
}
