use std::io::{stdout, Write};

pub fn echo(args: &[&str]) -> std::io::Result<()> {
    let binding = args.join(" ");
    let binding = binding.replace('"', "");
    let data = binding.as_bytes();

    stdout().write_all(data)?;
    stdout().write_all(b"\n")?;

    Ok(())
}
