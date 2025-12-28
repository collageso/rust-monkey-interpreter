use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }

            self.next_token();
        }

        Program {
            statements: statements,
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_peek(&Token::Identifier("".to_string())) {
            return None;
        }

        let name_value = match &self.current_token {
            Token::Identifier(ident) => ident.clone(),
            _ => unreachable!(),
        };

        let name = Identifier {
            token: self.current_token.clone(),
            value: name_value,
        };

        if !self.expect_peek(&Token::Assign) {
            return None;
        }

        while !self.is_current_token(&Token::Semicolon) {
            self.next_token();
            if self.current_token == Token::Eof {
                break;
            }
        }

        Some(Statement::Let(LetStatement {
            token,
            name,
            value: Expression::Dummy,
        }))
    }

    fn is_current_token(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(token)
    }

    fn is_peek_token(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(token)
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.is_peek_token(token) {
            self.next_token();
            return true;
        }

        self.peek_error(token);
        false
    }

    fn peek_error(&mut self, expected: &Token) {
        let message = format!(
            "expected next token to be {:?}. got {:?}",
            expected, self.peek_token
        );
        self.errors.push(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Node, Statement};

    #[test]
    fn test_let_statement() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (i, expected_ident) in expected_identifiers.into_iter().enumerate() {
            let statement = &program.statements[i];

            assert_eq!(
                statement.token_literal(),
                "let",
                "statement.token_literal not 'let'. got={}",
                statement.token_literal()
            );

            let let_statement = match statement {
                Statement::Let(l) => l,
                _ => {
                    panic!("statement is not Statement::Let. got={:?}", statement);
                }
            };

            assert_eq!(
                let_statement.name.value, expected_ident,
                "let_statement.name.value not '{}'. got={}",
                expected_ident, let_statement.name.value
            );

            assert_eq!(
                let_statement.name.token_literal(),
                expected_ident,
                "let_statement.name.token_literal not '{}'. got={}",
                expected_ident,
                let_statement.name.token_literal()
            );
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", parser.errors.len());
        for msg in &parser.errors {
            eprintln!("parser error: {}", msg);
        }

        panic!("parser errors encountered");
    }
}
