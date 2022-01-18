# Liz

Liz ( LuaWizard ) is a library and a command that features a bunch of functionalities for lua scripts inside the liz global variable.

## Features

### Codes Functions

    git_root_find(path: impl AsRef<Path>) -> Result<Option<String>, LizError>
    git_is_ignored(path: impl AsRef<Path>) -> Result<bool, LizError>
    git_has_changes(root: impl AsRef<Path>) -> Result<bool, LizError>

### Execs Functions

    spawn(path: String, args: Option<Vec<String>>) -> Spawned
    join(spawned: Spawned) -> Result<Vec<String>, LizError>
    cmd<S: AsRef<str>, P: AsRef<Path>>(name: &str, args: &[S], dir: P, print: bool, throw: bool) -> Result<(i32, String), LizError>
    pause()
    
### Files Functions

    has(path: impl AsRef<Path>) -> bool
    is_dir(path: impl AsRef<Path>) -> bool
    is_file(path: impl AsRef<Path>) -> bool
    cd(path: impl AsRef<Path>) -> Result<(), LizError>
    pwd() -> Result<String, LizError> 
    rn(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError>
    cp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError>
    cp_tmp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError>
    mv(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError>
    rm(path: impl AsRef<Path>) -> Result<(), LizError>
    read(path: impl AsRef<Path>) -> Result<String, LizError>
    mk_dir(path: impl AsRef<Path>) -> Result<(), LizError>
    touch(path: impl AsRef<Path>) -> Result<(), LizError>
    write(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError>
    append(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError>
    exe_ext() -> &'static str
    path_sep() -> String
    path_ext(path: impl AsRef<Path>) -> Result<String, LizError>
    path_name(path: impl AsRef<Path>) -> Result<String, LizError>
    path_stem(path: impl AsRef<Path>) -> Result<String, LizError>
    path_absolute(path: impl AsRef<Path>) -> Result<String, LizError>
    path_relative(path: impl AsRef<Path>, base: impl AsRef<Path>) -> Result<String, LizError>
    path_parent(path: impl AsRef<Path>) -> Result<String, LizError>
    path_parent_find(path: impl AsRef<Path>, with_name: &str) -> Result<String, LizError>
    path_join(path: impl AsRef<Path>, child: &str) -> Result<String, LizError>
    path_list(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_dirs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_dirs_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_files(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_files_subs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError>
    path_list_files_ext(path: impl AsRef<Path>, ext: &str) -> Result<Vec<String>, LizError>
    path_list_files_exts(path: impl AsRef<Path>, exts: &[&str]) -> Result<Vec<String>, LizError>
    path_list_files_ext_subs(path: impl AsRef<Path>, ext: &str) -> Result<Vec<String>, LizError>
    path_list_files_exts_subs(path: impl AsRef<Path>, exts: &[&str]) -> Result<Vec<String>, LizError>

### Texts Functions


    ask(message: &str) -> Result<String, LizError>
    ask_int(message: &str) -> Result<i32, LizError>
    ask_float(message: &str) -> Result<f64, LizError>
    ask_bool(message: &str) -> Result<bool, LizError>
    trim(text: &str) -> String
    find(text: &str, contents: &str) -> Option<usize>
    starts_with(text: &str, prefix: &str) -> bool
    ends_with(text: &str, suffix: &str) -> bool
    text_path_find(path: impl AsRef<Path>, contents: &str) -> Result<Option<Vec<String>>, LizError>
    text_dir_find(path: impl AsRef<Path>, contents: &str) -> Result<Option<Vec<String>>, LizError>
    text_file_find(path: impl AsRef<Path>, contents: &str) -> Result<Option<Vec<String>>, LizError>
    text_files_find(paths: Vec<String>, contents: String) -> Result<Option<Vec<String>>, LizError>
