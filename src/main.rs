mod lexer;
mod repl;

use lexer::*;
use repl::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        // TODO: Interpreter REPL needed to implement
        todo!()
    }

    let mode = args[1].as_str();

    match mode {
        "lexer" => start_repl(mode, |input| {
            let lexer = Lexer::new(input);
            let mut tokens = Vec::new();

            for token in lexer {
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => {
                        eprintln!("LexerError occured: {:?}", e);
                        return;
                    }
                }
            }

            println!("{:?}", tokens);
        }),
        _ => todo!(),
    }
}
