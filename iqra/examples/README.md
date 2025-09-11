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
