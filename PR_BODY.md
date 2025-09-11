# chore: remove global executor and make SystemExecutor per-instance

This PR removes the global system executor and introduces a per-instance `SystemExecutor` trait that is injected into `Runtime` via `Runtime::new_with_executor`.

## Summary of changes

- Introduced `SystemExecutor` trait and `default_system_executor()` factory (concrete executor kept private).
- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.
- Tests updated to use deterministic mock executors; added tests covering `IQRA_ALLOW_SHELL_FALLBACK`.
- Replaced `atty` usage with `std::io::IsTerminal` and removed the dependency.
- Added Arabic docs (`DOCS_AR.md`), CI updates, and repository templates.

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

Shell fallback is opt-in via the environment variable `IQRA_ALLOW_SHELL_FALLBACK`.

---

(See `PR_DESCRIPTION.md` for a longer form description.)
