use crate::lang::runtime::IqraError;
use anyhow::{Result, anyhow};
impl Value {
    pub fn to_number(&self) -> Result<f64> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::String(s) => s.parse::<f64>().map_err(|_| anyhow!(IqraError {
                kind: "تحويل إلى رقم | To Number".to_string(),
                message_ar: format!("لا يمكن تحويل السلسلة '{}' إلى رقم.", s),
                message_en: format!("Cannot convert string '{}' to number.", s),
                suggestion: Some("تأكد من أن السلسلة تمثل رقمًا صالحًا | Ensure the string is a valid number".to_string()),
                line: None,
            })),
            _ => Err(anyhow!(IqraError {
                kind: "تحويل إلى رقم | To Number".to_string(),
                message_ar: format!("لا يمكن تحويل النوع '{}' إلى رقم.", self.type_name_ar()),
                message_en: format!("Cannot convert type '{}' to number.", self.type_name()),
                suggestion: Some("استخدم نوعًا مناسبًا | Use a suitable type".to_string()),
                line: None,
            })),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(if *b { "صحيح".to_string() } else { "خطأ".to_string() }),
            Value::Nil => Ok("فارغ".to_string()),
            _ => Err(anyhow!(IqraError {
                kind: "تحويل إلى سلسلة | To String".to_string(),
                message_ar: format!("لا يمكن تحويل النوع '{}' إلى سلسلة.", self.type_name_ar()),
                message_en: format!("Cannot convert type '{}' to string.", self.type_name()),
                suggestion: Some("استخدم نوعًا مناسبًا | Use a suitable type".to_string()),
                line: None,
            })),
        }
    }

    pub fn to_list(&self) -> Result<&Vec<Value>> {
        match self {
            Value::List(l) => Ok(l),
            _ => Err(anyhow!(IqraError {
                kind: "تحويل إلى قائمة | To List".to_string(),
                message_ar: format!("لا يمكن تحويل النوع '{}' إلى قائمة.", self.type_name_ar()),
                message_en: format!("Cannot convert type '{}' to list.", self.type_name()),
                suggestion: Some("استخدم نوعًا مناسبًا | Use a suitable type".to_string()),
                line: None,
            })),
        }
    }

    pub fn to_map(&self) -> Result<&HashMap<String, Value>> {
        match self {
            Value::Map(m) => Ok(m),
            _ => Err(anyhow!(IqraError {
                kind: "تحويل إلى قاموس | To Map".to_string(),
                message_ar: format!("لا يمكن تحويل النوع '{}' إلى قاموس.", self.type_name_ar()),
                message_en: format!("Cannot convert type '{}' to map.", self.type_name()),
                suggestion: Some("استخدم نوعًا مناسبًا | Use a suitable type".to_string()),
                line: None,
            })),
        }
    }

    pub fn type_name_ar(&self) -> &'static str {
        match self {
            Value::Nil => "فارغ",
            Value::Bool(_) => "منطقي",
            Value::Number(_) => "رقم",
            Value::String(_) => "سلسلة",
            Value::List(_) => "قائمة",
            Value::Map(_) => "قاموس",
        }
    }
}
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Map(_) => "map",
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "فارغ"),
            Value::Bool(b) => write!(f, "{}", if *b { "صحيح" } else { "خطأ" }),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in m {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(n as f64)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(l: Vec<Value>) -> Self {
        Value::List(l)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(m: HashMap<String, Value>) -> Self {
        Value::Map(m)
    }
}
