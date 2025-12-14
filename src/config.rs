use directories::BaseDirs;
use std::cell::LazyCell;
use std::collections::{BTreeMap, HashMap};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use serde::Deserialize;
use serde::Serialize;

use crate::popup_err;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub window_title: String,
    pub ping_interval: u64,
    pub targets: BTreeMap<String, IpAddr>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "window naam".into(),
            ping_interval: 30,
            targets: BTreeMap::from_iter(
                [("test1", "127.0.0.1"), ("test2", "0.0.0.0")]
                    .map(|(n, a)| (n.to_string(), a.parse().unwrap())),
            ),
        }
    }
}

pub static mut BASEDIRS: LazyCell<BaseDirs> = LazyCell::new(|| match BaseDirs::new() {
    Some(bd) => bd,
    None => {
        popup_err(
            "failed to detect directories",
            "Failed to detect your home directory",
        );
        std::process::exit(1)
    }
});

impl Config {
    fn dir() -> PathBuf {
        let mut dir = unsafe { &*BASEDIRS }.config_dir().to_path_buf();

        dir.push("pingky/");

        dir
    }
    fn file_path() -> PathBuf {
        let mut path = Self::dir();
        path.push("config.toml");
        path
    }

    pub fn exists() -> bool {
        Self::file_path().is_file()
    }

    pub fn write(&self) -> anyhow::Result<()> {
        // check dir
        let dir = Self::dir();
        if !dir.is_dir() {
            println!("config dir {dir:?} does not exist. creating...");
            fs::create_dir(&dir)?;
        }

        // check file
        let conf_path = Self::file_path();
        let conf_str = toml::ser::to_string_pretty(self)?;

        println!("writing config to {conf_path:?}: {self:#?}");
        fs::write(conf_path, conf_str)?;

        Ok(())
    }

    pub fn read() -> anyhow::Result<Self> {
        let config_path = Self::file_path();

        println!("reading config {config_path:?}");

        let contents = fs::read_to_string(&config_path)?;

        let mut conf: Self = toml::from_str(&contents)?;

        Ok(conf)
    }
}
