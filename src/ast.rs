use crate::operator;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: Expression,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Illegal,
    Identifier(String),
    Integer(i32),
    Boolean(bool),
    PrefixExpression {
        operator: operator::Prefix,
        expression: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: operator::Infix,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
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
        let mut code = "".to_string();
        match self {
            Statement::Let { identifier, value } => {
                code.push_str("let ");
                code.push_str(identifier.to_code().as_str());
                code.push_str(" = ");
                code.push_str(value.to_code().as_str());
                code.push(';');
            }
            Statement::Return(expression) => {
                code.push_str("return ");
                code.push_str(expression.to_code().as_str());
                code.push(';');
            }
            Statement::Expression(expression) => {
                code.push_str(expression.to_code().as_str());
                code.push(';');
            }
            Statement::Block(statements) => {
                code.push('{');
                for statement in statements {
                    code.push('\n');
                    code.push_str(statement.to_code().as_str());
                }
                code.push('\n');
                code.push('}');
            }
        }
        code
    }
}

impl Expression {
    fn to_code(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.to_string(),
            Expression::Integer(integer) => integer.to_string(),
            Expression::Boolean(boolean) => match boolean {
                true => "true".to_string(),
                false => "false".to_string(),
            },
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
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let alternative = if let Some(alternative) = alternative {
                    "else ".to_string() + &alternative.to_code()
                } else {
                    "".to_string()
                };
                "if ".to_string()
                    + &condition.to_code()
                    + " "
                    + &consequence.to_code()
                    + " "
                    + &alternative
            }
            Expression::Function { parameters, body } => {
                let param_list = parameters
                    .iter()
                    .map(|param| param.to_code())
                    .collect::<Vec<String>>();
                let mut code = "".to_string();
                code.push_str("fn(");
                code.push_str(param_list.join(", ").as_str());
                code.push_str(")");
                code.push_str(&body.to_code());

                code
            }
            Expression::Call { function, args } => {
                let args_list = args
                    .iter()
                    .map(|arg| arg.to_code())
                    .collect::<Vec<String>>();
                let mut code = "".to_string();
                code.push_str(&function.to_code());
                code.push_str("(");
                code.push_str(args_list.join(", ").as_str());
                code.push_str(")");

                code
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
        let expected_code = "(!test);\n";
        let mut program = Program::new();

        let expression_id = Expression::Identifier("test".to_string());
        let expression_prefix = Expression::PrefixExpression {
            operator: operator::Prefix::Exclamation,
            expression: Box::new(expression_id),
        };
        let statement = Statement::Expression(expression_prefix);

        program.statements.push(statement);

        assert_eq!(program.to_code(), expected_code);
    }

    #[test]
    fn test_to_code_infix() {
        let expected_code = "(2 * test);\n";
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

        assert_eq!(program.to_code(), expected_code);
    }
}
