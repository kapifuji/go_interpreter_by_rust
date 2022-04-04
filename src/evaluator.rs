use crate::ast;
use crate::lexer;
use crate::object;
use crate::parser;

pub struct Evaluator {}

impl Evaluator {
    pub fn eval(root: &ast::Program) -> Result<object::Object, Box<dyn std::error::Error>> {
        let mut result = Ok(object::Object::Null);
        for statement in &root.statements {
            result = Evaluator::eval_statement(&statement);
        }

        return result;
    }

    fn eval_statement(
        statement: &ast::Statement,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match statement {
            ast::Statement::Expression(expression) => Evaluator::eval_expression(expression),
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_expression(
        expression: &ast::Expression,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match expression {
            ast::Expression::Integer(integer) => Ok(object::Object::Integer(*integer)),
            ast::Expression::Boolean(boolean) => Ok(object::Object::Boolean(*boolean)),
            _ => Ok(object::Object::Null),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = [("1", 1), ("12", 12)];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            test_integer_object(&evaluated, result);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = [("true", true), ("false", false)];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(&evaluated, result);
        }
    }

    fn test_eval(input: &str) -> object::Object {
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("parser error");
        Evaluator::eval(&program).expect("evaluator error")
    }

    fn test_integer_object(object: &object::Object, expected: i32) {
        let integer = if let object::Object::Integer(integer) = object {
            integer
        } else {
            panic!("Object::Integer を期待しましたが、{:?}でした。", object);
        };

        assert_eq!(*integer, expected);
    }

    fn test_boolean_object(object: &object::Object, expected: bool) {
        let boolean = if let object::Object::Boolean(boolean) = object {
            boolean
        } else {
            panic!("Object::Bool を期待しましたが、{:?}でした。", object);
        };

        assert_eq!(*boolean, expected);
    }
}
