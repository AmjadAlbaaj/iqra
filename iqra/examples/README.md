# Examples

This folder contains example IQRA scripts demonstrating language features.

Run an example (PowerShell):

```powershell
# Run basics.iqra (recommended)
cargo run --release -- run examples\basics.iqra

# Or run inline source
cargo run -- run --code "print 1 + 2;"
```

Notes

- `system_demo.iqra` contains system-call examples that are commented out for safety.
- Add new example files and update `docs/LANGUAGE_SPEC_EN.md` if you add new builtins.
- See also: `examples/builtins.iqra` and the full built-ins reference in `docs/BUILTINS_EN.md` / `docs/BUILTINS_AR.md`.

Security quick examples (PowerShell)

```powershell
# Sandbox file access to a directory
$env:IQRA_FS_ROOT = "C:\\temp"
cargo run -- run --code "print read_file('C:/temp/hello.txt')"
Remove-Item Env:IQRA_FS_ROOT

# Enforce command timeout (returns empty output if exceeded)
$env:IQRA_SYSTEM_TIMEOUT_MS = "200"
cargo run -- run --code "print system('ping 127.0.0.1 -n 5')"
Remove-Item Env:IQRA_SYSTEM_TIMEOUT_MS
```
