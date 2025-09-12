use crate::error::IqraError;
use crate::error::Result;
use crate::lang::token::{Token, TokenKind, bad_number, is_ident_continue, is_ident_start};

fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || matches!(c, '٠'..='٩')
}
fn arabic_to_ascii_digit(c: char) -> Option<char> {
    match c {
        '٠' => Some('0'),
        '١' => Some('1'),
        '٢' => Some('2'),
        '٣' => Some('3'),
        '٤' => Some('4'),
        '٥' => Some('5'),
        '٦' => Some('6'),
        '٧' => Some('7'),
        '٨' => Some('8'),
        '٩' => Some('9'),
        _ if c.is_ascii_digit() => Some(c),
        _ => None,
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>> {
    let mut toks = Vec::new();
    let mut chars = input.char_indices().peekable();
    let mut line = 1usize;
    let mut col = 1usize;
    let bytes = input.as_bytes();
    let len = bytes.len();
    while let Some((i, ch)) = chars.peek().cloned() {
        let start_line = line;
        let start_col = col;
        match ch {
            '[' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::LBracket,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            ']' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::RBracket,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            ' ' | '\t' => {
                chars.next();
                col += 1;
            }
            '\n' => {
                chars.next();
                line += 1;
                col = 1;
            }
            '\r' => {
                chars.next();
            }
            '#' => {
                // consume until end of line
                for (_j, c2) in chars.by_ref() {
                    if c2 == '\n' {
                        line += 1;
                        col = 1;
                        break;
                    } else {
                        col += 1;
                    }
                }
            }
            '"' | '\'' => {
                let quote = ch;
                chars.next();
                col += 1;
                let mut escaped = false;
                let mut buf = String::new();
                for (_j, c2) in chars.by_ref() {
                    match c2 {
                        '\n' => {
                            return Err(IqraError::Lex {
                                pos: i,
                                line: start_line,
                                col: start_col,
                                msg: "سلسلة نصية غير مكتملة".into(),
                            });
                        }
                        '\\' if !escaped => {
                            escaped = true;
                        }
                        q if q == quote && !escaped => {
                            break;
                        }
                        _ => {
                            if escaped {
                                // لا نترجم \n/\r/\t افتراضياً للحفاظ على مسارات Windows
                                // نسمح فقط بهروب نفس علامة الاقتباس أو الباك-سلاش
                                if c2 == '\\' || c2 == quote {
                                    buf.push(c2);
                                } else {
                                    buf.push('\\');
                                    buf.push(c2);
                                }
                                escaped = false;
                            } else {
                                buf.push(c2);
                            }
                        }
                    }
                    if c2 == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                toks.push(Token {
                    kind: TokenKind::String(buf),
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '{' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::LBrace,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '}' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::RBrace,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '+' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Plus,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '-' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Minus,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '*' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Star,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '/' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Slash,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '&' => {
                chars.next();
                col += 1;
                if let Some((_, '&')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::And,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    return Err(IqraError::Lex {
                        pos: i,
                        line: start_line,
                        col: start_col,
                        msg: "كان متوقع &&".into(),
                    });
                }
            }
            '|' => {
                chars.next();
                col += 1;
                if let Some((_, '|')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::Or,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    return Err(IqraError::Lex {
                        pos: i,
                        line: start_line,
                        col: start_col,
                        msg: "كان متوقع ||".into(),
                    });
                }
            }
            '(' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::LParen,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            ')' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::RParen,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            ';' | '؛' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Semi,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            ',' | '،' => {
                chars.next();
                col += 1;
                toks.push(Token {
                    kind: TokenKind::Comma,
                    pos: i,
                    line: start_line,
                    col: start_col,
                });
            }
            '=' => {
                chars.next();
                col += 1;
                if let Some((_, '=')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::Eq,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    toks.push(Token {
                        kind: TokenKind::Assign,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                }
            }
            '!' => {
                chars.next();
                col += 1;
                if let Some((_, '=')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::Ne,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    toks.push(Token {
                        kind: TokenKind::Not,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                }
            }
            '<' => {
                chars.next();
                col += 1;
                if let Some((_, '=')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::Le,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    toks.push(Token {
                        kind: TokenKind::Lt,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                }
            }
            '>' => {
                chars.next();
                col += 1;
                if let Some((_, '=')) = chars.peek().cloned() {
                    chars.next();
                    col += 1;
                    toks.push(Token {
                        kind: TokenKind::Ge,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    toks.push(Token {
                        kind: TokenKind::Gt,
                        pos: i,
                        line: start_line,
                        col: start_col,
                    });
                }
            }
            c if is_digit(c) => {
                let start = i;
                let mut dot_count = 0;
                let mut num_buf = String::new();
                // consume first digit
                chars.next();
                col += 1;
                if let Some(d) = arabic_to_ascii_digit(ch) {
                    num_buf.push(d);
                }
                while let Some((_j, d)) = chars.peek().cloned() {
                    if is_digit(d) || d == '.' {
                        if d == '.' {
                            dot_count += 1;
                            if dot_count > 1 {
                                break;
                            }
                            num_buf.push('.');
                        } else if let Some(ad) = arabic_to_ascii_digit(d) {
                            num_buf.push(ad);
                        }
                        chars.next();
                        col += 1;
                    } else {
                        break;
                    }
                }
                let val: f64 =
                    num_buf.parse().map_err(|_| bad_number(start, start_line, start_col))?;
                toks.push(Token {
                    kind: TokenKind::Number(val),
                    pos: start,
                    line: start_line,
                    col: start_col,
                });
            }
            c if is_ident_start(c) => {
                let start = i;
                chars.next();
                col += 1;
                let mut last_end = i + ch.len_utf8();
                while let Some((j, nc)) = chars.peek().cloned() {
                    if is_ident_continue(nc) {
                        chars.next();
                        col += 1;
                        last_end = j + nc.len_utf8();
                    } else {
                        break;
                    }
                }
                let ident = &input[start..last_end];
                let tk = if let Some(kind) = crate::lang::token::token_for_keyword(ident) {
                    kind
                } else {
                    TokenKind::Identifier(ident.into())
                };
                toks.push(Token { kind: tk, pos: start, line: start_line, col: start_col });
            }
            _ => {
                return Err(IqraError::Lex {
                    pos: i,
                    line: start_line,
                    col: start_col,
                    msg: format!("محرف غير متوقع '{ch}'"),
                });
            }
        }
    }
    toks.push(Token { kind: TokenKind::EOF, pos: len, line, col });
    Ok(toks)
}
