use std::fmt;
use crate::lang::runtime::IqraError;
use anyhow::{Result, anyhow};

// الدوال المساعدة يجب أن تكون هنا ليتمكن الـ impl من استخدامها
fn is_arabic_digit(ch: char) -> bool {
    matches!(ch, '٠'..='٩')
}

fn arabic_to_ascii_digit(ch: char) -> char {
    match ch {
        '٠' => '0',
        '١' => '1',
        '٢' => '2',
        '٣' => '3',
        '٤' => '4',
        '٥' => '5',
        '٦' => '6',
        '٧' => '7',
        '٨' => '8',
        '٩' => '9',
        _ => ch,
    }
}

fn is_arabic_letter(ch: char) -> bool {
    matches!(ch, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}')
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Error handling keywords (Arabic and English)
    Try,      // جرب / try
    Catch,    // امسك / catch
    Errors,   // أخطاء / errors
    // Literals
    Number(f64),
    String(String),
    Identifier(String),

    // Keywords (Arabic and English)
    If,    // اذا / إذا / if
    Else,  // وإلا / والا / وإلاّ / else
    While, // بينما / while
    True,  // صحيح / true
    False, // خطأ / false
    And,   // و / && / and
    Or,    // أو / || / or
    Not,   // ليس / ! / not
    Function, // دالة / function
    Return,   // ارجع / return

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Assign,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,

    // Special
    Newline,
    Eof,
}

// Display implementation for Token
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Identifier(id) => write!(f, "{}", id),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Not => write!(f, "not"),
            Token::Function => write!(f, "function"),
            Token::Return => write!(f, "return"),
            Token::Try => write!(f, "try"),
            Token::Catch => write!(f, "catch"),
            Token::Errors => write!(f, "errors"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Assign => write!(f, "="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Newline => write!(f, "\\n"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}


#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    line: usize,
}


impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();
        Self { input: chars, position: 0, current_char, line: 1 }
    }


    fn advance(&mut self) {
        self.position += 1;
        if self.current_char == Some('\n') {
            self.line += 1;
        }
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // Skip until end of line
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Result<f64> {
        let mut num_str = String::new();
        let start_line = self.line;
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else if is_arabic_digit(ch) {
                num_str.push(arabic_to_ascii_digit(ch));
                self.advance();
            } else {
                break;
            }
        }
        match num_str.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(anyhow!(IqraError {
                kind: "خطأ في الرقم | Number Error".to_string(),
                message_ar: format!("تعذر تحويل '{}' إلى رقم.", num_str),
                message_en: format!("Failed to parse '{}' as a number.", num_str),
                suggestion: Some("تأكد من صحة الرقم المدخل | Check the input number".to_string()),
                line: Some(start_line),
            }))
        }
    }

    fn read_string(&mut self) -> Result<String> {
        let mut string = String::new();
        let start_line = self.line;
        self.advance(); // Skip opening quote
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(string);
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        Err(anyhow!(IqraError {
            kind: "خطأ في السلسلة | String Error".to_string(),
            message_ar: "سلسلة غير منتهية بعلامة اقتباس.".to_string(),
            message_en: "Unterminated string literal.".to_string(),
            suggestion: Some("تأكد من إغلاق السلسلة بعلامة اقتباس | Close the string with a quote".to_string()),
            line: Some(start_line),
        }))
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || is_arabic_letter(ch) {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    pub fn next_token(&mut self) -> Result<Token> {
        loop {
            match self.current_char {
                None => return Ok(Token::Eof),
                Some(ch) if ch.is_whitespace() && ch != '\n' => {
                    self.skip_whitespace();
                    continue;
                }
                Some('\n') => {
                    self.advance();
                    return Ok(Token::Newline);
                }
                Some(ch) if ch.is_ascii_digit() || is_arabic_digit(ch) => {
                    let n = self.read_number();
                    match n {
                        Ok(val) => return Ok(Token::Number(val)),
                        Err(e) => return Err(e),
                    }
                }
                Some('"') => {
                    let s = self.read_string();
                    match s {
                        Ok(val) => return Ok(Token::String(val)),
                        Err(e) => return Err(e),
                    }
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' || is_arabic_letter(ch) => {
                    let identifier = self.read_identifier();
                    let t = match identifier.as_str() {
                        // Arabic keywords
                        "اذا" | "إذا" => Token::If,
                        "وإلا" | "والا" | "وإلاّ" => Token::Else,
                        "بينما" => Token::While,
                        "صحيح" => Token::True,
                        "خطأ" => Token::False,
                        "و" => Token::And,
                        "أو" => Token::Or,
                        "ليس" => Token::Not,
                        "دالة" => Token::Function,
                        "ارجع" => Token::Return,
                        "جرب" => Token::Try,
                        "امسك" => Token::Catch,

                        // English keywords
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "true" => Token::True,
                        "false" => Token::False,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        "function" => Token::Function,
                        "return" => Token::Return,
                        "try" => Token::Try,
                        "catch" => Token::Catch,

                        _ => Token::Identifier(identifier),
                    };
                    return Ok(t);
                }
                Some('+') => {
                    self.advance();
                    return Ok(Token::Plus);
                }
                Some('-') => {
                    self.advance();
                    return Ok(Token::Minus);
                }
                Some('*') => {
                    self.advance();
                    return Ok(Token::Multiply);
                }
                Some('/') => {
                    if self.peek() == Some('/') {
                        self.skip_comment();
                        continue;
                    } else {
                        self.advance();
                        return Ok(Token::Divide);
                    }
                }
                Some('%') => {
                    self.advance();
                    return Ok(Token::Modulo);
                }
                Some('=') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::Equal);
                    }
                    return Ok(Token::Assign);
                }
                Some('!') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::NotEqual);
                    }
                    return Ok(Token::Not);
                }
                Some('<') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::LessEqual);
                    }
                    return Ok(Token::Less);
                }
                Some('>') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::GreaterEqual);
                    }
                    return Ok(Token::Greater);
                }
                Some('&') => {
                    self.advance();
                    if self.current_char == Some('&') {
                        self.advance();
                        return Ok(Token::And);
                    }
                    return Ok(Token::Identifier("&".to_string()));
                }
                Some('|') => {
                    self.advance();
                    if self.current_char == Some('|') {
                        self.advance();
                        return Ok(Token::Or);
                    }
                    return Ok(Token::Identifier("|".to_string()));
                }
                Some('(') => {
                    self.advance();
                    return Ok(Token::LeftParen);
                }
                Some(')') => {
                    self.advance();
                    return Ok(Token::RightParen);
                }
                Some('{') => {
                    self.advance();
                    return Ok(Token::LeftBrace);
                }
                Some('}') => {
                    self.advance();
                    return Ok(Token::RightBrace);
                }
                Some('[') => {
                    self.advance();
                    return Ok(Token::LeftBracket);
                }
                Some(']') => {
                    self.advance();
                    return Ok(Token::RightBracket);
                }
                Some(',') => {
                    self.advance();
                    return Ok(Token::Comma);
                }
                Some(';') => {
                    self.advance();
                    return Ok(Token::Semicolon);
                }
                Some(ch) => {
                    let err = IqraError {
                        kind: "رمز غير معروف | Unknown Character".to_string(),
                        message_ar: format!("رمز غير معروف: '{}'", ch),
                        message_en: format!("Unknown character: '{}'", ch),
                        suggestion: Some("تأكد من صحة الكود | Check your code".to_string()),
                        line: Some(self.line),
                    };
                    self.advance();
                    return Err(anyhow!(err));
                }
            }
        }
    }
}
