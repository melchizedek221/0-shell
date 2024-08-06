use std::io::{stdout, Write};

pub fn clear() -> std::io::Result<()> {
    stdout().write_all(b"\x1B[2J\x1B[1;1H")?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    stdout().flush()?;
    Ok(())
}