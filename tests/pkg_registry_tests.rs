//! Integration tests for iqra-pkg registry operations
//! Uses mockito to simulate registry API responses

use std::process::Command;
use mockito::{mock, Matcher};

#[test]
fn test_registry_search() {
    let _m = mock("GET", "/packages")
        .match_query(Matcher::UrlEncoded("q".into(), "قائمة".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"name":"قائمة","description":"دالة لإنشاء قائمة","author":"Amjad"}]"#)
        .create();

    let output = Command::new("cargo")
        .args(["run", "--bin", "iqra-pkg", "search", "قائمة"])
        .env("MOCKITO_SERVER_URL", &mockito::server_url())
        .output()
        .expect("failed to run iqra-pkg");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("MOCKITO_SERVER_URL: {}", &mockito::server_url());
    println!("CLI stdout: {}", stdout);
    println!("CLI stderr: {}", stderr);
    assert!(stdout.contains("قائمة"), "stdout: {}\nstderr: {}", stdout, stderr);
    assert!(stdout.contains("دالة لإنشاء قائمة"), "stdout: {}\nstderr: {}", stdout, stderr);
}

#[test]
fn test_registry_install() {
    let m1 = mock("GET", "/packages/قائمة/latest/download")
        .with_status(200)
        .with_body("// iqra package: قائمة\n")
        .create();
    let m2 = mock("GET", "/packages/%D9%82%D8%A7%D8%A6%D9%85%D8%A9/latest/download")
        .with_status(200)
        .with_body("// iqra package: قائمة\n")
        .create();

    // Ensure package file is removed before test
    let pkg_path = std::path::Path::new("iqra_packages/قائمة");
    if pkg_path.exists() {
        let _ = std::fs::remove_file(pkg_path);
    }

    let download_url = format!("{}/packages/{}/latest/download", &mockito::server_url(), "قائمة");
    println!("Expected download URL: {}", download_url);
    let output = Command::new("cargo")
        .args(["run", "--bin", "iqra-pkg", "install", "قائمة"])
        .env("MOCKITO_SERVER_URL", &mockito::server_url())
        .output()
        .expect("failed to run iqra-pkg");
    println!("Raw mock matched: {}", m1.matched());
    println!("Encoded mock matched: {}", m2.matched());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("MOCKITO_SERVER_URL: {}", &mockito::server_url());
    println!("CLI stdout: {}", stdout);
    println!("CLI stderr: {}", stderr);
    assert!(stdout.contains("تم التثبيت بنجاح"), "stdout: {}\nstderr: {}", stdout, stderr);
    assert!(stdout.contains("Installed successfully"), "stdout: {}\nstderr: {}", stdout, stderr);
}

// Additional tests for publish, update, remove can be added similarly
