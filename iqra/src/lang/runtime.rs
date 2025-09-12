// دوال القوائم الأساسية
fn builtin_list_len(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => Ok(Value::Number(vs.len() as f64)),
        _ => Err(IqraError::runtime("يجب تمرير قائمة لدالة الطول")),
    }
}

fn builtin_list_get(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة وفهرس"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), Value::Number(idx)) => {
            let i = *idx as usize;
            vs.get(i).cloned().ok_or_else(|| IqraError::runtime("الفهرس خارج النطاق"))
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة جلب عنصر من القائمة")),
    }
}

fn builtin_list_append(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة وقيمة للإضافة"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), v) => {
            let mut new_list = vs.clone();
            new_list.push(v.clone());
            Ok(Value::List(new_list))
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة إضافة عنصر للقائمة")),
    }
}

fn builtin_list_remove(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة وقيمة للحذف"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), Value::Number(n)) => {
            // أولاً: حاول حذف القيمة نفسها إن وُجدت
            if let Some(pos) = vs.iter().position(|x| matches!(x, Value::Number(m) if m == n)) {
                let mut new_list = vs.clone();
                new_list.remove(pos);
                return Ok(Value::List(new_list));
            }
            // وإلا: اعتبر الرقم فهرسًا إذا كان ضمن النطاق
            let i = *n as usize;
            if i < vs.len() {
                let mut new_list = vs.clone();
                new_list.remove(i);
                Ok(Value::List(new_list))
            } else {
                Ok(Value::List(vs.clone()))
            }
        }
        // حذف أول ظهور للقيمة (غير رقمية أيضًا)
        (Value::List(vs), v) => {
            let mut new_list = Vec::with_capacity(vs.len());
            let mut removed = false;
            for item in vs {
                if !removed && item == v {
                    removed = true;
                } else {
                    new_list.push(item.clone());
                }
            }
            Ok(Value::List(new_list))
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة حذف عنصر من القائمة")),
    }
}

fn builtin_list_contains(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة وقيمة للفحص"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), v) => Ok(Value::Bool(vs.contains(v))),
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة يحتوي")),
    }
}

fn builtin_list_find(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة ودالة شرط"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), Value::NativeFunction { name, .. }) => {
            for v in vs {
                let is_match = match name.as_str() {
                    "is_number" | "رقم؟" => matches!(v, Value::Number(_)),
                    "is_string" | "نص؟" => matches!(v, Value::Str(_)),
                    _ => false,
                };
                if is_match {
                    return Ok(v.clone());
                }
            }
            Ok(Value::Nil)
        }
        // البحث عن قيمة وإرجاع الفهرس
        (Value::List(vs), needle) => {
            for (i, v) in vs.iter().enumerate() {
                if v == needle {
                    return Ok(Value::Number(i as f64));
                }
            }
            Ok(Value::Nil)
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة ابحث")),
    }
}

fn builtin_list_foreach(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمة ودالة"));
    }
    match (&args[0], &args[1]) {
        (Value::List(vs), Value::NativeFunction { .. }) => {
            // حالياً: تنفيذ بدون تأثير جانبي. يمكن توسيعه لاحقاً.
            let _ = vs; // منع التحذير
            Ok(Value::Nil)
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة لكل")),
    }
}

fn builtin_list_concat(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قائمتين للدمج"));
    }
    match (&args[0], &args[1]) {
        (Value::List(a), Value::List(b)) => {
            let mut new_list = a.clone();
            new_list.extend(b.iter().cloned());
            Ok(Value::List(new_list))
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة دمج القوائم")),
    }
}

fn builtin_list_sum(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => {
            let sum: f64 = vs
                .iter()
                .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                .sum();
            let normalized = if sum == 0.0 { 0.0 } else { sum };
            Ok(Value::Number(normalized))
        }
        _ => Err(IqraError::runtime("المعاملات غير صحيحة لدالة جمع")),
    }
}

// دوال القواميس الأساسية
fn builtin_map(args: &[Value]) -> Result<Value> {
    let mut map = HashMap::new();
    let mut i = 0;
    while i + 1 < args.len() {
        let key = match &args[i] {
            Value::Str(s) => s.clone(),
            v => v.to_string(),
        };
        let value = args[i + 1].clone();
        map.insert(key, value);
        i += 2;
    }
    Ok(Value::Map(map))
}

fn builtin_map_set(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(IqraError::runtime("يجب تمرير قاموس ومفتاح وقيمة"));
    }
    match &args[0] {
        Value::Map(m) => {
            let mut new_map = m.clone();
            let key = match &args[1] {
                Value::Str(s) => s.clone(),
                _ => args[1].to_string(),
            };
            new_map.insert(key, args[2].clone());
            Ok(Value::Map(new_map))
        }
        _ => Err(IqraError::runtime("المعامل الأول يجب أن يكون قاموس")),
    }
}

fn builtin_map_remove(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("يجب تمرير قاموس ومفتاح للحذف"));
    }
    match &args[0] {
        Value::Map(m) => {
            let mut new_map = m.clone();
            let key = match &args[1] {
                Value::Str(s) => s.clone(),
                _ => args[1].to_string(),
            };
            // mark the key as removed by setting its value to Nil so map_get returns nil
            // for removed keys while missing keys still produce an error
            new_map.insert(key, Value::Nil);
            Ok(Value::Map(new_map))
        }
        _ => Err(IqraError::runtime("المعامل الأول يجب أن يكون قاموس")),
    }
}
use crate::error::{IqraError, Result};
use crate::lang::ast::{Expr, Stmt};
use crate::lang::token::TokenKind;
use base64::{Engine as _, engine::general_purpose};
// global executor removed: per-instance injection via Runtime::new_with_executor is required
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

// Simple command-line splitter that respects double and single quotes and basic escapes.
// Returns a vector of arguments (first element is the program).
fn split_command(cmd: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut chars = cmd.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(&next) = chars.peek() {
                    // Take next char literally
                    cur.push(next);
                    chars.next();
                }
            }
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !cur.is_empty() {
                    args.push(cur);
                    cur = String::new();
                }
            }
            other => cur.push(other),
        }
    }
    if !cur.is_empty() {
        args.push(cur);
    }
    args
}

// Control whether the executor is allowed to fall back to invoking the platform shell
// when a program is not found. Default: disabled. Enable by setting the
// environment variable `IQRA_ALLOW_SHELL_FALLBACK=1` or `true`/`yes`.
fn allow_shell_fallback() -> bool {
    match env::var("IQRA_ALLOW_SHELL_FALLBACK") {
        Ok(v) => {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "yes"
        }
        Err(_) => false,
    }
}

// Optional filesystem sandbox root. When set, file operations are limited to this subtree.
fn fs_root() -> Option<PathBuf> {
    env::var_os("IQRA_FS_ROOT").map(PathBuf::from)
}

