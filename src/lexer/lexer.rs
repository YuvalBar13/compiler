use std::fs;
use std::io::{BufReader, Bytes, Read};
use crate::error;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum SymbolKind {
    Identifier,
    Number,
    String,
    Bool,
    Char,
    Punctuation,
    Operator,
    Whitespace,

    // AST nodes
    Assign,
    Declaration,
    DeclarationAssignment,
    Expr,
    BinaryOperation,

    Empty,
}

impl SymbolKind {
    pub fn can_add_char(char: char, current_state: &SymbolKind) -> bool {
        match current_state {
            SymbolKind::Identifier => char.is_alphanumeric() || char == '_',
            SymbolKind::Number => char.is_numeric(),
            SymbolKind::String => true,
            SymbolKind::Char => true,

            _ => false,
        }
    }
    pub fn choose_type_by_char(char: char) -> Option<SymbolKind> {
        match char {
            'a'..='z' | 'A'..='Z' => Some(SymbolKind::Identifier),
            '0'..='9' => Some(SymbolKind::Number),
            '"'  => Some(SymbolKind::String),
            '+' | '-' | '*' | '/' | '=' => Some(SymbolKind::Operator),
            '\'' => Some(SymbolKind::Char),
            ' ' | '\n' | '\t' => Some(SymbolKind::Whitespace),
            '(' | ')' | '{' | '}' | '[' | ']' | ';' => Some(SymbolKind::Punctuation),

            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct Token {
    kind: SymbolKind,
    lexeme: String,
    line_number: u32,
}

impl Token {
    pub fn new(kind: SymbolKind, lexeme: String, line_number: u32) -> Token {
        Token {
            kind,
            lexeme,
            line_number,
        }
    }
    pub fn get_kind(&self) -> SymbolKind {
        self.kind
    }
    pub fn get_value(&self) -> String {
        self.lexeme.clone()
    }
    pub fn get_line_number(&self) -> u32 {
        self.line_number
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
        while let Some(byte_result) = self.file_content.by_ref().next() {
            let char = byte_result.unwrap() as char;

            // windows is shit have to support this  :)
            if char == '\r' {
                continue;
            }
            if self.current_state == LexerState::InToken {
                if let Some(token) = self.handle_in_token(char) {
                    return Some(token);
                }
            } else if self.current_state == LexerState::Start {
                self.start_new_token(char);
            }
        }

        self.finalize_token()
    }

    fn handle_in_token(&mut self, char: char) -> Option<Token> {
        if let Some(current_token) = self.current_token.as_mut() {
            if SymbolKind::can_add_char(char, &current_token.kind) {
                current_token.lexeme.push(char);
            } else {
                if char == '\n' {
                    self.current_line += 1;
                }

                self.current_state = LexerState::Start;
                let mut last_token = self.current_token.take().unwrap();
                self.start_new_token(char);
                Self::update_token_before_return(&mut last_token);
                return Some(last_token);
            }

            if (current_token.kind == SymbolKind::String && char == '"' ) || (current_token.kind == SymbolKind::Char && char == '\''){
                self.current_state = LexerState::Start;
                current_token.lexeme = current_token
                    .lexeme
                    .chars()
                    .skip(1)
                    .take(current_token.lexeme.chars().count() - 2)
                    .collect();
                return self.current_token.take();
            }
        }
        None
    }

    fn start_new_token(&mut self, char: char) {
        let token_type = match SymbolKind::choose_type_by_char(char) {
            Some(t) => t,
            _ => {
                error!("Error at line {}, parse token failed", self.current_line);
                std::process::exit(1);
            }
        };


        self.current_token = Some(Token::new(token_type, String::new(), self.current_line));

        if let Some(current_token) = self.current_token.as_mut() {
            current_token.lexeme.push(char);
        }

        self.current_state = LexerState::InToken;
    }

    fn finalize_token(&mut self) -> Option<Token> {
        if let Some(current_token) = self.current_token.as_mut() {
            if current_token.kind == SymbolKind::Identifier
                && (current_token.lexeme == "true" || current_token.lexeme == "false")
            {
                current_token.kind = SymbolKind::Bool;
            }
        }

        let mut token = self.current_token.take()?;
        Self::update_token_before_return(&mut token);
        Some(token)
    }

    fn update_token_before_return(token: &mut Token) {
        if token.get_kind() == SymbolKind::Identifier {
            if token.lexeme == "true" || token.lexeme == "false" {
                token.kind = SymbolKind::Bool;
            }
        }

    }
}
