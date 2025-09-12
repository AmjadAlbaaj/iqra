pub mod diagnostics;
pub mod error;
pub mod lang;
pub use lang::runtime::Runtime;

/// Core greeting logic (placeholder for real domain logic)
pub fn make_greeting(name: &str) -> String {
    format!("مرحباً، {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_has_name() {
        let g = make_greeting("Iqra");
        assert!(g.contains("Iqra"));
    }

    #[test]
    fn lex_basic() {
        let toks = crate::lang::lex("a=1+2; print a").unwrap();
        assert!(toks.len() > 3);
    }
}
