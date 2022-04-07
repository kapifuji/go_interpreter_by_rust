use crate::ast;
use crate::environment;
use crate::error;
use crate::object;
use crate::operator;
use std::{cell::RefCell, rc::Rc};

pub struct Evaluator {}

impl Evaluator {
    pub fn eval(
        root: &ast::Program,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        Evaluator::eval_statements(&root.statements, true, env)
    }

    fn eval_statements(
        statements: &Vec<ast::Statement>,
        is_root: bool,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        let mut result = Ok(object::Object::Null);
        for statement in statements {
            let object = Evaluator::eval_statement(&statement, env)?;
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
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match statement {
            ast::Statement::Let { identifier, value } => {
                let identifier = if let ast::Expression::Identifier(ident) = identifier {
                    ident
                } else {
                    unreachable!();
                };
                let value = Evaluator::eval_expression(value, env)?;
                env.borrow_mut().set(identifier.clone(), value);
                Ok(object::Object::Null)
            }
            ast::Statement::Return(expression) => {
                let ret_val = Evaluator::eval_expression(expression, env)?;
                Ok(object::Object::ReturnValue(Box::new(ret_val)))
            }
            ast::Statement::Expression(expression) => Evaluator::eval_expression(expression, env),
            ast::Statement::Block(statements) => Evaluator::eval_statements(statements, false, env),
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_expression(
        expression: &ast::Expression,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match expression {
            ast::Expression::Identifier(identifier) => {
                if let Some(object) = env.borrow().get(identifier.clone()) {
                    Ok(object)
                } else {
                    Err(error::EvaluatorError::NotFoundIdentifier {
                        identifier: identifier.clone(),
                    })?
                }
            }
            ast::Expression::Integer(integer) => Ok(object::Object::Integer(*integer)),
            ast::Expression::Boolean(boolean) => Ok(object::Object::Boolean(*boolean)),
            ast::Expression::PrefixExpression {
                operator,
                expression,
            } => {
                let object = Evaluator::eval_expression(expression, env);
                Evaluator::eval_prefix_expression(operator.clone(), &(object?), env)
            }
            ast::Expression::InfixExpression {
                left,
                operator,
                right,
            } => {
                let left = Evaluator::eval_expression(left, env)?;
                let right = Evaluator::eval_expression(right, env)?;
                Evaluator::eval_infix_expression(&left, operator.clone(), &right, env)
            }
            ast::Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let condition = Evaluator::eval_expression(condition, env)?;
                Evaluator::eval_if_expression(&condition, consequence, &alternative, env)
            }
            ast::Expression::Function { parameters, body } => Ok(object::Object::Function {
                parameters: parameters.clone(),
                body: body.clone(),
                environment: environment::Environment::create_enclosed_environment(env.clone()),
            }),
            ast::Expression::Call { function, args } => {
                let function = Evaluator::eval_expression(function, env)?;
                let args = Evaluator::eval_expressions(args, env)?;
                Evaluator::apply_function(function, args)
            }
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_expressions(
        expressions: &Vec<ast::Expression>,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<Vec<object::Object>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();

        for expression in expressions {
            let evaluated = Evaluator::eval_expression(expression, env)?;
            result.push(evaluated);
        }

        Ok(result)
    }

    fn eval_prefix_expression(
        operator: operator::Prefix,
        object: &object::Object,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match operator {
            operator::Prefix::Exclamation => Evaluator::eval_exclamation_operator(object, env),
            operator::Prefix::Minus => Evaluator::eval_minus_prefix_operator(object, env),
        }
    }

    fn eval_infix_expression(
        left: &object::Object,
        operator: operator::Infix,
        right: &object::Object,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match (left, right) {
            (object::Object::Integer(left_int), object::Object::Integer(right_int)) => {
                Evaluator::eval_integer_infix_expression(*left_int, operator, *right_int, env)
            }
            (object::Object::Boolean(left_bool), object::Object::Boolean(right_bool)) => {
                Evaluator::eval_boolean_infix_expression(*left_bool, operator, *right_bool, env)
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
        env: &mut Rc<RefCell<environment::Environment>>,
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
        env: &mut Rc<RefCell<environment::Environment>>,
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
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        if condition.is_truthly() == true {
            Evaluator::eval_statement(consequence, env)
        } else {
            if let Some(alternative) = alternative {
                Evaluator::eval_statement(alternative, env)
            } else {
                Ok(object::Object::Null)
            }
        }
    }

    fn apply_function(
        object: object::Object,
        args: Vec<object::Object>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        if let object::Object::Function {
            parameters,
            body,
            environment,
        } = object
        {
            let new_env = environment::Environment::create_enclosed_environment(Rc::new(
                RefCell::new(environment),
            ));
            let mut new_env = Rc::new(RefCell::new(new_env));

            for (parameter, arg) in parameters.iter().zip(args.iter()) {
                if let ast::Expression::Identifier(identifier) = parameter {
                    new_env.borrow_mut().set(identifier.clone(), arg.clone());
                } else {
                    unreachable!();
                }
            }

            let evaluated = Evaluator::eval_statement(&body, &mut new_env)?;
            if let object::Object::ReturnValue(object) = evaluated {
                Ok(*object)
            } else {
                Ok(evaluated)
            }
        } else {
            unreachable!();
        }
    }

    fn eval_exclamation_operator(
        object: &object::Object,
        env: &mut Rc<RefCell<environment::Environment>>,
    ) -> Result<object::Object, Box<dyn std::error::Error>> {
        match object {
            object::Object::Boolean(boolean) => Ok(object::Object::Boolean(!boolean)),
            object::Object::Null => Ok(object::Object::Boolean(true)),
            _ => Ok(object::Object::Boolean(false)),
        }
    }

    fn eval_minus_prefix_operator(
        object: &object::Object,
        env: &mut Rc<RefCell<environment::Environment>>,
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
    fn test_eval_let_statement() {
        let tests = [
            ("let a = 1; a;", 1),
            ("let a = 1 + 2 * 3; a;", 7),
            ("let a = 1; let b = a; b;", 1),
            ("let a = 1; let b = 2; let c = a + b; c;", 3),
        ];

        for (input, result) in tests {
            let evaluated = test_eval(input);
            test_integer_object(&evaluated, result);
        }
    }

    #[test]
    fn test_eval_function_object() {
        let input = "fn(x) { x + 2; }";

        let evaluated = test_eval(input);
        let (params, body) = match evaluated {
            object::Object::Function {
                parameters, body, ..
            } => (parameters, body),
            other => panic!("Object::Functionを期待しましたが、{:?}でした。", other),
        };

        assert_eq!(params.len(), 1);
        assert_eq!(params[0].to_code(), "x");
        assert_eq!(body.to_code(), "{\n(x + 2);\n}");
    }

    #[test]
    fn test_eval_function_application() {
        let tests = [
            ("let identity = fn(x) { x }; identity(10);", 10),
            ("let identity = fn(x) { return x; }; identity(10);", 10),
            ("let double = fn(x) { x * 2; }; double(10);", 20),
            ("let add = fn(x, y) { x + y; }; add(10, 20);", 30),
            (
                "let add = fn(x, y) { x + y; }; add(add(10, 20), 30 + 40);",
                100,
            ),
            ("fn(x) { x; }(10);", 10),
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
            ("foo", "識別子が見つかりません。: foo"),
        ];

        for (input, result) in tests {
            let lexer = lexer::Lexer::new(input);
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program().expect("parser error");

            let environment = environment::Environment::new();
            let evaluated = Evaluator::eval(&program, &mut Rc::new(RefCell::new(environment)));

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
        let environment = environment::Environment::new();
        Evaluator::eval(&program, &mut Rc::new(RefCell::new(environment))).expect("evaluator error")
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
