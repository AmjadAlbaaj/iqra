# توثيق لغة "اقرأ"

مرحبًا بك في توثيق لغة اقرأ — لغة سكربت عربية قصيرة وموجهة للمبرمجين العرب.
هذا الملف يشرح كيفية تثبيت اللغة، كتابة البرامج البسيطة والمتقدمة، أمثلة عملية، وأدوات التطوير.

## نظرة عامة

اقرأ هي لغة مفسّرة مكتوبة بلغة Rust، مصمّمة لتكون بسيطة للتعلّم، وتدعم المفاهيم الشائعة: المتغيرات، الحلقات، الشروط، الدوال، القوائم (lists) والخرائط (maps)، ومجموعة من built-ins مفيدة.

## المتطلبات

- Rust toolchain مثبت عبر `rustup` (انظر: [https://rustup.rs](https://rustup.rs)).

## تثبيت وتشغيل

لا توجد حزمة تثبيت منفصلة — شغّل المشروع مباشرة من الكود المصدري عبر Cargo:

```powershell
# استعمل PowerShell على Windows
Set-Location -Path 'C:\path\to\iqra'
cargo run --release -- repl

# أو في وضع التطوير (debug)
cargo run -- repl
```

الخيار `repl` يفتح موجه تفاعلي (Read-Eval-Print Loop) حيث يمكنك كتابة أوامر اقرأ مباشرة.

## أول برنامج لك

أنشئ ملفًا `hello.iqra` بالمحتوى:

```iqra
اطبع "مرحبا بالعالم"
```

ثم شغّله عبر:

```powershell
# ملاحظة: المشروع قد يوفر لاحقًا أمرًا صريحًا لتشغيل ملف؛ كحل مؤقت يمكنك تشغيل REPL وقراءة الملف أو استخدام سكريبت بسيط.
cargo run -- run hello.iqra
```

## أساسيات اللغة

### المتغيرات

```iqra
س = ١
ن = "نص"
```

### الشروط

```iqra
اذا س > ٠ {
  اطبع "موجب"
} وإلا {
  اطبع "غير موجب"
}
```

### الحلقات

```iqra
بينما س < ٣ {
  اطبع س
  س = س + ١
}
```

### الدوال

```iqra
دالة اجمع(أ,ب) {
  ارجع أ + ب
}
```

### القوائم والخرائط

```iqra
ق = list(1,2,3)
اطبع sum(ق)
م = map('name','أحمد','age',30)
اطبع map_get(m,'name')
```

## built-ins مهمة

- `اطبع` / `print` — طباعة قيمة.
- `list(...)` — إنشاء قائمة.
- `sum(list)` — مجموع القيم العددية.
- `map(...)`, `map_get(map,key)` — خريطة وقارئها.
- `system(cmd)` و`system_with_io(cmd,input)` — تنفيذ أوامر نظامية (الحذر من الإدخال غير الموثوق).

> ملاحظة أمان: الوظائف التي تنفّذ أوامر النظام تم تحسينها لتقليل مخاطر استدعاء الشِل. استخدم واجهة الاختبار `SystemExecutor` لمحاكاة استدعاءات النظام في الاختبارات.

## الاختبار والمحاكاة (Testing / Mocking)

المشروع يوفّر آلية لحقن منفّذ الأوامر `SystemExecutor` داخل `Runtime` لجعل الاختبارات حتمية ولا تعتمد على بيئة النظام.

مثال سريع (اختبار Rust):

```rust
use iqra::lang::runtime::{Runtime, SystemExecutor, Value};

struct MockExec;
impl SystemExecutor for MockExec {
  fn exec(&self, _cmd: &str) -> std::io::Result<String> { Ok("ok".to_string()) }
  fn exec_with_io(&self, _cmd: &str, _input: &str) -> std::io::Result<String> { Ok("out".to_string()) }
}

#[test]
fn system_mock_example() {
  let mut rt = Runtime::new_with_executor(Box::new(MockExec));
  let res = rt.call_builtin("system", &[Value::Str("echo hi".into())]).unwrap();
  assert_eq!(res, Value::Str("ok".into()));
}
```

لتشغيل الاختبارات محليًا:

```powershell
cargo test --all
```

## سياسة الرجوع إلى الشِل (Shell fallback)

بشكل افتراضي، استدعاءات أوامر النظام في اقرأ لا تُرجع إلى مفسّق الشِل إذا لم يُعثر على البرنامج المحدد — هذا لتقليل مخاطر تنفيذ أوامر غير مقصودة أو استغلال سلاسل الإدخال غير الموثوقة.

إذا كنت تحتاج صراحةً لتفعيل تنفيذ التعليمات عبر الشِل (مثلاً للاستفادة من builtins الخاصة بالشِل مثل `dir` على ويندوز)، يمكنك تمكين ذلك مؤقتًا عبر المتغيّر البيئي التالي عند تشغيل البرنامج أو في CI:

```powershell
# Windows / PowerShell
$env:IQRA_ALLOW_SHELL_FALLBACK = "1"
cargo run -- run myscript.iqra

# Unix-like
IQRA_ALLOW_SHELL_FALLBACK=1 cargo run -- run myscript.iqra
```

تنبيه أمني: تفعيل هذا المتغيّر يجعل التطبيق يقوم باستدعاء الشِل عند تعذر إيجاد البرنامج مباشرةً، ما يزيد من خطر استغلال مدخلات غير موثوقة (مثل السلاسل التي تحتوي `;`, `&&`, backticks، الخ). لا تفعّله في بيئات غير موثوقة أو عند معالجة مدخلات من المستخدمين بدون تعقيم مناسب.

للاختبارات الآمنة داخل CI، استعمل `Runtime::new_with_executor(Box::new(MockExec))` أو `default_system_executor()` مع محاكاة معروفة بدلاً من تفعيل fallback.


## أدوات المطورين

في جذر المشروع يوجد `dev.ps1` لتسهيل أوامر شائعة (Windows):

- `./dev.ps1 fmt` — تطبيق التنسيق.
- `./dev.ps1 clippy` — فحص lint.
- `./dev.ps1 test` — تشغيل الاختبارات.

لمزيد من المعلومات راجع `README.md` وملفات الاختبارات داخل مجلد `tests/`.
