# Contributing to اقرأ (Iqra)

شكراً لرغبتك بالمساهمة في مشروع اقرأ! نرحب بكل مساهمة بناءة: تقارير أخطاء، إصلاحات، تحسين التوثيق، أو ميزات جديدة.

## القواعد العامة

- اتبع قواعد الترميز (`rustfmt`) واظهر `clippy` نظيفًا (`-D warnings`) قبل فتح PR.
- اكتب اختبارات وحدية (unit tests) لكل سلوك جديد أو تصحيح مرتبط.
- ضع وصفًا واضحًا في رسالة الالتزام (commit) وشرح التغيير في نصِّ الـ PR.

## بيئة التطوير المحلية

- تثبيت Rust: [rustup.rs](https://rustup.rs)





- لتشغيل كل الأوامر الشائعة على Windows (PowerShell) استخدم `dev.ps1` الموجود في جذر المشروع:


	- `./dev.ps1 fmt` — تطبيق التنسيق.
	- `./dev.ps1 clippy` — تشغيل clippy مع `-D warnings`.
	- `./dev.ps1 test` — تشغيل الاختبارات.

## كتابة اختبارات

- اختبر حالات النجاح والفشل (happy path + 1-2 حدود/حالات خطأ).
- لا تعتمد على بيئة النظام في الاختبارات؛ استعمل `Runtime::new_with_executor(Box::new(MockExec))` أو نماذج محاكاة أخرى لحقن تنفيذ أوامر النظام.

## سياسة Git و PR

1. افحص القاعدة الرئيسية وتأكد أنها محدثة:

	- `git fetch origin` و `git rebase origin/main` (أو `git merge origin/main` إذا رغبت).

2. اعمل فرعًا ذو اسم وصفي: `feat/<short-desc>`, `fix/<short-desc>`, `chore/<short-desc>`.

3. ارفع الفرع إلى remote وافتح PR مع وصف التغيير، خطوات التحقق، وروابط لأي قضايا ذات صلة.

4. سير العمل: المراجعة التلقائية ستشمل تنسيق، clippy، cargo test، و cargo audit.

## التعامل مع مشكلات الأمان

- إذا وجدت ثغرة أمنية، لا تفتح Issue عامًا؛ اتصل بالمحافظين عبر البريد المدرج في الملف `CODEOWNERS` أو عبر قنوات الاتصال المعلنة.

## الترخيص

بالمساهمة أنت توافق ضمنياً على أن تغييراتك ستكون مرخصة تحت نفس ترخيص المشروع (MIT/Apache-2.0 كما هو موضح في الملف LICENSE).

شكراً لمساهمتك!

---

## Quick start (English)

Run examples from PowerShell:

```powershell
cargo run --release -- Run --code "$(Get-Content examples\basics.iqra -Raw)"
```

Start REPL:

```powershell
cargo run --release -- Repl
```
