use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct RegexScheme {
    datetime: String,
    host: String,
    service: String,
    message: String,
    log_pattern: String,
}

#[derive(Deserialize, Debug, Clone)]
struct KubectlConfig {
    name: String,
    regex: RegexScheme,
    date_string: String
}

#[derive(Deserialize, Debug, Clone)]
struct JournaldConfig {
    name: String,
    host: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Targets {
    journald: Vec<JournaldConfig>,
    kubectl:Vec<KubectlConfig>
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Configfile {
    targets: Targets
}