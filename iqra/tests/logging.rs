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
        if log_path.exists() {
            if let Ok(contents) = fs::read_to_string(&log_path) {
                if contents.contains("Done") {
                    found = true;
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    if !found {
        let contents = fs::read_to_string(&log_path).unwrap_or_default();
        panic!("log file not populated as expected; contents=\n{}", contents);
    }
}
