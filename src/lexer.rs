use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::Assign,
            '+' => Token::Plus,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '\0' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::lookup_identifier(literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    let value = literal.parse::<i64>().unwrap_or(0);
                    return Token::Int(value);
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            if let Some(ch) = self.input[self.read_position..].chars().next() {
                self.ch = ch;
            } else {
                self.ch = '\0';
            }
        }

        self.position = self.read_position;

        if self.ch == '\0' {
            self.read_position += 1;
        } else {
            self.read_position += self.ch.len_utf8();
        }
    }

    fn read_identifier(&mut self) -> &'a str {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let position = self.position;

        while is_digit(self.ch) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"
    let five = 5;
    let ten = 10;

    let add = fn(x,y) {
      x + y;
    };

    let result = add(five,ten);
"#;

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            // let five = 5;
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            // let ten = 10;
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            // let add = fn(x,y) {
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            // x + y;
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            // };
            Token::RightBrace,
            Token::Semicolon,
            // let result = add(five,ten);
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens.into_iter() {
            let result = lexer.next_token();
            assert_eq!(result, expected);
        }
    }
}
