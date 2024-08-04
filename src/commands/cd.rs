use std::{
    env,
    io::{stdout, Write},
    path::PathBuf,
};
use colored::Colorize;

pub fn cd(args: &[&str]) -> std::io::Result<()> {
    match args.len() {
        0 => {
            let home_dir = env::var("HOME").expect("O-shell: cd: HOME not set");
            env::set_current_dir(home_dir)?;
        }
        1 => {
            let path = PathBuf::from(args[0]);
            if !path.exists() {
                let s = format!("O-shell: cd: {}: No such file or directory\n", path.display()).red().bold();
                stdout().write_all(s.as_bytes())?;
                return Ok(());
            }
            env::set_current_dir(&path)?;
        }
        _ => {
            let s =  "O-shell: cd: too many arguments\n".red().bold();
            stdout().write_all(s.as_bytes())?;
        }
    }

    Ok(())
}