use rlua::{UserData, UserDataMethods};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::liz_debug::{dbg_bleb, dbg_erro};
use crate::liz_debug::{dbg_call, dbg_reav, dbg_step};
use crate::liz_fires;
use crate::liz_forms::{self, Forms};
use crate::liz_group::{self, GroupPair};
use crate::liz_parse::{self, BlockBy};
use crate::liz_paths;
use crate::liz_winds;
use crate::utils;
use crate::LizError;

static LIZS_UPDATE: AtomicBool = AtomicBool::new(false);

pub fn is_lizs_update() -> bool {
    dbg_call!();
    dbg_reav!(LIZS_UPDATE.load(Ordering::Acquire));
}

pub fn set_lizs_update(always: bool) {
    dbg_call!(always);
    LIZS_UPDATE.store(always, Ordering::Release)
}

pub fn liz_suit_path(path: &str) -> Result<String, LizError> {
    dbg_call!(path);
    let os_sep = liz_paths::os_sep().to_string();
    dbg_step!(os_sep);
    let result = if path.contains("\\") && os_sep != "\\" {
        path.replace("\\", &os_sep)
    } else {
        String::from(path)
    };
    dbg_step!(result);
    let result = if result.contains("/") && os_sep != "/" {
        result.replace("/", &os_sep)
    } else {
        result
    };
    dbg_step!(result);
    let check_ext = result.to_lowercase();
    let result = if !(check_ext.ends_with(".liz") || check_ext.ends_with(".lua")) {
        format!("{}.liz", result)
    } else {
        result
    };
    dbg_step!(result);
    let result = if result.contains("$pwd") {
        result.replace(
            "$pwd",
            liz_paths::wd().map_err(|err| dbg_bleb!(err))?.as_ref(),
        )
    } else {
        result
    };
    dbg_step!(result);
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
        dbg_step!(lizs_pos);
        if is_lizs_update() || !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path).map_err(|err| dbg_bleb!(err))?;
            dbg_step!(path_dir);
            std::fs::create_dir_all(path_dir).map_err(|err| dbg_erro!(err))?;
            let net_path = (&path[lizs_pos + 7..]).replace("\\", "/");
            dbg_step!(net_path);
            get_lizs_file(&net_path, path).map_err(|err| dbg_bleb!(err))?;
        }
    }
    Ok(())
}

pub fn get_lizs(net_path: &str) -> Result<(), LizError> {
    dbg_call!(net_path);
    let local_path = liz_paths::path_join(".lizs", net_path).map_err(|err| dbg_bleb!(err))?;
    dbg_step!(local_path);
    get_lizs_file(&net_path, &local_path).map_err(|err| dbg_bleb!(err))
}

pub fn get_lizs_path_pos(path: &str) -> Option<usize> {
    dbg_call!(path);
    let separator = if path.contains("\\") { '\\' } else { '/' };
    dbg_step!(separator);
    let mut lizs_dir = format!(".lizs{}", separator);
    dbg_step!(lizs_dir);
    if path.starts_with(&lizs_dir) {
        dbg_reav!(Some(0));
    }
    lizs_dir.insert(0, separator);
    dbg_step!(lizs_dir);
    dbg_reav!(path.rfind(&lizs_dir));
}

pub fn get_lizs_file(net_path: &str, local_path: &str) -> Result<(), LizError> {
    dbg_call!(net_path, local_path);
    let origin = format!(
        "https://raw.githubusercontent.com/emuvi/lizs/main/{}",
        &net_path
    );
    dbg_step!(origin);
    liz_winds::download(&origin, local_path, None).map_err(|err| dbg_bleb!(err))
}

pub fn git_root_find(path: &str) -> Result<Option<String>, LizError> {
    dbg_call!(path);
    let mut actual = liz_paths::path_absolute(path).map_err(|err| dbg_bleb!(err))?;
    dbg_step!(actual);
    loop {
        let check = liz_paths::path_join(&actual, ".git").map_err(|err| dbg_bleb!(err))?;
        dbg_step!(check);
        if liz_paths::is_dir(&check) {
            dbg_reav!(Ok(Some(actual)));
        }
        actual = liz_paths::path_parent(&actual).map_err(|err| dbg_erro!(err))?;
        dbg_step!(actual);
        if actual.is_empty() {
            break;
        }
    }
    dbg_reav!(Ok(Some(actual)));
}

pub fn git_is_ignored(path: &str) -> Result<bool, LizError> {
    dbg_call!(path);
    if let Some(root) = git_root_find(path).map_err(|err| dbg_bleb!(err))? {
        dbg_step!(root);
        let relative = liz_paths::path_relative(path, &root).map_err(|err| dbg_bleb!(err))?;
        dbg_step!(relative);
        let (code, output) = liz_fires::cmd(
            "git",
            &["check-ignore", &relative],
            Some(&root),
            Some(false),
            Some(false),
        )
        .map_err(|err| dbg_bleb!(err))?;
        dbg_step!(code, output);
        dbg_reav!(Ok(code == 0 && !output.is_empty()));
    }
    dbg_reav!(Ok(false));
}

