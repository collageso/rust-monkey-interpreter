use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(LetStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(statement) => statement.token_literal(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Dummy,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.value.clone()
    }
}
