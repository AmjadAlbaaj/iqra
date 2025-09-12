# chore: DI SystemExecutor, Arabic-first UX, sandbox + timeout, logging flag, REPL history

This PR removes the global system executor in favor of dependency injection, makes the UX Arabic-first, and hardens system/file operations with a sandbox and timeouts. It also adds a `--log-file` flag (overrides `IQRA_LOG_FILE`) with examples for text/JSON logging, and persists REPL command history across sessions (Windows: `%APPDATA%\\iqra\\history.txt`). It prepares the project for PR review with clean CI and updated documentation.

## Summary of changes

- SystemExecutor DI: Introduced `SystemExecutor` trait and `default_system_executor()` factory; `Runtime::new_with_executor` wires it in.
- Security: Filesystem sandbox via `IQRA_FS_ROOT`; command allow-list with forbidden metacharacters; shell fallback gated by `IQRA_ALLOW_SHELL_FALLBACK`; command timeout via `IQRA_SYSTEM_TIMEOUT_MS` (timed-out commands return empty output).
- Arabic-first UX: Arabic is default output; `IQRA_OUTPUT_LANG` toggles AR/EN; lexer recognizes Arabic punctuation `،`/`؛`; CLI and messages localized.
- Built-ins: File (`read_file`/`write_file`/`list_files`), system (`system`/`system_with_io`/`system_info`), env (`env_var`) registered and callable; Arabic aliases documented.
- Behavior fixes: `len/length/طول` over strings/lists/maps; list utilities semantics aligned; Windows path handling improved in lexer.
- Docs: README Security section; `docs/BUILTINS_AR.md` and `docs/BUILTINS_EN.md` expanded; examples updated for sandbox/timeout.
- Logging UX: `--log-file` flag (precedes `IQRA_LOG_FILE`); docs updated; tests added for precedence and file logging.
- REPL: command history persistence across sessions.
- CI: format, clippy (deny warnings), tests, `cargo-audit`; weekly audit and coverage workflow.

## Local checks performed

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `cargo audit --deny warnings`
- Verified logging docs and examples for `--log-file` precedence and JSON/text modes

## Migration notes for reviewers

- Construct runtimes with an executor: `let mut rt = Runtime::new_with_executor(default_system_executor());`
- To allow shell fallback explicitly: set `IQRA_ALLOW_SHELL_FALLBACK=1`.
- To constrain file access: set `IQRA_FS_ROOT` to a directory root.
- To bound command runtime: set `IQRA_SYSTEM_TIMEOUT_MS`.

---

(See `PR_DESCRIPTION.md` for a longer-form description.)
