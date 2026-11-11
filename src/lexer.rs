#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Let,
    Print,
    If,
    Else,
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    EqEq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let end = token == Token::Eof;
            tokens.push(token);

            if end {
                break;
            }
        }
        
        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next_token(&mut self) -> Result<Token, String> {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        if c.is_ascii_digit() {
            return self.read_number();
        }

        if c.is_alphabetic() || c == '_' {
            return Ok(self.read_word());
        }

        self.pos += 1;
        let next_is_eq = self.peek() == Some('=');
        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => {
                if next_is_eq {
                    self.pos += 1;
                    Token::EqEq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if next_is_eq {
                    self.pos += 1;
                    Token::NotEq
                } else {
                    return Err("Лексическая ошибка: символ '!' допустим только в '!='".to_string());
                }
            }
            '>' => {
                if next_is_eq {
                    self.pos += 1;
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            }
            '<' => {
                if next_is_eq {
                    self.pos += 1;
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            other => {
                return Err(format!("Лексическая ошибка: неизвестный символ '{}'", other));
            }
        };
        Ok(token)
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let start = self.pos;
        let mut dot_seen = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if c == '.' && !dot_seen {
                dot_seen = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Лексическая ошибка: неверное число '{}'", text))
    }

    fn read_word(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        match text.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(text),
        }
    }
}
