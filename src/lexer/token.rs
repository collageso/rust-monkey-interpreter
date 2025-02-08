#[derive(Debug)]
pub enum Token {
    // Puctuations
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Operators
    Assign,
    Plus,
    // Identifier and Literals
    StringLiteral(String),
    IntLiteral(i64),
    // Reserved words and Identifier
    Function,
    Let,
    Return,
    Indent(String),
    // Etc
    EndOfFile,
}
