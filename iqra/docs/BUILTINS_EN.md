# IQRA Built-ins (EN)

This page lists the built-in functions available in IQRA. All names are case-sensitive. Many have Arabic aliases shown in parentheses.

Note: Statement separators accept both `,` and the Arabic comma `،`. Statement terminators accept `;` and the Arabic semicolon `؛`.

## Core

- `print(x, ...)` (Arabic: `اطبع`): Prints one or more values.
- `type(x)`: Returns the value type as a string.
- `len(x)` / `length(x)` / `طول(x)`: Length of strings, lists, or maps.
- `is_number(x)` (Arabic: `رقم؟`): Returns true if `x` is number.
- `is_string(x)` (Arabic: `نص؟`): Returns true if `x` is string.
- `to_number(x)` (Arabic: `إلى_رقم`): Converts string/number-like to number.
- `to_string(x)` (Arabic: `إلى_نص`): Converts value to string.

## Lists

- `list(a, b, ...)` (Arabic: `قائمة`): Constructs a list.
- `list_len(xs)` (Arabic: `طول_القائمة`): Returns list length.
- `get(xs, i)` (Arabic: `عنصر`): Zero-based indexing. Errors on out-of-range.
- `append(xs, v)` (Arabic: `أضف`): Returns a new list with `v` appended.
- `remove(xs, i)` (Arabic: `احذف`): Returns a new list without index `i`.
- `contains(xs, v)` (Arabic: `يحتوي`): True if list contains `v` (by equality).
- `sort(xs)` (Arabic: `رتب`): Returns a sorted copy (numbers/strings).
- `filter(xs, predicate)` (Arabic: `رشح`): Keep items matching predicate. Built-in predicates supported: `is_number`, `is_string`.
- `map(xs, mapper)` (Arabic: `حول`): Transform list using built-in mappers such as `to_number`, `to_string`.
- `find(xs, predicate)` (Arabic: `ابحث`): Returns first matching element or `nil`.
- `forEach(xs, action)` (Arabic: `لكل`): Applies an action (built-in) to each element; returns last result or `nil`.
- `concat(a, b)` (Arabic: `ادمج`): Concatenate two lists.
- `groupBy(xs, keyFn)` (Arabic: `صنف`): Group list into a map keyed by `keyFn(x)`.
- Aggregations: `sum(xs)` (Arabic: `جمع`), `average(xs)` (Arabic: `متوسط`), `max(xs)` (Arabic: `أكبر`), `min(xs)` (Arabic: `أصغر`).

## Maps (Dictionaries)

- `dict(k1, v1, k2, v2, ...)` (Arabic: `قاموس` / `خريطة`): Construct a map from key/value pairs.
- `map_get(m, k)` (Arabic: `جلب_عنصر`): Get value for key `k`. Errors if the key is missing.
- `map_set(m, k, v)` (Arabic: `تعيين_عنصر`): Returns a new map with `k:=v`.
- `map_remove(m, k)` (Arabic: `حذف_عنصر`): Returns a new map without `k`.

## Text and Date

- `word_count(s)` (Arabic: `عدد_الكلمات`): Count words in a string.
- `reverse(s)` (Arabic: `عكس`): Reverse a string.
- `today()` (Arabic: `تاريخ_اليوم`): Returns today’s date as `YYYY-MM-DD`.

## Utilities

- `uuid()` (Arabic: `معرف_فريد`): Random UUID v4 as a string.
- `base64_encode(s)` (Arabic: `تشفير_base64`): Encode string to Base64.
- `base64_decode(s)` (Arabic: `فك_تشفير_base64`): Decode Base64 string.
- `now_ms()` (Arabic: `الوقت_الحالي_ميلي`): Current time in milliseconds.
- File system: `read_file(path)`, `write_file(path, contents)`, `list_files(dir)`.
- Environment: `env_var(name)` (Arabic: `متغير_بيئة`).

## System and Safety

- `system(cmd)` (Arabic: `نفذ_أمر`), `system_with_io(cmd, input)` (Arabic: `نفذ_أمر_بمدخل`), `system_info()` (Arabic: `معلومات_النظام`).
- Shell fallback is disabled by default. To allow shell-only commands (like Windows `dir`), set environment variable `IQRA_ALLOW_SHELL_FALLBACK=1`. Execution still enforces an allow-list of simple commands and rejects dangerous symbols.

Tip: Prefer pure built-ins over system calls in scripts intended to be portable and safe.
