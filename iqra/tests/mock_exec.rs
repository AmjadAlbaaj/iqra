use iqra::lang::runtime::{Runtime, SystemExecutor, Value};
use std::io;

struct MockExec;
impl SystemExecutor for MockExec {
    fn exec(&self, cmd: &str) -> io::Result<String> {
        match cmd {
            "echo hi" => Ok("hi\n".to_string()),
            "whoami" => Ok("tester".to_string()),
            c if c.contains("fail") => Err(io::Error::other("simulated fail")),
            _ => Ok("ok".to_string()),
        }
    }

    fn exec_with_io(&self, cmd: &str, input: &str) -> io::Result<String> {
        if cmd == "cat" {
            Ok(input.to_string())
        } else if cmd.contains("fail") {
            Err(io::Error::other("simulated fail"))
        } else {
            Ok(format!("in:{} out:ok", input))
        }
    }
}

#[test]
fn mock_system_exec_ok() {
    let mut rt = Runtime::new_with_executor(Box::new(MockExec));
    let res = rt.call_builtin("system", &[Value::Str("echo hi".into())]).unwrap();
    assert_eq!(res, Value::Str("hi\n".into()));
}

#[test]
fn mock_system_with_io_ok() {
    let mut rt = Runtime::new_with_executor(Box::new(MockExec));
    let res = rt.call_builtin("system_with_io", &[Value::Str("cat".into()), Value::Str("input".into())]).unwrap();
    assert_eq!(res, Value::Str("input".into()));
}

#[test]
fn mock_system_exec_failure() {
    let mut rt = Runtime::new_with_executor(Box::new(MockExec));
    let err = rt.call_builtin("system", &[Value::Str("will_fail".into())]);
    assert!(err.is_err());
}
