use crate::token;

// anyhow, thiserror を利用すれば楽できるが、ローカル環境で git が通らないので妥協
#[derive(Debug)]
pub enum ParserError{
    UnexpectedToken{actual_token: token::Token, expected_token: token::Token},
    NotFoundLetIdentifier{found_token: token::Token},
    UnImplementationStatemant{found_token: token::Token},
}

impl std::fmt::Display for ParserError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            self::ParserError::UnexpectedToken{actual_token, expected_token} => {
                write!(f, "({:?}を期待しましたが、{:?}でした。)", expected_token, actual_token)
            },
            self::ParserError::NotFoundLetIdentifier{found_token} => {
                write!(f, "(Identifierを期待しましたが、{:?}でした。)", found_token)
            }
            _ => write!(f, "(未実装エラーです。)")
        }
    }
}

impl std::error::Error for ParserError{}