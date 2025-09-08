use thiserror::Error;

#[derive(Debug, Error)]
pub enum IqraError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("خطأ تحليل لفظي عند {line}:{col}: {msg}")]
    Lex { pos: usize, line: usize, col: usize, msg: String },
    #[error("خطأ تركيب نحوي عند {line}:{col}: {msg}")]
    Parse { line: usize, col: usize, msg: String },
    #[error("خطأ وقت التنفيذ: {msg}")]
    Runtime { msg: String },
}

impl IqraError {
    pub fn runtime<M: Into<String>>(msg: M) -> Self {
        IqraError::Runtime { msg: msg.into() }
    }

    /// دالة احترافية لإنشاء رسالة خطأ ثنائية اللغة
    pub fn new_localized<M: Into<String>, A: Into<String>>(en: M, ar: A) -> Self {
        let msg = format!("{} | {}", en.into(), ar.into());
        IqraError::Runtime { msg }
    }
}

pub type Result<T> = std::result::Result<T, IqraError>;
