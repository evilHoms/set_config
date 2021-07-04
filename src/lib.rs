use core::fmt;
use std::{env, fmt::Display, fs::{self}, io};
use std::ffi::OsString;

const SETTINGS_PATH: &str = "./settings.json";

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum SETTINGS {
    CONFIG_PATH,
    CONFIG_PATTERN,
    DEST_CONFIG_PATH,
    DEST_CONFIG_NAME,
}

impl Display for SETTINGS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_settings_value(settings: &serde_json::Value, key: SETTINGS) -> Result<String, io::Error> {
    let key = key.to_string();
    if let Some(value) = settings.get(&key) {
        Ok(value.as_str().unwrap().to_owned())
    } else {
        let error = format!("Error: no {} specified in {}", key, SETTINGS_PATH);
        return Err(io::Error::new(io::ErrorKind::NotFound, error));
    }
}

fn get_settings() -> Result<(String, String, String, String), io::Error> {
    let file = fs::File::open(SETTINGS_PATH)?;
    let settings: serde_json::Value = serde_json::from_reader(file)?;
    let config_path = get_settings_value(&settings, SETTINGS::CONFIG_PATH)?;
    let config_pattern = get_settings_value(&settings, SETTINGS::CONFIG_PATTERN)?;
    let dest_config_path = get_settings_value(&settings, SETTINGS::DEST_CONFIG_PATH)?;
    let dest_config_name = get_settings_value(&settings, SETTINGS::DEST_CONFIG_NAME)?;

    Ok((config_path, config_pattern, dest_config_path, dest_config_name))
}

pub fn get_config_names() -> Result<Vec<String>, io::Error> {
    let (config_path, config_pattern, _, _) = get_settings()?;

    let paths = fs::read_dir(&config_path)?;

    let mut config_names: Vec<String> = Vec::new();

    for path in paths {
        let config_file = path.unwrap().file_name().into_string().unwrap();

        if !config_file.contains(&config_pattern) {
            continue;
        }

        let config_name = config_file.split(".config").next();

        if let Some(name) = config_name {
            if name.len() == 0 {
                continue;
            }

            config_names.push(name.to_owned());
        }
    }

    if config_names.len() == 0 {
        let error = format!("Error: Config path: {} - no files in the folder", config_path);
        return Err(io::Error::new(io::ErrorKind::NotFound, error));
    }

    Ok(config_names)
}

pub fn set_config(config: &str) -> Result<(), io::Error> {
    println!("Config set to: {}", config);

    let (config_path, config_pattern, dest_config_path, dest_config_name) = get_settings()?;

    let dest_config_path = OsString::from(dest_config_path);

    if let Err(_) = fs::read_dir(&dest_config_path) {
        fs::create_dir_all(&dest_config_path)?;
    }

    let from = OsString::from(config_path.to_owned() + "/" + config + &config_pattern); 
    let to = OsString::from(dest_config_path.to_str().unwrap().to_owned() + "/" + &dest_config_name);

    fs::copy(from, to)?;

    Ok(())
}

pub fn get_env_name() -> Option<String> {
    let mut args = env::args();
    args.next();
    args.next()
}

pub fn print_info(config_names: Vec<String>) {
    println!("Available config names:");
    println!("{:?}", config_names);
    println!("Example of use:");
    println!("set_config {}", config_names.get(0).unwrap());
}
