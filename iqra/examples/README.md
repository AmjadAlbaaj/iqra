# Examples

This folder contains example IQRA scripts demonstrating language features.

Run an example (PowerShell):

```powershell
# Run basics.iqra (recommended)
cargo run -- run --file .\examples\basics.iqra

# Or run inline source
cargo run -- run --code "print 1 + 2"

# التحقق فقط بدون تنفيذ
cargo run -- check --file .\examples\basics.iqra
```

Notes

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

ملاحظات | Notes

- `system_demo.iqra` يحتوي على أمثلة أوامر نظام (معلّقة للتجربة الآمنة).
- أضف أمثلة جديدة وحدث `docs/LANGUAGE_SPEC_AR.md` و`docs/LANGUAGE_SPEC_EN.md` عند إضافة دوال مدمجة.
- راجع: `examples/builtins.iqra`، ومرجع الدوال الكامل: `docs/BUILTINS_AR.md` / `docs/BUILTINS_EN.md`.

لغة الإخراج | Output Language

```powershell
$env:IQRA_OUTPUT_LANG = "en"; cargo run -- repl
Remove-Item Env:IQRA_OUTPUT_LANG
```

السجلات | Logging

```powershell
# نصي افتراضي (stderr)
cargo run -- --log-level debug run --file .\examples\hello.iqra

# إلى ملف باستخدام العلم --log-file (صيغة JSON أو نصي)
cargo run -- --log-file C:\\temp\\iqra.log --log-level debug run --file .\examples\hello.iqra
cargo run -- --log-file C:\\temp\\iqra.json --log-format json run --file .\examples\hello.iqra

# إلى ملف نصي
$env:IQRA_LOG_FILE = "C:\\temp\\iqra.log"
cargo run -- --log-level debug run --file .\examples\hello.iqra
Remove-Item Env:IQRA_LOG_FILE

# صيغة JSON للسجلات
$env:IQRA_LOG_FILE = "C:\\temp\\iqra.json"
cargo run -- --log-format json run --file .\examples\hello.iqra
Remove-Item Env:IQRA_LOG_FILE
```

ملاحظة: إذا تم تحديد كلٍ من `--log-file` و`IQRA_LOG_FILE` فسيتم استخدام قيمة `--log-file`.

أمان سريع | Security quick examples (PowerShell)

```powershell
# تقييد ملفات لمجلد محدد
$env:IQRA_FS_ROOT = "C:\\temp"
cargo run -- run --code "print read_file('C:/temp/hello.txt')"
Remove-Item Env:IQRA_FS_ROOT

# مهلة لأوامر النظام (ناتج فارغ عند تجاوزها)
$env:IQRA_SYSTEM_TIMEOUT_MS = "200"
cargo run -- run --code "print system('ping 127.0.0.1 -n 5')"
Remove-Item Env:IQRA_SYSTEM_TIMEOUT_MS
```
