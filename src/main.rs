use tracelogs::tokenizer::LogLineTokenizer;
use tokio::fs;
use std::time::Duration;
use regex::Regex;
use tokio::time::sleep;

struct LogFileAdapter {
    file_path: String,
    tokenizer: LogLineTokenizer,
}

impl LogFileAdapter {
    pub fn new(file_path: String, tokenizer: LogLineTokenizer) -> Self {
        LogFileAdapter { file_path, tokenizer }
    }

    pub async fn watch(&mut self) {
        loop {
            sleep(Duration::from_millis(300)).await;

            let contents = fs::read_to_string(&self.file_path).await.unwrap();

            for (i, line) in contents.lines().enumerate() {
                if i < self.tokenizer.count { continue; }
                self.tokenizer.push(line);
            }
        }
    }
}


async fn run() {
    let newline_rgx = Regex::new(r"(?m)^").unwrap();
    let tokenizer = LogLineTokenizer::new(newline_rgx);
    let mut reader = LogFileAdapter::new(
        String::from("/tmp/test.txt"),
        tokenizer);
    reader.watch().await;
}


#[tokio::main]
async fn main() {
    run().await
}