use std::{
    env,
    io::{stdout, Write},
};

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let s = format!("{:?}\n", path);
    stdout().write_all(s.replace('"', "").as_bytes())?;
    Ok(())
}
