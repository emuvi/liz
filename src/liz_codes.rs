use rlua::{UserData, UserDataMethods};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::liz_debug::{dbg_bub, dbg_err, dbg_stp};
use crate::liz_fires;
use crate::liz_forms::{Form, Forms};
use crate::liz_parse::{Parser, CODE_PARSER};
use crate::liz_paths;
use crate::liz_winds;
use crate::utils;
use crate::LizError;

impl UserData for Forms {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("len", |_, var, ()| Ok(var.len()));
        methods.add_method("get", |_, var, index: usize| Ok(var.get(index).clone()));
        methods.add_method_mut("set", |_, var, (index, form): (usize, Form)| {
            Ok(var.set(index, form))
        });
        methods.add_method_mut("add", |_, var, (index, form): (usize, Form)| {
            Ok(var.add(index, form))
        });
        methods.add_method_mut("put", |_, var, form: Form| Ok(var.put(form)));
        methods.add_method_mut("del", |_, var, index: usize| Ok(var.del(index)));
        methods.add_method_mut("pop", |_, var, ()| Ok(var.pop()));
        methods.add_method_mut("change_all", |_, var, (of, to): (String, String)| {
            Ok(var.change_all(&of, &to))
        });
        methods.add_method("print_all", |_, var, ()| Ok(var.print_all()));
        methods.add_method("build", |_, var, ()| Ok(var.build()));
        methods.add_method("write", |_, var, path: String| {
            utils::treat_error(var.write(&path))
        });
    }
}

impl UserData for Form {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("print", |_, var, ()| Ok(var.print()));
        methods.add_method("is_whitespace", |_, var, ()| Ok(var.is_whitespace()));
        methods.add_method("is_linespace", |_, var, ()| Ok(var.is_linespace()));
        methods.add_method("is_linebreak", |_, var, ()| Ok(var.is_linebreak()));
        methods.add_method("is_code_brackets", |_, var, ()| Ok(var.is_code_brackets()));
        methods.add_method("is_text_brackets", |_, var, ()| Ok(var.is_text_brackets()));
        methods.add_method(
            "is_text_quotation",
            |_, var, ()| Ok(var.is_text_quotation()),
        );
    }
}

pub fn code(source: &str) -> Forms {
    dbg_stp!(source);
    CODE_PARSER.parse(source)
}

pub fn edit() -> Forms {
    dbg_stp!();
    Forms::edit()
}

pub fn desk(terms: Vec<String>) -> Forms {
    dbg_stp!(terms);
    let mut desk: Vec<Form> = Vec::new();
    for term in terms {
        desk.push(Form::from(term));
    }
    Forms::new(desk)
}

pub fn form(part: &str) -> Form {
    dbg_stp!(part);
    Form::new(part)
}

pub fn liz_suit_path(path: &str) -> Result<String, LizError> {
    dbg_stp!(path);
    let os_sep = liz_paths::os_sep().to_string();
    let path = if path.contains("\\") && os_sep != "\\" {
        path.replace("\\", &os_sep)
    } else {
        String::from(path)
    };
    let path = if path.contains("/") && os_sep != "/" {
        path.replace("/", &os_sep)
    } else {
        path
    };
    let check_ext = path.to_lowercase();
    let path = if !(check_ext.ends_with(".liz") || check_ext.ends_with(".lua")) {
        format!("{}.liz", path)
    } else {
        path
    };
    let path = if path.contains("$pwd") {
        path.replace(
            "$pwd",
            liz_paths::wd().map_err(|err| dbg_bub!(err))?.as_ref(),
        )
    } else {
        path
    };
    let path = if path.contains("$liz") {
        path.replace(
            "$liz",
            liz_fires::liz_dir().map_err(|err| dbg_bub!(err))?.as_ref(),
        )
    } else {
        path
    };
    Ok(path)
}

static UPDATE_LIZS: AtomicBool = AtomicBool::new(false);

pub fn is_update_lizs() -> bool {
    UPDATE_LIZS.load(Ordering::Acquire)
}

pub fn set_update_lizs(to: bool) {
    UPDATE_LIZS.store(to, Ordering::Release)
}

pub fn gotta_lizs(path: &str) -> Result<(), LizError> {
    dbg_stp!(path);
    if let Some(lizs_pos) = get_lizs_path_pos(path) {
        if is_update_lizs() || !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path).map_err(|err| dbg_bub!(err))?;
            std::fs::create_dir_all(path_dir).map_err(|err| dbg_err!(err))?;
            let net_path = (&path[lizs_pos + 7..]).replace("\\", "/");
            get_lizs_file(&net_path, path).map_err(|err| dbg_bub!(err))?;
        }
    }
    Ok(())
}

pub fn get_lizs(net_path: &str) -> Result<(), LizError> {
    dbg_stp!(net_path);
    let local_path = liz_paths::path_join(".lizs", net_path).map_err(|err| dbg_bub!(err))?;
    dbg_stp!(local_path);
    get_lizs_file(&net_path, &local_path).map_err(|err| dbg_bub!(err))
}

pub fn get_lizs_path_pos(path: &str) -> Option<usize> {
    dbg_stp!(path);
    let sep = if path.contains("\\") { "\\" } else { "/" };
    let lizs_dir = format!("{}.lizs{}", sep, sep);
    path.rfind(&lizs_dir)
}

pub fn get_lizs_file(net_path: &str, local_path: &str) -> Result<(), LizError> {
    dbg_stp!(net_path, local_path);
    let origin = format!(
        "https://raw.githubusercontent.com/emuvi/lizs/main/{}",
        &net_path
    );
    liz_winds::download(&origin, local_path, None).map_err(|err| dbg_bub!(err))
}

pub fn git_root_find(path: &str) -> Result<Option<String>, LizError> {
    dbg_stp!(path);
    let mut actual = liz_paths::path_absolute(path).map_err(|err| dbg_err!(err))?;
    loop {
        let check = liz_paths::path_join(&actual, ".git").map_err(|err| dbg_err!(err))?;
        if liz_paths::is_dir(&check) {
            return Ok(Some(actual));
        }
        actual = liz_paths::path_parent(&actual).map_err(|err| dbg_err!(err))?;
        if actual.is_empty() {
            break;
        }
    }
    Ok(None)
}

pub fn git_is_ignored(path: &str) -> Result<bool, LizError> {
    dbg_stp!(path);
    if let Some(root) = git_root_find(path).map_err(|err| dbg_err!(err))? {
        let relative = liz_paths::path_relative(path, &root).map_err(|err| dbg_err!(err))?;
        let (code, output) = liz_fires::cmd(
            "git",
            &["check-ignore", &relative],
            Some(&root),
            Some(false),
            Some(false),
        )
        .map_err(|err| dbg_err!(err))?;
        return Ok(code == 0 && !output.is_empty());
    }
    Ok(false)
}

pub fn git_has_changes(root: &str) -> Result<bool, LizError> {
    dbg_stp!(root);
    let (_, output) = liz_fires::cmd("git", &["status"], Some(root), Some(false), Some(true))
        .map_err(|err| dbg_err!(err))?;
    let output = output.trim();
    Ok(!output.ends_with("nothing to commit, working tree clean"))
}
