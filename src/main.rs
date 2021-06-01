#![allow(dead_code)]

extern crate tokio;

use regex::Regex;

mod reader;


fn split_text<'a>(separator: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    let mut lhs: usize = 0;

    let mut indices: Vec<usize> = separator.find_iter(text).map(|m| m.start()).collect();
    indices.push(text.len());

    for rhs in indices {
        slices.push(&text[lhs..rhs]);
        lhs = rhs;
    }

    slices
}

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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