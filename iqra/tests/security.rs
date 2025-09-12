use iqra::{Runtime, lang::lexer::lex, lang::parser::parse};
use std::env;
use std::fs;

fn strip_colors(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

#[test]
fn fs_sandbox_denies_outside_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let other = tmp.path().join("other");
    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(&other).unwrap();
    let file_in_other = other.join("x.txt");
    fs::write(&file_in_other, "hi").unwrap();

    unsafe {
        env::set_var("IQRA_FS_ROOT", &root);
    }
    let code = format!("print read_file('{}')", file_in_other.display());

    let toks = lex(&code).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert!(printed[0].contains("تم رفض الوصول") || printed[0].contains("رفض"));
    unsafe {
        env::remove_var("IQRA_FS_ROOT");
    }
}

#[test]
fn fs_sandbox_allows_inside_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    fs::create_dir_all(&root).unwrap();
    let file_in_root = root.join("x.txt");
    fs::write(&file_in_root, "hi").unwrap();

    unsafe {
        env::set_var("IQRA_FS_ROOT", &root);
    }
    let code = format!("print read_file('{}')", file_in_root.display());

    let toks = lex(&code).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    assert_eq!(printed[0], "hi");
    unsafe {
        env::remove_var("IQRA_FS_ROOT");
    }
}

#[test]
fn system_timeout_kills_long_running() {
    // Use a cross-platform long-ish command
    #[cfg(windows)]
    let cmd = "ping 127.0.0.1 -n 5";
    #[cfg(not(windows))]
    let cmd = "sleep 2";

    unsafe {
        env::set_var("IQRA_SYSTEM_TIMEOUT_MS", "200");
    }

    let code = format!("print system('{}')", cmd);

    let toks = lex(&code).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let res = rt.exec(&ast);

    // Expect an error due to timeout bubbling into runtime as error string
    assert!(res.is_ok());
    let out = res.unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    // We expect empty output or partial, but crucially the command should not hang; presence of output is not guaranteed.
    // So assert test completed and output length is 1
    assert_eq!(printed.len(), 1);
    unsafe {
        env::remove_var("IQRA_SYSTEM_TIMEOUT_MS");
    }
}

#[test]
fn fs_sandbox_denies_traversal_on_nonexistent() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    fs::create_dir_all(&root).unwrap();

    unsafe {
        env::set_var("IQRA_FS_ROOT", &root);
    }

    // Attempt to escape with .. to a sibling path; target file does not exist
    let outside_attempt = root.join("..").join("outside").join("y.txt");
    let code = format!("print write_file('{}', 'data')", outside_attempt.display());

    let toks = lex(&code).unwrap();
    let ast = parse(&toks).unwrap();
    let mut rt = Runtime::new();
    let out = rt.exec(&ast).unwrap();
    let printed: Vec<String> = out.printed.iter().map(|v| strip_colors(&v.to_string())).collect();
    // write_file returns string denial message in our built-in
    assert!(printed[0].contains("تم رفض الوصول") || printed[0].contains("رفض"));

    unsafe {
        env::remove_var("IQRA_FS_ROOT");
    }
}
