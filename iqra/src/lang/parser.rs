use crate::error::{IqraError, Result};
use crate::lang::ast::{Expr, Stmt};
use crate::lang::token::{Token, TokenKind};

pub fn parse(tokens: &[Token]) -> Result<Vec<Stmt>> {
    let mut p = Parser { toks: tokens, idx: 0 };
    let mut stmts = Vec::new();
    while !matches!(p.peek().kind, TokenKind::EOF) {
        stmts.push(p.statement()?);
    }
    Ok(stmts)
}

struct Parser<'a> {
    toks: &'a [Token],
    idx: usize,
}
impl<'a> Parser<'a> {
    fn peek(&self) -> &Token {
        &self.toks[self.idx]
    }
    fn advance(&mut self) {
        if self.idx < self.toks.len() - 1 {
            self.idx += 1;
        }
    }
    fn consume(&mut self, kind: &TokenKind, msg: &str) -> Result<()> {
        if &self.peek().kind == kind {
            self.advance();
            Ok(())
        } else {
            let t = self.peek();
            Err(IqraError::Parse { line: t.line, col: t.col, msg: msg.into() })
        }
    }

    fn statement(&mut self) -> Result<Stmt> {
        match &self.peek().kind {
            TokenKind::LBrace => self.block_stmt(),
            TokenKind::LBracket => {
                let expr = self.list_expr()?;
                self.optional_semi();
                Ok(Stmt::Expr(expr))
            }
            TokenKind::Print => {
                self.advance();
                let expr = self.expression()?;
                self.optional_semi();
                Ok(Stmt::Print(expr))
            }
            TokenKind::If => self.if_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Fn => self.function_stmt(),
            TokenKind::Return => {
                self.advance();
                if matches!(self.peek().kind, TokenKind::Semi) {
                    self.optional_semi();
                    return Ok(Stmt::Return(None));
                }
                let expr = self.expression()?;
                self.optional_semi();
                Ok(Stmt::Return(Some(expr)))
            }
            TokenKind::Let => {
                self.advance();
                if let TokenKind::Identifier(name) = &self.peek().kind {
                    let var_name = name.clone();
                    self.advance();
                    self.consume(&TokenKind::Assign, "= متوقع بعد let")?;
                    let value = self.expression()?;
                    self.optional_semi();
                    Ok(Stmt::Assign { name: var_name, value })
                } else {
                    let t = self.peek();
                    Err(IqraError::Parse {
                        line: t.line,
                        col: t.col,
                        msg: "اسم متغير متوقع بعد let".into(),
                    })
                }
            }
            TokenKind::Identifier(name) => {
                if self
                    .toks
                    .get(self.idx + 1)
                    .map(|t| matches!(t.kind, TokenKind::Assign))
                    .unwrap_or(false)
                {
                    let name = name.clone();
                    self.advance();
                    self.advance();
                    let value = self.expression()?;
                    self.optional_semi();
                    Ok(Stmt::Assign { name, value })
                } else {
                    let expr = self.expression()?;
                    self.optional_semi();
                    Ok(Stmt::Expr(expr))
                }
            }
            _ => {
                let expr = self.expression()?;
                self.optional_semi();
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn list_expr(&mut self) -> Result<Expr> {
        self.advance(); // skip LBracket
        let mut items = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBracket) {
            items.push(self.expression()?);
            if matches!(self.peek().kind, TokenKind::Comma) {
                self.advance();
            }
        }
        self.advance(); // skip RBracket
        Ok(Expr::List(items))
    }

    fn optional_semi(&mut self) {
        if matches!(self.peek().kind, TokenKind::Semi | TokenKind::Comma) {
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;
        while matches!(self.peek().kind, TokenKind::Or) {
            let op = self.peek().kind.clone();
            self.advance();
            let rhs = self.and()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(rhs) };
        }
        Ok(expr)
    }
    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;
        while matches!(self.peek().kind, TokenKind::And) {
            let op = self.peek().kind.clone();
            self.advance();
            let rhs = self.equality()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while matches!(self.peek().kind, TokenKind::Eq | TokenKind::Ne) {
            let op = self.peek().kind.clone();
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while matches!(
            self.peek().kind,
            TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge
        ) {
            let op = self.peek().kind.clone();
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = self.peek().kind.clone();
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash) {
            let op = self.peek().kind.clone();
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr> {
        match self.peek().kind {
            TokenKind::Not => {
                let op = self.peek().kind.clone();
                self.advance();
                let expr = self.unary()?;
                Ok(Expr::Unary { op, expr: Box::new(expr) })
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        let mut expr = match &self.peek().kind {
            TokenKind::Number(n) => {
                let v = *n;
                self.advance();
                Expr::Number(v)
            }
            TokenKind::String(s) => {
                let v = s.clone();
                self.advance();
                Expr::Str(v)
            }
            TokenKind::Bool(b) => {
                let v = *b;
                self.advance();
                Expr::Bool(v)
            }
            TokenKind::Nil => {
                self.advance();
                Expr::Nil
            }
            TokenKind::Identifier(name) => {
                let n = name.clone();
                self.advance();
                Expr::Var(n)
            }
            TokenKind::LParen => {
                self.advance();
                let e = self.expression()?;
                self.consume(&TokenKind::RParen, ") متوقع")?;
                e
            }
            TokenKind::LBracket => {
                self.advance(); // skip LBracket
                let mut items = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RBracket) {
                    items.push(self.expression()?);
                    if matches!(self.peek().kind, TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.advance(); // skip RBracket
                Expr::List(items)
            }
            _ => {
                let t = self.peek();
                return Err(IqraError::Parse {
                    line: t.line,
                    col: t.col,
                    msg: "رمز غير متوقع".into(),
                });
            }
        };
        // Handle call chains
        loop {
            if matches!(self.peek().kind, TokenKind::LParen) {
                self.advance(); // consume '('
                let mut args = Vec::new();
                if !matches!(self.peek().kind, TokenKind::RParen) {
                    loop {
                        let a = self.expression()?;
                        args.push(a);
                        if matches!(self.peek().kind, TokenKind::Comma) {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                }
                self.consume(&TokenKind::RParen, ") متوقع بعد الوسائط")?;
                expr = Expr::Call { callee: Box::new(expr), args };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn block_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenKind::LBrace, "{ متوقع")?;
        let mut stmts = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::EOF) {
            stmts.push(self.statement()?);
        }
        self.consume(&TokenKind::RBrace, "} متوقع")?;
        Ok(Stmt::Block(stmts))
    }

    fn if_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenKind::If, "مزامنة داخلية if")?; // consume 'if'
        let cond = self.expression()?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if matches!(self.peek().kind, TokenKind::Else) {
            self.advance();
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If { cond, then_branch, else_branch })
    }

    fn while_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenKind::While, "مزامنة داخلية while")?;
        let cond = self.expression()?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { cond, body })
    }

    fn function_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenKind::Fn, "مزامنة داخلية دالة")?;
        let name = match &self.peek().kind {
            TokenKind::Identifier(n) => {
                let v = n.clone();
                self.advance();
                v
            }
            _ => {
                let t = self.peek();
                return Err(IqraError::Parse {
                    line: t.line,
                    col: t.col,
                    msg: "اسم الدالة مفقود".into(),
                });
            }
        };
        self.consume(&TokenKind::LParen, "( متوقع")?;
        let mut params = Vec::new();
        if !matches!(self.peek().kind, TokenKind::RParen) {
            loop {
                match &self.peek().kind {
                    TokenKind::Identifier(p) => {
                        params.push(p.clone());
                        self.advance();
                    }
                    _ => {
                        let t = self.peek();
                        return Err(IqraError::Parse {
                            line: t.line,
                            col: t.col,
                            msg: "معامل غير صالح".into(),
                        });
                    }
                }
                if matches!(self.peek().kind, TokenKind::Comma) {
                    self.advance();
                    continue;
                }
                break;
            }
        }
        self.consume(&TokenKind::RParen, ") متوقع")?;
        let body_stmt = self.statement()?; // expect a block
        let body_vec = match body_stmt {
            Stmt::Block(stmts) => stmts,
            _ => vec![body_stmt],
        };
        Ok(Stmt::Function { name, params, body: body_vec })
    }
}
