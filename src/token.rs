use crate::operator;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,            // 不正トークン
    EndOfFile,          // ファイルの終端
    Identifier(String), // 識別子 (x, y, test など)
    Integer(i32),       // 数値 (0, 1000 など )
    Assign,             // =
    Plus,               // +
    Minus,              // -
    Exclamation,        // !
    Asterisk,           // *
    Slash,              // /
    LessThan,           // <
    GreaterThan,        // >
    Equal,              // ==
    NotEqual,           // !=
    Comma,              // ,
    Semicolon,          // ;
    Lparentheses,       // (
    Rparentheses,       // )
    Lbrace,             // {
    Rbrace,             // }
    Function,           // fn
    Let,                // let
    True,               // true
    False,              // false
    If,                 // if
    Else,               // else
    Return,             // return
}

impl Token {
    pub fn precedence(&self) -> operator::Precedences {
        match self {
            Token::Equal | Token::NotEqual => operator::Precedences::Equals,
            Token::LessThan | Token::GreaterThan => operator::Precedences::LessGreater,
            Token::Plus | Token::Minus => operator::Precedences::Sum,
            Token::Slash | Token::Asterisk => operator::Precedences::Product,
            Token::Lparentheses => operator::Precedences::Call,
            _ => operator::Precedences::Lowest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_infix() {
        let plus = Token::Plus.precedence();
        let minus = Token::Minus.precedence();
        let asterisk = Token::Asterisk.precedence();
        let slash = Token::Slash.precedence();
        let less_than = Token::LessThan.precedence();
        let greater_than = Token::GreaterThan.precedence();
        let equal = Token::Equal.precedence();
        let not_equal = Token::NotEqual.precedence();
        let identifier = Token::Identifier("test".to_string()).precedence();

        assert_eq!(plus == minus, true);
        assert_eq!(minus < asterisk, true);
        assert_eq!(asterisk == slash, true);
        assert_eq!(slash > less_than, true);
        assert_eq!(less_than == greater_than, true);
        assert_eq!(greater_than > equal, true);
        assert_eq!(equal == not_equal, true);
        assert_eq!(not_equal > identifier, true);
    }
}
