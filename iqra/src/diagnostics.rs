use crate::error::IqraError;
use owo_colors::OwoColorize;
use serde::Serialize;

pub fn render_error(src: &str, err: &IqraError) -> String {
    render_error_with_opts(src, err, false)
}

pub fn render_error_with_opts(src: &str, err: &IqraError, color: bool) -> String {
    let (line, col, phase, msg, simple) = match err {
        IqraError::Lex { line, col, msg, .. } => {
            (*line, *col, "\u{200F}خطأ لفظي", msg.as_str(), false)
        }
        IqraError::Parse { line, col, msg } => {
            (*line, *col, "\u{200F}خطأ نحوي", msg.as_str(), false)
        }
        IqraError::Runtime { msg } => (0, 0, "\u{200F}خطأ وقت التنفيذ", msg.as_str(), true),
        IqraError::InvalidInput(m) => (0, 0, "\u{200F}إدخال غير صالح", m.as_str(), true),
    };
    if simple {
        return if color {
            format!("{}: {}", phase.red().bold(), msg.red())
        } else {
            format!("{phase}: {msg}")
        };
    }
    let mut out = String::new();
    if color {
        out.push_str(&format!(
            "{} عند {}:{}: {}\n",
            phase.red().bold(),
            line,
            col,
            msg.bright_red()
        ));
    } else {
        out.push_str(&format!("{phase} عند {line}:{col}: {msg}\n"));
    }
    if line == 0 {
        return out;
    }
    if let Some((ln_text, ln_no)) = get_line(src, line) {
        if color {
            out.push_str(&format!("{:>4} | {}\n", ln_no.blue().bold(), ln_text));
            out.push_str(&format!("     | {}\n", caret_line(col, color)));
        } else {
            out.push_str(&format!("{:>4} | {ln_text}\n", ln_no));
            out.push_str(&format!("     | {}\n", caret_line(col, color)));
        }
    }
    out
}

#[derive(Serialize)]
pub struct JsonError<'a> {
    pub phase: &'a str,
    pub line: usize,
    pub col: usize,
    pub message: &'a str,
}

pub fn error_as_json(_src: &str, err: &IqraError) -> String {
    match err {
        IqraError::Lex { line, col, msg, .. } => {
            serde_json::to_string(&JsonError { phase: "lex", line: *line, col: *col, message: msg })
                .unwrap()
        }
        IqraError::Parse { line, col, msg } => serde_json::to_string(&JsonError {
            phase: "parse",
            line: *line,
            col: *col,
            message: msg,
        })
        .unwrap(),
        IqraError::Runtime { msg } => {
            serde_json::to_string(&JsonError { phase: "runtime", line: 0, col: 0, message: msg })
                .unwrap()
        }
        IqraError::InvalidInput(m) => {
            serde_json::to_string(&JsonError { phase: "input", line: 0, col: 0, message: m })
                .unwrap()
        }
    }
}

fn get_line(src: &str, target: usize) -> Option<(&str, usize)> {
    src.lines().enumerate().find_map(|(i, l)| {
        let n = i + 1;
        if n == target { Some((l, n)) } else { None }
    })
}
fn caret_line(col: usize, color: bool) -> String {
    let mut s = String::new();
    if col == 0 {
        s.push('^');
    } else {
        for _ in 1..col {
            s.push(' ');
        }
        s.push('^');
    }
    if color { s.red().to_string() } else { s }
}

pub fn render_success(msg: &str, color: bool) -> String {
    if color { format!("{}", msg.green().bold()) } else { msg.to_string() }
}

pub fn render_warning(msg: &str, color: bool) -> String {
    if color {
        format!("تحذير: {}", msg.yellow().bold())
    } else {
        format!("تحذير: {msg}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::IqraError;
    #[test]
    fn caret_basic() {
        let src = "abc\nxyz";
        let e = IqraError::Parse { line: 1, col: 2, msg: "oops".into() };
        let r = render_error(src, &e);
        assert!(r.contains("1 | abc"));
        assert!(r.contains("^"));
    }
}
