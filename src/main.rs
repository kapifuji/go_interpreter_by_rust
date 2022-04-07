use go_interpreter::environment::Environment;
use go_interpreter::evaluator::Evaluator;
use go_interpreter::lexer::Lexer;
use go_interpreter::parser::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    let prompt = ">> ";
    let mut environment = Environment::new();
    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut scan = String::new();
        stdin().read_line(&mut scan).expect("Failed to read line.");
        let lexer = Lexer::new(&scan);
        let mut parser = Parser::new(lexer);
        let ast_root = match parser.parse_program() {
            Ok(ast) => ast,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        let evaluated = match Evaluator::eval(&ast_root, &mut environment) {
            Ok(evaluated) => evaluated,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        print!("{}", evaluated.inspect());
    }
}
