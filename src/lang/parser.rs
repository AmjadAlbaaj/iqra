use crate::lang::lexer::{Lexer, Token};
use crate::lang::value::Value;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Identifier(String),
    Binary { left: Box<Expr>, operator: BinaryOp, right: Box<Expr> },
    Unary { operator: UnaryOp, operand: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
    List(Vec<Expr>),
    Index { object: Box<Expr>, index: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Assignment { name: String, value: Expr },
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Block(Vec<Stmt>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("Expected {:?}, found {:?}", expected, self.current_token))
        }
    }

    fn skip_newlines(&mut self) {
        while self.current_token == Token::Newline {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        self.skip_newlines();
        while self.current_token != Token::Eof {
            statements.push(self.statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt> {
        match &self.current_token {
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::LeftBrace => self.block_statement(),
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.current_token == Token::Assign {
                    self.advance();
                    let value = self.expression()?;
                    Ok(Stmt::Assignment { name, value })
                } else {
                    // Put the identifier back and parse as expression
                    let expr = Expr::Identifier(name);
                    // Parse the rest of the expression if there's more
                    let full_expr = self.parse_expression_continuation(expr)?;
                    Ok(Stmt::Expression(full_expr))
                }
            }
            _ => {
                let expr = self.expression()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.expect(Token::If)?;
        let condition = self.expression()?;
        self.expect(Token::LeftBrace)?;
        let then_branch = self.block_body()?;
        self.expect(Token::RightBrace)?;

        let else_branch = if self.current_token == Token::Else {
            self.advance();
            self.expect(Token::LeftBrace)?;
            let else_body = self.block_body()?;
            self.expect(Token::RightBrace)?;
            Some(else_body)
        } else {
            None
        };

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.expect(Token::While)?;
        let condition = self.expression()?;
        self.expect(Token::LeftBrace)?;
        let body = self.block_body()?;
        self.expect(Token::RightBrace)?;

        Ok(Stmt::While { condition, body })
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        self.expect(Token::LeftBrace)?;
        let body = self.block_body()?;
        self.expect(Token::RightBrace)?;

        Ok(Stmt::Block(body))
    }

    fn block_body(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        self.skip_newlines();
        while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
            statements.push(self.statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.or_expression()
    }

    fn parse_expression_continuation(&mut self, left: Expr) -> Result<Expr> {
        // Handle function calls and other operators
        match &self.current_token {
            Token::LeftParen => {
                if let Expr::Identifier(name) = left {
                    self.advance(); // consume '('
                    let args = self.argument_list()?;
                    self.expect(Token::RightParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Err(anyhow!("Invalid function call"))
                }
            }
            _ => {
                // Parse binary operations with the existing left expression
                self.parse_binary_with_left(left, 0)
            }
        }
    }

    fn or_expression(&mut self) -> Result<Expr> {
        let mut expr = self.and_expression()?;

        while self.current_token == Token::Or {
            self.advance();
            let right = self.and_expression()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and_expression(&mut self) -> Result<Expr> {
        let mut expr = self.equality_expression()?;

        while self.current_token == Token::And {
            self.advance();
            let right = self.equality_expression()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality_expression(&mut self) -> Result<Expr> {
        let mut expr = self.comparison_expression()?;

        while matches!(self.current_token, Token::Equal | Token::NotEqual) {
            let op = match self.current_token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.comparison_expression()?;
            expr = Expr::Binary { left: Box::new(expr), operator: op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn comparison_expression(&mut self) -> Result<Expr> {
        let mut expr = self.term_expression()?;

        while matches!(
            self.current_token,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
        ) {
            let op = match self.current_token {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.term_expression()?;
            expr = Expr::Binary { left: Box::new(expr), operator: op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn term_expression(&mut self) -> Result<Expr> {
        let mut expr = self.factor_expression()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = match self.current_token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.factor_expression()?;
            expr = Expr::Binary { left: Box::new(expr), operator: op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn factor_expression(&mut self) -> Result<Expr> {
        let mut expr = self.unary_expression()?;

        while matches!(self.current_token, Token::Multiply | Token::Divide | Token::Modulo) {
            let op = match self.current_token {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                Token::Modulo => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.unary_expression()?;
            expr = Expr::Binary { left: Box::new(expr), operator: op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn unary_expression(&mut self) -> Result<Expr> {
        match &self.current_token {
            Token::Not => {
                self.advance();
                let operand = self.unary_expression()?;
                Ok(Expr::Unary { operator: UnaryOp::Not, operand: Box::new(operand) })
            }
            Token::Minus => {
                self.advance();
                let operand = self.unary_expression()?;
                Ok(Expr::Unary { operator: UnaryOp::Minus, operand: Box::new(operand) })
            }
            _ => self.primary_expression(),
        }
    }

    fn primary_expression(&mut self) -> Result<Expr> {
        match &self.current_token.clone() {
            Token::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Literal(Value::Number(value)))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::Literal(Value::String(value)))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Literal(Value::Bool(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Literal(Value::Bool(false)))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.current_token == Token::LeftParen {
                    // Function call
                    self.advance();
                    let args = self.argument_list()?;
                    self.expect(Token::RightParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let elements = if self.current_token == Token::RightBracket {
                    Vec::new()
                } else {
                    self.expression_list()?
                };
                self.expect(Token::RightBracket)?;
                Ok(Expr::List(elements))
            }
            _ => Err(anyhow!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn parse_binary_with_left(&mut self, left: Expr, min_precedence: i32) -> Result<Expr> {
        let mut left = left;

        while let Some(op) = self.binary_operator() {
            let precedence = self.operator_precedence(&op);
            if precedence < min_precedence {
                break;
            }

            self.advance();
            let mut right = self.unary_expression()?;

            while let Some(next_op) = self.binary_operator() {
                let next_precedence = self.operator_precedence(&next_op);
                if next_precedence <= precedence {
                    break;
                }
                right = self.parse_binary_with_left(right, next_precedence)?;
            }

            left = Expr::Binary { left: Box::new(left), operator: op, right: Box::new(right) };
        }

        Ok(left)
    }

    fn binary_operator(&self) -> Option<BinaryOp> {
        match self.current_token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Subtract),
            Token::Multiply => Some(BinaryOp::Multiply),
            Token::Divide => Some(BinaryOp::Divide),
            Token::Modulo => Some(BinaryOp::Modulo),
            Token::Equal => Some(BinaryOp::Equal),
            Token::NotEqual => Some(BinaryOp::NotEqual),
            Token::Less => Some(BinaryOp::Less),
            Token::LessEqual => Some(BinaryOp::LessEqual),
            Token::Greater => Some(BinaryOp::Greater),
            Token::GreaterEqual => Some(BinaryOp::GreaterEqual),
            Token::And => Some(BinaryOp::And),
            Token::Or => Some(BinaryOp::Or),
            _ => None,
        }
    }

    fn operator_precedence(&self, op: &BinaryOp) -> i32 {
        match op {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Equal | BinaryOp::NotEqual => 3,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => 4,
            BinaryOp::Add | BinaryOp::Subtract => 5,
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => 6,
        }
    }

    fn argument_list(&mut self) -> Result<Vec<Expr>> {
        if self.current_token == Token::RightParen {
            return Ok(Vec::new());
        }

        self.expression_list()
    }

    fn expression_list(&mut self) -> Result<Vec<Expr>> {
        let mut expressions = vec![self.expression()?];

        while self.current_token == Token::Comma {
            self.advance();
            expressions.push(self.expression()?);
        }

        Ok(expressions)
    }
}
