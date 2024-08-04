use std::{
    fs,
    io::{self, stdout, Write},
    path::{Path, PathBuf},
};

use colored::Colorize;

pub fn mv(args: &[&str]) -> io::Result<()> {
    if args.len() != 2 {
        return write_message(if args.is_empty() {
            "mv: missing arguments\n".red().bold()
        } else {
            "mv: takes two arguments source destination\n".red().bold()
        });
    }

    let src = PathBuf::from(args[0]);
    let dst = PathBuf::from(args[1]);

    if src == dst {
        return Ok(());
    }

    if src.is_dir() {
        move_directory(&src, &dst)?;
    } else {
        move_file_or_to_dir(&src, &dst)?;
    }

    Ok(())
}

fn write_message(message: colored::ColoredString) -> io::Result<()> {
    stdout().write_all(message.as_bytes())
}

fn move_directory(src: &Path, dst: &Path) -> io::Result<()> {
    let new_dest = if dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };

    move_dir_recursive(src, &new_dest)?;
    fs::remove_dir(src)
}

fn move_file_or_to_dir(src: &Path, dst: &Path) -> io::Result<()> {
    let new_dest = if dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };

    fs::rename(src, new_dest)
}

fn move_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(src_path.file_name().unwrap());

        if src_path.is_dir() {
            move_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::rename(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
