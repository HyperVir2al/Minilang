use crate::ast::{BinaryOp, Expr, Stmt};
use crate::lexer::Token;
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while *self.peek() != Token::Eof {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token, what: &str) -> Result<(), String> {
        if *self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Синтаксическая ошибка: ожидалось {}, найдено {:?}",
                what,
                self.peek()
            ))
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => {
                self.advance();
                let name = match self.advance() {
                    Token::Ident(name) => name,
                    other => {
                        return Err(format!(
                            "Синтаксическая ошибка: после let ожидалось имя переменной, найдено {:?}",
                            other
                        ))
                    }
                };
                self.expect(Token::Assign, "'='")?;
                let expr = self.parse_expression()?;
                Ok(Stmt::Let(name, expr))
            }
            Token::Print => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::Print(expr))
            }
            Token::If => {
                self.advance();
                let condition = self.parse_expression()?;
                let then_block = self.parse_block()?;
                let else_block = if *self.peek() == Token::Else {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };
                Ok(Stmt::If(condition, then_block, else_block))
            }
            other => Err(format!(
                "Синтаксическая ошибка: неожиданный токен {:?}",
                other
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace, "'{'")?;
        let mut stmts = Vec::new();
        while *self.peek() != Token::RBrace {
            if *self.peek() == Token::Eof {
                return Err("Синтаксическая ошибка: не найдена закрывающая '}'".to_string());
            }
            stmts.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace, "'}'")?;
        Ok(stmts)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => BinaryOp::Eq,
                Token::NotEq => BinaryOp::NotEq,
                Token::Greater => BinaryOp::Greater,
                Token::Less => BinaryOp::Less,
                Token::GreaterEq => BinaryOp::GreaterEq,
                Token::LessEq => BinaryOp::LessEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if *self.peek() == Token::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Negate(Box::new(operand)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Literal(Value::Number(n))),
            Token::True => Ok(Expr::Literal(Value::Bool(true))),
            Token::False => Ok(Expr::Literal(Value::Bool(false))),
            Token::Ident(name) => Ok(Expr::Variable(name)),
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen, "')'")?;
                Ok(expr)
            }
            other => Err(format!(
                "Синтаксическая ошибка: ожидалось выражение, найдено {:?}",
                other
            )),
        }
    }
}
