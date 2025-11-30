#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    // Identifier + Literals
    Identifier(String),
    Int(i64),
    String(String),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
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
    If,
    Else,
    True,
    False,
    Return,
}

impl Token {
    pub fn lookup_identifier(ident: &str) -> Token {
        return match ident {
            "fn" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            "return" => Token::Return,
            _ => Token::Identifier(ident.to_string()),
        };
    }
}
