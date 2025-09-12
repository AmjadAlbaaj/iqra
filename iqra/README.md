# iqra

[![CI](https://github.com/AmjadAlbaaj/iqra/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/AmjadAlbaaj/iqra/actions/workflows/ci.yml)
[![Quality & Audit](https://github.com/AmjadAlbaaj/iqra/actions/workflows/quality-and-audit.yml/badge.svg?branch=main)](https://github.com/AmjadAlbaaj/iqra/actions/workflows/quality-and-audit.yml)
[![Coverage (Linux)](https://github.com/AmjadAlbaaj/iqra/actions/workflows/coverage.yml/badge.svg?branch=main)](https://github.com/AmjadAlbaaj/iqra/actions/workflows/coverage.yml)

لغة برمجة عربية بالكامل مع دعم إنجليزي، وتوثيق ثنائي اللغة، وبيئة تفاعلية.

Iqra is an Arabic-first scripting language with full English support, bilingual docs, and a friendly REPL.

---

## المتطلبات | Requirements

- Rust toolchain: <https://rustup.rs>

## البدء السريع | Quick Start

- REPL:

```powershell
cargo run -- repl
```

- Run inline code:

```powershell
cargo run -- run --code "print 1"
```

- Tests:

```powershell
cargo test
```

### لغة الإخراج | Output Language

- الوضع الافتراضي عربي. لتغيير لغة العرض إلى الإنجليزية مؤقتًا:

```powershell
$env:IQRA_OUTPUT_LANG = "en"; cargo run -- repl
```

- لإرجاعها للعربية في ذات الجلسة:

```powershell
Remove-Item Env:IQRA_OUTPUT_LANG
```

راجع `docs/OUTPUT_LANG_AR.md` و`docs/OUTPUT_LANG_EN.md` للتفاصيل.

## مثال | Example (AR)

```iqra
عدد = ١
بينما عدد < ٣ {
  اطبع عدد
  عدد = عدد + ١
}
```

## Example (EN)

```iqra
x = 1
while x < 3 {
  print x
  x = x + 1
}
```

---

## المميزات | Features

- دعم كامل للكلمات المفتاحية العربية والإنجليزية، مع معرّفات ويونيكود وأرقام هندية.
- ترقيم عربي مدعوم: يتعرف على الفاصلة العربية `،` والفاصلة المنقوطة العربية `؛` كفواصل وتعليمات منتهية.
- رسائل أخطاء وتشخيص عربية احترافية، مع إخراج JSON اختياري.
- دوال مدمجة للأعداد، القوائم، القواميس، النصوص، والتواريخ — أسماء عربية وإنجليزية.
- REPL مع إكمال تلقائي للكلمات المفتاحية.
- تنفيذ الأنظمة قابل للحقن للاختبار (SystemExecutor) بدون أي حالة عمومية.

ملاحظة الأنظمة | System Note:

- الإرجاع إلى الصدفة shell معطّل افتراضيًا. لتمكينه: اضبط `IQRA_ALLOW_SHELL_FALLBACK=1`.

---

## التوثيق | Documentation

- اللغة (AR): `docs/LANGUAGE_SPEC_AR.md`
- Language (EN): `docs/LANGUAGE_SPEC_EN.md`
- Built-ins (EN): `docs/BUILTINS_EN.md`
- الدوال المدمجة (AR): `docs/BUILTINS_AR.md`
- أمثلة | Examples: `examples/`
  - Built-ins tour: `examples/builtins.iqra`

---

## بعض الدوال المدمجة | Selected Built-ins

- قوائم | Lists: `list`, `list_len`, `get`, `append`, `remove`, `contains` / `قائمة`, `طول_القائمة`, `عنصر`, `أضف`, `احذف`, `يحتوي`
- قواميس | Maps: `map`, `map_get`, `map_set`, `map_remove` / `قاموس`, `جلب_عنصر`, `تعيين_عنصر`, `حذف_عنصر`
- تحويل/فحص | Convert/Check: `to_number`, `to_string`, `is_number`, `is_string` / `إلى_رقم`, `إلى_نص`, `رقم؟`, `نص؟`
- نوع/طول | Type/Len: `type`, `len` (strings, lists, maps) / `نوع`, `طول` (نصوص، قوائم، قواميس)
- نص/تواريخ | Text/Date: `word_count`, `reverse`, `today` / `عدد_الكلمات`, `عكس`, `تاريخ_اليوم`

اطّلع على `examples/` لمزيد من الأمثلة العملية.

---

## الأمان | Security

- `IQRA_FS_ROOT`: يحدد جذر مساحة العمل المسموح به لعمليات الملفات. أي مسار خارج هذا الجذر سيتم رفضه.

  - مثال (PowerShell):

    ```powershell
    $env:IQRA_FS_ROOT = "C:\\Users\\baaj\\workspace"
    cargo run -- run --code "print read_file('C:/Users/baaj/workspace/file.txt')"
    Remove-Item Env:IQRA_FS_ROOT
    ```

- `IQRA_SYSTEM_TIMEOUT_MS`: يحدد حدًا أقصى بالميلي ثانية لتنفيذ أوامر النظام عبر الدوال المدمجة. إذا تجاوز الأمر المدة سيتم إنهاؤه ويُعاد ناتج فارغ للنص (بدون إلقاء خطأ داخل السكربت) لضمان عدم تعليق التنفيذ.

  - مثال:

    ```powershell
    $env:IQRA_SYSTEM_TIMEOUT_MS = "200"
    cargo run -- run --code "print system('ping 127.0.0.1 -n 5')"
    Remove-Item Env:IQRA_SYSTEM_TIMEOUT_MS
    ```

ملاحظات:

- لا يتم استخدام الصدفة (shell) إلا إذا لم يُعثر على البرنامج وكان `IQRA_ALLOW_SHELL_FALLBACK=1`.
- لا تزال قائمة الأوامر المسموحة والرموز المحظورة مفعلة لتقليل المخاطر. الأوامر المسموحة افتراضيًا: `echo`, `dir`, `type`, `ls`, `cat`, `findstr`, `grep`, `whoami`, `hostname`, `date`, `time`, `ping`, `sleep`.

### دوال الملفات والنظام | File and System Built-ins

- ملفات:
  - `read_file(path)` / `اقرأ_ملف(path)`
  - `write_file(path, content)` / `اكتب_ملف(path, content)`
  - `list_files(dir)` / `قائمة_ملفات(dir)`

  أمثلة:

  ```powershell
  cargo run -- run --code "print read_file('C:/tmp/hello.txt')"
  cargo run -- run --code "print write_file('C:/tmp/hello.txt', 'hi')"
  cargo run -- run --code "print list_files('C:/tmp')"
  ```

- النظام:
  - `system(cmd)` / `نفذ_أمر(cmd)`
  - `system_with_io(cmd, input)` / `نفذ_أمر_بمدخل(cmd, input)`
  - `system_info()` / `معلومات_النظام()`

---

## المساهمة | Contributing

مرحبًا بمساهماتك! راجع `CONTRIBUTING.md` و`CODE_OF_CONDUCT.md`.

### تطوير محلي

```powershell
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

تغطية (Linux فقط على CI): راجع workflow `coverage.yml` (tarpaulin).

---

## الترخيص | License

MIT © المساهمون | the contributors


