use crate::ast;
use crate::error;
use crate::lexer;
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
            self.seek_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        match self.current_token {
            token::Token::Let => self.parse_let_statement(),
            token::Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        let identifier = if let token::Token::Identifier(identifier) = &self.next_token {
            ast::Expression::Identifier(identifier.to_owned())
        } else {
            return Err(error::ParserError::NotFoundLetIdentifier {
                found_token: self.next_token.clone(),
            })?;
        };

        self.seek_token(); // Identifier に進む

        self.expect_next(token::Token::Assign)?;

        self.seek_token(); // Assign に進む

        let expression = self.parse_expression()?;

        self.seek_token(); // Semicolon に進む

        Ok(ast::Statement::Let {
            identifier: identifier,
            value: expression,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>> {
        let expression = self.parse_expression()?;
        self.seek_token(); // Semicolon に進む
        Ok(ast::Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, Box<dyn std::error::Error>>{
        let expression = self.parse_expression2()?;
        self.seek_token(); // Semicolon に進む
        Ok(ast::Statement::Expression(expression))
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        loop {
            // 次のトークンが Semicolon となるまで進める
            if let Ok(_) = self.expect_next(token::Token::Semicolon) {
                break;
            };
            self.seek_token();
        }
        Ok(ast::Expression::Identifier("dummy".to_string())) // 仮の値
    }

    fn parse_expression2(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let expression = match self.current_token.clone(){
            token::Token::Identifier(identifier) => self.parse_identifier(identifier.as_str())?,
            other => {
                println!("{:?}", other);
                return Err(error::ParserError::UnImplementationParser("式のパーサーが未実装です。"))?
            }
        };
        
        //self.seek_token(); // 式の最後 に進む
        self.expect_next(token::Token::Semicolon)?;
        Ok(expression)
    }

    fn parse_identifier(&mut self, identifier: &str) -> Result<ast::Expression, Box<dyn std::error::Error>>{
        Ok(ast::Expression::Identifier(identifier.to_string()))
    }

    fn expect_next(&mut self, token: token::Token) -> Result<(), Box<dyn std::error::Error>> {
        if self.next_token == token {
            Ok(())
        } else {
            Err(error::ParserError::UnexpectedToken {
                actual_token: self.next_token.clone(),
                expected_token: token,
            })?
        }
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
        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(err) => panic!("エラー: {}", err),
        };

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
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

        let expression = if let ast::Statement::Expression(expression) = statement{
            expression
        }
        else{
            panic!("expected ast::Statement::Expression, but got {:?}", statement);
        };

        let identifier = if let ast::Expression::Identifier(identifier) = expression{
            identifier
        }
        else{
            panic!("expected ast::Expression::Identifier, but got {:?}", expression);
        };

        assert_eq!(identifier, "foobar");
    }
}
