use chrono::{DateTime, Local};
use colored::Colorize;
use libc::{getgrgid, getpwuid};
use std::cmp::Ordering;
use std::ffi::CStr;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut long_format = false;
    let mut all_files = false;
    let mut classify = false;

    let mut paths: Vec<PathBuf> = Vec::new();

    for arg in args {
        match *arg {
            "-l" => long_format = true,
            "-a" => all_files = true,
            "-F" => classify = true,
            _ => paths.push(PathBuf::from(arg)),
        }
    }

    if paths.is_empty() {
        paths.push(PathBuf::from("."));
    }

    for path in paths {
        if path.is_dir() {
            let mut entries: Vec<DirEntry> =
                fs::read_dir(&path)?.filter_map(|res| res.ok()).collect();

            let total_blocks = calculate_total_blocks(&entries, all_files)?;

            if all_files && classify && !long_format {
                print!("{}/  {}/", ".".blue(), "..".blue());
                if !entries.is_empty() {
                    print!("  ");
                }
            } else if all_files && !long_format {
                print!("{}  {}", ".".blue(), "..".blue());
                if !entries.is_empty() {
                    print!("  ");
                }
            } else if long_format && all_files {
                println!("total {}", total_blocks);

                let dot_metadata = fs::metadata(PathBuf::from("."))?;
                let dot_str = get_string_metadata(&dot_metadata)?;
                let parent_metadata = fs::metadata(PathBuf::from(".."))?;
                let parent_str = get_string_metadata(&parent_metadata)?;

                if classify {
                    println!("{} {}/", dot_str, ".".blue());
                    println!("{} {}/", parent_str, "..".blue());
                } else {
                    println!("{} {}", dot_str, ".".blue());
                    println!("{} {}", parent_str, "..".blue());
                }
            }

            sort_entries(&mut entries);

            for (i, entry) in entries.iter().enumerate() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().into_owned();

                if !all_files && file_name_str.starts_with('.') {
                    continue;
                }

                if long_format {
                    if !all_files && i == 0 {
                        println!("total {}", total_blocks);
                    }
                    let metadata = entry.metadata()?;
                    let str_metadata = get_string_metadata(&metadata)?;

                    let file_name_str = match metadata.is_dir() {
                        true => file_name_str.blue(),
                        false => file_name_str.normal(),
                    };

                    if classify {
                        if metadata.is_dir() {
                            println!("{} {}/", str_metadata, file_name_str);
                        } else if metadata.permissions().mode() & 0o111 != 0 {
                            println!("{} {}*", str_metadata, file_name_str);
                        } else {
                            println!("{} {}", str_metadata, file_name_str);
                        }
                    } else {
                        println!("{} {}", str_metadata, file_name_str);
                    }
                } else {
                    let metadata = entry.metadata()?;
                    let file_name_str = match metadata.is_dir() {
                        true => file_name_str.blue(),
                        false => file_name_str.normal(),
                    };
                    print!("{}", file_name_str);

                    if classify {
                        if metadata.is_dir() {
                            print!("/");
                        } else if metadata.permissions().mode() & 0o111 != 0 {
                            print!("*");
                        }
                    }

                    print!("  ");
                }
            }
            if !long_format {
                println!()
            }
        } else {
            if long_format {
                let metadata = fs::metadata(path.clone())?;
                let str_metadata = get_string_metadata(&metadata)?;

                println!("{} {:?}", str_metadata, path);
            } else {
                println!("{:?}", path);
            }
        }
    }

    Ok(())
}

fn sort_entries(entries: &mut Vec<DirEntry>) {
    entries.sort_by(|a, b| {
        let binding = a.file_name();
        let a_str = binding.to_string_lossy();
        let binding = b.file_name();
        let b_str = binding.to_string_lossy();

        // Special case for "." and ".."
        if a_str == "." && b_str == ".." {
            return Ordering::Less;
        }
        if a_str == ".." && b_str == "." {
            return Ordering::Greater;
        }

        // Determine the starting index for comparison
        let a_start = if a_str.starts_with('.') { 1 } else { 0 };
        let b_start = if b_str.starts_with('.') { 1 } else { 0 };

        // Compare the substrings starting from the determined indices
        let a_sub = &a_str[a_start..];
        let b_sub = &b_str[b_start..];

        a_sub.cmp(b_sub).then_with(|| a_str.cmp(&b_str))
    });
}

fn last_modified_time(metadata: &fs::Metadata) -> Result<String, io::Error> {
    let modified = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)
        .expect("error");
    let datetime: DateTime<Local> = DateTime::from(UNIX_EPOCH + modified);
    let formatted_time = datetime.format("%b %e %H:%M").to_string();
    Ok(formatted_time)
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
        other_exec,
    )
}

fn get_string_metadata(metadata: &fs::Metadata) -> Result<String, io::Error> {
    let mode = metadata.mode();
    let nlink = metadata.nlink();
    let (username, group) = get_user_group_names(metadata.uid(), metadata.gid());
    let size = metadata.size();
    let perm = get_file_permission(metadata.is_dir(), mode);
    let last_time = last_modified_time(metadata)?;

    Ok(format!(
        "{} {} {} {} {} {}",
        perm, nlink, username, group, size, last_time
    ))
}

fn get_user_group_names(uid: u32, gid: u32) -> (String, String) {
    // Retrieve user name from UID
    let user_name = unsafe {
        let pw = getpwuid(uid);
        if pw.is_null() {
            "unknown".to_string()
        } else {
            CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
        }
    };

    // Retrieve group name from GID
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

fn calculate_total_blocks(entries: &Vec<DirEntry>, for_hidden: bool) -> io::Result<u64> {
    let mut total_blocks = 0;

    for entry in entries {
        
            let metadata = entry.metadata()?;

        total_blocks += metadata.blocks() ;
    }

    if for_hidden {
        let dot_metadata = fs::metadata(PathBuf::from("."))?;
        total_blocks += &dot_metadata.blocks();
        let parent_metadata = fs::metadata(PathBuf::from(".."))?;
        total_blocks += &parent_metadata .blocks();
    }

    Ok(total_blocks /2)
}

