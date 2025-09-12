use assert_cmd::Command;

#[test]
fn default_greeting() {
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.assert().success().stdout(predicates::str::contains("مرحباً، world!"));
}

#[test]
fn custom_name() {
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.args(["greet", "--name", "Rustacean"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Rustacean"));
}

#[test]
fn check_ok_inline_code() {
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.args(["check", "--code", "عدد س = 1"]) // simple valid code
        .assert()
        .success()
        .stdout(predicates::str::contains("تم التحقق بنجاح"));
}

#[test]
fn check_parse_error_exit_code() {
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.args(["check", "--code", "عدد س = ("]).assert().failure().code(3);
}

#[test]
fn run_parse_error_exit_code() {
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.args(["run", "--code", "عدد س = ("]).assert().failure().code(3);
}
