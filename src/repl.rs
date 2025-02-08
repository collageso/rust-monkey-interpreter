use std::io::{self, BufRead, Write};

pub fn start_repl<F>(mode: &str, eval: F)
where
    F: Fn(&str),
{
    println!("Hello, Monkey Interpreter REPL!");
    println!("mode: {}", mode);

    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        print!("> ");
        stdout.lock().flush().expect("Failed to flush stdout");
        let input = stdin.lock().lines().next();

        match input {
            Some(Ok(line)) => {
                if line.trim() == ":q" {
                    break;
                }
                eval(&line);
            }
            Some(Err(e)) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
            None => {
                eprintln!("Input stream has ended. Exiting REPL.");
                break;
            }
        }
    }
}
