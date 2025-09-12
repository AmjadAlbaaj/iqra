# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Remove global system executor API
  - `set_global_executor` and `global_executor` have been removed.
  - Rationale: per-instance injection (`Runtime::new_with_executor`) is safer and testable.
  - Migration: replace usages of the global setter with constructing the runtime with a test
    double, e.g.: `let mut rt = Runtime::new_with_executor(Box::new(MockExec));`.

- Arabic UX improvements
  - Lexer recognizes Arabic comma `،` and semicolon `؛`.
  - Parser accepts them as item/statement separators/terminators.
  - Built-in `len`/`length`/`طول` now supports strings, lists, and maps.

- Docs & CI
  - Updated README, DOCS_AR, DOCS_EN to reflect punctuation and `len` behavior.
  - Added/confirmed cargo-audit in CI (push + weekly). Coverage workflow included.

- Security & System execution
  - Filesystem sandbox: `IQRA_FS_ROOT` restricts file operations to a root directory.
  - Command timeout: `IQRA_SYSTEM_TIMEOUT_MS` kills long-running commands and returns empty output.
  - Allow-list enforced for `system`/`system_with_io` and forbidden symbols blocked.
  - Optional shell fallback via `IQRA_ALLOW_SHELL_FALLBACK=1`.

- Built-ins exposure
  - Exposed `system`, `system_with_io`, `system_info`, `read_file`, `write_file`, `list_files`, `env_var` (with Arabic aliases) as callable native functions in scripts.

- Windows path handling
  - Lexer preserves backslashes in string literals; improved path prefix checks for sandbox.

- Docs & Examples
  - README: Security section expanded; added file/system examples.
  - Built-ins docs (AR/EN): Added safety/timeout details and allow-list.
  - Examples README: sandbox and timeout quick examples.

## Prior releases

- Initial project scaffolding and built-ins.
