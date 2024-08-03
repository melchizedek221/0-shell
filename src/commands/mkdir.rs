use std::{fs, path::PathBuf};

pub fn mkdir(args: &[&str]) -> std::io::Result<()> {
    for arg in args {
        let p = PathBuf::from(arg);
        fs::create_dir(p)?;
    }

    Ok(())
}
