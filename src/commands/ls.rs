use chrono::{DateTime, Local};
use colored::Colorize;
use libc::{getgrgid, getpwuid};
use std::cmp::Ordering;
use std::ffi::CStr;
use std::fs::{self, DirEntry};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use tabular::{Row, Table};

pub fn ls(args: &[&str]) -> io::Result<()> {
    let (long_format, all_files, classify, paths) = parse_args(args);

    for path in paths {
        if path.is_dir() {
            let entries: Vec<DirEntry> = get_dir_entries(&path)?;

            if long_format {
                let total_blocks = calculate_total_blocks(&entries, all_files)?;
                print!(
                    "total {}\n{}",
                    total_blocks,
                    format_long_output(&entries, all_files, classify)?
                );
            } else {
                print!("{}", display(&entries, all_files, classify)?);
            }
        } else {
            let metadata = fs::metadata(&path)?;
            if long_format {
                let (perm, nlink, username, group, size, last_time) = extract_file_info(&metadata)?;
                println!(
                    "{} {} {} {} {} {} {}",
                    perm,
                    nlink,
                    username,
                    group,
                    size,
                    last_time,
                    path.to_string_lossy()
                );
            } else {
                println!("{}", path.to_string_lossy());
            }
        }
    }

    Ok(())
}

fn parse_args(args: &[&str]) -> (bool, bool, bool, Vec<PathBuf>) {
    let mut long_format = false;
    let mut all_files = false;
    let mut classify = false;
    let mut paths = Vec::new();

    for &arg in args {
        match arg {
            "-l" => long_format = true,
            "-a" => all_files = true,
            "-F" => classify = true,
            _ => paths.push(PathBuf::from(arg)),
        }
    }

    if paths.is_empty() {
        paths.push(PathBuf::from("."));
    }

    (long_format, all_files, classify, paths)
}

/// Get all entries in a directory and sort them by filename.
fn get_dir_entries(path: &PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = fs::read_dir(path)?.filter_map(|res| res.ok()).collect();

    entries.sort_by(|a, b| {
        let a_str = a.file_name().to_string_lossy().into_owned();
        let b_str = b.file_name().to_string_lossy().into_owned();
        compare_entries(&a_str, &b_str)
    });

    Ok(entries)
}

/// Compares two strings based on their filename.
fn compare_entries(a_str: &str, b_str: &str) -> Ordering {
    let a_start = if a_str.starts_with('.') { 1 } else { 0 };
    let b_start = if b_str.starts_with('.') { 1 } else { 0 };

    let a_sub = &a_str[a_start..].to_lowercase();
    let b_sub = &b_str[b_start..].to_lowercase();

    a_sub.cmp(b_sub).then_with(|| a_str.cmp(b_str))
}

/// Add color to the file name if it is a directory.
fn colored_file_name_str(file_name: &str, metadata: &fs::Metadata) -> String {
    if metadata.is_dir() {
        file_name.blue().bold().to_string()
    } else {
        file_name.normal().to_string()
    }
}

fn display(entries: &[DirEntry], all_files: bool, classify: bool) -> io::Result<String> {
    let mut output = String::new();

    if all_files {
        let dot = ".".blue().bold().to_string();
        let dot = if classify { dot + "/  " } else { dot + "  " };
        output.push_str(&dot);
        let parent = "..".blue().bold().to_string();
        let parent = if classify {
            parent + "/  "
        } else {
            parent + "  "
        };
        output.push_str(&parent);
    }
    for (i, entry) in entries.iter().enumerate() {
        let file_name_str = entry.file_name().to_string_lossy().into_owned();

        if !all_files && file_name_str.starts_with('.') {
            continue;
        }
        let metadata = entry.metadata()?;
        let file_name_str = colored_file_name_str(&file_name_str, &metadata);
        output.push_str(&format!(
            "{}{}",
            file_name_str,
            classify_suffix(&metadata, classify)
        ));

        if i < entries.len() - 1 {
            output.push_str("  ");
        } else {
            output.push('\n');
        }
    }

    Ok(output)
}

fn classify_suffix(metadata: &fs::Metadata, classify: bool) -> String {
    if !classify {
        return String::new();
    }

    if metadata.is_dir() {
        "/".to_string()
    } else if metadata.permissions().mode() & 0o111 != 0 {
        "*".to_string()
    } else {
        String::new()
    }
}