fn canonicalize(path: &Path) -> std::io::Result<PathBuf> {
    std::fs::canonicalize(path)
}

#[inline]
fn normalize_path_for_cmp(p: &Path) -> String {
    let s = p.to_string_lossy().to_string();
    #[cfg(windows)]
    {
        let s = if let Some(rest) = s.strip_prefix("\\\\?\\") { rest.to_string() } else { s };
        s.replace('\u{005C}', "/").to_ascii_lowercase()
    }
    #[cfg(not(windows))]
    {
        s
    }
}

// Normalize a path without touching the filesystem: collapse '.' and '..' segments
// and produce an absolute-like string for comparison. When the input is relative,
// it will be resolved against the current working directory best-effort.
fn normalize_abs_or_canon(target: &Path) -> PathBuf {
    if let Ok(can) = canonicalize(target) {
        return can;
    }
    let mut s = if target.is_absolute() {
        normalize_path_for_cmp(target)
    } else {
        let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let joined = base.join(target);
        normalize_path_for_cmp(&joined)
    };
    // Split by '/', collapse '.' and '..'
    let mut parts: Vec<String> = Vec::new();
    let is_abs = s.starts_with('/');
    let mut drive: Option<String> = None;
    if !is_abs {
        // handle Windows drive like 'c:' prefix
        if let Some(pos) = s.find(':') {
            let (d, rest) = s.split_at(pos + 1);
            drive = Some(d.to_string());
            s = rest.to_string();
        }
    } else if let Some(pos) = s.find(':') {
        // Absolute with drive, e.g. 'c:/...'
        let (d, rest) = s.split_at(pos + 1);
        drive = Some(d.to_string());
        s = rest.to_string();
    }
    for comp in s.split('/') {
        if comp.is_empty() || comp == "." {
            continue;
        }
        if comp == ".." {
            if let Some(last) = parts.pop()
                && last.is_empty()
            {
                parts.push(last);
            }
            continue;
        }
        parts.push(comp.to_string());
    }
    let mut out = String::new();
    let has_drive = drive.is_some();
    if let Some(d) = drive {
        out.push_str(&d);
    }
    if is_abs || has_drive {
        out.push('/');
    }
    out.push_str(&parts.join("/"));
    PathBuf::from(out)
}

#[allow(dead_code)]
fn in_sandbox(target: &Path) -> bool {
    match fs_root() {
        None => true,
        Some(root) => {
            let root_c = canonicalize(root.as_path()).unwrap_or(root.clone());
            let tgt_c = normalize_abs_or_canon(target);
            let mut rs = normalize_path_for_cmp(&root_c);
            let ts = normalize_path_for_cmp(&tgt_c);
            if !rs.ends_with('/') {
                rs.push('/');
            }
            ts == rs.trim_end_matches('/') || ts.starts_with(&rs)
        }
    }
}

fn in_sandbox_with_root(root_opt: &Option<PathBuf>, target: &Path) -> bool {
    match root_opt {
        None => true,
        Some(root) => {
            let root_c = canonicalize(root).unwrap_or(root.clone());
            let tgt_c = normalize_abs_or_canon(target);
            let mut rs = normalize_path_for_cmp(&root_c);
            let ts = normalize_path_for_cmp(&tgt_c);
            if !rs.ends_with('/') {
                rs.push('/');
            }
            ts == rs.trim_end_matches('/') || ts.starts_with(&rs)
        }
    }
}

