pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Expression,
        value: Expression,
    },
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
}

impl Statement {}

impl Expression {}
