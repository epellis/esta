use super::tokens::{Token, TokenType, Punctuation, Keyword};
use std::str::FromStr;
use regex::{Regex, CaptureMatches};

const PATTERN: &'static str = r"(?x)
[\w]+   # Ascii characters
|[\d]+  # Numbers
|;      # Semicolon
|\(     # Left Paren
|\)     # Right Paren
|=      # Equal
|\+     # Plus
|\*     # Star
|\\     # Slash
|\-     # Minus";

pub fn scan(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut stream = &input[..];

    let re = Regex::new(PATTERN).unwrap();
    let captures: Vec<&str> = re.captures_iter(input)
        .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
        .collect();
    println!("{:?}", captures);

    tokens
}
