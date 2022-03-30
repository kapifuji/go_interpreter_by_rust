use go_interpreter::lexer::Lexer;
use go_interpreter::parser::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    let prompt = ">> ";
    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut scan = String::new();
        stdin().read_line(&mut scan).expect("Failed to read line.");
        let lexer = Lexer::new(&scan);
        let mut parser = Parser::new(lexer);

        match parser.parse_program() {
            Ok(program) => println!("{}", program.to_code()),
            Err(err) => println!("エラー: {}", err),
        };
    }
}
