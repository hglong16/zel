use std::process::Command;


pub fn execute_command(args: &[String]) -> bool{
    let command = &args[0];
    let arguments = &args[1..];

    let output = Command::new(command)
        .args(arguments)
        .spawn();

    match output {
        Ok(mut child) => {
            match child.wait() {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Err: {}", e);
                    true
                }
            }
        },
        Err(e) => {
            eprintln!("Err, cannot process {}, {}", command, e);
            true
        }
    }


    
}