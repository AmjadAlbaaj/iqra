use iqra::lang::runtime::{Runtime, SystemExecutor};
use iqra::lang::value::Value;
use std::collections::HashMap;

// Mock system executor for testing
struct MockSystemExecutor;

impl SystemExecutor for MockSystemExecutor {
    fn exec(&self, _cmd: &str) -> std::io::Result<String> {
        Ok("mocked output".to_string())
    }

    fn exec_with_io(&self, _cmd: &str, _input: &str) -> std::io::Result<String> {
        Ok("mocked output with input".to_string())
    }

    fn read_file(&self, _path: &str) -> std::io::Result<String> {
        Ok("mocked file content".to_string())
    }

    fn write_file(&self, _path: &str, _content: &str) -> std::io::Result<bool> {
        Ok(true)
    }

    fn list_files(&self, _path: &str) -> std::io::Result<Vec<String>> {
        Ok(vec!["file1.txt".to_string(), "file2.txt".to_string()])
    }

    fn get_env_var(&self, _name: &str) -> Option<String> {
        Some("mocked env value".to_string())
    }

    fn system_info(&self) -> std::io::Result<HashMap<String, String>> {
        let mut info = HashMap::new();
        info.insert("os".to_string(), "Linux".to_string());
        info.insert("arch".to_string(), "x86_64".to_string());
        Ok(info)
    }
}

