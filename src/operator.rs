#[derive(Debug, PartialEq)]
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
