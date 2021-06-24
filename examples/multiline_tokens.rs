#![allow(dead_code)]


use tracelogs::tokenizer::LogLineTokenizer;
use regex::Regex;


fn main() {
    let mut tokenizer = LogLineTokenizer::new(Regex::new(r"(?m)^\[").unwrap());
    tokenizer.push("this will not be tokenized\n");
    tokenizer.push("also this will [not]");
    tokenizer.push("do anything\n");  // Fixme: There is a emtpy line after me
    tokenizer.push("[this will trigger it!");
    tokenizer.push("\n[");
}

