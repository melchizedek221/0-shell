mod commands;

use std::io::Write;
use std::io::{stdin, stdout};

use commands::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm};

fn main() {
    loop {
        print!("$ ");

        stdout().flush().unwrap();

        let mut user_input = String::new();

        let n = stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input");

        if n == 0 {
            println!("\nexit");
            break;
        }

        let command_to_execute = user_input.trim();
        let command_args: Vec<&str> = command_to_execute.split_whitespace().collect();

        match command_args[0] {
            "cat" => {
                let _ = cat(&command_args[1..]);
            }
            "cd" => {
                let _ = cd(&command_args[1..]);
            }
            "cp" => {
                let _ = cp(&command_args[1..]);
            }
            "echo" => {
                let _ = echo(&command_args[1..]);
            }
            "ls" => {
                let _ = ls(&command_args[1..]);
            }
            "mkdir" => {
                let _ = mkdir(&command_args[1..]);
            }
            "mv" => {
                let _ = mv(&command_args[1..]);
            }
            "pwd" => {
                let _ = pwd();
            }
            "rm" => {
                let _ = rm(&command_args[1..]);
            }
            "exit" => break,
            unknown => {
                eprintln!("Command '{}' not found", unknown);
            }
        }

        // println!("{:?}", command_args);

        // let mut child = Command::new(command_args[0])
        //     .args(&command_args[1..])
        //     .spawn()
        //     .expect("Unable to execute command");

        // child.wait().unwrap();
    }
}
