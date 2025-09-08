use crate::error::IqraError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Identifier(String),
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Fn,
    Return,
    And,
    Or,
    Not,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Assign,
    Semi,
    Comma,
    Print,
    Let,
    If,
    Else,
    While,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

pub fn bad_number(pos: usize, line: usize, col: usize) -> IqraError {
    IqraError::Lex { pos, line, col, msg: "عدد غير صالح".into() }
}

pub type TokStream = Vec<Token>;

// Allow full unicode alphabetic starts (to support العربية) plus underscore
pub fn is_ident_start(c: char) -> bool {
    // Allow underscore and any alphabetic Unicode character (to support العربية)
    c == '_' || c.is_alphabetic()
}

pub fn is_ident_continue(c: char) -> bool {
    // Allow alphabetic starts, Unicode numeric digits (Arabic-Indic, etc.), underscore,
    // and Arabic question mark '؟' used in some identifiers in tests/examples.
    is_ident_start(c) || c.is_numeric() || c == '؟'
}

/// Convert an identifier/keyword into a keyword `TokenKind` if it matches.
/// Returns None when the identifier should remain an `Identifier(String)`.
pub fn token_for_keyword(ident: &str) -> Option<TokenKind> {
    Some(match ident {
        "print" | "اطبع" => TokenKind::Print,
        "true" | "صحيح" | "نعم" => TokenKind::Bool(true),
        "false" | "خطأ" | "لا" => TokenKind::Bool(false),
        "nil" | "لاشيء" => TokenKind::Nil,
        "if" | "اذا" | "إذا" => TokenKind::If,
        "else" | "وإلا" | "والا" | "وإلاّ" => TokenKind::Else,
        "while" | "بينما" => TokenKind::While,
        "let" | "دع" | "عرف" | "متغير" => TokenKind::Let,
        "fn" | "function" | "دالة" => TokenKind::Fn,
        "return" | "ارجع" | "أعد" | "اعِد" | "رجع" => TokenKind::Return,
        // logical words EN
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "not" => TokenKind::Not,
        // logical words AR
        "و" => TokenKind::And,
        "أو" | "او" => TokenKind::Or,
        "ليس" => TokenKind::Not,
        _ => return None,
    })
}
