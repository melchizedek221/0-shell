use std::io::{stdout, Write};

pub fn echo(args: &[&str]) -> std::io::Result<()> {
    let data = args.join(" ").replace('"', "");

    stdout().write_all(data.as_bytes())?;
    stdout().write_all(b"\n")?;
    stdout().flush()?;

    Ok(())
}
