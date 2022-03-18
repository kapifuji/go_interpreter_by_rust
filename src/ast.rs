pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Expression,
        value: Expression,
    },
    Return(Expression)
}

#[derive(Debug)]
pub enum Expression {
    Illegal,
    Identifier(String),
    Integer(i32),
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    pub fn to_code(&self) -> String{
        let mut code = String::new();
        for statement in &self.statements{
            code.push_str(statement.to_code().as_str());
            code.push('\n');
        }
        code
    }
}

impl Statement {
    pub fn to_code(&self) -> String{
        let mut statement = String::new();
        match self{
            Statement::Let{identifier, value} => {
                statement.push_str("let ");
                statement.push_str(identifier.to_code().as_str());
                statement.push_str(" = ");
                statement.push_str(value.to_code().as_str());
                statement.push(';');
            },
            Statement::Return(expression) => {
                statement.push_str("return ");
                statement.push_str(expression.to_code().as_str());
                statement.push(';');
            }
        }
        statement
    }
}

impl Expression {
    pub fn to_code(&self) -> String{
        match self{
            Expression::Identifier(identifier) => identifier.to_string(),
            Expression::Integer(integer) => integer.to_string(),
            Expression::Illegal => "[illegal expression]".to_string()
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_to_code() {
        let expected_code = "let x = 100;
return x;
";
        let mut program = Program::new();

        let statement1 = Statement::Let{
            identifier: Expression::Identifier("x".to_string()),
            value: Expression::Integer(100)
        };

        let statement2 = Statement::Return(
            Expression::Identifier("x".to_string())
        );

        program.statements.push(statement1);
        program.statements.push(statement2);

        assert_eq!(program.to_code(), expected_code);
    }
}
