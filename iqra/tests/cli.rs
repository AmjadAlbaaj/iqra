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
