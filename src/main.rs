mod commands;

use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::io::{self, stdin, Error, ErrorKind, Write};

use commands::{cat, cd, clear, cp, echo, ls, mkdir, mv, pwd, rm};

type CommandFn = fn(&[&str]) -> io::Result<()>;

fn main() {
    let mut command_map: HashMap<&str, CommandFn> = HashMap::new();

    // Builtins commands
    command_map.insert("cat", cat);
    command_map.insert("cd", cd);
    command_map.insert("cp", cp);
    command_map.insert("echo", echo);
    command_map.insert("ls", ls);
    command_map.insert("mkdir", mkdir);
    command_map.insert("mv", mv);
    command_map.insert("pwd", |_: &[&str]| pwd());
    command_map.insert("rm", rm);

    // OTHER Command
    command_map.insert("clear", |_: &[&str]| clear());

    loop {
        if let Err(e) = run_shell(&command_map) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                break;
            }
            eprintln!("{}", format!("Error: {}", e).red().to_string().as_str());
        }
    }
}

/// Runs the shell and executes commands based on user input
fn run_shell(command_map: &HashMap<&str, CommandFn>) -> io::Result<()> {
    let path = env::current_dir()?;
    let path_str = format!("{:?}\n", path)
        .trim_end()
        .replace('"', "")
        .cyan()
        .bold();

    print!("[{}] -> $ ", path_str);
    io::stdout().flush()?;

    let mut user_input = String::new();

    if stdin().read_line(&mut user_input)? == 0 {
        println!("\nexit");
        return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
    }

    let command_to_execute = user_input.trim();
    if command_to_execute.is_empty() {
        return Ok(());
    }

    let args: Vec<&str> = command_to_execute.split_whitespace().collect();

    match args[0] {
        "exit" => std::process::exit(0),
        cmd_input => {
            if let Some(&cmd_func) = command_map.get(cmd_input) {
                cmd_func(&args[1..])?;
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Command '{}' not found", cmd_input),
                ));
            }
        }
    }

    Ok(())
}
