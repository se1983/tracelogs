use regex::Regex;

use tracelogs::reader::LogFileAdapter;
use tracelogs::tokenizer::LogLineTokenizer;

async fn run() {
    // TODO: Allow multiple LogSources
    // TODO: Add data persistence
    // TODO: Add declarative configuration

    let newline_rgx = Regex::new(r"(?m)^\[").unwrap();
    let logsource = "/var/log/Xorg.0.log";
    let tokenizer = LogLineTokenizer::new(newline_rgx, logsource);
    let mut file_reader = LogFileAdapter::new(String::from(logsource), tokenizer);

    file_reader.watch().await;
}

#[tokio::main]
async fn main() {
    run().await
}
