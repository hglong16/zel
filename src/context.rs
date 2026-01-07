// src/context.rs

use std::env;
use std::path::PathBuf;

pub struct Context {
    pub current_dir: PathBuf,
}

impl Context {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| {
            PathBuf::from("/")
        });

        Self {
            current_dir
        }
    }
}

