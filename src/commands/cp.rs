use std::{fs, path::PathBuf};

pub fn cp(args: &[&str]) -> std::io::Result<()> {
    let src = PathBuf::from(args[0]);
    let dst = PathBuf::from(args[1]);

    if src.is_file() {
        match dst.is_dir() {
            true => {
                let p = dst.join(src.clone());
                fs::copy(src, p)?;
            }
            false => {
                fs::copy(src.clone(), dst.clone())?;
            }
        }
    }

    Ok(())
}
