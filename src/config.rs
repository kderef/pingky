use std::{cell::OnceCell, collections::HashMap, fs, io, path::Path, sync::OnceLock};
use thiserror::Error;

pub static CONFIG_EXAMPLE: &str = r#"window_title = deurbel
ping_interval = 30

[ping]
google = https://google.com
youtube = ttps://youtube.com
"#;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    unsafe { CONFIG.get().unwrap() }
}

pub struct Config {
    pub window_title: String,
    pub ping_interval: u64,
    pub ping_targets: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to open file: {0}")]
    FileError(#[from] io::Error),

    #[error("Failed to read config file: {0}")]
    ReadError(#[from] ini::ParseError),

    #[error("option {0} is missing")]
    OptionMissing(&'static str),

    #[error("Invalid number value {0} for {1}")]
    InvalidNumber(String, &'static str),

    #[error("ping section contains no targets.")]
    NoTargets,

    #[error("duplicate option {0}")]
    DuplicateOption(String),
}

impl Config {
    pub const PATH: &str = "pingconfig.ini";
    pub const EXAMPLE_PATH: &str = "pingconfig-EXAMPLE.ini";

    pub fn write_example() -> Result<(), io::Error> {
        fs::write(Self::EXAMPLE_PATH, CONFIG_EXAMPLE)
    }

    pub fn read() -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(Self::PATH)?;

        let table = ini::Ini::load_from_str(&contents)?;

        // --- global config

        let section = table.general_section();

        let get = |opt| section.get(opt).ok_or(ConfigError::OptionMissing(opt));

        let window_title = get("window_title")?.to_string();
        let interval = get("ping_interval")?;

        let ping_interval = interval
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidNumber(interval.to_string(), "ping_interval"))?;

        // --- ping targets
        let targets = table
            .section(Some("ping"))
            .ok_or(ConfigError::OptionMissing("ping"))?;

        if targets.len() == 0 {
            return Err(ConfigError::NoTargets);
        }

        let mut ping_targets = HashMap::with_capacity(targets.len());
        println!("{ping_targets:?}");

        for (k, v) in targets {
            ping_targets.insert(k.to_string(), v.to_string());
        }

        Ok(Self {
            window_title,
            ping_interval,
            ping_targets,
        })
    }
}
