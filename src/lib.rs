use std::error::Error;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};
use winreg::RegKey;

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env = hklm.open_subkey_with_flags(
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
        KEY_READ | KEY_WRITE,
    )?;
    let env_path: String = env.get_value("PATH").unwrap_or_default();

    // if not exist then add
    if config.query == "add" && !env_path.split(";").any(|p| p == config.path) {
        let new_env_path = format!("{};{}", env_path, config.path);
        env.set_value("PATH", &new_env_path)?;
    }

    // remove from path
    if config.query == "remove" {
        let new_env_path = env_path
            .split(";")
            .filter(|p| *p != config.path)
            .collect::<Vec<&str>>()
            .join(";");
        env.set_value("PATH", &new_env_path)?;
    }

    Ok(())
}
