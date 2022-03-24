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
    GreaterThan,         // >
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
