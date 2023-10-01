use std::{fmt::{Display, Formatter, self, Debug}, str::FromStr, rc::Rc, io::Error};

use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, 
    RightParen, 
    LeftBrace, 
    RightBrace, 
    Comma, 
    Dot, 
    Minus, 
    Plus,
    Semicolon, 
    Slash, 
    Star,

    Bang, 
    BangEqual, 
    Equal, 
    EqualEqual, 
    Greater, 
    GreaterEqual, 
    Less, 
    LessEqual,

    Identifier, 
    String(String), 
    Number(f64),

    And, 
    Else, 
    False, 
    Fn, 
    For, 
    If, 
    Null, 
    Or, 
    Print, 
    Return, 
    True, 
    Let, 
    While,

    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub line: u32,
    pub typ: TokenType
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.lexeme, self.line)
    }
}

impl Token {
    fn new(lexeme: &str, line: u32, typ: TokenType) -> Token {
        Token { lexeme: String::from_str(lexeme).expect("string expected"), line, typ }
    }
}

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Rc<Token>>,
    pub start: usize,
    pub current: usize,
    pub line: u32
}

impl Scanner {
    pub fn scan_tokens(mut self) -> Vec<Rc<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                println!("Error occured while scanning: {}", e);
            }
        }

        self.tokens.push(Rc::new(Token::new("", self.line, TokenType::EOF)));
        self.tokens
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => match self.char_match('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang)
            },
            '=' => match self.char_match('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal)
            },
            '<' => match self.char_match('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less)
            },
            '>' => match self.char_match('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater)
            },
            
            '/' => match self.char_match('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                },
                false => self.add_token(TokenType::Slash)
            },

            '"' => self.string(),

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            c => {
                if Self::is_digit(c) {
                    self.number()
                } else if Self::is_alpha(c) {
                    self.identifier()
                } else {
                    return Err(error(self.line, &format!("Unexpected token '{}'.", c)));
                }
            }
        };
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = &self.source[self.current..self.current+1];
        self.current += 1;
        char::from_str(c).expect("char expected")
    }

    fn get_lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn add_token(&mut self, typ: TokenType) {
        self.tokens.push(Rc::new(Token::new(self.get_lexeme(), self.line, typ)));
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if char::from_str(&self.source[self.current..self.current+1]).expect("char expected") != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        char::from_str(&self.source[self.current..self.current+1]).expect("char expected")
    }

    fn peek_next(&mut self) -> char{
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        char::from_str(&self.source[self.current+1..self.current+2]).expect("char expected")
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        self.add_token(TokenType::String(String::from_str(&self.source[self.start+1..self.current-1]).expect("string expected")));
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number(f64::from_str(&self.source[self.start..self.current]).expect("number expected")));
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let txt = &self.source[self.start..self.current];
        let typ = match txt {
            "and" => TokenType::And,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "let" => TokenType::Let,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(typ);
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_digit(c) || Self::is_alpha(c)
    }
}