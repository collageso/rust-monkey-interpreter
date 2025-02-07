pub enum Token {
    // Identifier and Literals
    Identifier(String),
    IntLiteral(i64),
    // Operators
    Assign,
    Plus,
    // Puctuations
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Reserved words
    Function,
    Let,
    Return,
}