pub fn git_has_changes(root: &str) -> Result<bool, LizError> {
    dbg_call!(root);
    let (_, output) = liz_fires::cmd("git", &["status"], Some(root), Some(false), Some(true))
        .map_err(|err| dbg_bleb!(err))?;
    dbg_step!(output);
    let output = output.trim();
    dbg_step!(output);
    dbg_reav!(Ok(
        !output.ends_with("nothing to commit, working tree clean")
    ));
}

pub fn edit() -> Forms {
    dbg_call!();
    dbg_reav!(Forms { desk: Vec::new() });
}

pub fn code(source: String) -> Forms {
    dbg_call!(source);
    dbg_reav!(Forms { desk: vec![source] });
}

pub fn desk(terms: Vec<String>) -> Forms {
    dbg_call!(terms);
    dbg_reav!(Forms { desk: terms });
}

impl UserData for Forms {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Basic Methods
        methods.add_method("len", |_, slf, ()| Ok(liz_forms::kit_len(&slf.desk)));

        methods.add_method("get", |_, slf, index: usize| {
            Ok(liz_forms::kit_get(&slf.desk, index).to_string())
        });

        methods.add_method_mut("set", |_, slf, (index, form): (usize, String)| {
            Ok(liz_forms::kit_set(&mut slf.desk, index, form))
        });

        methods.add_method_mut("add", |_, slf, (index, form): (usize, String)| {
            Ok(liz_forms::kit_add(&mut slf.desk, index, form))
        });

        methods.add_method_mut("add_range", |_, slf, (on, range): (usize, Vec<String>)| {
            Ok(liz_forms::kit_add_range(&mut slf.desk, on, range))
        });

        methods.add_method_mut("put", |_, slf, form: String| {
            Ok(liz_forms::kit_put(&mut slf.desk, form))
        });

        methods.add_method_mut("del", |_, slf, index: usize| {
            Ok(liz_forms::kit_del(&mut slf.desk, index))
        });

        methods.add_method_mut("del_range", |_, slf, (from, till): (usize, usize)| {
            Ok(liz_forms::kit_del_range(&mut slf.desk, from, till))
        });

        methods.add_method_mut("pop", |_, slf, ()| Ok(liz_forms::kit_pop(&mut slf.desk)));

        // Find Methods
        methods.add_method("find_all", |_, slf, term: String| {
            Ok(liz_forms::kit_find_all(&slf.desk, &term))
        });

        methods.add_method("find_all_like", |_, slf, term: String| {
            Ok(liz_forms::kit_find_all_like(&slf.desk, &term))
        });

        methods.add_method("first_some", |_, slf, ()| {
            Ok(liz_forms::kit_first_some(&slf.desk))
        });

        methods.add_method("prior_some", |_, slf, of: usize| {
            Ok(liz_forms::kit_prior_some(&slf.desk, of))
        });

        methods.add_method("next_some", |_, slf, of: usize| {
            Ok(liz_forms::kit_next_some(&slf.desk, of))
        });

        methods.add_method("last_some", |_, slf, ()| {
            Ok(liz_forms::kit_last_some(&slf.desk))
        });

        // Mutate Methods
        methods.add_method_mut("change_all", |_, slf, (of, to): (String, String)| {
            Ok(liz_forms::kit_change_all(&mut slf.desk, &of, &to))
        });

        // Finish Methods
        methods.add_method("print_all", |_, slf, ()| {
            Ok(liz_forms::kit_print_all(&slf.desk))
        });

        methods.add_method("build", |_, slf, ()| Ok(liz_forms::kit_build(&slf.desk)));

        methods.add_method("write", |_, slf, path: String| {
            utils::treat_error(liz_forms::kit_write(&slf.desk, &path))
        });

        // Gather Methods
        methods.add_method_mut(
            "group_all",
            |_, slf, (groups, recursive): (Vec<GroupPair>, bool)| {
                let groupers = match utils::treat_error(liz_group::get_groupers(groups)) {
                    Ok(parsers) => parsers,
                    Err(err) => (return Err(err)),
                };
                utils::treat_error(liz_group::rig_group_all(&mut slf.desk, &groupers, recursive))
            },
        );

        methods.add_method_mut(
            "group_on",
            |_, slf, (from, till, groups, recursive): (usize, usize, Vec<GroupPair>, bool)| {
                let groupers = match utils::treat_error(liz_group::get_groupers(groups)) {
                    Ok(parsers) => parsers,
                    Err(err) => (return Err(err)),
                };
                utils::treat_error(liz_group::rig_group_on(
                    &mut slf.desk,
                    from,
                    till,
                    &groupers,
                    recursive,
                ))
            },
        );

        methods.add_method_mut("parse_all", |_, slf, blocks: Vec<BlockBy>| {
            let parsers = match utils::treat_error(liz_parse::get_parsers(blocks)) {
                Ok(parsers) => parsers,
                Err(err) => (return Err(err)),
            };
            utils::treat_error(liz_parse::rig_parse_all(&mut slf.desk, &parsers))
        });

        methods.add_method_mut(
            "parse_on",
            |_, slf, (from, till, blocks): (usize, usize, Vec<BlockBy>)| {
                let parsers = match utils::treat_error(liz_parse::get_parsers(blocks)) {
                    Ok(parsers) => parsers,
                    Err(err) => (return Err(err)),
                };
                utils::treat_error(liz_parse::rig_parse_on(&mut slf.desk, from, till, &parsers))
            },
        );
    }
}
