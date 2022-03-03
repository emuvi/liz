use rlua::{UserData, UserDataMethods};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::liz_debug::{dbg_bleb, dbg_erro};
use crate::liz_debug::{dbg_call, dbg_reav, dbg_seal};
use crate::liz_fires;
use crate::liz_forms::{Form, Forms};
use crate::liz_parse::{Parser, CODE_PARSER};
use crate::liz_paths;
use crate::liz_winds;
use crate::utils;
use crate::LizError;

static UPDATE_LIZS: AtomicBool = AtomicBool::new(false);

pub fn is_update_lizs() -> bool {
    dbg_call!();
    dbg_reav!(UPDATE_LIZS.load(Ordering::Acquire));
}

pub fn set_update_lizs(to: bool) {
    dbg_call!(to);
    UPDATE_LIZS.store(to, Ordering::Release)
}

impl UserData for Forms {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("len", |_, slf, ()| Ok(slf.len()));
        methods.add_method("get", |_, slf, index: usize| Ok(slf.get(index).clone()));
        methods.add_method_mut("set", |_, slf, (index, form): (usize, Form)| {
            Ok(slf.set(index, form))
        });
        methods.add_method_mut("add", |_, slf, (index, form): (usize, Form)| {
            Ok(slf.add(index, form))
        });
        methods.add_method_mut("put", |_, slf, form: Form| Ok(slf.put(form)));
        methods.add_method_mut("del", |_, slf, index: usize| Ok(slf.del(index)));
        methods.add_method_mut("pop", |_, slf, ()| Ok(slf.pop()));
        methods.add_method("find_all", |_, slf, term: String| {
            Ok(slf.find_all(&term))
        });
        methods.add_method("find_all_like", |_, slf, term: String| {
            Ok(slf.find_all_like(&term))
        });
        methods.add_method_mut("change_all", |_, slf, (of, to): (String, String)| {
            Ok(slf.change_all(&of, &to))
        });
        methods.add_method("print_all", |_, slf, ()| Ok(slf.print_all()));
        methods.add_method("build", |_, slf, ()| Ok(slf.build()));
        methods.add_method("write", |_, slf, path: String| {
            utils::treat_error(slf.write(&path))
        });
    }
}

impl UserData for Form {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("print", |_, slf, ()| Ok(slf.print()));
        methods.add_method("is_whitespace", |_, slf, ()| Ok(slf.is_whitespace()));
        methods.add_method("is_linespace", |_, slf, ()| Ok(slf.is_linespace()));
        methods.add_method("is_linebreak", |_, slf, ()| Ok(slf.is_linebreak()));
        methods.add_method("is_code_brackets", |_, slf, ()| Ok(slf.is_code_brackets()));
        methods.add_method("is_text_brackets", |_, slf, ()| Ok(slf.is_text_brackets()));
        methods.add_method(
            "is_text_quotation",
            |_, slf, ()| Ok(slf.is_text_quotation()),
        );
    }
}

pub fn code(source: &str) -> Forms {
    dbg_call!(source);
    dbg_reav!(CODE_PARSER.parse(source));
}

pub fn edit() -> Forms {
    dbg_call!();
    dbg_reav!(Forms::new());
}

pub fn desk(terms: Vec<String>) -> Forms {
    dbg_call!(terms);
    dbg_reav!(Forms::take(terms));
}

pub fn form(part: &str) -> Form {
    dbg_call!(part);
    dbg_reav!(Form::from(part));
}

pub fn liz_suit_path(path: &str) -> Result<String, LizError> {
    dbg_call!(path);
    let os_sep = liz_paths::os_sep().to_string();
    dbg_seal!(os_sep);
    let result = if path.contains("\\") && os_sep != "\\" {
        path.replace("\\", &os_sep)
    } else {
        String::from(path)
    };
    dbg_seal!(result);
    let result = if result.contains("/") && os_sep != "/" {
        result.replace("/", &os_sep)
    } else {
        result
    };
    dbg_seal!(result);
    let check_ext = result.to_lowercase();
    let result = if !(check_ext.ends_with(".liz") || check_ext.ends_with(".lua")) {
        format!("{}.liz", result)
    } else {
        result
    };
    dbg_seal!(result);
    let result = if result.contains("$pwd") {
        result.replace(
            "$pwd",
            liz_paths::wd().map_err(|err| dbg_bleb!(err))?.as_ref(),
        )
    } else {
        result
    };
    dbg_seal!(result);
    let result = if result.contains("$liz") {
        result.replace(
            "$liz",
            liz_fires::liz_dir().map_err(|err| dbg_bleb!(err))?.as_ref(),
        )
    } else {
        result
    };
    dbg_reav!(Ok(result))
}

