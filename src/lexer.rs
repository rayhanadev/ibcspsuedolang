use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Assign,
    Output,
    Identifier(String),
    Number(i64),
    String(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    If,
    Then,
    Else,
    EndIf,
    Loop,
    EndLoop,
    While,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Not,
    Mod,
    Div,
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
            line: 1,
            column: 0,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.input.next();
    }

    fn identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn number(&mut self) -> i64 {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result.parse().unwrap()
    }

    fn string(&mut self) -> String {
        let mut result = String::new();
        self.advance(); // Skip the opening quote
        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            result.push(c);
            self.advance();
        }
        self.advance(); // Skip the closing quote
        result
    }

    pub fn get_next_token(&mut self) -> TokenInfo {
        while let Some(c) = self.current_char {
            let token = match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.advance();
                    continue;
                }
                '=' => {
                    self.advance();
                    Token::Assign
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Star
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::GreaterThanOrEqual
                    } else {
                        Token::GreaterThan
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::LessThanOrEqual
                    } else {
                        Token::LessThan
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        panic!("Unexpected character: {}", c);
                    }
                }
                '"' => Token::String(self.string()),
                c if c.is_digit(10) => Token::Number(self.number()),
                c if c.is_alphabetic() => {
                    let id = self.identifier();
                    match id.as_str() {
                        "output" => Token::Output,
                        "if" => Token::If,
                        "then" => Token::Then,
                        "else" => Token::Else,
                        "endif" => Token::EndIf,
                        "loop" => Token::Loop,
                        "endloop" => Token::EndLoop,
                        "while" => Token::While,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        "mod" => Token::Mod,
                        "div" => Token::Div,
                        _ => Token::Identifier(id),
                    }
                }
                _ => panic!("Unexpected character: {}", c),
            };
            return TokenInfo {
                token,
                line: self.line,
                column: self.column,
            };
        }
        TokenInfo {
            token: Token::Eof,
            line: self.line,
            column: self.column,
        }
    }
}
