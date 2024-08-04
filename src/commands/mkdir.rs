use std::{fs, path::Path};

pub fn mkdir(args: &[&str]) -> std::io::Result<()> {
    args.iter().map(Path::new).try_for_each(fs::create_dir)
}