pub fn gotta_lizs(path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    if let Some(lizs_pos) = get_lizs_path_pos(path) {
        dbg_seal!(lizs_pos);
        if is_update_lizs() || !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path).map_err(|err| dbg_bleb!(err))?;
            dbg_seal!(path_dir);
            std::fs::create_dir_all(path_dir).map_err(|err| dbg_erro!(err))?;
            let net_path = (&path[lizs_pos + 7..]).replace("\\", "/");
            dbg_seal!(net_path);
            get_lizs_file(&net_path, path).map_err(|err| dbg_bleb!(err))?;
        }
    }
    Ok(())
}

pub fn get_lizs(net_path: &str) -> Result<(), LizError> {
    dbg_call!(net_path);
    let local_path = liz_paths::path_join(".lizs", net_path).map_err(|err| dbg_bleb!(err))?;
    dbg_seal!(local_path);
    get_lizs_file(&net_path, &local_path).map_err(|err| dbg_bleb!(err))
}

pub fn get_lizs_path_pos(path: &str) -> Option<usize> {
    dbg_call!(path);
    let separator = if path.contains("\\") { '\\' } else { '/' };
    dbg_seal!(separator);
    let mut lizs_dir = format!(".lizs{}", separator);
    dbg_seal!(lizs_dir);
    if path.starts_with(&lizs_dir) {
        dbg_reav!(Some(0));
    }
    lizs_dir.insert(0, separator);
    dbg_seal!(lizs_dir);
    dbg_reav!(path.rfind(&lizs_dir));
}

pub fn get_lizs_file(net_path: &str, local_path: &str) -> Result<(), LizError> {
    dbg_call!(net_path, local_path);
    let origin = format!(
        "https://raw.githubusercontent.com/emuvi/lizs/main/{}",
        &net_path
    );
    dbg_seal!(origin);
    liz_winds::download(&origin, local_path, None).map_err(|err| dbg_bleb!(err))
}

pub fn git_root_find(path: &str) -> Result<Option<String>, LizError> {
    dbg_call!(path);
    let mut actual = liz_paths::path_absolute(path).map_err(|err| dbg_bleb!(err))?;
    dbg_seal!(actual);
    loop {
        let check = liz_paths::path_join(&actual, ".git").map_err(|err| dbg_bleb!(err))?;
        dbg_seal!(check);
        if liz_paths::is_dir(&check) {
            dbg_reav!(Ok(Some(actual)));
        }
        actual = liz_paths::path_parent(&actual).map_err(|err| dbg_erro!(err))?;
        dbg_seal!(actual);
        if actual.is_empty() {
            break;
        }
    }
    dbg_reav!(Ok(Some(actual)));
}

pub fn git_is_ignored(path: &str) -> Result<bool, LizError> {
    dbg_call!(path);
    if let Some(root) = git_root_find(path).map_err(|err| dbg_bleb!(err))? {
        dbg_seal!(root);
        let relative = liz_paths::path_relative(path, &root).map_err(|err| dbg_bleb!(err))?;
        dbg_seal!(relative);
        let (code, output) = liz_fires::cmd(
            "git",
            &["check-ignore", &relative],
            Some(&root),
            Some(false),
            Some(false),
        )
        .map_err(|err| dbg_bleb!(err))?;
        dbg_seal!(code, output);
        dbg_reav!(Ok(code == 0 && !output.is_empty()));
    }
    dbg_reav!(Ok(false));
}

pub fn git_has_changes(root: &str) -> Result<bool, LizError> {
    dbg_call!(root);
    let (_, output) = liz_fires::cmd("git", &["status"], Some(root), Some(false), Some(true))
        .map_err(|err| dbg_bleb!(err))?;
    dbg_seal!(output);
    let output = output.trim();
    dbg_seal!(output);
    dbg_reav!(Ok(
        !output.ends_with("nothing to commit, working tree clean")
    ));
}
