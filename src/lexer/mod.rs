mod lexer_error;
mod token;

pub use lexer_error::LexerError;
pub use token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    next_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            next_position: 0,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.next_position >= self.input.len() {
            return None;
        }

        self.input[self.next_position..].chars().next()
    }

    fn read_char(&mut self) -> Option<char> {
        let current_char = self.peek_char()?;
        self.position = self.next_position;
        self.next_position += current_char.len_utf8();

        Some(current_char)
    }

    fn lex_combined_operator(
        &mut self,
        combined: Token,
        single: Token,
    ) -> Result<Token, LexerError> {
        let next_token = self.peek_char().unwrap_or('0');

        if next_token == '=' {
            self.read_char();
            Ok(combined)
        } else {
            Ok(single)
        }
    }

    fn lex_string(&mut self) -> Result<Token, LexerError> {
        let position = self.next_position;
        let mut is_end = false;

        while let Some(ch) = self.read_char() {
            if ch == '"' {
                is_end = true;
                break;
            }
        }

        if !is_end {
            return Err(LexerError::UnterminatedString(self.position));
        }

        let str = &self.input[position..self.position];

        Ok(Token::StringLiteral(str.to_string()))
    }

    fn lex_number(&mut self) -> Result<Token, LexerError> {
        let position = self.position;

        while let Some(next_ch) = self.peek_char() {
            if next_ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        let number_str = &self.input[position..self.next_position];
        match number_str.parse::<i64>() {
            Ok(n) => Ok(Token::IntLiteral(n)),
            Err(_) => Err(LexerError::InvalidNumber(
                number_str.to_string(),
                (position, self.position),
            )),
        }
    }

    fn lex_ident(&mut self) -> Result<Token, LexerError> {
        let position = self.position;

        while let Some(next_ch) = self.peek_char() {
            if next_ch.is_ascii_alphabetic() || next_ch == '_' {
                self.read_char();
            } else {
                break;
            }
        }

        let str = &self.input[position..self.next_position];

        match str {
            "fn" => Ok(Token::Function),
            "let" => Ok(Token::Let),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "return" => Ok(Token::Return),
            _ => Ok(Token::Indent(str.to_string())),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.read_char() {
            match ch {
                ',' => return Some(Ok(Token::Comma)),
                ';' => return Some(Ok(Token::Semicolon)),
                '(' => return Some(Ok(Token::LeftParen)),
                ')' => return Some(Ok(Token::RightParen)),
                '{' => return Some(Ok(Token::LeftBrace)),
                '}' => return Some(Ok(Token::RightBrace)),
                '+' => return Some(Ok(Token::Plus)),
                '-' => return Some(Ok(Token::Minus)),
                '*' => return Some(Ok(Token::Asterisk)),
                '/' => return Some(Ok(Token::Slash)),
                '=' => return Some(self.lex_combined_operator(Token::Equal, Token::Assign)),
                '!' => return Some(self.lex_combined_operator(Token::NotEqual, Token::Bang)),
                '>' => {
                    return Some(
                        self.lex_combined_operator(Token::GreaterThanEqual, Token::GreaterThan),
                    )
                }
                '<' => {
                    return Some(self.lex_combined_operator(Token::LessThanEqual, Token::LessThan))
                }
                '"' => return Some(self.lex_string()),
                ch if ch.is_ascii_digit() => return Some(self.lex_number()),
                ch if (ch.is_ascii_alphabetic() || ch == '_') => return Some(self.lex_ident()),
                ch if ch.is_whitespace() => continue,
                _ => return Some(Err(LexerError::UnexpectedCharacter(ch, self.position))),
            }
        }
        None
    }
}
