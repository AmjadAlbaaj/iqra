# IQRA Language Specification

This document describes the core grammar, keywords, data types, and statement structure for IQRA — a reference for users and contributors.

## Overview

IQRA is a small scripting language designed for education and lightweight automation, with first-class support for Arabic and English tokens.

Note: Output display language defaults to Arabic. To switch printed output to English, see `docs/OUTPUT_LANG_EN.md`.

## Grammar

- Statements are typically terminated by `;`.
- Expressions: numbers, strings, booleans (`true`/`false`; Arabic display `صحيح`/`خطأ`), and `nil` (Arabic display `لاشيء`).
- Arithmetic operators: `+`, `-`, `*`, `/`.
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- Logical operators: `&&`, `||`, `!` and word-forms `and`, `or`, `not` as well as Arabic forms `و`, `أو`, `ليس`.

## Keywords

- Control: `if`, `else`, `while` (Arabic: `اذا`, `وإلا`, `بينما`).
- Declarations/Functions: `let`, `fn`, `return` (Arabic: `دع`, `دالة`, `رجع`).
- Print: `print` (Arabic: `اطبع`).
- Values: Booleans display in Arabic as `صحيح`/`خطأ`. `nil` displays as `لاشيء`.

## Data Types

- Number: IEEE 754 double (f64)
- String: UTF-8 string
- Bool: boolean
- Nil: none
- List: ordered sequence via `list(...)`
- Map/Dict: key/value store via `dict(...)` (Arabic aliases: `قاموس`, `خريطة`)
- Function: callable value

## Statements

- Expression statement: `expr;`
- Assignment: `x = 1;` or `let x = 1;`
- Blocks: `{ ... }`
- If/Else:

```iqra
if cond { ... } else { ... }
```

- While:

```iqra
while cond { ... }
```

- Function definition:

```iqra
fn name(args) { ... }
```

Arabic function example:

```iqra
دالة جمع(أ,ب) { رجع أ + ب; }
```

## Builtins

- See `docs/BUILTINS_EN.md` for a complete list and details.

## Arabic punctuation

- The lexer recognizes Arabic comma `،` and semicolon `؛`.
- The parser accepts `،` as an item/statement separator and `؛` as a statement terminator.

## Naming rules

- Identifiers start with an alphabetic Unicode character or `_` and may include Unicode digits (including Arabic-Indic), `_`, and `؟`.

## Errors

- Lexer errors return line/col positions. Runtime errors include descriptive messages (Arabic translations present where available).

## Security

- Filesystem sandbox (optional) via `IQRA_FS_ROOT`: file operations (`read_file`/`write_file`/`list_files`) are denied outside the configured root with an "Access denied" message.
- Safer system execution: allow-list of commands and forbidden metacharacters (`&`, `|`, `;`, `>`, `<`); shell fallback is opt-in via `IQRA_ALLOW_SHELL_FALLBACK`.
- Command timeout via `IQRA_SYSTEM_TIMEOUT_MS` (milliseconds). On timeout, the process is terminated and an empty output string is returned (no runtime error).
- For tests, inject a mock `SystemExecutor` via `Runtime::new_with_executor` to avoid touching the real system.

## Performance

- Consider benching critical paths using `criterion.rs`.

---

This is a concise English spec — request expansions or examples.
