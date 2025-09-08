use iqra::lang::runtime::{Runtime, SystemExecutor, Value};
use std::io;
use std::sync::{Arc, Mutex};

struct RecorderMock {
    calls: Arc<Mutex<Vec<String>>>,
}

impl RecorderMock {
    fn new() -> Self {
        RecorderMock { calls: Arc::new(Mutex::new(Vec::new())) }
    }

    fn calls(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.calls)
    }
}

impl SystemExecutor for RecorderMock {
    fn exec(&self, cmd: &str) -> io::Result<String> {
        let mut v = self.calls.lock().unwrap();
        v.push(format!("exec:{}", cmd));
        Ok("rec-ok".to_string())
    }

    fn exec_with_io(&self, cmd: &str, input: &str) -> io::Result<String> {
        let mut v = self.calls.lock().unwrap();
        v.push(format!("exec_io:{}|{}", cmd, input));
        Ok(format!("in:{}", input))
    }
}

#[test]
fn runtime_uses_injected_executor_for_system() {
    let mock = RecorderMock::new();
    let calls = mock.calls();
    let mut rt = Runtime::new_with_executor(Box::new(mock));

    let res = rt.call_builtin("system", &[Value::Str("echo hello".into())]).unwrap();
    assert_eq!(res, Value::Str("rec-ok".into()));

    let v = calls.lock().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], "exec:echo hello");
}

#[test]
fn runtime_uses_injected_executor_for_system_with_io() {
    let mock = RecorderMock::new();
    let calls = mock.calls();
    let mut rt = Runtime::new_with_executor(Box::new(mock));

    let res = rt
        .call_builtin(
            "system_with_io",
            &[Value::Str("cat".into()), Value::Str("input-data".into())],
        )
        .unwrap();
    assert_eq!(res, Value::Str("in:input-data".into()));

    let v = calls.lock().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], "exec_io:cat|input-data");
}
