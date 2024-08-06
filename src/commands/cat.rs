use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn cat(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader)?;
    } else {
        for &arg in args {
            let byte_arr = fs::read(PathBuf::from(arg))?;
            io::stdout().write_all(&byte_arr)?;
        }
    }

    io::stdout().flush()?;

    Ok(())
}

fn process_lines<T: BufRead>(reader: T) -> io::Result<()> {
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
