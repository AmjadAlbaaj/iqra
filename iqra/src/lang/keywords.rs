// قائمة الكلمات والدوال المدعومة للاقتراح التلقائي في REPL
// كما نعرّف هنا ثوابت لبعض المعاملات والـ mapping من الكلمة العربية إلى اسم التوكن
// (تُستخدم عادةً من الـ lexer أو لإظهار اقتراحات ذكية في الـ REPL).
//
// ملاحظة: لا نحاول استخدام هياكل بيانات ثقيلة هنا لتجنب تبعيات إضافية —
// نحتفظ بمصفوفات ثابتة وسلوك بسيط للوصول.
pub const KEYWORDS: &[&str] = &[
    // كلمات التحكم
    "print",
    "اطبع",
    "if",
    "اذا",
    "إذا",
    "else",
    "وإلا",
    "while",
    "بينما",
    "true",
    "صحيح",
    "false",
    "خطأ",
    // دوال القوائم
    "list",
    "قائمة",
    "list_len",
    "طول_القائمة",
    "get",
    "عنصر",
    "append",
    "أضف",
    "remove",
    "احذف",
    "contains",
    "يحتوي",
    "sort",
    "رتب",
    "filter",
    "رشح",
    "map",
    "حول",
    "find",
    "ابحث",
    "forEach",
    "لكل",
    "concat",
    "ادمج",
    "groupBy",
    "صنف",
    // دوال القواميس
    "dict",
    "قاموس",
    "خريطة",
    "map_get",
    "جلب_عنصر",
    "map_set",
    "تعيين_عنصر",
    "map_remove",
    "حذف_عنصر",
    // دوال التحويل والفحص
    "to_number",
    "إلى_رقم",
    "to_string",
    "إلى_نص",
    "is_number",
    "رقم؟",
    "is_string",
    "نص؟",
    // دوال النوع والطول
    "type",
    "نوع",
    "len",
    "length",
    "طول",
    // دوال رياضية ونصية وتاريخية
    "sum",
    "جمع",
    "average",
    "متوسط",
    "max",
    "أكبر",
    "min",
    "أصغر",
    "word_count",
    "عدد_الكلمات",
    "reverse",
    "عكس",
    "today",
    "تاريخ_اليوم",
];

// معاملات ومشغلّات (رموز ثابتة يمكن استخدامها في أماكن مختلفة)
pub const EQ: &str = "==";
pub const NE: &str = "!=";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const LE: &str = "<=";
pub const GE: &str = ">=";

pub const AND: &str = "و";
pub const OR: &str = "أو";
pub const NOT: &str = "ليس";

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const MULT: &str = "*";
pub const DIV: &str = "/";

// خريطة بسيطة من الكلمات العربية إلى أسماء التوكن (تُستخدم من الـ lexer/parser)
pub const KEYWORD_TOKENS: &[(&str, &str)] = &[
    ("بسم", "BASMALA"),
    ("الله", "ALLAH"),
    ("اطبع", "PRINT"),
    ("دالة", "FUNC_DEF"),
    ("انتهى", "END"),
    ("ارجع", "RETURN"),
    ("اذا", "IF"),
    ("والا", "ELSE"),
    ("طالما", "WHILE"),
    ("ادخل", "INPUT"),
    ("عشوائي", "RANDOM"),
];

/// Helper: احصل على اسم التوكن المقابل لكلمة عربية إن وجدت.
pub fn token_for_keyword(word: &str) -> Option<&'static str> {
    match word {
        "بسم" => Some("BASMALA"),
        "الله" => Some("ALLAH"),
        "اطبع" => Some("PRINT"),
        "دالة" => Some("FUNC_DEF"),
        "انتهى" => Some("END"),
        "ارجع" => Some("RETURN"),
        "اذا" => Some("IF"),
        "والا" => Some("ELSE"),
        "طالما" => Some("WHILE"),
        "ادخل" => Some("INPUT"),
        "عشوائي" => Some("RANDOM"),
        _ => None,
    }
}
