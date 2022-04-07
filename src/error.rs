use crate::object;
use crate::operator;
use crate::token;

// anyhow, thiserror を利用すれば楽できるが、ローカル環境で git が通らないので妥協
#[derive(Debug)]
pub enum ParserError<'a> {
    UnexpectedToken {
        actual_token: token::Token,
        expected_token: token::Token,
    },
    NotFoundInfixToken {
        found_token: token::Token,
    },
    NotFoundLetIdentifier {
        found_token: token::Token,
    },
    UnImplementationStatemant(&'a str),
    UnImplementationParser(&'a str),
}

#[derive(Debug)]
pub enum EvaluatorError {
    TypeMissMatch {
        left: object::Object,
        operator: operator::Infix,
        right: object::Object,
    },
    UnknowInfixOperator {
        left: object::Object,
        operator: operator::Infix,
        right: object::Object,
    },
    UnknowPrefixOperator {
        operator: operator::Prefix,
        right: object::Object,
    },
    NotFoundIdentifier {
        identifier: String,
    },
}

impl<'a> std::fmt::Display for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            self::ParserError::UnexpectedToken {
                actual_token,
                expected_token,
            } => {
                write!(
                    f,
                    "({:?}を期待しましたが、{:?}でした。)",
                    expected_token, actual_token
                )
            }
            self::ParserError::NotFoundInfixToken { found_token } => {
                write!(f, "(Infixを期待しましたが、{:?}でした。)", found_token)
            }
            self::ParserError::NotFoundLetIdentifier { found_token } => {
                write!(f, "(Identifierを期待しましたが、{:?}でした。)", found_token)
            }
            self::ParserError::UnImplementationParser(message) => {
                write!(f, "({})", (message))
            }
            _ => write!(f, "(未実装エラーです。)"),
        }
    }
}

impl std::fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            self::EvaluatorError::TypeMissMatch {
                left,
                operator,
                right,
            } => {
                write!(
                    f,
                    "型のミスマッチ: {} {} {}",
                    left.inspect(),
                    operator.to_code(),
                    right.inspect()
                )
            }
            self::EvaluatorError::UnknowInfixOperator {
                left,
                operator,
                right,
            } => {
                write!(
                    f,
                    "未知の演算子: {} {} {}",
                    left.inspect(),
                    operator.to_code(),
                    right.inspect()
                )
            }
            self::EvaluatorError::UnknowPrefixOperator { operator, right } => {
                write!(f, "未知の演算子: {}{}", operator.to_code(), right.inspect())
            }
            self::EvaluatorError::NotFoundIdentifier { identifier } => {
                write!(f, "識別子が見つかりません。: {}", identifier)
            }
        }
    }
}

impl<'a> std::error::Error for ParserError<'a> {}
impl<'a> std::error::Error for EvaluatorError {}
