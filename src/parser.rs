use crate::ast;
use crate::lexer;
use crate::token;
use crate::error;

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

    pub fn parse_program(&mut self) -> Result<ast::Program, error::ParserError> {
        let mut program = ast::Program::new();
        while self.current_token != token::Token::EndOfFile {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.seek_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        match self.current_token {
            token::Token::Let => self.parse_let_statement(),
            _ => Err(error::ParserError::UnImplementation),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        let identifier = if let token::Token::Identifier(identifier) = &self.next_token {
            ast::Expression::Identifier(identifier.to_owned())
        } else {
            return Err(error::ParserError::NotFoundIdentifier{
                found_token: self.next_token.clone()
            });
        };

        self.seek_token(); // Identifier に進む

        self.expect_next(token::Token::Assign)?;

        self.seek_token(); // Assign に進む

        loop { // 次のトークンが Semicolon となるまで進める
            if let Ok(_) = self.expect_next(token::Token::Semicolon){
                break;
            };
            self.seek_token();
        }

        self.seek_token(); // Semicolon に進む

        Ok(ast::Statement::Let {
            identifier: identifier,
            value: ast::Expression::Identifier("dummy".to_string()), // 未実装なので仮の値
        })
    }

    fn expect_next(&mut self, token: token::Token) -> Result<(), error::ParserError>{
        if self.next_token == token{
            Ok(())
        }
        else{
            Err(error::ParserError::UnexpectedToken{
                actual_token: self.next_token.clone(),
                expected_token: token
            })
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

        let program = parser.parse_program().unwrap();
        if program.statements.len() != 3 {
            panic!(
                "expected 3 statements, but got {}",
                program.statements.len()
            );
        }
        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
            test_let_statement(&program.statements[i], test)
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
}
