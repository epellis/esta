#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Keyword(Keyword),
    Literal,
    Punctuation(Punctuation),
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub type_of: TokenType,
}
