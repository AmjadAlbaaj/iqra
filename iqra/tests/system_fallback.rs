use iqra::lang::runtime::default_system_executor;

#[test]
fn default_executor_can_run_rustc() {
    // Run `rustc --version` as a smoke test for the default executor. This should
    // be available in the test environment where cargo is running.
    let exe = default_system_executor();
    let res = exe.exec("rustc --version");
    assert!(res.is_ok(), "expected rustc --version to run");
    if let Ok(out) = res {
        assert!(out.to_lowercase().contains("rustc"));
    }
}
