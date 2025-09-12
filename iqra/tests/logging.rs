use assert_cmd::Command;
use std::{fs, thread, time::Duration};

#[test]
fn file_logging_writes_greet_info() {
    // Create a temporary directory to hold the log file
    let tmp = tempfile::tempdir().expect("tmpdir");
    let log_path = tmp.path().join("iqra_test.log");
    let log_path_str = log_path.to_string_lossy().to_string();

    // Run a simple command that emits an info log ("Done")
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.env("IQRA_LOG_FILE", &log_path_str)
        .env("IQRA_LOG_FORMAT", "text")
        .args(["greet", "--name", "Logger"])
        .assert()
        .success();

    // Non-blocking appender flushes asynchronously; wait briefly for contents
    let mut found = false;
    for _ in 0..30 {
        if log_path.exists()
            && let Ok(contents) = fs::read_to_string(&log_path)
            && contents.contains("Done")
        {
            found = true;
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    if !found {
        let contents = fs::read_to_string(&log_path).unwrap_or_default();
        panic!("log file not populated as expected; contents=\n{}", contents);
    }
}

#[test]
fn file_logging_flag_precedence() {
    let tmp = tempfile::tempdir().expect("tmpdir");
    let env_log = tmp.path().join("env.log");
    let flag_log = tmp.path().join("flag.log");
    let env_log_str = env_log.to_string_lossy().to_string();
    let flag_log_str = flag_log.to_string_lossy().to_string();

    // Provide both env and flag; flag should win
    let mut cmd = Command::cargo_bin("iqra").unwrap();
    cmd.env("IQRA_LOG_FILE", &env_log_str)
        .env("IQRA_LOG_FORMAT", "text")
        .args(["--log-file", &flag_log_str, "greet"]) // default greet
        .assert()
        .success();

    // Wait briefly for appender flush
    for _ in 0..30 {
        if flag_log.exists() { break; }
        thread::sleep(Duration::from_millis(50));
    }

    // Assert flag log exists and env log either empty or missing
    let flag_contents = fs::read_to_string(&flag_log).unwrap_or_default();
    assert!(flag_contents.contains("Done"));
    let env_contents = fs::read_to_string(&env_log).unwrap_or_default();
    assert!(env_contents.is_empty());
}
