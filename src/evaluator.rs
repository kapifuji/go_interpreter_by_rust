use crate::ast;
use crate::error;
use crate::object;
use crate::operator;

pub struct Evaluator {}

impl Evaluator {
    pub fn eval(root: &ast::Program) -> Result<object::Object, Box<dyn std::error::Error>> {
        Evaluator::eval_statements(&root.statements, true)
    }

    fn eval_statements(
        statements: &Vec<ast::Statement>,
        is_root: bool,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        let mut result = Ok(object::Object::Null);
        for statement in statements {
            let object = Evaluator::eval_statement(&statement)?;
            if let object::Object::ReturnValue(value) = object {
                result = if is_root == true {
                    Ok(*value)
                } else {
                    Ok(object::Object::ReturnValue(value))
                };
                break;
            } else {
                result = Ok(object);
            }
        }

        return result;
    }

    fn eval_statement(
        statement: &ast::Statement,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match statement {
            ast::Statement::Return(expression) => {
                let ret_val = Evaluator::eval_expression(expression)?;
                Ok(object::Object::ReturnValue(Box::new(ret_val)))
            }
            ast::Statement::Expression(expression) => Evaluator::eval_expression(expression),
            ast::Statement::Block(statements) => Evaluator::eval_statements(statements, false),
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
            ast::Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let condition = Evaluator::eval_expression(condition)?;
                Evaluator::eval_if_expression(&condition, consequence, &alternative)
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
        }
    }

    fn eval_infix_expression(
        left: &object::Object,
        operator: operator::Infix,
        right: &object::Object,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match left {
            object::Object::Integer(left_int) => {
                if let object::Object::Integer(right_int) = right {
                    Evaluator::eval_integer_infix_expression(*left_int, operator, *right_int)
                } else {
                    Err(error::EvaluatorError::TypeMissMatch {
                        left: left.clone(),
                        operator: operator,
                        right: right.clone(),
                    })?
                }
            }
            object::Object::Boolean(left_bool) => {
                if let object::Object::Boolean(right_bool) = right {
                    Evaluator::eval_boolean_infix_expression(*left_bool, operator, *right_bool)
                } else {
                    Err(error::EvaluatorError::TypeMissMatch {
                        left: left.clone(),
                        operator: operator,
                        right: right.clone(),
                    })?
                }
            }
            _ => Err(error::EvaluatorError::TypeMissMatch {
                left: left.clone(),
                operator: operator,
                right: right.clone(),
            })?,
        }
    }

    fn eval_integer_infix_expression(
        left: i32,
        operator: operator::Infix,
        right: i32,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match operator {
            operator::Infix::Plus => Ok(object::Object::Integer(left + right)),
            operator::Infix::Minus => Ok(object::Object::Integer(left - right)),
            operator::Infix::Asterisk => Ok(object::Object::Integer(left * right)),
            operator::Infix::Slash => Ok(object::Object::Integer(left / right)),
            operator::Infix::LessThan => Ok(object::Object::Boolean(left < right)),
            operator::Infix::GreaterThan => Ok(object::Object::Boolean(left > right)),
            operator::Infix::Equal => Ok(object::Object::Boolean(left == right)),
            operator::Infix::NotEqual => Ok(object::Object::Boolean(left != right)),
        }
    }

    fn eval_boolean_infix_expression(
        left: bool,
        operator: operator::Infix,
        right: bool,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match operator {
            operator::Infix::Equal => Ok(object::Object::Boolean(left == right)),
            operator::Infix::NotEqual => Ok(object::Object::Boolean(left != right)),
            _ => Err(error::EvaluatorError::UnknowInfixOperator {
                left: object::Object::Boolean(left),
                operator: operator,
                right: object::Object::Boolean(right),
            })?,
        }
    }

    fn eval_if_expression(
        condition: &object::Object,
        consequence: &ast::Statement,
        alternative: &Option<Box<ast::Statement>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        if condition.is_truthly() == true {
            Evaluator::eval_statement(consequence)
        } else {
            if let Some(alternative) = alternative {
                Evaluator::eval_statement(alternative)
            } else {
                Ok(object::Object::Null)
            }
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
            _ => Err(error::EvaluatorError::UnknowPrefixOperator {
                operator: operator::Prefix::Minus,
                right: object.clone(),
            })?,
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
        let tests = [
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("1 > 2", false),
            ("1 < 2", true),
            ("1 == 1", true),
            ("2 != 2", false),
            ("true == true", true),
            ("true != true", false),
            ("true == false", false),
            ("true != false", true),
            ("(1 > 2) == true", false),
            ("(1 > 2) != false", false),
            ("(1 < 2) == false", false),
            ("(1 > 2) != true", true),
        ];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(&evaluated, result);
        }
    }

    #[test]
    fn test_eval_if_expression() {
        let tests = [
            ("if (true) { 1 }", object::Object::Integer(1)),
            ("if (false) { 1 }", object::Object::Null),
            ("if (true) { 1 } else { 2 }", object::Object::Integer(1)),
            ("if (false) { 1 } else { 2 }", object::Object::Integer(2)),
            ("if (5) { 1 } else { 2 }", object::Object::Integer(1)),
            ("if (!5) { 1 } else { 2 }", object::Object::Integer(2)),
            ("if (1 < 2) { 1 } else { 2 }", object::Object::Integer(1)),
            ("if (1 > 2) { 1 } else { 2 }", object::Object::Integer(2)),
            ("if (1 > 2) { 1 }", object::Object::Null),
        ];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            assert_eq!(evaluated, result);
        }
    }

    #[test]
    fn test_eval_return_statement() {
        let tests = [
            ("return 10;", 10),
            ("return 100/10", 10),
            ("return 10; 1234;", 10),
            ("2*3; return 10; 1234;", 10),
            (
                "if (true) {
                  if (true) {
                    return 10;
                  }
                  0;
                }",
                10,
            ),
        ];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            test_integer_object(&evaluated, result);
        }
    }

    #[test]
    fn test_eval_error() {
        let tests = [
            ("5 + true;", "型のミスマッチ: 5 + true"),
            ("5 + true; 5;", "型のミスマッチ: 5 + true"),
            ("-true", "未知の演算子: -true"),
            ("true + false", "未知の演算子: true + false"),
            ("if (true) { true * false; }", "未知の演算子: true * false"),
            (
                "if (true) {
                    if (true) {
                        return false / false;
                    }
                    0;
                }",
                "未知の演算子: false / false",
            ),
            ("-true + 100", "未知の演算子: -true"),
        ];

        for (input, result) in tests {
            let lexer = lexer::Lexer::new(input);
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program().expect("parser error");

            let evaluated = Evaluator::eval(&program);

            match evaluated {
                Ok(ok) => panic!("エラーを期待しましたが、{:?}でした。", ok),
                Err(err) => {
                    let err_info = format!("{}", err);
                    assert_eq!(err_info, result);
                }
            }
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
