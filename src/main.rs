use std::io::{self, Write};

fn main() {
    println!("--- Rust Shell (Step 1) ---");
    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Err: {}", e);
            continue;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();
                if command.is_empty() {
                    continue;
                }

                if command == "exit" {
                    println!("Goodbye.");
                    break;
                }

                println!("Quick input: {}", command);
            },
            Err(error) => { 
                eprintln!("Error input, {}", error)
             }
        
        }
    }
    println!("Hello, world!");
}
