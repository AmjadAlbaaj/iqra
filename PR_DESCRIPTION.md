# chore/remove-global-executor

## Summary

This branch removes the old global system executor API and replaces it with a per-instance
`SystemExecutor` injected into `Runtime` via `Runtime::new_with_executor`. The change
improves testability and reduces global mutable state.

## Key changes

- Introduced `SystemExecutor` trait and `default_system_executor()` factory.

- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.

- Added deterministic mock-based tests under `tests/` and internal tests to assert
  the `IQRA_ALLOW_SHELL_FALLBACK` policy.

- Updated `DOCS_AR.md` with instructions and security guidance about `IQRA_ALLOW_SHELL_FALLBACK`.

- CI workflow updated to run `cargo fmt --all -- --check`, `clippy` (all targets/features),
  `cargo audit --deny warnings`, and `cargo test --all`.

## Migration notes for reviewers

Any code that previously relied on a global executor must now construct the runtime
with an executor: `let mut rt = Runtime::new_with_executor(default_system_executor());`

## Review checklist

- [ ] Code is formatted (`cargo fmt --all -- --check`).

- [ ] Clippy passes with -D warnings.

- [ ] Tests pass on all platforms (CI will run on ubuntu/windows/macos).

- [ ] Documentation updated (`DOCS_AR.md`, `CONTRIBUTING.md` where relevant).

## How I tested locally

- Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all` after cleaning leftover artifacts.

## Notes

- I did not expose the concrete `DefaultSystemExecutor` type; tests should use
  the public factory `default_system_executor()` or inject mocks via `Runtime::new_with_executor`.
Branch: chore/remove-global-executor

Summary
-------
This branch removes the old global system executor API and replaces it with a per-instance
`SystemExecutor` injected into `Runtime` via `Runtime::new_with_executor`. The change
improves testability and reduces global mutable state.

Key changes
- Introduced `SystemExecutor` trait and `default_system_executor()` factory.
- `Runtime` now stores a `Box<dyn SystemExecutor>` and exposes `Runtime::new_with_executor`.
- Added deterministic mock-based tests under `tests/` and internal tests to assert
  the `IQRA_ALLOW_SHELL_FALLBACK` policy.
- Updated `DOCS_AR.md` with instructions and security guidance about `IQRA_ALLOW_SHELL_FALLBACK`.
- CI workflow updated to run `cargo fmt --all -- --check`, `clippy` (all targets/features),
  `cargo audit --deny warnings`, and `cargo test --all`.

Migration notes for reviewers
- Any code that previously relied on a global executor must now construct the runtime
  with an executor: `let mut rt = Runtime::new_with_executor(default_system_executor());`

Review checklist
- [ ] Code is formatted (`cargo fmt --all -- --check`).
- [ ] Clippy passes with -D warnings.
- [ ] Tests pass on all platforms (CI will run on ubuntu/windows/macos).
- [ ] Documentation updated (`DOCS_AR.md`, `CONTRIBUTING.md` where relevant).

How I tested locally
- Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all` after cleaning leftover artifacts.

Notes
- I did not expose the concrete `DefaultSystemExecutor` type; tests should use
  the public factory `default_system_executor()` or inject mocks via `Runtime::new_with_executor`.
