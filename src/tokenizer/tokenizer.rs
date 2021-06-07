use regex::Regex;
use crate::tokenizer::LogLineToken;


pub struct LogLineTokenizer {
    separator: Regex,
    buffer: String,
    lines: Vec<LogLineToken>
}

impl LogLineTokenizer{
    pub fn new(separator: Regex) -> LogLineTokenizer {
        let buffer = String::from("");
        let lines: Vec<LogLineToken> = vec![];
        LogLineTokenizer{separator, buffer, lines}
    }

    pub fn append(&mut self, text: &str) {
        self.buffer.push_str(text);
        self.tokenize();
    }

    fn tokenize(&mut self) {
        let tokenized = split_text(&self.separator, &self.buffer);

        if let Some((last, elements)) = tokenized.split_last(){
            for log_line in elements {

                let line = LogLineToken::new(&log_line);
                println!("new logline: {:?}", &line);
                self.lines.push(line)

            }
            self.buffer = String::from(*last)
        }
    }
}

impl Drop for LogLineTokenizer {
    fn drop(&mut self) {
        let line = LogLineToken::new(&self.buffer);
        self.buffer = String::from("");
        println!("new logline: {:?}", &line);
        self.lines.push(line);
    }
}

fn find_indices(re: &Regex, text: &str) -> Vec<usize>{
    let mut indices: Vec<usize> = re.find_iter(text).map(|m| m.start()).collect();
    indices.push(text.len());
    indices
}

fn split_text<'a>(separator: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    let mut lhs: usize = 0;
    let indices = find_indices(&separator, &text);

    for rhs in indices {
        slices.push(&text[lhs..rhs]);
        lhs = rhs;
    }

    slices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        let separator = Regex::new(r"(?m)^\[").unwrap();
        let text = "Mary \n[had a \nlittle lamb\n[";
        assert_eq!(split_text(&separator, &text), vec!["Mary \n", "[had a \nlittle lamb\n", "["])
    }

    #[test]
    fn test_split_text_no_match() {
        let separator = Regex::new(r"(?m)^\[").unwrap();
        let text = "little lamb";
        assert_eq!(split_text(&separator, &text), vec!["little lamb"])
    }
}