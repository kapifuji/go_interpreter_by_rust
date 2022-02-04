use crate::token::Token;

pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    current_char: char,
    next_char: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: '\u{0}',
            next_char: '\u{0}',
        };
        // 準備
        lexer.seek_char();
        lexer.seek_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {
            '=' => {
                if self.next_char == '='{
                    self.seek_char();
                    Token::Equal
                }
                else{
                    Token::Assign
                }
            },
            ';' => Token::Semicolon,
            '(' => Token::Lparentheses,
            ')' => Token::Rparentheses,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.next_char == '='{
                    self.seek_char();
                    Token::NotEqual
                }
                else{
                    Token::Exclamation
                }
            },
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LessThan,
            '>' => Token::GraterThan,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\u{0}' => Token::EndOfFile,
            ch => {
                if is_letter(ch) {
                    let identifier = self.read_by_checker(is_letter);
                    return Lexer::lookup_identifier(identifier);
                } else if is_digit(ch) {
                    let number_str = self.read_by_checker(is_digit);
                    return Token::Integer(number_str.parse().unwrap());
                }
                Token::Illegal
            }
        };
        self.seek_char();
        token
    }

    fn seek_char(&mut self) {
        self.current_char = self.next_char;
        self.next_char = self.input.next().unwrap_or('\u{0}');
    }

    fn read_by_checker<F>(&mut self, checker_fn: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut letter = String::new();
        while checker_fn(self.current_char) {
            letter.push(self.current_char);
            self.seek_char()
        }
        letter
    }

    fn skip_whitespace(&mut self) {
        while (self.current_char == ' ')
            || (self.current_char == '\t')
            || (self.current_char == '\n')
            || (self.current_char == '\r')
        {
            self.seek_char();
        }
    }

    fn lookup_identifier(identifier: String) -> Token {
        match identifier.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifer(identifier),
        }
    }
}

fn is_digit(ch: char) -> bool {
    ('0' <= ch) && (ch <= '9')
}

fn is_letter(ch: char) -> bool {
    let is_lower_alpha = 'a' <= ch && ch <= 'z';
    let is_upper_alpha = 'A' <= ch && ch <= 'Z';
    let is_under_score = ch == '_';
    is_lower_alpha || is_upper_alpha || is_under_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let tokens = [
            Token::Let,
            Token::Identifer("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifer("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifer("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparentheses,
            Token::Identifer("x".to_string()),
            Token::Comma,
            Token::Identifer("y".to_string()),
            Token::Rparentheses,
            Token::Lbrace,
            Token::Identifer("x".to_string()),
            Token::Plus,
            Token::Identifer("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifer("result".to_string()),
            Token::Assign,
            Token::Identifer("add".to_string()),
            Token::Lparentheses,
            Token::Identifer("five".to_string()),
            Token::Comma,
            Token::Identifer("ten".to_string()),
            Token::Rparentheses,
            Token::Semicolon,
            Token::Exclamation,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::GraterThan,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::Lparentheses,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::Rparentheses,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Integer(10),
            Token::Equal,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEqual,
            Token::Integer(9),
            Token::Semicolon,
            Token::EndOfFile,
        ];

        let mut lexer = Lexer::new(input);

        for tok in tokens.iter() {
            let next_token = &lexer.next_token();
            assert_eq!(next_token, tok);
        }
    }
}
