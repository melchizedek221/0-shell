use std::{
    env,
    io::{stdout, Write},
    path::PathBuf,
};

pub fn cd(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        #[allow(deprecated)]
        if let Some(p) = env::home_dir() {
            env::set_current_dir(p)?;
        }
    } else if args.len() != 1 {
        stdout().write_all(b"O-shell: cd: too many arguments\n")?;
    } else {
        let p = PathBuf::from(args[0]);
        if !p.exists() {
            let s = format!("O-shell: cd: {:?}: No such file or directory\n", p);
            stdout().write_all(s.replace('"', "").as_bytes())?;
        }
        env::set_current_dir(PathBuf::from(args[0]))?;
    }

    Ok(())
}