fn system_timeout_ms() -> Option<u128> {
    match env::var("IQRA_SYSTEM_TIMEOUT_MS") {
        Ok(v) => v.parse::<u128>().ok(),
        Err(_) => None,
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;
    use std::env;

    // Helper to temporarily set an env var and restore it after the closure runs.
    fn with_env_var<F: FnOnce() -> R, R>(key: &str, val: &str, f: F) -> R {
        let prev = env::var_os(key);
        unsafe {
            env::set_var(key, val);
        }
        let res = f();
        match prev {
            Some(v) => unsafe {
                env::set_var(key, v.to_string_lossy().as_ref());
            },
            None => unsafe {
                env::remove_var(key);
            },
        }
        res
    }

    #[test]
    fn allow_shell_fallback_default_false() {
        with_env_var("IQRA_ALLOW_SHELL_FALLBACK", "0", || {
            assert!(!allow_shell_fallback());
        });
    }

    #[test]
    fn allow_shell_fallback_true_when_set() {
        with_env_var("IQRA_ALLOW_SHELL_FALLBACK", "true", || {
            assert!(allow_shell_fallback());
        });
    }

    #[test]
    fn exec_nonexistent_without_fallback_returns_err() {
        with_env_var("IQRA_ALLOW_SHELL_FALLBACK", "0", || {
            let exe = DefaultSystemExecutor;
            let res = exe.exec("__iqra_definitely_not_a_command__");
            assert!(res.is_err());
            if let Err(e) = res {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
        });
    }
}

/// Abstraction around executing system commands so we can mock it in tests
pub trait SystemExecutor: Send + Sync + 'static {
    fn exec(&self, cmd: &str) -> std::io::Result<String>;
    fn exec_with_io(&self, cmd: &str, input: &str) -> std::io::Result<String>;
}

#[allow(dead_code)]
struct DefaultSystemExecutor;
impl SystemExecutor for DefaultSystemExecutor {
    fn exec(&self, cmd: &str) -> std::io::Result<String> {
        let parts = split_command(cmd);
        let prog = parts.first().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command")
        })?;
        let args: Vec<&str> = parts.iter().skip(1).map(|s| s.as_str()).collect();

        let spawn_and_wait = |mut cmd: Command| -> std::io::Result<String> {
            let timeout = system_timeout_ms();
            if let Some(ms) = timeout {
                cmd.stdout(std::process::Stdio::piped());
                let mut child = cmd.spawn()?;
                use std::time::{Duration, Instant};
                let start = Instant::now();
                loop {
                    if let Some(_status) = child.try_wait()? {
                        let out = child.wait_with_output()?;
                        break Ok(String::from_utf8_lossy(&out.stdout).to_string());
                    }
                    if start.elapsed() >= Duration::from_millis(ms as u64) {
                        let _ = child.kill();
                        break Err(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "command timed out",
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            } else {
                let out = cmd.output()?;
                Ok(String::from_utf8_lossy(&out.stdout).to_string())
            }
        };

        let mut base = Command::new(prog);
        base.args(&args);
        match spawn_and_wait(base) {
            Ok(s) => Ok(s),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if allow_shell_fallback() {
                    #[cfg(windows)]
                    {
                        let mut c = Command::new("cmd");
                        c.args(["/C", cmd]);
                        spawn_and_wait(c)
                    }
                    #[cfg(not(windows))]
                    {
                        let mut c = Command::new("sh");
                        c.arg("-c").arg(cmd);
                        spawn_and_wait(c)
                    }
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn exec_with_io(&self, cmd: &str, input: &str) -> std::io::Result<String> {
        use std::io::Write;

        let parts = split_command(cmd);
        let prog = parts.first().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command")
        })?;
        let args: Vec<&str> = parts.iter().skip(1).map(|s| s.as_str()).collect();

        let spawn_and_wait = |mut cmd: Command, input: &str| -> std::io::Result<String> {
            cmd.stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped());
            let timeout = system_timeout_ms();
            let mut child = cmd.spawn()?;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(input.as_bytes())?;
            }
            if let Some(ms) = timeout {
                use std::time::{Duration, Instant};
                let start = Instant::now();
                loop {
                    if let Some(_status) = child.try_wait()? {
                        let out = child.wait_with_output()?;
                        break Ok(String::from_utf8_lossy(&out.stdout).to_string());
                    }
                    if start.elapsed() >= Duration::from_millis(ms as u64) {
                        let _ = child.kill();
                        break Err(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "command timed out",
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            } else {
                let out = child.wait_with_output()?;
                Ok(String::from_utf8_lossy(&out.stdout).to_string())
            }
        };

        let mut base = Command::new(prog);
        base.args(&args);
        match spawn_and_wait(base, input) {
            Ok(s) => Ok(s),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if allow_shell_fallback() {
                    #[cfg(windows)]
                    {
                        let mut c = Command::new("cmd");
                        c.args(["/C", cmd]);
                        spawn_and_wait(c, input)
                    }
                    #[cfg(not(windows))]
                    {
                        let mut c = Command::new("sh");
                        c.arg("-c").arg(cmd);
                        spawn_and_wait(c, input)
                    }
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }
}

/// Public factory to obtain the default system executor without exposing the
/// concrete `DefaultSystemExecutor` type. Tests and callers should use this
/// to construct the default executor when they need one.
pub fn default_system_executor() -> Box<dyn SystemExecutor> {
    Box::new(DefaultSystemExecutor)
}

// Note: The project no longer exposes a global SystemExecutor. Tests and callers must
// construct a `Runtime` with an injected executor via `Runtime::new_with_executor`.
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Runtime {
    vars: Vec<HashMap<String, Value>>,
    builtins: HashMap<String, BuiltinFn>,
    sys_exec: Box<dyn SystemExecutor>,
    fs_root: Option<PathBuf>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

type BuiltinFn = Arc<dyn Fn(&mut Runtime, &[Value]) -> Result<Value> + Send + Sync + 'static>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Function { params: Vec<String>, body: Vec<Stmt> },
    NativeFunction { name: String, arity: usize },
}

// تعريف Display خارج الدوال
use std::fmt;
#[derive(Copy, Clone, PartialEq, Eq)]
enum OutputLang {
    Arabic,
    English,
}
fn output_lang() -> OutputLang {
    match std::env::var("IQRA_OUTPUT_LANG") {
        Ok(v) => {
            let v = v.to_ascii_lowercase();
            if v == "en" || v == "english" { OutputLang::English } else { OutputLang::Arabic }
        }
        Err(_) => OutputLang::Arabic,
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => match output_lang() {
                OutputLang::Arabic => {
                    if *b {
                        write!(f, "صحيح")
                    } else {
                        write!(f, "خطأ")
                    }
                }
                OutputLang::English => {
                    if *b {
                        write!(f, "true")
                    } else {
                        write!(f, "false")
                    }
                }
            },
            Value::Nil => match output_lang() {
                OutputLang::Arabic => write!(f, "لاشيء"),
                OutputLang::English => write!(f, "nil"),
            },
            Value::List(vs) => {
                write!(f, "[{}]", vs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
            }
            Value::Map(m) => write!(
                f,
                "{{{}}}",
                m.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", ")
            ),
            Value::Function { .. } => match output_lang() {
                OutputLang::Arabic => write!(f, "<دالة>"),
                OutputLang::English => write!(f, "<function>"),
            },
            Value::NativeFunction { name, .. } => match output_lang() {
                OutputLang::Arabic => write!(f, "<دالة_مدمجة:{}>", name),
                OutputLang::English => write!(f, "<native:{}>", name),
            },
        }
    }
}

// دالة توليد UUID فريد
fn builtin_uuid(_args: &[Value]) -> Result<Value> {
    let id = uuid::Builder::from_random_bytes(rand::random()).into_uuid().to_string();
    Ok(Value::Str(id))
}

// دالة تشفير Base64
fn builtin_base64_encode(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة تشفير Base64"));
    }
    let input = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المدخل نصاً")),
    };
    let encoded = general_purpose::STANDARD.encode(input);
    Ok(Value::Str(encoded))
}

// دالة فك تشفير Base64
fn builtin_base64_decode(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة فك تشفير Base64"));
    }
    let input = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المدخل نصاً")),
    };
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Ok(Value::Str(s)),
            Err(_) => Err(IqraError::runtime("\u{200F}الناتج ليس نصاً صالحاً UTF-8")),
        },
        Err(e) => Err(IqraError::runtime(format!("\u{200F}فشل فك التشفير: {}", e))),
    }
}

// دالة الوقت الحالي بالميلي ثانية
fn builtin_now_ms(_args: &[Value]) -> Result<Value> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH);
    match now {
        Ok(dur) => Ok(Value::Number(dur.as_millis() as f64)),
        Err(e) => Err(IqraError::runtime(format!("\u{200F}فشل جلب الوقت: {}", e))),
    }
}

// دالة معلومات النظام
fn builtin_system_info(_args: &[Value]) -> Result<Value> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let cpu = match sys_info::cpu_num() {
        Ok(n) => n.to_string(),
        Err(_) => "?".to_string(),
    };
    let mem = match sys_info::mem_info() {
        Ok(m) => format!("{} MB", m.total / 1024),
        Err(_) => "?".to_string(),
    };
    let info = format!("OS: {}\nArch: {}\nCPUs: {}\nMemory: {}", os, arch, cpu, mem);
    Ok(Value::Str(info))
}

// دالة قراءة ملف
#[allow(dead_code)]
fn builtin_read_file(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة قراءة ملف"));
    }
    let path = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
    };
    let p = Path::new(path);
    // لسيناريوهات الطباعة داخل السكريبت، نعيد رسالة مفهومة بدلاً من خطأ عند خرق الصندوق الرملي
    if !in_sandbox(p) {
        return Ok(Value::Str("\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)".to_string()));
    }
    match fs::read_to_string(p) {
        Ok(content) => Ok(Value::Str(content)),
        Err(e) => Err(IqraError::runtime(format!("\u{200F}تعذر قراءة الملف '{}': {}", path, e))),
    }
}

