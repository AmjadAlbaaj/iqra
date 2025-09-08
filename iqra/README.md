# iqra

![CI](https://github.com/USERNAME/iqra/actions/workflows/ci.yml/badge.svg)

مشروع Rust نموذجي بهيكل احترافي مبدئي.

## دليل سريع للمستخدم | Quick Start Guide

## المتطلبات

- تثبيت Rust: https://rustup.rs

## تشغيل اقرأ

```powershell
cargo run -- repl
```

## أوامر مطوّر سريعة

PowerShell (Windows):
# iqra

![CI](https://github.com/USERNAME/iqra/actions/workflows/ci.yml/badge.svg)

Iqra — a small Arabic-first scripting language and interpreter written in Rust.

## Quick start / دليل سريع

Requirements / المتطلبات:

- Rust (via rustup): [https://rustup.rs](https://rustup.rs)

Run the REPL:

```powershell
cargo run -- repl
```

Developer quick commands (Windows PowerShell):

```powershell
.\dev.ps1 fmt      # format
.\dev.ps1 clippy   # lint
.\dev.ps1 test     # run tests
```

Unix / macOS:

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all --verbose
```

## Examples / أمثلة

Arabic example:

```iqra
عدد = ١
بينما عدد < ٣ {
  اطبع عدد
  عدد = عدد + ١
}
```

English example:

```iqra
x = 1
while x < 3 {
  print x
  x = x + 1
}
```

Short list/map examples:

```iqra
l = list(1,2,3)
اطبع sum(l)      # 6
m = map('name','Ahmed','age',30)
اطبع map_get(m,'name')
```

## Contributing / المساهمة

- Run tests and linters before PRs:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all --verbose
```

- CI runs formatting, clippy and tests on push/PR (see `.github/workflows/ci.yml`).

---

For more examples and the full list of built-ins, see the `tests/` directory and source code in `src/lang/`.
    cargo fmt --all

## Testing / Mocking system calls

The runtime supports injecting a `SystemExecutor` so tests can mock system calls and remain deterministic.

Example (Rust test):

```rust
use iqra::lang::runtime::{Runtime, SystemExecutor, Value, Result};

struct MockExec;
impl SystemExecutor for MockExec {
  fn exec(&self, _cmd: &str) -> std::io::Result<String> { Ok("ok".to_string()) }
  fn exec_with_io(&self, _cmd: &str, _input: &str) -> std::io::Result<String> { Ok("out".to_string()) }
}

#[test]
fn system_mock_example() {
  let mut rt = Runtime::new_with_executor(Box::new(MockExec));
  let res = rt.call_builtin("system", &[Value::Str("echo hi".into())]).unwrap();
  assert_eq!(res, Value::Str("ok".into()));
}
```

Note: The older global setter API (`set_global_executor`) has been removed; prefer
`Runtime::new_with_executor(Box::new(...))` to inject test doubles or custom executors per
runtime instance.

### Advanced mocking

You can write integration tests that simulate different command outputs and failures by
implementing `SystemExecutor`. Example tests are included in `tests/mock_exec.rs` and show:

- returning platform-like stdout (e.g. `echo hi` -> "hi\n")
- echoing input back for `system_with_io` cases (simulating `cat`)
- simulating errors (commands containing `fail` return an error)

Run the tests as usual:

```powershell
cargo test --all
```
