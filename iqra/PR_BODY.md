# chore: remove global executor; DI for SystemExecutor; Arabic UX polish

This PR removes the global system executor and introduces a per-instance `SystemExecutor` injected into `Runtime` via `Runtime::new_with_executor`. It also adds Arabic punctuation support and aligns built-in `len/طول` behavior.

## Summary of changes

- Introduced `SystemExecutor` trait and `default_system_executor()` factory (concrete executor kept private).
- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.
- Shell fallback is opt-in via `IQRA_ALLOW_SHELL_FALLBACK` and covered by tests.
- Arabic punctuation: the lexer recognizes Arabic comma `،` and Arabic semicolon `؛`, and the parser accepts them as statement separators.
- Built-in `len`/`length`/`طول` now supports strings, lists, and maps. Tests added.
- Documentation updated in `README.md`, `DOCS_AR.md`, and `DOCS_EN.md` to reflect punctuation and `len` behavior.
- CI updated: format, clippy (deny warnings), tests, `cargo-audit` on push and weekly; coverage workflow included.

## Local checks performed

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `cargo audit --deny warnings`

## Migration notes for reviewers

Any code that previously used the global executor should now construct the runtime with an executor, for example:

```rust
let mut rt = Runtime::new_with_executor(default_system_executor());
```

Enable shell fallback only if you explicitly set `IQRA_ALLOW_SHELL_FALLBACK=1` in the environment.

---

(See `PR_DESCRIPTION.md` for a longer form description.)
