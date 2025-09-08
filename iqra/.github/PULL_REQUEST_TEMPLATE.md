<!-- Title: short summary of your change -->
# chore: remove global executor and make system execution injectable

## Summary
This PR finalizes the refactor to remove the global system executor and make system execution injectable and mockable via the `SystemExecutor` trait. It also contains Unicode-aware lexer/token improvements, Arabic language docs and examples, Criterion benches, and CI that enforces rustfmt, clippy, tests and cargo-audit.

## What I changed

- Replace global executor with per-instance `SystemExecutor` injection (runtime).
- Keep concrete executor private; expose `default_system_executor()` factory.
- Make shell fallback opt-in via `IQRA_ALLOW_SHELL_FALLBACK`.
- Add Unicode-aware lexer/token helpers and Arabic keyword mappings.
- Add docs (`docs/LANGUAGE_SPEC_EN.md`, `docs/LANGUAGE_SPEC_AR.md`) and examples under `examples/`.
- Add Criterion benches under `benches/`.
- Add CI workflow for format/lint/tests/cargo-audit: `.github/workflows/quality-and-audit.yml`.

## Checklist (requirements mapping)

- [ ] Remove global executor / make builtins mockable — Done (see `src/lang/runtime.rs`).
- [ ] Keep concrete executor private & provide `default_system_executor()` — Done.
- [ ] Make shell fallback opt-in via `IQRA_ALLOW_SHELL_FALLBACK` — Done.
- [ ] rustfmt & clippy clean (CI will enforce) — local check passed; CI added.
- [ ] Add cargo-audit to CI — Done (RustSec action).
- [ ] Add Arabic documentation and examples — Done under `docs/` and `examples/`.

## How I tested

- Ran `cargo test --all` locally (all tests pass).
- Ran `cargo fmt` / `cargo clippy` locally during development.
- Ran `cargo audit` locally (no vulnerabilities reported).

## Notes for reviewers / follow-ups

- CI will run on PR; please watch for platform-specific failures (Windows/macOS matrix may uncover issues).
- If you'd like me to open the PR programmatically, provide a GitHub token with repo permissions or allow me to run `gh pr create` from your environment.

---

<!-- You can paste additional testing instructions or a changelog entry here -->


