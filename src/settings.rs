use std::path::PathBuf;
extern crate config;
extern crate dirs;
extern crate exitcode;
extern crate serde;

use log::debug;
use std::collections::HashMap;

const DEFAULT_CONNECTION_NAME: &str = "default";

fn default_true() -> bool {
    return true;
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    hosts: String,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_true")]
    debug: bool,
    connections: HashMap<String, Connection>,
}

impl Default for Settings {
    fn default() -> Self {
        let default_connection = Connection {
            hosts: "0.0.0.0:9200".to_string(),
            username: None,
            password: None,
        };
        let mut connections = HashMap::new();
        connections.insert(DEFAULT_CONNECTION_NAME.to_string(), default_connection);
        return Settings {
            debug: true,
            connections: connections,
        };
    }
}

impl Settings {
    pub fn load(path: Option<PathBuf>) -> Result<Self, config::ConfigError> {
        let conf_path = Settings::select_path(path);
        match conf_path {
            Err(error_description) => {
                debug!(
                    "Error while reading config: \"{}\", loading defaults...",
                    error_description
                );
                return Ok(Default::default());
            }
            Ok(path) => {
                debug!("Using \"{}\" as a path config file", path.to_str().unwrap());
                let mut settings = config::Config::default();
                settings
                    .merge(config::File::with_name(path.to_str().unwrap()))
                    .unwrap();

                // Print out our settings (as a HashMap)
                println!(
                    "{:?}",
                    settings
                        .get::<HashMap<String, Connection>>("connections")
                        .unwrap()
                );
                return settings.try_into();
            }
        }
    }

    fn select_path(given_path: Option<PathBuf>) -> Result<PathBuf, String> {
        if given_path.is_some() {
            let path = given_path.unwrap();
            if path.exists() && path.is_file() {
                return Ok(path);
            } else {
                eprintln!(
                    "ERROR! Cannot read config file at \"{}\"",
                    path.to_str().unwrap_or("<unparsed path str>")
                );
                std::process::exit(exitcode::CONFIG);
            }
        }

        let home_path = dirs::home_dir().unwrap();
        let default_config_path = home_path.join(".config").join("elser").join("config.yaml");
        if default_config_path.exists() && default_config_path.is_file() {
            return Ok(default_config_path);
        }
        return Err("Failed to locate config file".to_string());
    }
}
