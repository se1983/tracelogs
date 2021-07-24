use regex::Regex;
use std::time::Duration;
use tracelogs::tokenizer::LogLineTokenizer;

struct LogFileAdapter {
    file_path: String,
    tokenizer: LogLineTokenizer,
}

impl LogFileAdapter {
    pub fn new(file_path: String, tokenizer: LogLineTokenizer) -> Self {
        LogFileAdapter {
            file_path,
            tokenizer,
        }
    }

    async fn next(&mut self) -> Result<(), tokio::io::Error> {
        let file_cont = tokio::fs::read_to_string(&self.file_path).await?;

        for (i, line) in file_cont.lines().enumerate() {
            if i < self.tokenizer.count {
                continue;
            }
            let line = format!("{}\n", line);
            self.tokenizer.push(&line);
        }

        Ok(())
    }

    pub async fn watch(&mut self) {
        loop {
            match self.next().await {
                Ok(_) => tokio::time::sleep(Duration::from_millis(300)).await,
                Err(err) => {
                    eprintln!("error open file {}  [{}]", self.file_path, err);
                    tokio::time::sleep(Duration::from_millis(3000)).await;
                }
            }
        }
    }
}

async fn run() {
    let newline_rgx = Regex::new(r"(?m)^\[").unwrap();
    let tokenizer = LogLineTokenizer::new(newline_rgx);
    let mut reader = LogFileAdapter::new(String::from("/var/log/Xorg.0.log"), tokenizer);
    reader.watch().await;
}

#[tokio::main]
async fn main() {
    run().await
}
