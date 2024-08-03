use std::{
    fs,
    io::{stdout, Write},
    path::PathBuf,
};

pub fn rm(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() || (args.len() == 1 && args[0].eq_ignore_ascii_case("-r")) {
        stdout().write_all(b"rm: missing operand\n")?
    } else if args[0].eq_ignore_ascii_case("-r") {
        for arg in &args[1..] {
            let p = PathBuf::from(arg);
            if !p.exists() {
                let s = format!("rm: cannot remove {:?}: No such file or directory\n", p);
                stdout().write_all(s.replace('"', "").as_bytes())?;
            } else if p.is_dir() {
                fs::remove_dir_all(&p)?;
            } else {
                fs::remove_file(&p)?;
            }
        }
    } else {
        for arg in args {
            let p = PathBuf::from(arg);
            if !p.exists() {
                let s = format!("rm: cannot remove {:?}: No such file or directory\n", p);
                stdout().write_all(s.replace('"', "").as_bytes())?;
            } else {
                fs::remove_file(&p)?;
            }
        }
    }

    Ok(())
}
