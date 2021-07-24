use regex::Regex;
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;
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
        let file_cont = fs::read_to_string(&self.file_path).await?;

        for (i, line) in file_cont.lines().enumerate() {
            if i < self.tokenizer.count {
                continue;
            }
            let line = "\n".to_owned() + line;
            self.tokenizer.push(&line);
        }

        Ok(())
    }

    pub async fn watch(&mut self) {
        loop {
            match self.next().await {
                Ok(_) => sleep(Duration::from_millis(300)).await,
                Err(err) => {
                    eprintln!("error open file {}  [{}]", self.file_path, err);
                    sleep(Duration::from_millis(3000)).await;
                }
            }
        }
    }
}

async fn run() {
    let newline_rgx = Regex::new(r"(?m)\n^\[").unwrap();
    let tokenizer = LogLineTokenizer::new(newline_rgx);
    let mut reader = LogFileAdapter::new(String::from("/var/log/Xorg.0.log"), tokenizer);
    reader.watch().await;
}

#[tokio::main]
async fn main() {
    run().await
}