fn last_modified_time(metadata: &fs::Metadata) -> Result<String, io::Error> {
    let modified = metadata.modified()?;
    let duration_since_epoch = modified
        .duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let datetime: DateTime<Local> = DateTime::from(UNIX_EPOCH + duration_since_epoch);
    Ok(datetime.format("%b %e %H:%M").to_string())
}

fn get_user_group_names(uid: u32, gid: u32) -> (String, String) {
    let user_name = unsafe {
        let pw = getpwuid(uid);
        if pw.is_null() {
            "unknown".to_string()
        } else {
            CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
        }
    };

    let group_name = unsafe {
        let gr = getgrgid(gid);
        if gr.is_null() {
            "unknown".to_string()
        } else {
            CStr::from_ptr((*gr).gr_name).to_string_lossy().into_owned()
        }
    };

    (user_name, group_name)
}

fn get_file_permission(is_dir: bool, mode: u32) -> String {
    let file_type = if is_dir { 'd' } else { '-' };
    let user_perms = if mode & 0o400 != 0 { 'r' } else { '-' };
    let user_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let user_exec = if mode & 0o100 != 0 { 'x' } else { '-' };
    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_exec = if mode & 0o010 != 0 { 'x' } else { '-' };
    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_exec = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type,
        user_perms,
        user_write,
        user_exec,
        group_read,
        group_write,
        group_exec,
        other_read,
        other_write,
        other_exec
    )
}

fn calculate_total_blocks(entries: &[DirEntry], for_hidden: bool) -> io::Result<u64> {
    let mut total_blocks = 0;
    for entry in entries {
        let metadata = entry.metadata()?;
        total_blocks += metadata.blocks();
    }

    if for_hidden {
        let dot_metadata = fs::metadata(".")?;
        total_blocks += dot_metadata.blocks();
        let parent_metadata = fs::metadata("..")?;
        total_blocks += parent_metadata.blocks();
    }

    Ok(total_blocks / 2)
}

fn format_long_output(entries: &[DirEntry], all_files: bool, classify: bool) -> io::Result<String> {
    let mut table = Table::new("{:<} {:>} {:>} {:>} {:>} {:>} {:<}");

    if all_files {
        let dot_metadata = fs::metadata(PathBuf::from("."))?;
        let (perm, nlink, username, group, size, last_time) = extract_file_info(&dot_metadata)?;
        table.add_row(
            Row::new()
                .with_cell(perm)
                .with_cell(nlink)
                .with_cell(username)
                .with_cell(group)
                .with_cell(size)
                .with_cell(last_time)
                .with_cell(
                    ".".blue().bold().to_string() + &classify_suffix(&dot_metadata, classify),
                ),
        );
        let parent_metadata = fs::metadata(PathBuf::from(".."))?;
        let (perm, nlink, username, group, size, last_time) = extract_file_info(&parent_metadata)?;
        table.add_row(
            Row::new()
                .with_cell(perm)
                .with_cell(nlink)
                .with_cell(username)
                .with_cell(group)
                .with_cell(size)
                .with_cell(last_time)
                .with_cell(
                    "..".blue().bold().to_string() + &classify_suffix(&parent_metadata, classify),
                ),
        );
    }
    for entry in entries {
        let file_name_str = entry.file_name().to_string_lossy().into_owned();
        if !all_files && file_name_str.starts_with('.') {
            continue;
        }
        let metadata = entry.metadata()?;

        let file_name_str = colored_file_name_str(&file_name_str, &metadata);
        let file_name_str = file_name_str + &classify_suffix(&metadata, classify);
        let (perm, nlink, username, group, size, last_time) = extract_file_info(&metadata)?;

        table.add_row(
            Row::new()
                .with_cell(perm)
                .with_cell(nlink)
                .with_cell(username)
                .with_cell(group)
                .with_cell(size)
                .with_cell(last_time)
                .with_cell(file_name_str),
        );
    }
    Ok(format!("{}", table))
}

fn extract_file_info(
    metadata: &fs::Metadata,
) -> Result<(String, u64, String, String, u64, String), io::Error> {
    let mode = metadata.mode();
    let nlink = metadata.nlink();
    let (username, group) = get_user_group_names(metadata.uid(), metadata.gid());
    let size = metadata.size();
    let perm = get_file_permission(metadata.is_dir(), mode);
    let last_time = last_modified_time(metadata)?;
    Ok((perm, nlink, username, group, size, last_time))
}
