use crate::ast;
use crate::object;
use crate::operator;

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
            ast::Expression::PrefixExpression {
                operator,
                expression,
            } => {
                let object = Evaluator::eval_expression(expression);
                Evaluator::eval_prefix_expression(operator.clone(), &(object?))
            }
            ast::Expression::InfixExpression {
                left,
                operator,
                right,
            } => {
                let left = Evaluator::eval_expression(left)?;
                let right = Evaluator::eval_expression(right)?;
                Evaluator::eval_infix_expression(&left, operator.clone(), &right)
            }
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_prefix_expression(
        operator: operator::Prefix,
        object: &object::Object,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match operator {
            operator::Prefix::Exclamation => Evaluator::eval_exclamation_operator(object),
            operator::Prefix::Minus => Evaluator::eval_minus_prefix_operator(object),
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_infix_expression(
        left: &object::Object,
        operator: operator::Infix,
        right: &object::Object,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        let left = if let object::Object::Integer(integer) = left {
            integer
        } else {
            return Ok(object::Object::Null);
        };

        let right = if let object::Object::Integer(integer) = right {
            integer
        } else {
            return Ok(object::Object::Null);
        };

        match operator {
            operator::Infix::Plus => Ok(object::Object::Integer(left + right)),
            operator::Infix::Minus => Ok(object::Object::Integer(left - right)),
            operator::Infix::Asterisk => Ok(object::Object::Integer(left * right)),
            operator::Infix::Slash => Ok(object::Object::Integer(left / right)),
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_exclamation_operator(
        object: &object::Object,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match object {
            object::Object::Boolean(boolean) => Ok(object::Object::Boolean(!boolean)),
            object::Object::Null => Ok(object::Object::Boolean(true)),
            _ => Ok(object::Object::Boolean(false)),
        }
    }

    fn eval_minus_prefix_operator(
        object: &object::Object,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match object {
            object::Object::Integer(integer) => Ok(object::Object::Integer(-integer)),
            _ => Ok(object::Object::Null),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn test_eval_integer_expression() {
        let tests = [
            ("1", 1),
            ("12", 12),
            ("-1", -1),
            ("-12", -12),
            ("1 + 2 - 3", 0),
            ("1 + 2 * 3", 7),
            ("3 * 4 / 2 + 10 - 8", 8),
            ("(1 + 2) * 3 - -1", 10),
            ("-1 * -1", 1),
            ("-10 + -1 * 2", -12),
            ("(10 + 20) / (10 - 0)", 3),
        ];

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

    #[test]
    fn test_eval_bang_operator() {
        let tests = [
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!!true", false),
            ("!!5", true),
        ];

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
