use std::{
    env,
    io::{stdout, Write},
};

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let path_str = format!("{:?}\n", path).replace('"', "");
    stdout().write_all(path_str.as_bytes())?;
    stdout().flush()?;
    Ok(())
}