// دالة كتابة ملف
#[allow(dead_code)]
fn builtin_write_file(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة كتابة ملف"));
    }
    let path = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
    };
    let content = match &args[1] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المحتوى نصاً")),
    };
    let p = Path::new(path);
    if !in_sandbox(p) {
        return Err(IqraError::runtime("\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)"));
    }
    match fs::write(p, content) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => {
            Err(IqraError::runtime(format!("\u{200F}تعذر الكتابة إلى الملف '{}': {}", path, e)))
        }
    }
}

// دالة قائمة الملفات في مجلد
#[allow(dead_code)]
fn builtin_list_files(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة قائمة الملفات"));
    }
    let path = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
    };
    let p = Path::new(path);
    if !in_sandbox(p) {
        return Err(IqraError::runtime("\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)"));
    }
    match fs::read_dir(p) {
        Ok(entries) => {
            let files: Vec<Value> = entries
                .filter_map(|e| e.ok())
                .map(|e| Value::Str(e.path().display().to_string()))
                .collect();
            Ok(Value::List(files))
        }
        Err(e) => Err(IqraError::runtime(format!("\u{200F}فشل قراءة المجلد: {}", e))),
    }
}

// دالة قراءة متغير بيئة
fn builtin_env_var(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة متغير البيئة"));
    }
    let name = match &args[0] {
        Value::Str(s) => s,
        _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون اسم المتغير نصاً")),
    };
    match env::var(name) {
        Ok(val) => Ok(Value::Str(val)),
        Err(_) => Ok(Value::Nil),
    }
}
fn builtin_map_get(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::runtime("جلب عنصر من القاموس يتطلب معاملين: القاموس والمفتاح"));
    }
    match (&args[0], &args[1]) {
        (Value::Map(m), Value::Str(k)) => {
            m.get(k).cloned().ok_or_else(|| IqraError::runtime("المفتاح غير موجود في القاموس"))
        }
        (Value::Map(m), k) => m
            .get(&k.to_string())
            .cloned()
            .ok_or_else(|| IqraError::runtime("المفتاح غير موجود في القاموس")),
        _ => Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة جلب عنصر من القاموس")),
    }
}

// متوسط عناصر قائمة رقمية
fn builtin_list_average(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => {
            let nums: Vec<f64> = vs
                .iter()
                .filter_map(|x| if let Value::Number(n) = x { Some(*n) } else { None })
                .collect();
            if nums.is_empty() {
                return Ok(Value::Nil);
            }
            let avg = nums.iter().sum::<f64>() / nums.len() as f64;
            Ok(Value::Number(avg))
        }
        _ => Err(IqraError::new_localized(
            "average expects a list of numbers",
            "متوسط يتطلب قائمة أرقام",
        )),
    }
}

// أكبر عنصر في قائمة رقمية
fn builtin_list_max(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => {
            let max = vs
                .iter()
                .filter_map(|x| if let Value::Number(n) = x { Some(*n) } else { None })
                .max_by(|a, b| a.partial_cmp(b).unwrap());
            match max {
                Some(m) => Ok(Value::Number(m)),
                None => Ok(Value::Nil),
            }
        }
        _ => {
            Err(IqraError::new_localized("max expects a list of numbers", "أكبر يتطلب قائمة أرقام"))
        }
    }
}

// أصغر عنصر في قائمة رقمية
fn builtin_list_min(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => {
            let min = vs
                .iter()
                .filter_map(|x| if let Value::Number(n) = x { Some(*n) } else { None })
                .min_by(|a, b| a.partial_cmp(b).unwrap());
            match min {
                Some(m) => Ok(Value::Number(m)),
                None => Ok(Value::Nil),
            }
        }
        _ => {
            Err(IqraError::new_localized("min expects a list of numbers", "أصغر يتطلب قائمة أرقام"))
        }
    }
}

// عدد الكلمات في نص
fn builtin_word_count(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::Str(s)) => Ok(Value::Number(s.split_whitespace().count() as f64)),
        _ => Err(IqraError::new_localized("word_count expects a string", "عدد_الكلمات يتطلب نصاً")),
    }
}

// عكس نص أو قائمة
fn builtin_reverse(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::Str(s)) => Ok(Value::Str(s.chars().rev().collect())),
        Some(Value::List(vs)) => Ok(Value::List(vs.iter().cloned().rev().collect())),
        _ => Err(IqraError::new_localized(
            "reverse expects a string or list",
            "عكس يتطلب نصاً أو قائمة",
        )),
    }
}

// تاريخ اليوم
fn builtin_today(_args: &[Value]) -> Result<Value> {
    use chrono::Local;
    Ok(Value::Str(Local::now().format("%Y-%m-%d").to_string()))
}
fn builtin_list_filter(args: &[Value]) -> Result<Value> {
    match (args.first(), args.get(1)) {
        (Some(Value::List(vs)), Some(Value::NativeFunction { name, .. })) => {
            let mut filtered = Vec::new();
            for v in vs {
                let res = match name.as_str() {
                    "is_number" | "رقم؟" => matches!(v, Value::Number(_)),
                    "is_string" | "نص؟" => matches!(v, Value::Str(_)),
                    _ => false,
                };
                if res {
                    filtered.push(v.clone());
                }
            }
            Ok(Value::List(filtered))
        }
        _ => Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة الترشيح للقائمة")),
    }
}

