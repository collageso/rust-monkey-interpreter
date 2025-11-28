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

impl Token {
    pub fn lookup_identifier(ident: &str) -> Token {
        return match ident {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Identifier(ident.to_string()),
        };
    }
}
