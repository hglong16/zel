mod parser;
mod context;
mod excutor;
mod display;
mod builtins;
mod db;
mod history;


use std::io::{self, Write};

use crate::{context::Context, excutor::execute_command, history::History};

fn main() {
    println!("--- Rust Shell (Step 1) ---");
    let mut ctx = Context::new();
    let mut history = History::new().expect("Cannot create db");
    loop {
        let prompt = display::get_prompt(&ctx);
        print!("{} > ", prompt);
        io::stdout().flush().unwrap();
        
        if let Err(e) = io::stdout().flush() {
            eprintln!("Err: {}", e);
            continue;
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd_line = input.trim();
                if !cmd_line.is_empty() {
                    history.save(cmd_line).expect("ERROR: CANNOT SAVE CMD");
                }
                let args = parser::parse_cmd(&input);
                if args.is_empty() {
                    continue;
                }
                let command = &args[0];
                let should_continue = match command.as_str() {
                    "cd" => builtins::run_cd(&args, &mut ctx),
                    "history" => builtins::run_history(&args, &history),
                    "exit" => builtins::run_exit(&args),
                    _ => execute_command(&args)
                };

                if !should_continue {
                    break;
                }
            },
            Err(error) => { 
                eprintln!("Error input, {}", error)
             }
        
        }
    }
    println!("Hello, world!");
}
