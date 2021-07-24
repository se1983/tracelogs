use regex::Regex;

use tracelogs::reader::LogFileAdapter;
use tracelogs::tokenizer::LogLineTokenizer;

async fn run() {
    // TODO: Allow multiple LogSources
    // TODO: Add data persistence

    let newline_rgx = Regex::new(r"(?m)^\[").unwrap();
    let tokenizer = LogLineTokenizer::new(newline_rgx);
    let mut reader = LogFileAdapter::new(String::from("/var/log/Xorg.0.log"), tokenizer);
    reader.watch().await;
}

#[tokio::main]
async fn main() {
    run().await
}