fn builtin_list_map(args: &[Value]) -> Result<Value> {
    match (args.first(), args.get(1)) {
        (Some(Value::List(vs)), Some(Value::NativeFunction { name, .. })) => {
            let mapped: Vec<Value> = vs
                .iter()
                .map(|v| match name.as_str() {
                    "to_string" | "إلى_نص" => match v {
                        Value::Number(n) => Value::Str(n.to_string()),
                        Value::Bool(b) => Value::Str(b.to_string()),
                        Value::Str(s) => Value::Str(s.clone()),
                        _ => Value::Str("".to_string()),
                    },
                    "to_number" | "إلى_رقم" => match v {
                        Value::Str(s) => s.parse::<f64>().map(Value::Number).unwrap_or(Value::Nil),
                        Value::Number(n) => Value::Number(*n),
                        _ => Value::Nil,
                    },
                    _ => v.clone(),
                })
                .collect();
            Ok(Value::List(mapped))
        }
        _ => Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة التحويل للقائمة")),
    }
}
fn builtin_list_sort(args: &[Value]) -> Result<Value> {
    match args.first() {
        Some(Value::List(vs)) => {
            if vs.iter().all(|x| matches!(x, Value::Number(_))) {
                let mut nums: Vec<_> =
                    vs.iter().map(|x| if let Value::Number(n) = x { *n } else { 0.0 }).collect();
                nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Ok(Value::List(nums.into_iter().map(Value::Number).collect()))
            } else if vs.iter().all(|x| matches!(x, Value::Str(_))) {
                let mut strs: Vec<_> = vs
                    .iter()
                    .map(|x| if let Value::Str(s) = x { s.clone() } else { "".to_string() })
                    .collect();
                strs.sort();
                Ok(Value::List(strs.into_iter().map(Value::Str).collect()))
            } else {
                Err(IqraError::runtime("يمكن ترتيب القوائم الرقمية أو النصية فقط"))
            }
        }
        Some(_) => Err(IqraError::runtime("الوسيط يجب أن يكون قائمة")),
        None => Err(IqraError::runtime("يجب تمرير قائمة واحدة على الأقل")),
    }
}
// دالة صنف/GroupBy الاحترافية للقوائم
fn builtin_list_group_by(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(IqraError::new_localized(
            "groupBy expects a list and a key function",
            "صنف يتطلب قائمة ودالة مفتاح",
        ));
    }
    let list = match &args[0] {
        Value::List(l) => l,
        _ => {
            return Err(IqraError::new_localized(
                "First argument must be a list",
                "المعامل الأول يجب أن يكون قائمة",
            ));
        }
    };
    let key_fn = match &args[1] {
        Value::NativeFunction { name, .. } => name,
        _ => {
            return Err(IqraError::new_localized(
                "Second argument must be a function",
                "المعامل الثاني يجب أن يكون دالة",
            ));
        }
    };
    let mut groups: std::collections::HashMap<String, Vec<Value>> =
        std::collections::HashMap::new();
    for v in list {
        let key = match key_fn.as_str() {
            "type" | "نوع" => match v {
                Value::Number(_) => "number".to_string(),
                Value::Str(_) => "string".to_string(),
                Value::Bool(_) => "bool".to_string(),
                Value::Nil => "nil".to_string(),
                Value::List(_) => "list".to_string(),
                Value::Map(_) => "map".to_string(),
                Value::Function { .. } => "function".to_string(),
                Value::NativeFunction { .. } => "native".to_string(),
            },
            "to_string" | "إلى_نص" => v.to_string(),
            _ => v.to_string(),
        };
        groups.entry(key).or_default().push(v.clone());
    }
    let mut result = std::collections::HashMap::new();
    for (k, vs) in groups {
        result.insert(k, Value::List(vs));
    }
    Ok(Value::Map(result))
}
// -------------------------------------------------------------
// دوال اقرأ المدمجة (Iqra Built-in Functions)
//
// القوائم:
//   list(...), قائمة(...): إنشاء قائمة من عناصر
//   list_len(l), طول_القائمة(ق): عدد عناصر القائمة
//   get(l, i), عنصر(ق، ف): جلب عنصر من القائمة بفهرس
// التحويل:
//   to_number(x), إلى_رقم(س): تحويل إلى رقم
//   to_string(x), إلى_نص(س): تحويل إلى نص
// الفحص:
//   is_number(x), رقم؟(س): هل رقم؟
//   is_string(x), نص؟(س): هل نص؟
// النوع:
//   type(x), نوع(س): نوع القيمة
// الطول:
//   len(x), طول(س): طول نص أو قائمة
// -------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecOutput {
    pub printed: Vec<Value>,
}

