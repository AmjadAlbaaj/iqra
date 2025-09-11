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

## Prior releases

- Initial project scaffolding and built-ins.
