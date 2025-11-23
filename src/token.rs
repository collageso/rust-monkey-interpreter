pub enum Token {
    // Identifier + Literals
    Identifier,
    Int,
    // Operators
    Assign,
    Plus,
    // Punctuations
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Reserved Words
    Function,
    Let,
}
