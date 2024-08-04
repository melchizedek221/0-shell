use std::{fs, path::PathBuf};

pub fn cp(args: &[&str]) -> std::io::Result<()> {
    let src = PathBuf::from(args[0]);
    let mut dst = PathBuf::from(args[1]);

    if dst.is_dir() {
        dst = dst.join(src.file_name().unwrap());
    }

    if src.is_file() {
        fs::copy(src, dst)?;
    }

    Ok(())
}
