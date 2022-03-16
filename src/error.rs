use crate::token;

#[derive(Debug)]
pub enum ParserError{
    UnexpectedToken{actual_token: token::Token, expected_token: token::Token},
    NotFoundIdentifier{found_token: token::Token},
    UnImplementation
}