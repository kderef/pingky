use ini::Ini;
use std::{
    cell::OnceCell,
    collections::HashMap,
    fs, io,
    net::IpAddr,
    path::Path,
    rc::Rc,
    sync::{OnceLock, RwLock},
};
use thiserror::Error;

pub const CFG_WINDOW_TITLE: &str = "window_naam";
pub const CFG_PING_INTERVAL: &str = "ping_interval";
pub const CFG_PING_SECTION: &str = "ping";

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Debug)]
pub struct Config {
    pub window_title: String,
    pub ping_interval: u64,
    pub ping_targets: Vec<(String, IpAddr)>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to open file: {0}")]
    FileError(#[from] io::Error),

    #[error("Failed to read config file: {0}")]
    ReadError(#[from] ini::ParseError),

    #[error("option {0} is missing")]
    OptionMissing(&'static str),

    #[error("Invalid IP address {1} for {0}")]
    InvalidAddr(String, String),

    #[error("Invalid number value {0} for {1}")]
    InvalidNumber(String, &'static str),

    #[error("ping section contains no targets.")]
    NoTargets,
}

impl Config {
    pub const PATH: &str = "pingconfig.ini";

    pub fn example() -> (Self, Ini) {
        let ex = Self {
            window_title: "test naam".into(),
            ping_interval: 30,
            ping_targets: vec![
                ("test1".into(), "0.0.0.0".parse().unwrap()),
                ("test2".into(), "127.0.0.1".parse().unwrap()),
            ],
        };

        let mut conf = Ini::new();

        conf.with_general_section()
            .set(CFG_WINDOW_TITLE, ex.window_title.clone())
            .set(CFG_PING_INTERVAL, ex.ping_interval.to_string());

        let mut ping_s = conf.with_section(Some(CFG_PING_SECTION));

        for (name, addr) in &ex.ping_targets {
            ping_s.set(name, addr.to_string());
        }

        (ex, conf)
    }

    pub fn write_example() -> io::Result<Self> {
        let (example, ini) = Self::example();

        ini.write_to_file(Self::PATH)?;

        Ok(example)
    }

    pub fn read() -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(Self::PATH)?;

        let table = ini::Ini::load_from_str(&contents)?;

        // --- global config

        let section = table.general_section();

        let get = |opt| section.get(opt).ok_or(ConfigError::OptionMissing(opt));

        let window_title = get(CFG_WINDOW_TITLE)?.to_string();
        let interval = get(CFG_PING_INTERVAL)?;

        let ping_interval = interval
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidNumber(interval.to_string(), CFG_PING_INTERVAL))?;

        // --- ping targets
        let targets = table
            .section(Some(CFG_PING_SECTION))
            .ok_or(ConfigError::OptionMissing(CFG_PING_SECTION))?;

        if targets.len() == 0 {
            return Err(ConfigError::NoTargets);
        }

        let mut ping_targets = Vec::with_capacity(targets.len());

        for (k, v) in targets {
            let addr = v
                .parse()
                .map_err(|_| ConfigError::InvalidAddr(k.to_string(), v.to_string()))?;
            ping_targets.push((k.to_string(), addr));
        }

        let conf = Self {
            window_title,
            ping_interval,
            ping_targets,
        };

        println!("{conf:#?}");

        Ok(conf)
    }
}
