#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    Punctuation(Punctuation),
    Literal(DataType),
    Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Bang,
    Or,
    And,
    BangEqual,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Semicolon,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    For,
    While,
    If,
    Var,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String(String),
    Bool(bool),
    Int(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub lexeme: String,
    pub type_of: TokenType,
}
