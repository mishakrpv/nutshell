use std::env;
use std::path::PathBuf;

use anyhow::{ensure, Context, Result};

pub fn config_path() -> Result<PathBuf> {
    let path = match env::var_os("_NUTSH_CONFIG_DIR") {
        Some(path) => PathBuf::from(path),
        None => dirs::config_local_dir()
            .context("could not find config directory, please set _NUTSH_CONFIG_DIR manually")?
            .join("nutshell.lua"),
    };

    ensure!(
        path.is_absolute(),
        "_NUTSH_CONFIG_DIR must be an absolute path"
    );
    Ok(path)
}

pub fn echo() -> bool {
    env::var_os("_NUTSH_ECHO").map_or(false, |var| var == "1")
}

pub fn max_latency_ms() -> Result<u16> {
    let latency: u16 = match env::var_os("_NUTSH_MAX_LATENCY_MS") {
        Some(latency) => latency
            .into_string()
            .unwrap_or("5000".to_string())
            .parse::<u16>()
            .unwrap(),
        None => 5000_u16,
    };

    Ok(latency)
}
