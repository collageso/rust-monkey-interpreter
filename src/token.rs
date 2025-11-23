#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    // Identifier + Literals
    Identifier(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    // Punctuations
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Reserved Words
    Function,
    Let,
}
