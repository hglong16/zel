// src/builtins.rs

use std::env;
use std::path::Path;

use crate::context::Context;

pub fn run_cd(args: &[String], ctx: &mut Context) -> bool {
    let new_dir = if args.len() < 2 {
        match env::var("HOME") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Err, cannot find HOME");
                return true;
            }
        }
    } else {
        args[1].clone()
    };

    let path = Path::new(&new_dir);
    match env::set_current_dir(path) {
        Ok(_) => {
            ctx.current_dir = env::current_dir().unwrap_or(path.to_path_buf());
        }
        Err(e) => eprintln!("cd error, cannot change dir to '{}', {}", new_dir, e),
    };

    true
}

pub fn run_exit(_args: &[String]) -> bool {
    false
}
