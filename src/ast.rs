use crate::operator;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Expression,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Illegal,
    Identifier(String),
    Integer(i32),
    PrefixExpression {
        operator: operator::Prefix,
        expression: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: operator::Infix,
        right: Box<Expression>,
    },
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    pub fn to_code(&self) -> String {
        let mut code = String::new();
        for statement in &self.statements {
            code.push_str(statement.to_code().as_str());
            code.push('\n');
        }
        code
    }
}

impl Statement {
    fn to_code(&self) -> String {
        let mut statement = String::new();
        match self {
            Statement::Let { identifier, value } => {
                statement.push_str("let ");
                statement.push_str(identifier.to_code().as_str());
                statement.push_str(" = ");
                statement.push_str(value.to_code().as_str());
                statement.push(';');
            }
            Statement::Return(expression) => {
                statement.push_str("return ");
                statement.push_str(expression.to_code().as_str());
                statement.push(';');
            }
            Statement::Expression(expression) => {
                statement.push_str(expression.to_code().as_str());
                statement.push(';');
            }
        }
        statement
    }
}

impl Expression {
    fn to_code(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.to_string(),
            Expression::Integer(integer) => integer.to_string(),
            Expression::PrefixExpression {
                operator,
                expression,
            } => "(".to_string() + &operator.to_code() + &expression.to_code() + ")",
            Expression::InfixExpression {
                left,
                operator,
                right,
            } => {
                "(".to_string()
                    + &left.to_code()
                    + " "
                    + &operator.to_code()
                    + " "
                    + &right.to_code()
                    + ")"
            }
            Expression::Illegal => "[illegal expression]".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_code() {
        let expected_code = "let x = 100;
return x;
500;
";
        let mut program = Program::new();

        let statement1 = Statement::Let {
            identifier: Expression::Identifier("x".to_string()),
            value: Expression::Integer(100),
        };

        let statement2 = Statement::Return(Expression::Identifier("x".to_string()));

        let statement3 = Statement::Expression(Expression::Integer(500));

        program.statements.push(statement1);
        program.statements.push(statement2);
        program.statements.push(statement3);

        assert_eq!(program.to_code(), expected_code);
    }

    #[test]
    fn test_to_code_prefix() {
        let expected_code = "!test;";
        let mut program = Program::new();

        let expression_id = Expression::Identifier("test".to_string());
        let expression_prefix = Expression::PrefixExpression {
            operator: operator::Prefix::Exclamation,
            expression: Box::new(expression_id),
        };
        let statement = Statement::Expression(expression_prefix);

        program.statements.push(statement);

        assert_eq!(program.to_code(), "(!test);\n");
    }

    #[test]
    fn test_to_code_infix() {
        let expected_code = "2 * test;";
        let mut program = Program::new();

        let expression_l = Expression::Integer(2);
        let expression_r = Expression::Identifier("test".to_string());
        let expression_prefix = Expression::InfixExpression {
            left: Box::new(expression_l),
            operator: operator::Infix::Asterisk,
            right: Box::new(expression_r),
        };
        let statement = Statement::Expression(expression_prefix);

        program.statements.push(statement);

        assert_eq!(program.to_code(), "(2 * test);\n");
    }
}
