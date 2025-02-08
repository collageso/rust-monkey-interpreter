#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, usize),
    InvalidNumber(String, (usize, usize)),
    UnterminatedString(usize),
}
