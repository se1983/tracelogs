#![allow(dead_code)]

extern crate tokio;

use regex::Regex;

mod reader;

fn pattern_indices(re: &Regex, text: &str) -> Vec<usize> {
    re.find_iter(text).map(|m| m.start()).collect()
}

fn split_text(mut idxs: Vec<usize>, text: &str) -> Vec<&str> {
    let mut slices = Vec::new();
    let mut lhs: usize = 0;
    if idxs.last().unwrap() != &text.len() {
        idxs.insert(idxs.len(), text.len())
    }

    for i in idxs {
        slices.push(&text[lhs..i]);
        lhs = i;
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
    fn test_newline_indexer() {
        let rgx = Regex::new(r"(?m)^\[").unwrap();
        assert_eq!(pattern_indices(&rgx, "foo\n[bar"), [4]);
        assert_eq!(pattern_indices(&rgx, "[foo\nbar"), [0]);
        assert_eq!(pattern_indices(&rgx, "[foo\n[bar"), [0, 5]);
        assert_eq!(pattern_indices(&rgx, "foo\nbar"), [])
    }

    #[test]
    fn test_split_text() {
        let mut ids: Vec<usize> = vec![1, 4, 5];
        assert_eq!(split_text(ids, "Mary had a little lamb"),
                   vec!["M", "ary", " ", "had a little lamb"])
    }

    #[test]
    fn test_combination() {
        let rgx = Regex::new(r"(?m)^\[").unwrap();
        let text = "Mary \n[had a \nlittle lamb";
        let idxs = pattern_indices(&rgx, &text);
        assert_eq!(split_text(idxs, text), vec!["Mary \n", "[had a \nlittle lamb"])
    }
}