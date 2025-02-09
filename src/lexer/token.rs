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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    // Identifier and Literals
    StringLiteral(String),
    IntLiteral(i64),
    // Reserved words and Identifier
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Indent(String),
}
