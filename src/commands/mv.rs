use std::{
    fs,
    io::{self, stdout, Write},
    path::PathBuf,
};

pub fn mv(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        stdout().write_all(b"mv: missing arguments\n")?
    } else if args.len() != 2 {
        stdout().write_all(b"mv: takes two arguments source destination\n")?;
    } else {
        let src = PathBuf::from(args[0]);
        let dst = PathBuf::from(args[1]);

        if src == dst {
            return Ok(());
        }

        if src.is_dir() {
            // Handle moving directories
            if dst.is_dir() {
                let new_dest = dst.join(src.file_name().unwrap());
                move_dir(&src, &new_dest)?;
            } else {
                // If destination is not a directory, treat it as a file path
                move_dir(&src, &dst)?;
            }
        } else if dst.is_dir() {
            let new_dest = dst.join(src.file_name().unwrap());
            fs::rename(src, new_dest)?;
        } else {
            fs::rename(src, dst)?;
        }
    }

    Ok(())
}

/// Recursively move a directory and its contents to a new location.
fn move_dir(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    // Create the destination directory
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src = entry.path();
        let dest_path = dest.join(src.file_name().unwrap());

        if src.is_dir() {
            move_dir(&src, &dest_path)?;
        } else {
            fs::rename(&src, &dest_path)?;
        }
    }

    // Remove the source directory after moving all contents
    fs::remove_dir(src)?;
    Ok(())
}
