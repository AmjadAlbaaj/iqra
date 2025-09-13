# Copilot Instructions for iqra

## Project Overview
- **iqra** is a bilingual (Arabic/English) programming language implemented in Rust, with a professional, modular architecture.
- The codebase supports writing programs in both Arabic and English, with dual-language documentation and error messages.
- The REPL (Read-Eval-Print Loop) is the main entry point for interactive use.

## فلسفة المشروع
- اقرأ تضع اللغة العربية أولاً في كل شيء: الكلمات المفتاحية، الدوال المدمجة، رسائل الخطأ، والتوثيق.
- يجب أن تكون جميع الميزات متاحة بالاسم العربي والإنجليزي، مع أولوية للوضوح العربي.

## Key Components
- `src/main.rs`: Entry point for CLI and REPL.
- `src/lib.rs`: Core library, exposes main APIs.
- `src/lang/`: Contains the language engine:
  - `lexer.rs`, `parser.rs`, `runtime.rs`, `value.rs`: Tokenization, parsing, execution, and value types.
  - All language features (keywords, built-ins) are available in both Arabic and English.
- `examples/`: Example programs in both languages.
- `tests/integration_tests.rs`: Integration tests for language features.

## Arabic Built-ins & Keywords
- جميع الكلمات المفتاحية والدوال المدمجة لها مقابل عربي (مثال: `بينما`, `اطبع`, `قائمة`, `قاموس`, إلخ).
- راجع قسم "الكلمات المفتاحية العربية" و"الدوال المدمجة" في README.md.
- عند إضافة دالة أو ميزة جديدة، أضف الاسم العربي أولاً، ثم الإنجليزي، وحدث التوثيق والأمثلة لكليهما.

## Developer Workflows
- **Run REPL:**
  - `cargo run -- repl`
- **Run a script:**
  - `cargo run -- path/to/script.iqra`
- **Run tests:**
  - `cargo test`
- **Debug:**
  - Use `RUST_BACKTRACE=1` for detailed error traces.
- **Format code:**
  - `cargo fmt`

## Error Handling & Diagnostics
- رسائل الخطأ تظهر بالعربية بشكل افتراضي، ويجب أن تكون واضحة ومهنية.
- عند إضافة أخطاء جديدة أو تحسين التشخيص، اكتب الرسالة أولاً بالعربية، ثم أضف ترجمة إنجليزية إذا لزم الأمر.

## Conventions & Patterns
- All built-in functions and keywords have both Arabic and English names (see README for mapping).
- Error messages and diagnostics are shown in Arabic by default.
- Example programs and documentation are always provided in both languages.
- Use `src/lang/` for any language engine changes; keep CLI logic in `src/cli/`.
- Follow Rust module conventions; keep logic modular and readable.

## Advanced Features & Extensibility
- دوال النظام (`system`, `نفذ_أمر`, `read_file`, `اكتب_ملف`, إلخ) متاحة بالاسمين وتدعم جميع المنصات.
- عند إضافة دوال متقدمة (مثل التعامل مع الملفات أو النظام)، أضف دعم اللغة العربية بالكامل.
- اختبر الميزات الجديدة باستخدام أمثلة عملية بالعربية والإنجليزية في مجلد `examples/`.

- جميع دوال النظام يجب أن تكون متاحة بالاسم العربي والإنجليزي (مثال: `system` و`نفذ_أمر`).
- عند إضافة أو تعديل دوال النظام، أضف أمثلة عملية في README.md لكلا الاسمين.
- اختبر دوال النظام عبر اختبارات التكامل، وتأكد من ظهور رسائل الخطأ بالعربية أولاً.
- راجع أمثلة الاستخدام العملي في README.md للتأكد من وضوح التوثيق.

## Integration Points
- No external services; all dependencies are Rust crates (see `Cargo.toml`).
- For advanced logging, tracing is planned but not yet implemented.

## Tips for AI Agents
- Always provide examples in both Arabic and English when updating docs or adding features.
- When adding built-ins, update both the Arabic and English keyword lists and document them in the README.
- Keep error messages clear and professional, prioritizing Arabic output.
- Use the REPL for quick feature testing and debugging.

- عند تطوير ميزات جديدة، راجع اختبارات التكامل في `tests/integration_tests.rs` وتأكد من تغطية الحالات العربية.
- حافظ على التوثيق ثنائي اللغة، وحدث README.md عند أي تغيير في الكلمات المفتاحية أو الدوال.

## References
- See `README.md` for full feature list, keyword mapping, and usage examples.
- See `src/lang/` for language engine details.
- See `tests/integration_tests.rs` for test patterns.

- راجع أمثلة الاستخدام العملي في README.md، خاصة الأمثلة العربية، قبل إضافة أو تعديل أي ميزة.
