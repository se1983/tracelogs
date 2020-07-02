use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RegexScheme {
    pub(crate) datetime: String,
    pub(crate) host: String,
    pub(crate) service: String,
    pub(crate) message: String,
    pub(crate) log_pattern: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KubectlConfig {
    pub name: String,
    pub regex: RegexScheme,
    pub date_string: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JournaldConfig {
    pub(crate) name: String,
    pub(crate) host: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Targets {
    pub(crate) journald: Vec<JournaldConfig>,
    pub(crate) kubectl: Vec<KubectlConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ConfigFile {
    pub(crate) targets: Targets
}

pub(crate) fn load_config(file_path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    let file_loader = std::fs::File::open(file_path)?;
    let config_file: ConfigFile = serde_yaml::from_reader(file_loader)?;
    Ok(config_file)
}