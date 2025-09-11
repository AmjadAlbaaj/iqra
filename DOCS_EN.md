# Iqra — English documentation

Welcome to Iqra, an Arabic-first small scripting language implemented in Rust.

Quick start

Install Rust via rustup. Run the REPL or a script:

```powershell
cargo run -- repl
cargo run -- run examples/hello.iqra
```

Security note: system calls go through a `SystemExecutor` abstraction for testability and safety.
Do not enable shell fallback in untrusted environments. Use the env var `IQRA_ALLOW_SHELL_FALLBACK=1` only when you understand the risks.
