// src/parser.rs

pub fn parse_cmd(input: &str) -> Vec<String> {
    match shell_words::split(input) {
        Ok(args) => args,
        Err(_) => {
            eprintln!(" Err command");
            Vec::new()
        }
    }
}