use std::fs;
use std::io::{BufReader, Bytes, Read};

#[derive(Debug)]
pub enum TokenTypes {
    Identifier,
    Number,
    Punctuation,
    Operator,
    Whitespace,
}

impl TokenTypes {
    pub fn can_add_char(char: char, current_state: &TokenTypes) -> bool {
        match current_state {
            TokenTypes::Identifier => char.is_alphanumeric() || char == '_',
            TokenTypes::Number => char.is_numeric(),
            _ => false,
        }
    }
    pub fn choose_type_by_char(char: char) -> Result<TokenTypes, ()> {
        match char {
            'a'..='z' => Ok(TokenTypes::Identifier),
            '0'..='9' => Ok(TokenTypes::Number),
            '+' | '-' | '*' | '/' | '=' => Ok(TokenTypes::Operator),
            ' ' | '\n' | '\t' => Ok(TokenTypes::Whitespace),
            '(' | ')' | '{' | '}' | '[' | ']' | ';' => Ok(TokenTypes::Punctuation),

            _ => Err(()),
        }
    }
}
#[derive(Debug)]
pub struct Token {
    kind: TokenTypes,
    lexeme: String,
    line_number: u32,
}

impl Token {
    pub fn new(kind: TokenTypes, lexeme: String, line_number: u32) -> Token {
        Token {
            kind,
            lexeme,
            line_number,
        }
    }
}
#[derive(Eq, PartialEq, Debug)]
enum LexerState {
    Start,
    InToken,
}

#[derive(Debug)]
pub struct Lexer {
    file_content: Bytes<BufReader<fs::File>>,
    current_token: Option<Token>,
    current_state: LexerState,
    current_line: u32,
}
macro_rules! start_new_token {
    ($self:expr, $char:expr) => {{
        let token_type = TokenTypes::choose_type_by_char($char)
            .unwrap_or_else(|_| panic!("Error parsing token {}", $self.current_line));

        $self.current_token = Some(Token::new(token_type, String::new(), $self.current_line));
        if let Some(current_token) = $self.current_token.as_mut() {
            current_token.lexeme.push($char);
        }
        $self.current_state = LexerState::InToken;
    }};
}
impl Lexer {
    pub fn new(file_name: &str) -> Lexer {
        let file_content = BufReader::new(
            fs::File::open(file_name).expect(format!("Error opening file {file_name}").as_str()),
        )
        .bytes();
        Lexer {
            file_content,
            current_token: None,
            current_state: LexerState::Start,
            current_line: 1,
        }
    }


    pub fn get_next_token(&mut self) -> Option<Token> {
        for byte in &mut self.file_content {
            let char = byte.unwrap() as char;
            let mut last_token: Token = Token::new(TokenTypes::Whitespace, String::new(), 0);

            if self.current_state == LexerState::InToken {
                if let Some(current_token) = self.current_token.as_mut() {
                    if TokenTypes::can_add_char(char, &current_token.kind) {
                        current_token.lexeme.push(char);
                    } else {
                        if char == '\n' {
                            self.current_line += 1;
                        }
                        self.current_state = LexerState::Start;
                        last_token = self.current_token.take().unwrap();
                        start_new_token!(self, char);
                        return Some(last_token);
                    }
                }
            }

            if self.current_state == LexerState::Start {
                start_new_token!(self, char);
            }
        }
        self.current_token.take()
    }

}
