use go_interpreter::lexer::Lexer;
use go_interpreter::parser::Parser;
use go_interpreter::evaluator::Evaluator;
use go_interpreter::object::Object;
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
        let ast_root = parser.parse_program().expect("Filed to parse program.");
        
        let evaluated = Evaluator::eval(&ast_root).expect("Filed to eval ast.");
        println!("{}", evaluated.inspect());
    }
}
