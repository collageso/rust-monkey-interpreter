mod lexer_error;
mod token;

use lexer_error::LexerError;
use token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn read_char(&mut self) -> Option<char> {}
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Self::Item {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimplemented() {}
}
