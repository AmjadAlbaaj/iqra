# Output Language Selection

Iqra displays values in Arabic by default. You can switch the output language to English using the `IQRA_OUTPUT_LANG` environment variable.

- Default: Arabic
- Accepted values for English: `en`, `english` (case-insensitive)

Examples (Windows PowerShell):

- Temporary for a single command:
  `$env:IQRA_OUTPUT_LANG = 'en'; cargo run -q -- examples/hello.iqra`
- Set for the current session:
  `$env:IQRA_OUTPUT_LANG = 'english'`

What changes:

- Booleans: `صحيح/خطأ` ↔ `true/false`
- Nil: `لاشيء` ↔ `nil`
- Function display: `<دالة>` / `<دالة مدمجة>` ↔ `<function>` / `<native>`

Notes:

- This only affects display (to_string and printed output). Program semantics and accepted source keywords remain unchanged.
- Arabic remains the default when the variable is unset or set to any value other than `en`/`english`.
