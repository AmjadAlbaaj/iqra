# مواصفات مدير الحزم العبقري للغة اقرأ | Iqra Package Manager Spec

## الهدف
تصميم مدير حزم احترافي للغة اقرأ يدعم التثبيت، النشر، التحديث، وإدارة المكتبات البرمجية بسهولة وباللغة العربية والإنجليزية.

---
## بنية الحزمة القياسية | Package Structure

```
my-library/
├── iqra.pkg.toml        # ملف تعريف الحزمة (اسم، وصف، مؤلف، إصدار، تبعيات)
├── src/
│   └── main.iqra        # الكود الرئيسي
├── README.md            # توثيق الحزمة
└── tests/
    └── test.iqra        # اختبارات الحزمة
```

---
## ملف تعريف الحزمة | Package Manifest (iqra.pkg.toml)

```toml
[package]
name = "my-library"
description = "مكتبة رياضيات متقدمة للغة اقرأ"
author = "Amjad Albaaj"
version = "0.1.0"

[dependencies]
math = "^1.0.0"
http = "^0.2.0"
```

---
## أوامر مدير الحزم | Package Manager Commands

```bash
# تثبيت مكتبة
iqra install math

# تحديث جميع المكتبات
iqra update

# نشر مكتبة خاصة بك
iqra publish my-library

# إزالة مكتبة
iqra remove math

# البحث عن مكتبة
iqra search web
```

---
## ميزات متقدمة
- دعم التثبيت من مصادر متعددة (Git, URL, local)
- إدارة الإصدارات والتبعيات تلقائياً
- دعم التوثيق العربي والإنجليزي
- تكامل مع منصة نشر مركزية (iqra-pkgs.com)
- دعم اختبارات الحزم

---
## مثال ملف حزمة عملي
```toml
[package]
name = "web-utils"
description = "أدوات ويب متقدمة"
author = "Sara Alhassan"
version = "1.2.0"

[dependencies]
http = "^0.2.0"
json = "^0.1.0"
```

---
## خطوات التنفيذ القادمة
- بناء أداة سطر أوامر iqra-pkg
- دعم قراءة وكتابة iqra.pkg.toml
- تنفيذ أوامر التثبيت والنشر والتحديث
- توثيق جميع الأوامر والأخطاء بشكل ثنائي اللغة
- اختبار التكامل مع مشروع iqra الأساسي