impl Runtime {
    /// استدعاء دالة مدمجة باسمها
    pub fn call_builtin(&mut self, name: &str, args: &[Value]) -> Result<Value> {
        if let Some(f) = self.builtins.get(name).cloned() {
            (f)(self, args)
        } else {
            Err(IqraError::runtime("اسم الدالة المدمجة غير موجود"))
        }
    }
    pub fn new() -> Self {
        let mut r = Self {
            vars: vec![HashMap::new()],
            builtins: HashMap::new(),
            sys_exec: Box::new(DefaultSystemExecutor),
            fs_root: fs_root(),
        };
        r.install_builtins();
        r
    }
    pub fn new_with_executor(exec: Box<dyn SystemExecutor>) -> Self {
        let mut r = Self {
            vars: vec![HashMap::new()],
            builtins: HashMap::new(),
            sys_exec: exec,
            fs_root: fs_root(),
        };
        r.install_builtins();
        r
    }
    fn install_builtins(&mut self) {
        // shims: adapt old fn(&[Value]) builtins to the new boxed BuiltinFn type
        let wrap = |f: fn(&[Value]) -> Result<Value>| -> BuiltinFn {
            Arc::new(move |_: &mut Runtime, args: &[Value]| f(args))
        };
        self.builtins.insert("uuid".to_string(), wrap(builtin_uuid));
        self.builtins.insert("معرف_فريد".to_string(), wrap(builtin_uuid));
        self.builtins.insert("base64_encode".to_string(), wrap(builtin_base64_encode));
        self.builtins.insert("تشفير_base64".to_string(), wrap(builtin_base64_encode));
        self.builtins.insert("base64_decode".to_string(), wrap(builtin_base64_decode));
        self.builtins.insert("فك_تشفير_base64".to_string(), wrap(builtin_base64_decode));
        self.builtins.insert("now_ms".to_string(), wrap(builtin_now_ms));
        self.builtins.insert("الوقت_الحالي_ميلي".to_string(), wrap(builtin_now_ms));
        self.builtins.insert(
            "system_with_io".to_string(),
            Arc::new(|rt: &mut Runtime, args: &[Value]| -> Result<Value> {
                if args.len() != 2 {
                    return Err(IqraError::runtime(
                        "\u{200F}المعاملات غير صحيحة لدالة تنفيذ أمر النظام مع مدخل",
                    ));
                }
                let cmd = match &args[0] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون الأمر نصاً")),
                };
                let input = match &args[1] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المدخل نصاً")),
                };
                let allowed_cmds = [
                    "echo", "dir", "type", "ls", "cat", "findstr", "grep", "whoami", "hostname",
                    "date", "time", "ping", "sleep",
                ];
                let first = cmd.split_whitespace().next().unwrap_or("").trim().to_lowercase();
                if !allowed_cmds.iter().any(|a| a == &first) {
                    return Err(IqraError::runtime(format!(
                        "\u{200F}هذا الأمر ('{}') غير مسموح — مسموح فقط: {}",
                        first,
                        allowed_cmds.join(", "),
                    )));
                }
                let forbidden_symbols = ["&", "|", ";", ">", "<"];
                for sym in forbidden_symbols.iter() {
                    if cmd.contains(sym) {
                        return Err(IqraError::runtime(
                            "\u{200F}تنفيذ هذا الأمر غير مسموح لأسباب أمنية (رمز محظور)",
                        ));
                    }
                }
                match rt.sys_exec.exec_with_io(cmd, input) {
                    Ok(s) => Ok(Value::Str(s)),
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut {
                            // في حال انتهاء المهلة نعيد ناتجاً فارغاً بدلاً من الخطأ
                            Ok(Value::Str(String::new()))
                        } else {
                            Err(IqraError::runtime(format!("\u{200F}فشل التنفيذ: {}", e)))
                        }
                    }
                }
            }),
        );
        self.builtins.insert(
            "نفذ_أمر_بمدخل".to_string(),
            self.builtins.get("system_with_io").cloned().unwrap(),
        );
        self.builtins.insert("system_info".to_string(), wrap(builtin_system_info));
        self.builtins.insert("معلومات_النظام".to_string(), wrap(builtin_system_info));
        self.builtins.insert(
            "system".to_string(),
            Arc::new(|rt: &mut Runtime, args: &[Value]| -> Result<Value> {
                if args.len() != 1 {
                    return Err(IqraError::runtime(
                        "\u{200F}المعاملات غير صحيحة لدالة تنفيذ أمر النظام",
                    ));
                }
                let cmd = match &args[0] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون الأمر نصاً")),
                };
                let allowed_cmds = [
                    "echo", "dir", "type", "ls", "cat", "findstr", "grep", "whoami", "hostname",
                    "date", "time", "ping", "sleep",
                ];
                let first = cmd.split_whitespace().next().unwrap_or("").trim().to_lowercase();
                if !allowed_cmds.iter().any(|a| a == &first) {
                    return Err(IqraError::runtime(format!(
                        "\u{200F}هذا الأمر ('{}') غير مسموح — مسموح فقط: {}",
                        first,
                        allowed_cmds.join(", "),
                    )));
                }
                let forbidden_symbols = ["&", "|", ";", ">", "<"];
                for sym in forbidden_symbols.iter() {
                    if cmd.contains(sym) {
                        return Err(IqraError::runtime(
                            "\u{200F}تنفيذ هذا الأمر غير مسموح لأسباب أمنية (رمز محظور)",
                        ));
                    }
                }
                match rt.sys_exec.exec(cmd) {
                    Ok(s) => Ok(Value::Str(s)),
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut {
                            Ok(Value::Str(String::new()))
                        } else {
                            Err(IqraError::runtime(format!("\u{200F}فشل التنفيذ: {}", e)))
                        }
                    }
                }
            }),
        );
        self.builtins.insert("نفذ_أمر".to_string(), self.builtins.get("system").cloned().unwrap());
        // دوال الملفات: تُنفذ باستخدام إعدادات الصندوق الرملي المخزنة داخل Runtime
        self.builtins.insert(
            "read_file".to_string(),
            Arc::new(|rt: &mut Runtime, args: &[Value]| -> Result<Value> {
                if args.len() != 1 {
                    return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة قراءة ملف"));
                }
                let path = match &args[0] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
                };
                let p = Path::new(path);
                if !in_sandbox_with_root(&rt.fs_root, p) {
                    return Ok(Value::Str(
                        "\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)".to_string(),
                    ));
                }
                match fs::read_to_string(p) {
                    Ok(content) => Ok(Value::Str(content)),
                    Err(e) => Err(IqraError::runtime(format!(
                        "\u{200F}تعذر قراءة الملف '{}': {}",
                        path, e
                    ))),
                }
            }),
        );
        self.builtins
            .insert("اقرأ_ملف".to_string(), self.builtins.get("read_file").cloned().unwrap());
        self.builtins.insert(
            "write_file".to_string(),
            Arc::new(|rt: &mut Runtime, args: &[Value]| -> Result<Value> {
                if args.len() != 2 {
                    return Err(IqraError::runtime("\u{200F}المعاملات غير صحيحة لدالة كتابة ملف"));
                }
                let path = match &args[0] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
                };
                let content = match &args[1] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المحتوى نصاً")),
                };
                let p = Path::new(path);
                if !in_sandbox_with_root(&rt.fs_root, p) {
                    return Ok(Value::Str(
                        "\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)".to_string(),
                    ));
                }
                match fs::write(p, content) {
                    Ok(_) => Ok(Value::Bool(true)),
                    Err(e) => Err(IqraError::runtime(format!(
                        "\u{200F}تعذر الكتابة إلى الملف '{}': {}",
                        path, e
                    ))),
                }
            }),
        );
        self.builtins
            .insert("اكتب_ملف".to_string(), self.builtins.get("write_file").cloned().unwrap());
        self.builtins.insert(
            "list_files".to_string(),
            Arc::new(|rt: &mut Runtime, args: &[Value]| -> Result<Value> {
                if args.len() != 1 {
                    return Err(IqraError::runtime(
                        "\u{200F}المعاملات غير صحيحة لدالة قائمة الملفات",
                    ));
                }
                let path = match &args[0] {
                    Value::Str(s) => s,
                    _ => return Err(IqraError::runtime("\u{200F}يجب أن يكون المسار نصاً")),
                };
                let p = Path::new(path);
                if !in_sandbox_with_root(&rt.fs_root, p) {
                    return Ok(Value::Str(
                        "\u{200F}تم رفض الوصول (خارج مساحة العمل المسموح بها)".to_string(),
                    ));
                }
                match fs::read_dir(p) {
                    Ok(entries) => {
                        let files: Vec<Value> = entries
                            .filter_map(|e| e.ok())
                            .map(|e| Value::Str(e.path().display().to_string()))
                            .collect();
                        Ok(Value::List(files))
                    }
                    Err(e) => Err(IqraError::runtime(format!("\u{200F}فشل قراءة المجلد: {}", e))),
                }
            }),
        );
        self.builtins
            .insert("قائمة_ملفات".to_string(), self.builtins.get("list_files").cloned().unwrap());
        self.builtins.insert("env_var".to_string(), wrap(builtin_env_var));
        self.builtins.insert("متغير_بيئة".to_string(), wrap(builtin_env_var));
        let builtins = [
            ("len", 1),
            ("length", 1),
            ("طول", 1),
            ("type", 1),
            ("نوع", 1),
            ("is_number", 1),
            ("رقم؟", 1),
            ("is_string", 1),
            ("نص؟", 1),
            ("to_number", 1),
            ("إلى_رقم", 1),
            ("to_string", 1),
            ("إلى_نص", 1),
            ("list", usize::MAX),
            ("قائمة", usize::MAX),
            ("list_len", 1),
            ("طول_القائمة", 1),
            ("get", 2),
            ("عنصر", 2),
            ("append", 2),
            ("أضف", 2),
            ("remove", 2),
            ("احذف", 2),
            ("contains", 2),
            ("يحتوي", 2),
            ("sort", 1),
            ("رتب", 1),
            ("filter", 2),
            ("رشح", 2),
            ("map", 2),
            ("حول", 2),
            ("find", 2),
            ("ابحث", 2),
            ("forEach", 2),
            ("لكل", 2),
            ("concat", 2),
            ("ادمج", 2),
            ("groupBy", 2),
            ("صنف", 2),
            ("dict", usize::MAX),
            ("قاموس", usize::MAX),
            ("خريطة", usize::MAX),
            ("map_get", 2),
            ("جلب_عنصر", 2),
            ("map_set", 3),
            ("تعيين_عنصر", 3),
            ("map_remove", 2),
            ("حذف_عنصر", 2),
            // دوال رياضية ونصية جديدة
            ("sum", 1),
            ("جمع", 1),
            ("average", 1),
            ("متوسط", 1),
            ("max", 1),
            ("أكبر", 1),
            ("min", 1),
            ("أصغر", 1),
            ("word_count", 1),
            ("عدد_الكلمات", 1),
            ("reverse", 1),
            ("عكس", 1),
            ("today", 0),
            ("تاريخ_اليوم", 0),
        ];
        for (n, arity) in builtins {
            self.set_var(n.to_string(), Value::NativeFunction { name: n.to_string(), arity });
        }
        // اجعل جميع الدوال المسجلة في self.builtins متاحة كدوال مدمجة قابلة للاستدعاء من السكريبت
        let keys: Vec<String> = self.builtins.keys().cloned().collect();
        for k in keys {
            // استخدم arity غير محدود لتجاوز فحص عدد الوسائط هنا، يتم التحقق داخل الدالة نفسها
            self.set_var(k.clone(), Value::NativeFunction { name: k, arity: usize::MAX });
        }
    }
    pub fn exec(&mut self, stmts: &[Stmt]) -> Result<ExecOutput> {
        let mut printed = Vec::new();
        for s in stmts {
            if let Some(_ret) = self.exec_stmt(s, &mut printed)? { /* ignore top-level returns */ }
        }
        Ok(ExecOutput { printed })
    }
    fn exec_block(&mut self, stmts: &[Stmt], printed: &mut Vec<Value>) -> Result<Option<Value>> {
        for s in stmts {
            if let Some(ret) = self.exec_stmt(s, printed)? {
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }
    fn exec_stmt(&mut self, s: &Stmt, printed: &mut Vec<Value>) -> Result<Option<Value>> {
        match s {
            Stmt::Assign { name, value } => {
                let v = self.eval(value)?;
                self.set_var(name.clone(), v);
            }
            Stmt::Expr(e) => {
                let _ = self.eval(e)?;
            }
            Stmt::Print(e) => {
                let v = self.eval(e)?;
                printed.push(v);
            }
            Stmt::Block(stmts) => {
                if let Some(ret) = self.exec_block(stmts, printed)? {
                    return Ok(Some(ret));
                }
            }
            Stmt::If { cond, then_branch, else_branch } => {
                let c = self.eval(cond)?;
                if truthy(&c) {
                    if let Some(r) = self.exec_stmt(then_branch, printed)? {
                        return Ok(Some(r));
                    }
                } else if let Some(e) = else_branch
                    && let Some(r) = self.exec_stmt(e, printed)?
                {
                    return Ok(Some(r));
                }
            }
            Stmt::While { cond, body } => {
                while truthy(&self.eval(cond)?) {
                    if let Some(r) = self.exec_stmt(body, printed)? {
                        return Ok(Some(r));
                    }
                }
            }
            Stmt::Function { name, params, body } => {
                self.set_var(
                    name.clone(),
                    Value::Function { params: params.clone(), body: body.clone() },
                );
            }
            Stmt::Return(expr_opt) => {
                let v = if let Some(e) = expr_opt { self.eval(e)? } else { Value::Nil };
                return Ok(Some(v));
            }
        }
        Ok(None)
    }
    fn eval(&mut self, e: &Expr) -> Result<Value> {
        match e {
            Expr::List(items) => {
                let vals = items.iter().map(|x| self.eval(x)).collect::<Result<Vec<_>>>();
                Ok(Value::List(vals?))
            }
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Nil => Ok(Value::Nil),
            Expr::Var(name) => self
                .get_var(name)
                .ok_or_else(|| IqraError::runtime(format!("متغير غير معرف {name}"))),
            Expr::Binary { left, op, right } => match op {
                TokenKind::And => {
                    let l = self.eval(left)?;
                    if !truthy(&l) {
                        return Ok(Value::Bool(false));
                    }
                    let r = self.eval(right)?;
                    Ok(Value::Bool(truthy(&r)))
                }
                TokenKind::Or => {
                    let l = self.eval(left)?;
                    if truthy(&l) {
                        return Ok(Value::Bool(true));
                    }
                    let r = self.eval(right)?;
                    Ok(Value::Bool(truthy(&r)))
                }
                _ => {
                    let l = self.eval(left)?;
                    let r = self.eval(right)?;
                    match op {
                        TokenKind::Plus => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
                            (Value::Str(a), v) | (v, Value::Str(a)) => {
                                Ok(Value::Str(format!("{}{}", a, v)))
                            }
                            _ => Err(IqraError::runtime("خطأ نوع في المعامل '+'")),
                        },
                        TokenKind::Minus => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                            _ => Err(IqraError::runtime("خطأ نوع في المعامل '-'")),
                        },
                        TokenKind::Star => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                            _ => Err(IqraError::runtime("خطأ نوع في المعامل '*'")),
                        },
                        TokenKind::Slash => match (l, r) {
                            (Value::Number(_), Value::Number(0.0)) => {
                                Err(IqraError::runtime("قسمة على صفر"))
                            }
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                            _ => Err(IqraError::runtime("خطأ نوع في المعامل '/'")),
                        },
                        TokenKind::Eq => Ok(Value::Bool(l == r)),
                        TokenKind::Ne => Ok(Value::Bool(l != r)),
                        TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => {
                            cmp_numbers(op, l, r)
                        }
                        _ => Err(IqraError::runtime("معامل غير صالح")),
                    }
                }
            },
            Expr::Unary { op, expr } => {
                let v = self.eval(expr)?;
                match op {
                    TokenKind::Not => Ok(Value::Bool(!truthy(&v))),
                    _ => Err(IqraError::runtime("معامل أحادي غير مدعوم")),
                }
            }
            Expr::Call { callee, args } => {
                let cal = self.eval(callee)?;
                match cal {
                    Value::Function { params, body } => {
                        if params.len() != args.len() {
                            return Err(IqraError::runtime("عدد الوسائط لا يطابق عدد المعاملات"));
                        }
                        self.push_scope();
                        for (p, arg_expr) in params.iter().zip(args.iter()) {
                            let val = self.eval(arg_expr)?;
                            self.set_var(p.clone(), val);
                        }
                        let mut ret = Value::Nil;
                        for stmt in &body {
                            if let Some(r) = self.exec_stmt(stmt, &mut Vec::new())? {
                                ret = r;
                                break;
                            }
                        }
                        self.pop_scope();
                        Ok(ret)
                    }
                    Value::NativeFunction { name, arity } => {
                        if !(arity == usize::MAX || args.len() == arity) {
                            return Err(IqraError::runtime("عدد الوسائط لا يطابق المعاملات"));
                        }
                        let mut evaled = Vec::new();
                        for a in args {
                            evaled.push(self.eval(a)?);
                        }
                        match name.as_str() {
                            "len" | "length" | "طول" => builtin_len(&evaled),
                            "type" => builtin_type(&evaled),
                            "نوع" => builtin_type_ar(&evaled),
                            "is_number" | "رقم؟" => builtin_is_number(&evaled),
                            "is_string" | "نص؟" => builtin_is_string(&evaled),
                            "to_number" | "إلى_رقم" => builtin_to_number(&evaled),
                            "to_string" | "إلى_نص" => builtin_to_string(&evaled),
                            "list" | "قائمة" => Ok(Value::List(evaled)),
                            "list_len" | "طول_القائمة" => builtin_list_len(&evaled),
                            "get" | "عنصر" => builtin_list_get(&evaled),
                            "append" | "أضف" => builtin_list_append(&evaled),
                            "remove" | "احذف" => builtin_list_remove(&evaled),
                            "contains" | "يحتوي" => builtin_list_contains(&evaled),
                            "sort" | "رتب" => builtin_list_sort(&evaled),
                            "filter" | "رشح" => builtin_list_filter(&evaled),
                            "map" | "حول" => builtin_list_map(&evaled),
                            "find" | "ابحث" => builtin_list_find(&evaled),
                            "forEach" | "لكل" => builtin_list_foreach(&evaled),
                            "concat" | "ادمج" => builtin_list_concat(&evaled),
                            "groupBy" | "صنف" => builtin_list_group_by(&evaled),
                            "dict" | "قاموس" | "خريطة" => builtin_map(&evaled),
                            "sum" | "جمع" => builtin_list_sum(&evaled),
                            "average" | "متوسط" => builtin_list_average(&evaled),
                            "max" | "أكبر" => builtin_list_max(&evaled),
                            "min" | "أصغر" => builtin_list_min(&evaled),
                            "word_count" | "عدد_الكلمات" => builtin_word_count(&evaled),
                            "reverse" | "عكس" => builtin_reverse(&evaled),
                            "today" | "تاريخ_اليوم" => builtin_today(&evaled),
                            "map_get" | "جلب_عنصر" => builtin_map_get(&evaled),
                            "map_set" | "تعيين_عنصر" => builtin_map_set(&evaled),
                            "map_remove" | "حذف_عنصر" => builtin_map_remove(&evaled),
                            _ => {
                                if self.builtins.contains_key(&name) {
                                    self.call_builtin(&name, &evaled)
                                } else {
                                    Err(IqraError::runtime(format!("دالة مدمجة مجهولة: {name}")))
                                }
                            }
                        }
                    }
                    _ => Err(IqraError::runtime("المستدعى ليس دالة")),
                }
            }
        }
    }
}

