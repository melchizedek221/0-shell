use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};

pub fn cat(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader);
    } else {
        for arg in args {
            let byte_arr = fs::read(PathBuf::from(arg)).expect("Unable to read file ");
            io::stdout().write_all(&byte_arr)?;
        }
    }

    Ok(())
}

fn process_lines<T: BufRead + Sized>(reader: T) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{}", line);
    }
}
