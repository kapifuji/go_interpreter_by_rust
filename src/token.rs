#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,
    Identifer(String),
    Integer(i32),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparentheses,
    Rparentheses,
    Lbrace,
    Rbrace,
    Function,
    Let,
}
