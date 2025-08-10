# -*- coding: utf-8 -*-
"""تعريف استثناءات لغة "اقرأ" وأدوات إدارة سياق التنفيذ (context).
يوفّر تتبُّعًا مبسّطًا لسياق العقد أثناء التنفيذ لرسائل أخطاء أكثر فائدة.
"""

from dataclasses import dataclass
import threading
import traceback

_local = threading.local()

def _get_ctx_stack():
    """إرجاع الستاك المحلي الخاص بالخيط (thread-local)."""
    if not hasattr(_local, "ctx_stack"):
        _local.ctx_stack = []
    return _local.ctx_stack

def push_context(ctx: str):
    """دفع عنصر سياق جديد إلى الستاك (عادةً اسم العقدة أو وصف مختصر)."""
    _get_ctx_stack().append(ctx)

def pop_context():
    """نزْع أحدث عنصر سياق إذا وُجد."""
    stack = _get_ctx_stack()
    if stack:
        stack.pop()

def current_context():
    """إرجاع نسخة من الستاك الحالي (قائمة)."""
    return list(_get_ctx_stack())

@dataclass
class TraceEntry:
    message: str

class خطأاقرأ(Exception):
    """استثناء عام للغة اقرأ.
    الحقول:
      - رسالة (عربي)
      - سطر، عمود (اختياريان) لتحديد الموقع في مصدر النص إن وُجد
      - _context: لقطة من ستاك السياق أثناء إنشاء الخطأ
      - _py_tb: تتبُّع بايثون الأصلي (نصي) إن طُلب ذلك
    """
    def __init__(self, رسالة, سطر=None, عمود=None, *, cause=None, include_traceback=False):
        self.رسالة = رسالة
        self.سطر = سطر
        self.عمود = عمود
        # snapshot of current context stack
        self._context = current_context()
        self._cause = cause
        self._py_tb = None
        if include_traceback and cause is not None:
            try:
                # format_exc will include the traceback of the latest exception
                self._py_tb = traceback.format_exc()
            except Exception:
                self._py_tb = None

        # construct base Exception message for Python ecosystems
        if سطر is not None and عمود is not None:
            super().__init__(f"{رسالة} (في السطر {سطر}، العمود {عمود})")
        elif سطر is not None:
            super().__init__(f"{رسالة} (في السطر {سطر})")
        else:
            super().__init__(رسالة)

    def with_source(self, مصدر: str):
        """إن وُفّر مصدر النص (string) والإحداثيات موجودة، ترجع رسالة موسعة
        تعرض السطر المعني وسهمًا يشير للعمود، مع سياق التنفيذ.
        مفيد لطباعة الأخطاء للمستخدم.
        """
        if self.سطر is None:
            return str(self)
        lines = مصدر.splitlines()
        if 1 <= self.سطر <= len(lines):
            line_text = lines[self.سطر - 1]
            col = self.عمود or 1
            if col < 1:
                col = 1
            if col > len(line_text) + 1:
                col = len(line_text) + 1
            pointer = ' ' * (col - 1) + '^'
            ctx = self.format_context()
            return f"{self.رسالة} (في السطر {self.سطر}، العمود {col})\n{line_text}\n{pointer}\n{ctx}"
        return str(self)

    def format_context(self):
        """إرجاع نص منسق يبيّن سياق التنفيذ (عناوين العقد/الناتجات) بترتيب
        يعرض الأحدث أولاً. إذا وُجد تتبُّع بايثون الأصلي يُدرج بعد السياق.
        """
        if not self._context:
            return ""
        lines = ["--- سياق تنفيذ (مقلوب، الأحدث أول):"]
        for i, c in enumerate(reversed(self._context), 1):
            lines.append(f"  {i}. {c}")
        if self._py_tb:
            lines.append("--- تتبُّع بايثون الأصلي ---")
            lines.append(self._py_tb)
        return "\n".join(lines)

    def __str__(self):
        base = self.رسالة
        if self.سطر is not None:
            base = f"{base} (سطر {self.سطر})"
        ctx = self.format_context()
        if ctx:
            return f"{base}\n{ctx}"
        return base
