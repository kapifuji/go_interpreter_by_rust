use std::io::{stdin, stdout, Write};
use go_interpreter::token::Token;
use go_interpreter::lexer::Lexer;

fn main() {
    let prompt = ">> ";
    loop{
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut scan = String::new();
        stdin().read_line(&mut scan).expect("Failed to read line.");
        let mut lexer = Lexer::new(&scan);
        loop{
            let tok = lexer.next_token();
            if tok == Token::EndOfFile{
                break;
            }
            else{
                println!("{:?}", tok);
            }
        }
    }
}
