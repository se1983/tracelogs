use crate::tokenizer::LogLineToken;
use regex::Regex;

pub struct LogLineTokenizer<'a> {
    separator: Regex,
    buffer: String,
    pub count: usize,
    lines: Vec<LogLineToken<'a>>,
    log_source: &'a str,
    host_name: &'a str,
}

impl<'a> LogLineTokenizer<'a> {
    pub fn new(separator: Regex, log_source: &'a str, host_name: &'a str) -> LogLineTokenizer<'a> {
        let buffer = String::from("");
        let lines: Vec<LogLineToken> = vec![];
        let count = 0;
        LogLineTokenizer {
            separator,
            buffer,
            lines,
            count,
            log_source,
            host_name,
        }
    }

    pub fn push(&mut self, text: &str) {
        self.count += 1;
        self.buffer.push_str(text);
        self.tokenize();
    }

    fn tokenize(&mut self) {
        let tokenized = split_text(&self.separator, &self.buffer);

        if let Some((last, elements)) = tokenized.split_last() {
            for log_line in elements {
                let line = self.make_token(log_line);
                println!("new logline: {:?}", &line);
                self.lines.push(line)
            }
            self.buffer = String::from(*last)
        }
    }

    fn make_token(&self, text: &str) -> LogLineToken<'a> {
        LogLineToken::new(text, self.log_source, self.host_name)
    }
}

impl<'a> Drop for LogLineTokenizer<'a> {
    fn drop(&mut self) {
        let line = self.make_token(&self.buffer);
        self.buffer = String::from("");
        println!("new logline: {:?}", &line);
        self.lines.push(line);
    }
}

fn find_indices(re: &Regex, text: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = re.find_iter(text).map(|m| m.start()).collect();
    indices.push(text.len());
    indices
}

fn split_text<'a>(separator: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    let mut lhs: usize = 0;
    let indices = find_indices(&separator, &text);

    for rhs in indices {
        if rhs == lhs {
            continue;
        }
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
        assert_eq!(
            split_text(&separator, &text),
            vec!["Mary \n", "[had a \nlittle lamb\n", "["]
        )
    }

    #[test]
    fn test_split_text_no_match() {
        let separator = Regex::new(r"(?m)^\[").unwrap();
        let text = "little lamb";
        assert_eq!(split_text(&separator, &text), vec!["little lamb"])
    }

    #[test]
    fn test_split_text_newline() {
        let separator = Regex::new(r"(?m)^").unwrap();
        let text = "little\nlamb";
        assert_eq!(split_text(&separator, &text), vec!["little\n", "lamb"])
    }

    #[test]
    fn test_full_text_example() {
        let separator = Regex::new(r"(?m)^").unwrap();
        let text = r#"Whether that mattress was stuffed with corn-cobs or
broken crockery, there is no telling, but I rolled about a
good deal, and could not sleep for a long time. At last
I slid off into a light doze, and had pretty nearly made a
good offing toward the land of Nod, when I heard a
heavy footfall in the passage, and saw a glimmer of light
come into the room from under the door."#;
        assert_eq!(
            split_text(&separator, &text),
            vec![
                "Whether that mattress was stuffed with corn-cobs or\n",
                "broken crockery, there is no telling, but I rolled about a\n",
                "good deal, and could not sleep for a long time. At last\n",
                "I slid off into a light doze, and had pretty nearly made a\n",
                "good offing toward the land of Nod, when I heard a\n",
                "heavy footfall in the passage, and saw a glimmer of light\n",
                "come into the room from under the door."
            ]
        )
    }
}
