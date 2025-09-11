# chore/remove-global-executor

## Summary

This branch removes the old global system executor API and replaces it with a per-instance `SystemExecutor` injected into `Runtime` via `Runtime::new_with_executor`. The change improves testability and reduces global mutable state.

Additionally, it introduces Arabic punctuation support and aligns built-in `len/طول` behavior across core types.

## Key changes

- Introduced `SystemExecutor` trait and `default_system_executor()` factory.
- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.
- Shell fallback is now opt-in via the `IQRA_ALLOW_SHELL_FALLBACK` environment variable; tests added.
- Arabic punctuation: lexer recognizes `،` and `؛`; parser accepts them as statement separators.
- Built-in `len`/`length`/`طول` now supports strings, lists, and maps; tests added.
- CI workflows run fmt, clippy (deny warnings), tests, and cargo-audit; weekly security audit enabled.
- Docs updated in `README.md`, `DOCS_AR.md`, `DOCS_EN.md`.

## Migration notes for reviewers

Any code that previously relied on a global executor must now construct the runtime with an executor: `let mut rt = Runtime::new_with_executor(default_system_executor());`

To enable shell fallback intentionally, set `IQRA_ALLOW_SHELL_FALLBACK=1`.

## Review checklist

- [ ] Code is formatted (`cargo fmt --all -- --check`).
- [ ] Clippy passes with -D warnings.
- [ ] Tests pass on all platforms (CI will run on ubuntu/windows/macos).
- [ ] Documentation updated (`DOCS_AR.md`, `CONTRIBUTING.md` where relevant).

## How I tested locally

- Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all`.

## Notes

- The concrete `DefaultSystemExecutor` type remains private; use the public factory `default_system_executor()` or inject mocks via `Runtime::new_with_executor`.
