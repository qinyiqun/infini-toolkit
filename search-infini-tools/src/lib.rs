#![deny(warnings)]

use std::{
    env::{set_var, var},
    path::PathBuf,
};

pub fn find_infer_cc() -> Option<PathBuf> {
    find_env("INFER_CC_SRC").map(PathBuf::from)
}

pub fn get_or_set_infini_root() -> PathBuf {
    const INFINI_ROOT: &str = "INFINI_ROOT";
    find_env(INFINI_ROOT).map(PathBuf::from).unwrap_or_else(|| {
        const HOME: &str = if cfg!(windows) { "HOMEPATH" } else { "HOME" };
        let root = find_env(HOME).map(PathBuf::from).unwrap().join(".infini");
        set_var(INFINI_ROOT, &root);
        root
    })
}

pub fn extra_config() -> String {
    find_env("INFER_CC_EXTRA_CONFIG").unwrap_or_default()
}

fn find_env(key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={key}");
    var(key).ok()
}