fn cmp_numbers(op: &TokenKind, l: Value, r: Value) -> Result<Value> {
    match (l, r) {
        (Value::Number(a), Value::Number(b)) => {
            let res = match op {
                TokenKind::Lt => a < b,
                TokenKind::Le => a <= b,
                TokenKind::Gt => a > b,
                TokenKind::Ge => a >= b,
                _ => unreachable!(),
            };
            Ok(Value::Bool(res))
        }
        _ => Err(IqraError::runtime("خطأ نوع في المقارنة")),
    }
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Nil => false,
        Value::Number(n) => *n != 0.0,
        Value::Str(s) => !s.is_empty(),
        Value::List(vs) => !vs.is_empty(),
        Value::Map(m) => !m.is_empty(),
        _ => true,
    }
}
fn builtin_len(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Str(s) => Ok(Value::Number(s.chars().count() as f64)),
        Value::List(vs) => Ok(Value::Number(vs.len() as f64)),
        Value::Map(m) => Ok(Value::Number(m.len() as f64)),
        _ => Err(IqraError::runtime("الوسيـط يجب أن يكون نصاً أو قائمة أو قاموساً")),
    }
}
fn builtin_type(args: &[Value]) -> Result<Value> {
    let t = match &args[0] {
        Value::Number(_) => "number",
        Value::Str(_) => "string",
        Value::Bool(_) => "bool",
        Value::Nil => "nil",
        Value::List(_) => "list",
        Value::Map(_) => "map",
        Value::Function { .. } => "function",
        Value::NativeFunction { .. } => "native",
    };
    Ok(Value::Str(t.into()))
}
fn builtin_type_ar(args: &[Value]) -> Result<Value> {
    let t = match &args[0] {
        Value::Number(_) => "عدد",
        Value::Str(_) => "نص",
        Value::Bool(_) => "منطقي",
        Value::Nil => "لاشيء",
        Value::List(_) => "قائمة",
        Value::Map(_) => "قاموس",
        Value::Function { .. } => "دالة",
        Value::NativeFunction { .. } => "دالة مدمجة",
    };
    Ok(Value::Str(t.into()))
}
fn builtin_is_number(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Number(_))))
}
fn builtin_is_string(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Str(_))))
}
fn builtin_to_number(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::Str(s) => s
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| IqraError::runtime("لا يمكن تحويل النص إلى رقم")),
        Value::Bool(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Nil => Ok(Value::Number(0.0)),
        _ => Err(IqraError::runtime("نوع غير مدعوم للتحويل إلى رقم")),
    }
}

