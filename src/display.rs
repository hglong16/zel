// src/display.rs

use std::env;
use std::path::{Path, PathBuf};
use crate::context::Context;

pub fn get_prompt(ctx: &Context) -> String {
    let home_dir = env::var("HOME").unwrap_or_default();
    let home_path = Path::new(&home_dir);

    let current_dir = &ctx.current_dir;

    if current_dir.starts_with(home_path) && !home_dir.is_empty() {
        match current_dir.strip_prefix(home_path) {
            Ok(rest) => {
                if rest.as_os_str().is_empty() {
                    format!("~")
                } else {
                    format!("~/{}", rest.display())
                }
            },
            Err(_) => current_dir.display().to_string()
        }

    } else {
        current_dir.display().to_string()
    }
}
