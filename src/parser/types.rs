
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
}
impl OperatorType {
    pub fn from_char(char: char) -> Option<OperatorType> {
        match char {
            '+' => Some(OperatorType::Add),
            '-' => Some(OperatorType::Sub),
            '*' => Some(OperatorType::Mul),
            '/' => Some(OperatorType::Div),
            '=' => Some(OperatorType::Equal),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Type {
    Integer,
    String,
    Bool,
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "int" => Some(Type::Integer),
            "string" => Some(Type::String),
            "bool" => Some(Type::Bool),
            _ => None,
        }
    }

}


#[derive(Debug, PartialEq, Eq, Hash,)]
pub enum Punctuation {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
}

impl Punctuation {
    pub fn from_char(ch: char) -> Option<Punctuation> {
        match ch {
            '{' => Some(Punctuation::OpenBrace),
            '}' => Some(Punctuation::CloseBrace),
            '(' => Some(Punctuation::OpenParen),
            ')' => Some(Punctuation::CloseParen),
            '[' => Some(Punctuation::OpenBracket),
            ']' => Some(Punctuation::CloseBracket),
            ';' => Some(Punctuation::Semicolon),
            _ => None,
        }
    }
}