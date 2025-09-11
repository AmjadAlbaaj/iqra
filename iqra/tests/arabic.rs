use iqra::lang::{Runtime, lex, parse};

#[test]
fn arabic_variable_and_print() {
    let src = "عدد=1; اطبع عدد;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    assert_eq!(out.printed.len(), 1);
    assert_eq!(out.printed[0].to_string(), "1");
}

#[test]
fn arabic_unterminated_string_error() {
    let src = "اطبع 'مرحبا\n"; // newline before closing quote triggers unterminated
    let err = lex(src).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("سلسلة نصية غير مكتملة"));
}

#[test]
fn arabic_digits_number() {
    let src = "عدد=١٢٣; اطبع عدد;"; // Arabic-Indic digits -> 123
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    assert_eq!(out.printed[0].to_string(), "123");
}

#[test]
fn arabic_punctuation_semi_comma() {
    // Use Arabic semicolon (U+061B) and Arabic comma (U+060C)
    let src = "عدد=١، ق=[١،٢]؛ اطبع طول(ق)؛";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    assert_eq!(out.printed[0].to_string(), "2");
}

#[test]
fn arabic_boolean_synonyms() {
    // نعم / لا synonyms for true / false
    let src = "اذا نعم { اطبع 1; } وإلا { اطبع 2; } اذا لا { اطبع 3; } وإلا { اطبع 4; }";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    // First if prints 1, second if condition is false so else branch prints 4
    assert_eq!(out.printed.len(), 2);
    assert_eq!(out.printed[0].to_string(), "1");
    assert_eq!(out.printed[1].to_string(), "4");
}

#[test]
fn builtin_len_arabic() {
    let src = "اطبع طول('مرحبا');"; // 5 letters
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    assert_eq!(out.printed[0].to_string(), "5");
}
