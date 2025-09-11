#[test]
fn builtin_type_english() {
    let src = "print type(123); print type(\"abc\"); print type(true); print type(nil);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["number", "string", "bool", "nil"]);
}

#[test]
fn builtin_type_arabic() {
    let src = "اطبع نوع(123); اطبع نوع('abc'); اطبع نوع(صحيح); اطبع نوع(nil);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["number", "string", "bool", "nil"]);
}

#[test]
fn builtin_is_number() {
    let src = "print is_number(123); print is_number(\"abc\"); print is_number(true);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["true", "false", "false"]);
}

#[test]
fn builtin_is_number_arabic() {
    let src = "اطبع رقم؟(123); اطبع رقم؟('abc'); اطبع رقم؟(صحيح);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["true", "false", "false"]);
}

#[test]
fn builtin_is_string() {
    let src = "print is_string(\"abc\"); print is_string(123); print is_string(false);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["true", "false", "false"]);
}

#[test]
fn builtin_is_string_arabic() {
    let src = "اطبع نص؟('abc'); اطبع نص؟(123); اطبع نص؟(صحيح);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["true", "false", "false"]);
}
use iqra::lang::{Expr, Runtime, Stmt, TokenKind, lex, parse};

#[test]
fn lexer_control_and_positions() {
    let src = "if true { print 'ok'; } while false { }";
    let toks = lex(src).expect("lex");
    // Ensure key tokens exist in order: If, Bool(true), LBrace, Print, String("ok"), Semi, RBrace, While, Bool(false), LBrace, RBrace, EOF
    let kinds: Vec<&TokenKind> = toks.iter().map(|t| &t.kind).collect();
    assert!(matches!(kinds[0], TokenKind::If));
    assert!(kinds.iter().any(|k| matches!(k, TokenKind::While)));
    // Check string token present
    assert!(kinds.iter().any(|k| matches!(k, TokenKind::String(s) if s=="ok")));
    // Check positions roughly (first token line/col = 1)
    assert_eq!(toks[0].line, 1);
    assert_eq!(toks[0].col, 1);
}

#[test]
fn parser_if_else_structure() {
    let src = "if 1 { print 'yes'; } else { print 'no'; }";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    assert_eq!(ast.len(), 1);
    match &ast[0] {
        Stmt::If { cond, then_branch, else_branch } => {
            // cond should be Number(1.0)
            assert!(matches!(cond, Expr::Number(n) if (*n - 1.0).abs() < 1e-9));
            // then branch is Block
            assert!(matches!(**then_branch, Stmt::Block(_)));
            assert!(else_branch.is_some());
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn runtime_while_loop_and_assign() {
    let src = "x=0; while x<2 { print x; x = x + 1; }";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["0", "1"]);
}

#[test]
fn runtime_string_concat_and_bool() {
    let src = "a='Hi'; b='Rust'; print a + ' ' + b; print a==b;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "Hi Rust");
    assert_eq!(printed[1], "false");
}

#[test]
fn arabic_keywords_and_runtime() {
    let src = "اذا 1 { اطبع 'مرحبا'; } بينما خطأ { اطبع 'لن يظهر'; }";
    let toks = lex(src).unwrap();
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::If)));
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Print)));
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Bool(false))));
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["مرحبا"]);
}

#[test]
fn function_definition_and_call_english() {
    let src = "fn add(a,b){ return a + b; } print add(2,3);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["5"]);
}

#[test]
fn function_definition_and_call_arabic() {
    let src = "دالة جمع(أ,ب){ رجع أ + ب; } اطبع جمع(4,6);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["10"]);
}

#[test]
fn logical_operators_boolean() {
    let src = "print true && false; print true || false; print !false;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["false", "true", "true"]);
}

#[test]
fn logical_short_circuit_and() {
    let src = "print false && (1/0); print true || (1/0);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["false", "true"]);
}

#[test]
fn logical_arabic_keywords() {
    let src = "اطبع صحيح و خطأ; اطبع ليس خطأ; اطبع صحيح أو خطأ;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["false", "true", "true"]);
}

#[test]
fn builtin_len_english() {
    let src = "print len(\"abcd\");"; // 4
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    assert_eq!(out.printed[0].to_string(), "4");
}

#[test]
fn builtin_list_english() {
    let src = "x = list(); print type(x); print list_len(x); print x;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "list");
    assert_eq!(printed[1], "0");
    assert!(printed[2].contains("["));
    assert!(printed[2].contains("]"));
}

#[test]
fn builtin_list_arabic() {
    let src = "س = قائمة(); اطبع نوع(س); اطبع طول_القائمة(س); اطبع س;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "list");
    assert_eq!(printed[1], "0");
    assert!(printed[2].contains("["));
    assert!(printed[2].contains("]"));
}

#[test]
fn builtin_list_get_and_len() {
    let src = "l = list(1,2,3); print list_len(l); print get(l, 1); print get(l, 0);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["3", "2", "1"]);
}

