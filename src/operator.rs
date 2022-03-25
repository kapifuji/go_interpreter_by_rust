#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Minus,
    Exclamation,
}

impl Prefix {
    pub fn to_code(&self) -> String {
        match self {
            Prefix::Minus => "-".to_string(),
            Prefix::Exclamation => "!".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
}

impl Infix {
    pub fn to_code(&self) -> String {
        match self {
            Infix::Plus => "+".to_string(),
            Infix::Minus => "-".to_string(),
            Infix::Asterisk => "*".to_string(),
            Infix::Slash => "/".to_string(),
            Infix::LessThan => "<".to_string(),
            Infix::GreaterThan => ">".to_string(),
            Infix::Equal => "==".to_string(),
            Infix::NotEqual => "!=".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedences {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_precedence() {
        assert_eq!(Precedences::Lowest < Precedences::Equals, true);
        assert_eq!(Precedences::Equals < Precedences::LessGreater, true);
        assert_eq!(Precedences::LessGreater < Precedences::Sum, true);
        assert_eq!(Precedences::Sum < Precedences::Product, true);
        assert_eq!(Precedences::Product < Precedences::Prefix, true);
        assert_eq!(Precedences::Prefix < Precedences::Call, true);
    }
}
