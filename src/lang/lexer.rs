use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();
        Self { input: chars, position: 0, current_char }
    }

    fn advance(&mut self) {
        self.position += 1;
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

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else if is_arabic_digit(ch) {
                // Convert Arabic digits to ASCII
                num_str.push(arabic_to_ascii_digit(ch));
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
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

        string
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

    pub fn next_token(&mut self) -> Token {
        loop {
            match self.current_char {
                None => return Token::Eof,
                Some(ch) if ch.is_whitespace() && ch != '\n' => {
                    self.skip_whitespace();
                }
                Some('\n') => {
                    self.advance();
                    return Token::Newline;
                }
                Some(ch) if ch.is_ascii_digit() || is_arabic_digit(ch) => {
                    return Token::Number(self.read_number());
                }
                Some('"') => {
                    return Token::String(self.read_string());
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' || is_arabic_letter(ch) => {
                    let identifier = self.read_identifier();
                    return match identifier.as_str() {
                        // Arabic keywords
                        "اذا" | "إذا" => Token::If,
                        "وإلا" | "والا" | "وإلاّ" => Token::Else,
                        "بينما" => Token::While,
                        "صحيح" => Token::True,
                        "خطأ" => Token::False,
                        "و" => Token::And,
                        "أو" => Token::Or,
                        "ليس" => Token::Not,

                        // English keywords
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "true" => Token::True,
                        "false" => Token::False,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,

                        _ => Token::Identifier(identifier),
                    };
                }
                Some('+') => {
                    self.advance();
                    return Token::Plus;
                }
                Some('-') => {
                    self.advance();
                    return Token::Minus;
                }
                Some('*') => {
                    self.advance();
                    return Token::Multiply;
                }
                Some('/') => {
                    if self.peek() == Some('/') {
                        // Line comment
                        self.skip_comment();
                        continue;
                    } else {
                        self.advance();
                        return Token::Divide;
                    }
                }
                Some('%') => {
                    self.advance();
                    return Token::Modulo;
                }
                Some('=') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::Equal;
                    }
                    return Token::Assign;
                }
                Some('!') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::NotEqual;
                    }
                    return Token::Not;
                }
                Some('<') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::LessEqual;
                    }
                    return Token::Less;
                }
                Some('>') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::GreaterEqual;
                    }
                    return Token::Greater;
                }
                Some('&') => {
                    self.advance();
                    if self.current_char == Some('&') {
                        self.advance();
                        return Token::And;
                    }
                    // Single & is not supported, treat as identifier
                    return Token::Identifier("&".to_string());
                }
                Some('|') => {
                    self.advance();
                    if self.current_char == Some('|') {
                        self.advance();
                        return Token::Or;
                    }
                    // Single | is not supported, treat as identifier
                    return Token::Identifier("|".to_string());
                }
                Some('(') => {
                    self.advance();
                    return Token::LeftParen;
                }
                Some(')') => {
                    self.advance();
                    return Token::RightParen;
                }
                Some('{') => {
                    self.advance();
                    return Token::LeftBrace;
                }
                Some('}') => {
                    self.advance();
                    return Token::RightBrace;
                }
                Some('[') => {
                    self.advance();
                    return Token::LeftBracket;
                }
                Some(']') => {
                    self.advance();
                    return Token::RightBracket;
                }
                Some(',') => {
                    self.advance();
                    return Token::Comma;
                }
                Some(';') => {
                    self.advance();
                    return Token::Semicolon;
                }
                Some(_ch) => {
                    // Skip unknown characters
                    self.advance();
                    continue;
                }
            }
        }
    }
}

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
