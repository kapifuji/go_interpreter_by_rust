use crate::ast;
use crate::error;
use crate::lexer;
use crate::operator;
use crate::token;

struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    current_token: token::Token,
    next_token: token::Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: lexer::Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            current_token: token::Token::Illegal,
            next_token: token::Token::Illegal,
        };

        parser.seek_token();
        parser.seek_token();
        parser
    }

    fn seek_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.read_next_token();
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Box<dyn std::error::Error>> {
        let mut program = ast::Program::new();
        while self.current_token != token::Token::EndOfFile {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.seek_token(); // 次の文 へ進む
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        match self.current_token {
            token::Token::Let => self.parse_let_statement(),
            token::Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        self.seek_token(); // Identifier に進む
        let identifier = if let token::Token::Identifier(identifier) = &self.current_token {
            ast::Expression::Identifier(identifier.to_owned())
        } else {
            return Err(error::ParserError::NotFoundLetIdentifier {
                found_token: self.next_token.clone(),
            })?;
        };

        self.seek_token(); // Assign に進む
        self.expect_current(token::Token::Assign)?;

        self.seek_token(); // 式 に進む
        let expression = self.parse_expression(self.current_token.precedence())?;

        self.seek_token(); // Semicolon に進む
        self.expect_current(token::Token::Semicolon)?;

        Ok(ast::Statement::Let {
            identifier: identifier,
            value: expression,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        self.seek_token(); // 式 に進む
        let expression = self.parse_expression(self.current_token.precedence())?;

        self.seek_token(); // Semicolon に進む
        self.expect_current(token::Token::Semicolon)?;

        Ok(ast::Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        // 式文は文のトークンが無いのでここでseek不要
        let expression = self.parse_expression(self.current_token.precedence())?;

        self.seek_token(); // Semicolon に進む
        self.expect_current(token::Token::Semicolon)?;

        Ok(ast::Statement::Expression(expression))
    }

    fn parse_expression(
        &mut self,
        precedence: operator::Precedences,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let mut expression = match self.current_token.clone() {
            token::Token::Identifier(identifier) => self.parse_identifier(identifier.as_str())?,
            token::Token::Integer(integer) => self.parse_integer(integer)?,
            token::Token::Minus => {
                self.seek_token(); // Prefix の右辺式 に進む
                self.parse_prefix_expression(operator::Prefix::Minus)?
            }
            token::Token::Exclamation => {
                self.seek_token(); // Prefix の右辺式 に進む
                self.parse_prefix_expression(operator::Prefix::Exclamation)?
            }
            token::Token::True => self.parse_boolean(true)?,
            token::Token::False => self.parse_boolean(false)?,
            token::Token::Lparentheses => self.parse_grouped_expression()?,
            other => {
                println!("{:?}", other);
                return Err(error::ParserError::UnImplementationParser(
                    "式のパーサーが未実装です。",
                ))?;
            }
        };

        while (self.next_token != token::Token::Semicolon)
            && (precedence < self.next_token.precedence())
        {
            self.seek_token(); // Infix に進む
            expression = self.parse_infix_expression(expression.clone())?;
        }

        Ok(expression)
    }

    fn parse_identifier(
        &mut self,
        identifier: &str,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        Ok(ast::Expression::Identifier(identifier.to_string()))
    }

    fn parse_integer(
        &mut self,
        identifier: i32,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        Ok(ast::Expression::Integer(identifier))
    }

    fn parse_boolean(
        &mut self,
        boolean: bool,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        Ok(ast::Expression::Boolean(boolean))
    }

    fn parse_prefix_expression(
        &mut self,
        operator: operator::Prefix,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let expression = self.parse_expression(operator::Precedences::Prefix)?;
        Ok(ast::Expression::PrefixExpression {
            operator: operator,
            expression: Box::new(expression),
        })
    }

    fn parse_infix_expression(
        &mut self,
        left: ast::Expression,
    ) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let infix = match self.current_token {
            token::Token::Plus => operator::Infix::Plus,
            token::Token::Minus => operator::Infix::Minus,
            token::Token::Asterisk => operator::Infix::Asterisk,
            token::Token::Slash => operator::Infix::Slash,
            token::Token::LessThan => operator::Infix::LessThan,
            token::Token::GreaterThan => operator::Infix::GreaterThan,
            token::Token::Equal => operator::Infix::Equal,
            token::Token::NotEqual => operator::Infix::NotEqual,
            _ => Err(error::ParserError::NotFoundInfixToken {
                found_token: self.current_token.clone(),
            })?,
        };

        let precedence = self.current_token.precedence(); // 中置演算子の優先度
        self.seek_token(); // infix の右辺式 に進む
        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::InfixExpression {
            left: Box::new(left),
            operator: infix,
            right: Box::new(right),
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        self.seek_token(); // 式 に進む
        let expression = self.parse_expression(self.current_token.precedence())?;

        self.seek_token(); // Rparentheses に進む
        self.expect_current(token::Token::Rparentheses)?;

        Ok(expression)
    }

    fn expect_current(&mut self, token: token::Token) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_token == token {
            Ok(())
        } else {
            Err(error::ParserError::UnexpectedToken {
                actual_token: self.current_token.clone(),
                expected_token: token,
            })?
        }
    }

    fn check_current_token(&self, token: token::Token) -> bool {
        self.current_token == token
    }

    fn check_next_token(&self, token: token::Token) -> bool {
        self.next_token == token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
let x = 5;
let y = 5;
let foobar = 838383;
";
        let results = ["x", "y", "foobar"];

        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(err) => panic!("エラー: {}", err),
        };

        assert_eq!(program.statements.len(), 3);

        for (i, test) in results.iter().enumerate() {
            test_let_statement(&program.statements[i], test);
        }
    }

    fn test_let_statement(statement: &ast::Statement, expected_name: &str) {
        if let ast::Statement::Let { identifier, .. } = statement {
            if let ast::Expression::Identifier(name) = identifier {
                assert_eq!(name, expected_name);
            }
        } else {
            panic!("expected ast::Statement::Let, but got {:?}", statement);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
return 5;
return 10;
return 993322;
";
        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(err) => panic!("エラー: {}", err),
        };

        assert_eq!(program.statements.len(), 3);

        for i in 0..3 {
            test_return_statement(&program.statements[i]);
        }
    }

    fn test_return_statement(statement: &ast::Statement) {
        if let ast::Statement::Return(_) = statement {
        } else {
            panic!("expected ast::Statement::Return, but got {:?}", statement);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(err) => panic!("エラー: {}", err),
        };

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        let expression = test_expression_statement(statement);

        test_identifier_literal(&expression, "foobar".to_string());
    }

    #[test]
    fn test_integer_expression() {
        let input = "300;";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(err) => panic!("エラー: {}", err),
        };

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        let expression = test_expression_statement(statement);

        test_integer_literal(&expression, 300);
    }

    #[test]
    fn test_boolean_expression() {
        let inputs = ["true;", "false;"];
        let results = [true, false];

        for (input, result) in inputs.iter().zip(results.iter()) {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(err) => panic!("エラー: {}", err),
            };

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];

            let expression = test_expression_statement(statement);

            test_boolean_literal(&expression, *result);
        }
    }

    fn test_expression_statement(statement: &ast::Statement) -> ast::Expression {
        if let ast::Statement::Expression(expression) = statement {
            expression.clone()
        } else {
            panic!(
                "expected ast::Statement::Expression, but got {:?}",
                statement
            );
        }
    }

    fn test_integer_literal(expression: &ast::Expression, cmp_num: i32) {
        let integer = if let ast::Expression::Integer(integer) = expression {
            integer
        } else {
            panic!(
                "expected ast::Expression::Integer, but got {:?}",
                expression
            );
        };

        assert_eq!(*integer, cmp_num);
    }

    fn test_identifier_literal(expression: &ast::Expression, cmp_num: String) {
        let identifier = if let ast::Expression::Identifier(identifier) = expression {
            identifier
        } else {
            panic!(
                "expected ast::Expression::Identifier, but got {:?}",
                expression
            );
        };

        assert_eq!(*identifier, cmp_num);
    }

    fn test_boolean_literal(expression: &ast::Expression, cmp_bool: bool) {
        let boolean = if let ast::Expression::Boolean(boolean) = expression {
            boolean
        } else {
            panic!(
                "expected ast::Expression::Boolean, but got {:?}",
                expression
            );
        };

        assert_eq!(*boolean, cmp_bool);
    }

    #[test]
    fn test_prefix_expression() {
        let problem = [
            ("!5;", operator::Prefix::Exclamation, 5),
            ("-15;", operator::Prefix::Minus, 15),
        ];

        for (input, result_op, result_r) in problem {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(err) => panic!("エラー: {}", err),
            };

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];

            let expression = test_expression_statement(statement);

            let (operator, expression_right) = if let ast::Expression::PrefixExpression {
                operator,
                expression,
            } = expression
            {
                (operator, expression)
            } else {
                panic!(
                    "expected ast::Expression::PrefixExpression, but got {:?}",
                    expression
                );
            };

            assert_eq!(operator, result_op);
            test_integer_literal(&expression_right, result_r);
        }
    }

    #[test]
    fn test_infix_expression() {
        let problem = [
            ("1 + 2;", 1, operator::Infix::Plus, 2),
            ("2 - 3;", 2, operator::Infix::Minus, 3),
            ("3 * 4;", 3, operator::Infix::Asterisk, 4),
            ("4 / 5;", 4, operator::Infix::Slash, 5),
            ("5 < 6;", 5, operator::Infix::LessThan, 6),
            ("6 > 7;", 6, operator::Infix::GreaterThan, 7),
            ("7 == 8;", 7, operator::Infix::Equal, 8),
            ("8 != 9;", 8, operator::Infix::NotEqual, 9),
        ];

        for (input, result_l, result_op, result_r) in problem {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(err) => panic!("エラー: {}", err),
            };

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];

            let expression = test_expression_statement(statement);

            let (expression_left, operator, expression_right) =
                if let ast::Expression::InfixExpression {
                    left,
                    operator,
                    right,
                } = expression
                {
                    (left, operator, right)
                } else {
                    panic!(
                        "expected ast::Expression::InfixExpression, but got {:?}",
                        expression
                    );
                };

            test_integer_literal(&expression_left, result_l);
            assert_eq!(operator, result_op);
            test_integer_literal(&expression_right, result_r);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let problem = [
            ("a + b;", "(a + b);\n"),
            ("!-a;", "(!(-a));\n"),
            ("a + b - c;", "((a + b) - c);\n"),
            ("a * b / c;", "((a * b) / c);\n"),
            ("a + b * c;", "(a + (b * c));\n"),
            (
                "a + b * c + d / e - f;",
                "(((a + (b * c)) + (d / e)) - f);\n",
            ),
            ("1 + 2; -3 * 4;", "(1 + 2);\n((-3) * 4);\n"),
            ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4));\n"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));\n",
            ),
            ("true;", "true;\n"),
            ("true == false;", "(true == false);\n"),
            ("1 > 2 == false;", "((1 > 2) == false);\n"),
            ("(1 + 2) * 3;", "((1 + 2) * 3);\n"),
            ("1 + (2 - 3);", "(1 + (2 - 3));\n"),
            ("-(1 + 2);", "(-(1 + 2));\n"),
            ("!(true == true);", "(!(true == true));\n"),
            ("1 + (2 - 3) * 4;", "(1 + ((2 - 3) * 4));\n"),
            ("(1 + -(2 + 3)) * 4;", "((1 + (-(2 + 3))) * 4);\n"),
        ];

        for (input, result) in problem {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(err) => panic!("エラー: {}", err),
            };

            assert_eq!(result.to_string(), program.to_code());
        }
    }
}
