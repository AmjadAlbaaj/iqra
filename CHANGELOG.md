# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Remove global system executor API
  - `set_global_executor` and `global_executor` have been removed.
  - Rationale: per-instance injection (`Runtime::new_with_executor`) is safer and testable.
  - Migration: replace usages of the global setter with constructing the runtime with a test
    double, e.g.: `let mut rt = Runtime::new_with_executor(Box::new(MockExec));`.

## Prior releases

- Initial project scaffolding and built-ins.