#[test]
fn builtin_len_on_list_and_map() {
    let src = "print len(list(1,2)); m = dict('a',1,'b',2); print len(m);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["2", "2"]);
}

#[test]
fn builtin_list_get_out_of_bounds() {
    let src = "l = list(1); print get(l, 5);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast);
    assert!(out.is_err());
    let msg = format!("{}", out.unwrap_err());
    assert!(msg.contains("فهرس خارج النطاق"));
}

#[test]
fn builtin_map_english() {
    let src = "m = dict('a', 1, 'b', 2); print type(m); print map_get(m, 'a'); print map_get(m, 'b'); print m;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "map".to_string());
    assert_eq!(printed[1], "1".to_string());
    assert_eq!(printed[2], "2".to_string());
    assert!(printed[3].contains("a") && printed[3].contains("b"));
}

#[test]
fn builtin_map_arabic() {
    let src = "ق = قاموس('س', ١, 'ص', ٢); اطبع نوع(ق); اطبع جلب_عنصر(ق, 'س'); اطبع جلب_عنصر(ق, 'ص'); اطبع ق;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "map");
    assert_eq!(printed[1], "1");
    assert_eq!(printed[2], "2");
    assert!(
        printed[3].contains("س")
            && printed[3].contains("1")
            && printed[3].contains("ص")
            && printed[3].contains("2")
    );
}

#[test]
fn builtin_map_set_and_remove() {
    let src = "m = dict('x', 5); m = map_set(m, 'y', 7); print map_get(m, 'y'); m = map_remove(m, 'x'); print m;";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "7");
    assert!(printed[1].contains("y") && printed[1].contains("7") && !printed[1].contains("x: 5"));
}

#[test]
fn builtin_map_get_missing_key() {
    let src = "m = dict('a', 1); print map_get(m, 'zzz');";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast);
    assert!(out.is_err());
    let msg = format!("{}", out.unwrap_err());
    assert!(msg.contains("مفتاح غير موجود"));
}

#[test]
fn builtin_list_append_remove_contains() {
    let src = "l = list(1,2); l = append(l, 3); print l; l = remove(l, 2); print l; print contains(l, 3); print contains(l, 2);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert!(printed[0].contains("1") && printed[0].contains("2") && printed[0].contains("3"));
    assert!(
        printed[1].contains("1") && printed[1].contains("3") && !printed[1].contains("2, 3, 1")
    );
    assert_eq!(printed[2], "true");
    assert_eq!(printed[3], "false");
}

#[test]
fn builtin_list_append_remove_contains_arabic() {
    let src = "ق = قائمة(١,٢); ق = أضف(ق, ٣); اطبع ق; ق = احذف(ق, ٢); اطبع ق; اطبع يحتوي(ق, ٣); اطبع يحتوي(ق, ٢);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert!(printed[0].contains("1") && printed[0].contains("2") && printed[0].contains("3"));
    assert!(
        printed[1].contains("1") && printed[1].contains("3") && !printed[1].contains("2, 3, 1")
    );
    assert_eq!(printed[2], "true");
    assert_eq!(printed[3], "false");
}

#[test]
fn builtin_list_find_english() {
    let src = "l = list(1, 'a', 2, 'b', 3); print find(l, is_string); print find(l, is_number);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "a".to_string());
    assert_eq!(printed[1], "1".to_string());
}

#[test]
fn builtin_list_find_arabic() {
    let src = "ق = قائمة(1, 'a', 2, 'b', 3); اطبع ابحث(ق, نص؟); اطبع ابحث(ق, رقم؟);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "a".to_string());
    assert_eq!(printed[1], "1".to_string());
}

#[test]
fn builtin_list_concat_english() {
    let src = "l1 = list(1, 2); l2 = list(3, 4); print concat(l1, l2);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3, 4]".to_string());
}

#[test]
fn builtin_list_concat_arabic() {
    let src = "ق1 = قائمة(1, 2); ق2 = قائمة(3, 4); اطبع ادمج(ق1, ق2);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3, 4]".to_string());
}

#[test]
fn builtin_list_foreach_english() {
    let src = "l = list(1, 2, 3); forEach(l, to_string); print 'done';";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "done".to_string());
}

#[test]
fn builtin_list_foreach_arabic() {
    let src = "ق = قائمة(1, 2, 3); لكل(ق, إلى_نص); اطبع 'تم';";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "تم".to_string());
}

use regex::Regex;

fn strip_colors(s: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

#[test]
fn builtin_list_sort_english() {
    let src = "l = list(3,1,2); print sort(l); l2 = list(\"b\",\"a\",\"c\"); print sort(l2);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[a, b, c]".to_string());
}

#[test]
fn builtin_list_sort_arabic() {
    let src = "ق = قائمة(3,1,2); اطبع رتب(ق); ق2 = قائمة(\"b\",\"a\",\"c\"); اطبع رتب(ق2);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[a, b, c]".to_string());
}

#[test]
fn builtin_list_filter_english() {
    let src =
        "l = list(1, 'a', 2, 'b', 3); print filter(l, is_number); print filter(l, is_string);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[a, b]".to_string());
}