fn builtin_to_string(args: &[Value]) -> Result<Value> {
    Ok(Value::Str(match &args[0] {
        Value::Number(n) => n.to_string(),
        Value::Str(s) => s.clone(),
        Value::Bool(b) => match output_lang() {
            OutputLang::Arabic => {
                if *b {
                    "صحيح".into()
                } else {
                    "خطأ".into()
                }
            }
            OutputLang::English => {
                if *b {
                    "true".into()
                } else {
                    "false".into()
                }
            }
        },
        Value::Nil => match output_lang() {
            OutputLang::Arabic => "لاشيء".into(),
            OutputLang::English => "nil".into(),
        },
        Value::List(vs) => {
            format!("[{}]", vs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
        }
        Value::Map(m) => format!(
            "{{{}}}",
            m.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", ")
        ),
        Value::Function { .. } => match output_lang() {
            OutputLang::Arabic => "<دالة>".into(),
            OutputLang::English => "<function>".into(),
        },
        Value::NativeFunction { .. } => match output_lang() {
            OutputLang::Arabic => "<دالة مدمجة>".into(),
            OutputLang::English => "<native>".into(),
        },
    }))
}

impl Runtime {
    fn push_scope(&mut self) {
        self.vars.push(HashMap::new());
    }
    fn pop_scope(&mut self) {
        self.vars.pop();
    }
    fn set_var(&mut self, name: String, val: Value) {
        if let Some(idx) = self.vars.iter().rposition(|m| m.contains_key(&name)) {
            self.vars[idx].insert(name, val);
        } else if let Some(top) = self.vars.last_mut() {
            top.insert(name, val);
        }
    }
    fn get_var(&self, name: &str) -> Option<Value> {
        for map in self.vars.iter().rev() {
            if let Some(v) = map.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}
