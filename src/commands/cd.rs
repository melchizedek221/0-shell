use std::{
    env,
    io::{Error, ErrorKind},
    path::PathBuf,
};

pub fn cd(args: &[&str]) -> std::io::Result<()> {
    match args.len() {
        0 => {
            let home_dir = env::var("HOME").expect("O-shell: cd: HOME not set");
            env::set_current_dir(home_dir)?;
        }
        1 => {
            let path = PathBuf::from(args[0]);
            if !path.exists() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("O-shell: cd: {}: No such file or directory", path.display()),
                ));
            }
            env::set_current_dir(&path)?;
        }
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                "O-shell: cd: too many arguments",
            ));
        }
    }

    Ok(())
}
