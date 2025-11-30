use std::io::{self, Write};

use crate::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub struct Repl {}

impl Repl {
    pub fn new() -> Self {
        Repl {}
    }

    pub fn start(&self) {
        println!("Hello This is the Monkey Programming Language!");
        println!("Feel free to type in commands");

        let stdio = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("{}", PROMPT);
            stdout.flush().expect("Failed to flush stdout");

            let mut line = String::new();

            match stdio.read_line(&mut line) {
                Ok(0) => {
                    println!("\nBye!");
                    return;
                }
                Ok(_) => {
                    let mut lexer = Lexer::new(&line);

                    loop {
                        let token = lexer.next_token();

                        if token == Token::Eof {
                            break;
                        }

                        println!("{:?}", token);
                    }
                }
                Err(error) => {
                    println!("Error reading input {}", error);
                }
            }
        }
    }
}