#[test]
fn test_arabic_arithmetic() {
    let mut runtime = Runtime::new();

    // Test Arabic numbers
    let result = runtime.execute("١ + ٢").unwrap();
    assert_eq!(result, Value::Number(3.0));

    let result = runtime.execute("٥ * ٣").unwrap();
    assert_eq!(result, Value::Number(15.0));

    let result = runtime.execute("١٠ / ٢").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_english_arithmetic() {
    let mut runtime = Runtime::new();

    let result = runtime.execute("1 + 2").unwrap();
    assert_eq!(result, Value::Number(3.0));

    let result = runtime.execute("5 * 3").unwrap();
    assert_eq!(result, Value::Number(15.0));

    let result = runtime.execute("10 / 2").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_variables() {
    let mut runtime = Runtime::new();

    // Arabic variable assignment
    runtime.execute("عدد = ٥").unwrap();
    let result = runtime.execute("عدد").unwrap();
    assert_eq!(result, Value::Number(5.0));

    // English variable assignment
    runtime.execute("number = 10").unwrap();
    let result = runtime.execute("number").unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_arabic_conditionals() {
    let mut runtime = Runtime::new();

    let code = r#"
        عدد = ٥
        اذا عدد > ٣ {
            نتيجة = "كبير"
        } وإلا {
            نتيجة = "صغير"
        }
        نتيجة
    "#;

    let result = runtime.execute(code).unwrap();
    assert_eq!(result, Value::String("كبير".to_string()));
}

#[test]
fn test_english_conditionals() {
    let mut runtime = Runtime::new();

    let code = r#"
        number = 5
        if number > 3 {
            result = "big"
        } else {
            result = "small"
        }
        result
    "#;

    let result = runtime.execute(code).unwrap();
    assert_eq!(result, Value::String("big".to_string()));
}

#[test]
fn test_arabic_while_loop() {
    let mut runtime = Runtime::new();

    let code = r#"
        عدد = ١
        مجموع = ٠
        بينما عدد <= ٣ {
            مجموع = مجموع + عدد
            عدد = عدد + ١
        }
        مجموع
    "#;

    let result = runtime.execute(code).unwrap();
    assert_eq!(result, Value::Number(6.0)); // 1 + 2 + 3 = 6
}

#[test]
fn test_english_while_loop() {
    let mut runtime = Runtime::new();

    let code = r#"
        number = 1
        sum = 0
        while number <= 3 {
            sum = sum + number
            number = number + 1
        }
        sum
    "#;

    let result = runtime.execute(code).unwrap();
    assert_eq!(result, Value::Number(6.0)); // 1 + 2 + 3 = 6
}

#[test]
fn test_arabic_list_functions() {
    let mut runtime = Runtime::new();

    // Create list
    runtime.execute("ق = قائمة(١, ٢, ٣, ٤, ٥)").unwrap();

    // Test length
    let result = runtime.execute("طول_القائمة(ق)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test sum
    let result = runtime.execute("جمع(ق)").unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Test average
    let result = runtime.execute("متوسط(ق)").unwrap();
    assert_eq!(result, Value::Number(3.0));

    // Test max
    let result = runtime.execute("أكبر(ق)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test min
    let result = runtime.execute("أصغر(ق)").unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_english_list_functions() {
    let mut runtime = Runtime::new();

    // Create list
    runtime.execute("l = list(1, 2, 3, 4, 5)").unwrap();

    // Test length
    let result = runtime.execute("list_len(l)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test sum
    let result = runtime.execute("sum(l)").unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Test average
    let result = runtime.execute("average(l)").unwrap();
    assert_eq!(result, Value::Number(3.0));

    // Test max
    let result = runtime.execute("max(l)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test min
    let result = runtime.execute("min(l)").unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_arabic_map_functions() {
    let mut runtime = Runtime::new();

    // Create map
    runtime.execute(r#"ق = قاموس("اسم", "أحمد", "عمر", "٣٠")"#).unwrap();

    // Test get
    let result = runtime.execute(r#"جلب_عنصر(ق, "اسم")"#).unwrap();
    assert_eq!(result, Value::String("أحمد".to_string()));

    // Test set
    runtime.execute(r#"ق = تعيين_عنصر(ق, "مدينة", "الرياض")"#).unwrap();
    let result = runtime.execute(r#"جلب_عنصر(ق, "مدينة")"#).unwrap();
    assert_eq!(result, Value::String("الرياض".to_string()));
}

#[test]
fn test_english_map_functions() {
    let mut runtime = Runtime::new();

    // Create map
    runtime.execute(r#"m = map("name", "Ahmed", "age", "30")"#).unwrap();

    // Test get
    let result = runtime.execute(r#"map_get(m, "name")"#).unwrap();
    assert_eq!(result, Value::String("Ahmed".to_string()));

    // Test set
    runtime.execute(r#"m = map_set(m, "city", "Riyadh")"#).unwrap();
    let result = runtime.execute(r#"map_get(m, "city")"#).unwrap();
    assert_eq!(result, Value::String("Riyadh".to_string()));
}

#[test]
fn test_type_functions() {
    let mut runtime = Runtime::new();

    // Test Arabic type functions
    let result = runtime.execute(r#"نوع(١)"#).unwrap();
    assert_eq!(result, Value::String("number".to_string()));

    let result = runtime.execute(r#"نوع("نص")"#).unwrap();
    assert_eq!(result, Value::String("string".to_string()));

    let result = runtime.execute(r#"رقم؟(١)"#).unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute(r#"نص؟("مرحبا")"#).unwrap();
    assert_eq!(result, Value::Bool(true));

    // Test English type functions
    let result = runtime.execute(r#"type(1)"#).unwrap();
    assert_eq!(result, Value::String("number".to_string()));

    let result = runtime.execute(r#"type("text")"#).unwrap();
    assert_eq!(result, Value::String("string".to_string()));

    let result = runtime.execute(r#"is_number(1)"#).unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute(r#"is_string("hello")"#).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_string_functions() {
    let mut runtime = Runtime::new();

    // Test Arabic string functions
    let result = runtime.execute(r#"عدد_الكلمات("مرحبا بالعالم")"#).unwrap();
    assert_eq!(result, Value::Number(2.0));

    let result = runtime.execute(r#"عكس("مرحبا")"#).unwrap();
    assert_eq!(result, Value::String("ابحرم".to_string()));

    let result = runtime.execute(r#"طول("مرحبا")"#).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test English string functions
    let result = runtime.execute(r#"word_count("hello world")"#).unwrap();
    assert_eq!(result, Value::Number(2.0));

    let result = runtime.execute(r#"reverse("hello")"#).unwrap();
    assert_eq!(result, Value::String("olleh".to_string()));

    let result = runtime.execute(r#"len("hello")"#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_date_functions() {
    let mut runtime = Runtime::new();

    // Test Arabic date function
    let result = runtime.execute("تاريخ_اليوم()").unwrap();
    assert!(matches!(result, Value::String(_)));

    // Test English date function
    let result = runtime.execute("today()").unwrap();
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_system_functions_with_mock() {
    let mut runtime = Runtime::new_with_executor(Box::new(MockSystemExecutor));

    // Test Arabic system functions
    let result = runtime.execute(r#"نفذ_أمر("echo test")"#).unwrap();
    assert_eq!(result, Value::String("mocked output".to_string()));

    let result = runtime.execute(r#"اقرأ_ملف("test.txt")"#).unwrap();
    assert_eq!(result, Value::String("mocked file content".to_string()));

    let result = runtime.execute(r#"اكتب_ملف("test.txt", "content")"#).unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute(r#"قائمة_ملفات(".")"#).unwrap();
    assert!(matches!(result, Value::List(_)));

    let result = runtime.execute(r#"متغير_بيئة("PATH")"#).unwrap();
    assert_eq!(result, Value::String("mocked env value".to_string()));

    // Test English system functions
    let result = runtime.execute(r#"system("echo test")"#).unwrap();
    assert_eq!(result, Value::String("mocked output".to_string()));

    let result = runtime.execute(r#"read_file("test.txt")"#).unwrap();
    assert_eq!(result, Value::String("mocked file content".to_string()));

    let result = runtime.execute(r#"write_file("test.txt", "content")"#).unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute(r#"list_files(".")"#).unwrap();
    assert!(matches!(result, Value::List(_)));

    let result = runtime.execute(r#"env_var("PATH")"#).unwrap();
    assert_eq!(result, Value::String("mocked env value".to_string()));
}

#[test]
fn test_conversion_functions() {
    let mut runtime = Runtime::new();

    // Test Arabic conversion functions
    let result = runtime.execute(r#"إلى_رقم("٣")"#).unwrap();
    assert_eq!(result, Value::Number(3.0));

    let result = runtime.execute(r#"إلى_نص(٣)"#).unwrap();
    assert_eq!(result, Value::String("3".to_string()));

    // Test English conversion functions
    let result = runtime.execute(r#"to_number("3")"#).unwrap();
    assert_eq!(result, Value::Number(3.0));

    let result = runtime.execute(r#"to_string(3)"#).unwrap();
    assert_eq!(result, Value::String("3".to_string()));
}

#[test]
fn test_boolean_operations() {
    let mut runtime = Runtime::new();

    // Test Arabic boolean keywords
    let result = runtime.execute("صحيح").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("خطأ").unwrap();
    assert_eq!(result, Value::Bool(false));

    let result = runtime.execute("صحيح و صحيح").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("صحيح أو خطأ").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("ليس صحيح").unwrap();
    assert_eq!(result, Value::Bool(false));

    // Test English boolean keywords
    let result = runtime.execute("true").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("false").unwrap();
    assert_eq!(result, Value::Bool(false));

    let result = runtime.execute("true and true").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("true or false").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("not true").unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn test_list_manipulation() {
    let mut runtime = Runtime::new();

    // Test Arabic list operations
    runtime.execute("ق = قائمة(١, ٢, ٣)").unwrap();

    let result = runtime.execute("أضف(ق, ٤)").unwrap();
    if let Value::List(list) = result {
        assert_eq!(list.len(), 4);
        assert_eq!(list[3], Value::Number(4.0));
    } else {
        panic!("Expected list");
    }

    let result = runtime.execute("احذف(ق, ٢)").unwrap();
    if let Value::List(list) = result {
        assert_eq!(list.len(), 2);
        assert!(!list.contains(&Value::Number(2.0)));
    } else {
        panic!("Expected list");
    }

    let result = runtime.execute("يحتوي(ق, ١)").unwrap();
    assert_eq!(result, Value::Bool(true));

    let result = runtime.execute("يحتوي(ق, ٥)").unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn test_complex_expressions() {
    let mut runtime = Runtime::new();

    // Test complex Arabic expression
    let result = runtime.execute("(١ + ٢) * ٣ - ٤ / ٢").unwrap();
    assert_eq!(result, Value::Number(7.0)); // (1 + 2) * 3 - 4 / 2 = 9 - 2 = 7

    // Test complex English expression
    let result = runtime.execute("(1 + 2) * 3 - 4 / 2").unwrap();
    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_nested_operations() {
    let mut runtime = Runtime::new();

    let code = r#"
        ق١ = قائمة(١, ٢, ٣)
        ق٢ = قائمة(٤, ٥, ٦)
        مجموع١ = جمع(ق١)
        مجموع٢ = جمع(ق٢)
        مجموع١ + مجموع٢
    "#;

    let result = runtime.execute(code).unwrap();
    assert_eq!(result, Value::Number(21.0)); // (1+2+3) + (4+5+6) = 6 + 15 = 21
}
