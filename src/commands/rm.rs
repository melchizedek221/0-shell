use std::{
    fs,
    io::{self, Error, ErrorKind},
    path::PathBuf,
};

fn remove_path(p: &PathBuf, recursive: bool) -> io::Result<()> {
    if !p.exists() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "rm: cannot remove {:?}: No such file or directory",
                p.display()
            ),
        ));
    } else if p.is_dir() && recursive {
        fs::remove_dir_all(p)?;
    } else if p.is_file() && p.is_symlink() {
        fs::remove_file(p)?;
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "rm: cannot remove {:?}: No such file or directory",
                p.display()
            ),
        ));
    }
    Ok(())
}

pub fn rm(args: &[&str]) -> io::Result<()> {
    if args.is_empty() || (args.len() == 1 && args[0].eq_ignore_ascii_case("-r")) {
        return Err(Error::new(ErrorKind::Other, "rm: missing arguments"));
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
