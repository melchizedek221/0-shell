use colored::Colorize;
use std::{
    fs,
    io::{self, stdout, Write},
    path::PathBuf,
};

fn remove_path(p: &PathBuf, recursive: bool) -> io::Result<()> {
    if !p.exists() {
        stdout().write_all(
            format!("rm: cannot remove {:?}: No such file or directory\n", p)
                .replace('"', "")
                .red()
                .bold()
                .as_bytes(),
        )?;
    } else if p.is_dir() && recursive {
        fs::remove_dir_all(p)?;
    } else if p.is_file() && p.is_symlink() {
        fs::remove_file(p)?;
    } else {
        stdout().write_all(
            format!("rm: cannot remove {:?}: Not a file or directory\n", p)
                .replace('"', "")
                .red()
                .bold()
                .as_bytes(),
        )?;
    }
    Ok(())
}

pub fn rm(args: &[&str]) -> io::Result<()> {
    if args.is_empty() || (args.len() == 1 && args[0].eq_ignore_ascii_case("-r")) {
        stdout().write_all("rm: missing operand\n".red().bold().as_bytes())?;
    } else {
        let recursive = args[0].eq_ignore_ascii_case("-r");
        let paths = if recursive { &args[1..] } else { args };

        for arg in paths {
            let p = PathBuf::from(arg);
            remove_path(&p, recursive)?;
        }
    }
    Ok(())
}
