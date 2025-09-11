# IQRA Language Specification

This document describes the core grammar, keywords, data types, and statement structure for IQRA — a reference for users and contributors.

## Overview

IQRA is a small scripting language designed for education and lightweight automation, with first-class support for Arabic and English tokens.

## Grammar

- Statements are typically terminated by `;`.
- Expressions: numbers, strings, booleans (`true`/`false`), and `nil`.
- Arithmetic operators: `+`, `-`, `*`, `/`.
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- Logical operators: `&&`, `||`, `!` and word-forms `and`, `or`, `not` as well as Arabic forms `و`, `أو`, `ليس`.

## Keywords

- Control: `if`, `else`, `while` (Arabic: `اذا`, `وإلا`, `بينما`).
- Declarations/Functions: `let`, `fn`, `return` (Arabic: `دع`, `دالة`, `رجع`).
- Print: `print` (Arabic: `اطبع`).
- Values: `true`/`false` (Arabic: `صحيح`/`خطأ`), `nil` (Arabic: `لاشيء`).

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

- System execution is opt-in (via `IQRA_ALLOW_SHELL_FALLBACK`) and tests should inject a `SystemExecutor` mock.

## Performance

- Consider benching critical paths using `criterion.rs`.

---

This is a concise English spec — request expansions or examples.