#[test]
fn builtin_list_filter_arabic() {
    let src = "ق = قائمة(1, 'a', 2, 'b', 3); اطبع رشح(ق, رقم؟); اطبع رشح(ق, نص؟);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[a, b]".to_string());
}

#[test]
fn builtin_list_map_english() {
    let src = "l = list(1, 2, 3); print map(l, to_string); l2 = list('1', '2', '3'); print map(l2, to_number);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[1, 2, 3]".to_string());
}

#[test]
fn builtin_list_map_arabic() {
    let src = "ق = قائمة(1, 2, 3); اطبع حول(ق, إلى_نص); ق2 = قائمة('1', '2', '3'); اطبع حول(ق2, إلى_رقم);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "[1, 2, 3]".to_string());
    assert_eq!(printed[1], "[1, 2, 3]".to_string());
}

#[test]
fn builtin_list_groupby_english() {
    let src =
        "l = list(1, 'a', 2, 'b', 3, 'c'); print groupBy(l, type); print groupBy(l, to_string);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert!(printed[0].contains("number") && printed[0].contains("string"));
    assert!(printed[1].contains("1") && printed[1].contains("a"));
}

#[test]
fn builtin_list_groupby_arabic() {
    let src = "ق = قائمة(1, 'س', 2, 'ص', 3, 'ع'); اطبع صنف(ق, نوع); اطبع صنف(ق, إلى_نص);";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert!(printed[0].contains("number") && printed[0].contains("string"));
    assert!(printed[1].contains("1") && printed[1].contains("س"));
}

#[test]
fn builtin_list_sum_english() {
    let src = "print sum(list(1,2,3,4)); print sum(list());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["10", "0"]);
}

#[test]
fn builtin_list_sum_arabic() {
    let src = "اطبع جمع(قائمة(1,2,3,4)); اطبع جمع(قائمة());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["10", "0"]);
}

#[test]
fn builtin_list_average_english() {
    let src = "print average(list(2,4,6)); print average(list());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "4");
    assert_eq!(printed[1], "nil");
}

#[test]
fn builtin_list_average_arabic() {
    let src = "اطبع متوسط(قائمة(2,4,6)); اطبع متوسط(قائمة());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "4");
    assert_eq!(printed[1], "nil");
}

#[test]
fn builtin_list_max_min_english() {
    let src =
        "print max(list(1,5,3)); print min(list(1,5,3)); print max(list()); print min(list());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["5", "1", "nil", "nil"]);
}

#[test]
fn builtin_list_max_min_arabic() {
    let src =
        "اطبع أكبر(قائمة(1,5,3)); اطبع أصغر(قائمة(1,5,3)); اطبع أكبر(قائمة()); اطبع أصغر(قائمة());";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["5", "1", "nil", "nil"]);
}

#[test]
fn builtin_word_count_english() {
    let src = "print word_count('hello world'); print word_count('');";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["2", "0"]);
}

#[test]
fn builtin_word_count_arabic() {
    let src = "اطبع عدد_الكلمات('مرحبا بالعالم'); اطبع عدد_الكلمات('');";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["2", "0"]);
}

#[test]
fn builtin_reverse_english() {
    let src = "print reverse('abc'); print reverse(list(1,2,3));";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "cba");
    assert!(printed[1].contains("3") && printed[1].contains("1"));
}

#[test]
fn builtin_reverse_arabic() {
    let src = "اطبع عكس('مرحبا'); اطبع عكس(قائمة(1,2,3));";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "ابحرم");
    assert!(printed[1].contains("3") && printed[1].contains("1"));
}

#[test]
fn builtin_today_english() {
    let src = "print today();";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert!(printed[0].len() == 10 && printed[0].contains("-"));
}

#[test]
fn builtin_today_arabic() {
    let src = "اطبع تاريخ_اليوم();";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert!(printed[0].len() == 10 && printed[0].contains("-"));
}

#[test]
fn test_builtin_list_len() {
    let src = "print list_len([1,2,3,4]); اطبع طول_القائمة([1,2,3])";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed, vec!["4", "3"]);
}

#[test]
fn test_builtin_list_get_append_remove_contains_find_concat() {
    let src = "print get([10,20,30],1); print append([1,2],3); print remove([5,6,7],1); print contains([1,2,3],2); print find([1,2,3],3); print concat([1,2],[3,4])";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "20");
    assert_eq!(printed[1], "[1, 2, 3]");
    assert_eq!(printed[2], "[5, 7]");
    assert_eq!(printed[3], "true");
    assert_eq!(printed[4], "2");
    assert_eq!(printed[5], "[1, 2, 3, 4]");
}

#[test]
fn test_builtin_map_set_remove() {
    let src = "let d = dict(); let d2 = map_set(d,'x',123); print map_get(d2,'x'); let d3 = map_remove(d2,'x'); print map_get(d3,'x');";
    let toks = lex(src).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| v.to_string()).collect();
    assert_eq!(printed[0], "123");
    assert_eq!(printed[1], "nil");
}
