mod client;
mod env;
mod repo;
mod turbo;
mod user;

use std::path::{Path, PathBuf};

use anyhow::Result;
pub use client::{ClientConfig, ClientConfigLoader};
use config::ConfigError;
#[cfg(not(windows))]
use dirs_next::config_dir;
// Go's xdg implementation uses FOLDERID_LocalAppData for config home
// https://github.com/adrg/xdg/blob/master/paths_windows.go#L28
// Rust xdg implementations uses FOLDERID_RoamingAppData for config home
// We use cache_dir so we can find the config dir that the Go code uses
#[cfg(windows)]
use dirs_next::data_local_dir as config_dir;
pub use env::MappedEnvironment;
pub use repo::{get_repo_config_path, RepoConfig, RepoConfigLoader};
use serde::Serialize;
use thiserror::Error;
pub use turbo::{RawTurboJson, SpacesJson};
pub use user::{UserConfig, UserConfigLoader};

#[derive(Debug, Error)]
pub enum Error {
    #[error("default config path not found")]
    NoDefaultConfigPath,
    #[error(
        "Could not find turbo.json. Follow directions at https://turbo.build/repo/docs to create \
         one"
    )]
    NoTurboJson,
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(
        "Package tasks (<package>#<task>) are not allowed in single-package repositories: found \
         {task_id}"
    )]
    PackageTaskInSinglePackageMode { task_id: String },
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub fn default_user_config_path() -> Result<PathBuf, Error> {
    config_dir()
        .map(|p| p.join("turborepo").join("config.json"))
        .ok_or(Error::NoDefaultConfigPath)
}

#[allow(dead_code)]
pub fn data_dir() -> Option<PathBuf> {
    dirs_next::data_dir().map(|p| p.join("turborepo"))
}

fn write_to_disk<T>(path: &Path, config: &T) -> Result<(), Error>
where
    T: Serialize,
{
    if let Some(parent_dir) = path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }
    let config_file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(&config_file, &config)?;
    config_file.sync_all()?;
    Ok(())
}
