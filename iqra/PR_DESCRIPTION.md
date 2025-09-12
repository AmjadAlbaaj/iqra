# chore/remove-global-executor

## Summary

This branch removes the old global system executor API and replaces it with a per-instance `SystemExecutor` injected into `Runtime` via `Runtime::new_with_executor`. The change improves testability and reduces global mutable state.

Security hardening and Arabic-first UX are included: a filesystem sandbox (`IQRA_FS_ROOT`), safer system execution (allow-list, forbidden metacharacters, optional shell fallback), and an execution timeout (`IQRA_SYSTEM_TIMEOUT_MS`). The user-facing experience defaults to Arabic with an `IQRA_OUTPUT_LANG` toggle. Logging UX has been polished with a `--log-file` flag (overrides `IQRA_LOG_FILE`) and examples for text/JSON output. REPL now persists command history across sessions (Windows: `%APPDATA%\\iqra\\history.txt`).

## Key changes

- Introduced `SystemExecutor` trait and `default_system_executor()` factory.
- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.
- Security: Filesystem sandbox via `IQRA_FS_ROOT`; command allow-list and forbidden metacharacters; shell fallback gated by `IQRA_ALLOW_SHELL_FALLBACK`; command timeout via `IQRA_SYSTEM_TIMEOUT_MS` (timed-out commands return empty output).
- Arabic punctuation and UX: lexer recognizes `،` and `؛`; parser accepts them as separators; Arabic is default output; `IQRA_OUTPUT_LANG` toggles AR/EN.
- Built-ins: File/system/env built-ins registered and callable from scripts with Arabic aliases; semantics aligned across platforms.
- CI workflows run fmt, clippy (deny warnings), tests, and cargo-audit; weekly security audit and coverage workflow enabled.
- Docs updated in `README.md`, `docs/BUILTINS_AR.md`, `docs/BUILTINS_EN.md`, and examples. Logging docs now include `--log-file` usage and precedence notes.

## Migration notes for reviewers

Any code that previously relied on a global executor must now construct the runtime with an executor: `let mut rt = Runtime::new_with_executor(default_system_executor());`

To enable shell fallback intentionally, set `IQRA_ALLOW_SHELL_FALLBACK=1`. To constrain file access, set `IQRA_FS_ROOT` to a safe directory. To bound command runtime, set `IQRA_SYSTEM_TIMEOUT_MS` (milliseconds). To write logs to a file, prefer the `--log-file` flag (takes precedence over `IQRA_LOG_FILE`).

## Review checklist

- [ ] Code is formatted (`cargo fmt --all -- --check`).
- [ ] Clippy passes with -D warnings.
- [ ] Tests pass on all platforms (CI will run on ubuntu/windows/macos).
- [ ] Documentation updated (`README.md`, `docs/BUILTINS_AR.md`, `docs/BUILTINS_EN.md`, `CONTRIBUTING.md` where relevant).
- [ ] Logging docs reflect `--log-file` and precedence; REPL history noted.

## How I tested locally

- Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all`.
- Verified security-focused tests (sandbox, timeout) and Windows path handling.

## Notes

- The concrete `DefaultSystemExecutor` type remains private; use the public factory `default_system_executor()` or inject mocks via `Runtime::new_with_executor`.
- Timeouts return an empty output string by design to avoid surprising script failures.
