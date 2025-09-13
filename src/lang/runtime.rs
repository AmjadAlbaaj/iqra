#[derive(Debug, Clone)]
pub struct IqraError {
    pub kind: String,
    pub message_ar: String,
    pub message_en: String,
    pub suggestion: Option<String>,
    pub line: Option<usize>,
}

impl std::fmt::Display for IqraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} | {}", self.kind, self.message_ar, self.message_en)?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\nاقتراح: {} | Suggestion: {}", suggestion, suggestion)?;
        }
        if let Some(line) = self.line {
            write!(f, "\nالسطر: {} | Line: {}", line, line)?;
        }
        Ok(())
    }
}
use crate::lang::lexer::Lexer;
use crate::lang::parser::{BinaryOp, Expr, Parser, Stmt, UnaryOp};
use crate::lang::value::Value;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

pub trait SystemExecutor {
    fn exec(&self, cmd: &str) -> std::io::Result<String>;
    fn exec_with_io(&self, cmd: &str, input: &str) -> std::io::Result<String>;
    fn read_file(&self, path: &str) -> std::io::Result<String>;
    fn write_file(&self, path: &str, content: &str) -> std::io::Result<bool>;
    fn list_files(&self, path: &str) -> std::io::Result<Vec<String>>;
    fn get_env_var(&self, name: &str) -> Option<String>;
    fn system_info(&self) -> std::io::Result<HashMap<String, String>>;
}

pub struct DefaultSystemExecutor;

impl SystemExecutor for DefaultSystemExecutor {
    fn exec(&self, cmd: &str) -> std::io::Result<String> {
        let allow_shell_fallback = env::var("IQRA_ALLOW_SHELL_FALLBACK").is_ok();

        let output = if allow_shell_fallback {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd").args(["/C", cmd]).output()?
            }
            #[cfg(not(target_os = "windows"))]
            {
                Command::new("sh").args(["-c", cmd]).output()?
            }
        } else {
            // Try to execute the command directly without shell
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty command"));
            }
            Command::new(parts[0]).args(&parts[1..]).output()?
        };

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn exec_with_io(&self, cmd: &str, input: &str) -> std::io::Result<String> {
        let allow_shell_fallback = env::var("IQRA_ALLOW_SHELL_FALLBACK").is_ok();

        let mut command = if allow_shell_fallback {
            #[cfg(target_os = "windows")]
            {
                let mut cmd_obj = Command::new("cmd");
                cmd_obj.args(["/C", cmd]);
                cmd_obj
            }
            #[cfg(not(target_os = "windows"))]
            {
                let mut cmd_obj = Command::new("sh");
                cmd_obj.args(["-c", cmd]);
                cmd_obj
            }
        } else {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty command"));
            }
            let mut cmd_obj = Command::new(parts[0]);
            cmd_obj.args(&parts[1..]);
            cmd_obj
        };

        use std::io::Write;
        use std::process::Stdio;

        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = command.spawn()?;

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin.write_all(input.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn read_file(&self, path: &str) -> std::io::Result<String> {
        fs::read_to_string(path)
    }

    fn write_file(&self, path: &str, content: &str) -> std::io::Result<bool> {
        fs::write(path, content)?;
        Ok(true)
    }

    fn list_files(&self, path: &str) -> std::io::Result<Vec<String>> {
        let entries = fs::read_dir(path)?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }

        files.sort();
        Ok(files)
    }

    fn get_env_var(&self, name: &str) -> Option<String> {
        env::var(name).ok()
    }

    fn system_info(&self) -> std::io::Result<HashMap<String, String>> {
        let mut info = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            use sys_info::*;
            if let Ok(os) = os_type() {
                info.insert("os".to_string(), os);
            }
            if let Ok(os_version) = os_release() {
                info.insert("os_version".to_string(), os_version);
            }
            if let Ok(hostname) = hostname() {
                info.insert("hostname".to_string(), hostname);
            }
            if let Ok(cpu_num) = cpu_num() {
                info.insert("cpu_cores".to_string(), cpu_num.to_string());
            }
            if let Ok(cpu_speed) = cpu_speed() {
                info.insert("cpu_speed_mhz".to_string(), cpu_speed.to_string());
            }
            if let Ok(mem_info) = mem_info() {
                info.insert("total_memory_kb".to_string(), mem_info.total.to_string());
                info.insert("free_memory_kb".to_string(), mem_info.free.to_string());
            }
        }

        // Basic fallback info
        if info.is_empty() {
            info.insert("os".to_string(), env::consts::OS.to_string());
            info.insert("arch".to_string(), env::consts::ARCH.to_string());
        }

        Ok(info)
    }
}

pub struct Runtime {
    variable_stack: Vec<HashMap<String, Value>>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    system_executor: Box<dyn SystemExecutor>,
    today_cache: Option<String>,
    system_info_cache: Option<HashMap<String, String>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    /// Returns a reference to the current variables (for REPL step mode)
    pub fn get_variables(&self) -> &HashMap<String, Value> {
        self.variable_stack.last().unwrap()
    }
    fn call_user_function(&mut self, name: &str, args: &[Value]) -> Result<Value> {
        // Lazy evaluation: defer block execution, avoid unnecessary evaluation
        let (params, body) = self.functions.get(name).ok_or_else(|| anyhow!(IqraError {
            kind: "دالة غير معرفة".to_string(),
            message_ar: format!("الدالة غير معرفة: {}", name),
            message_en: format!("Undefined function: {}", name),
            suggestion: Some("تأكد من كتابة اسم الدالة بشكل صحيح".to_string()),
            line: None,
        }))?.clone();
        if args.len() != params.len() {
            return Err(anyhow!(IqraError {
                kind: "عدد وسائط غير صحيح".to_string(),
                message_ar: "عدد الوسائط لا يطابق عدد المعاملات".to_string(),
                message_en: "Argument count mismatch".to_string(),
                suggestion: Some("تأكد من عدد الوسائط المدخلة".to_string()),
                line: None,
            }));
        }
        // Save current variables (future: use stack frames for true lazy scope)
        let old_vars = self.variable_stack.last().unwrap().clone();
        for (p, v) in params.iter().zip(args.iter()) {
            self.variable_stack.last_mut().unwrap().insert(p.clone(), v.clone());
        }
        // Execute body lazily: only evaluate statements as needed
        let mut ret = Value::Nil;
        for stmt in &body {
            match self.execute_statement(stmt) {
                Ok(v) => ret = v,
                Err(e) => {
                    let msg = format!("{}", e);
                    if msg.starts_with("__RETURN__:") {
                        let val_str = msg.trim_start_matches("__RETURN__:");
                        if val_str.contains("String(") {
                            let s = val_str.split("String(").nth(1).unwrap().split(")").next().unwrap();
                            ret = Value::String(s.to_string());
                        } else if val_str.contains("Number(") {
                            let n = val_str.split("Number(").nth(1).unwrap().split(")").next().unwrap();
                            ret = Value::Number(n.parse().unwrap_or(0.0));
                        } else {
                            ret = Value::Nil;
                        }
                        break;
                    } else {
                        *self.variable_stack.last_mut().unwrap() = old_vars.clone();
                        // Wrap error in IqraError if not already
                        if msg.contains("IqraError") {
                            return Err(anyhow!(msg));
                        } else {
                            return Err(anyhow!(IqraError {
                                kind: "خطأ في تنفيذ الدالة".to_string(),
                                message_ar: format!("خطأ أثناء تنفيذ الدالة: {}", msg),
                                message_en: format!("Error during function execution: {}", msg),
                                suggestion: Some("راجع الكود داخل الدالة".to_string()),
                                line: None,
                            }));
                        }
                    }
                }
            }
        }
    *self.variable_stack.last_mut().unwrap() = old_vars;
        Ok(ret)
    }
    pub fn new() -> Self {
            Runtime {
                variable_stack: vec![HashMap::new()],
                functions: HashMap::new(),
                system_executor: Box::new(DefaultSystemExecutor),
                today_cache: None,
                system_info_cache: None,
            }
    }

    pub fn new_with_executor(executor: Box<dyn SystemExecutor>) -> Self {
            Runtime {
                variable_stack: vec![HashMap::new()],
                functions: HashMap::new(),
                system_executor: executor,
                today_cache: None,
                system_info_cache: None,
            }
    }

    pub fn execute(&mut self, input: &str) -> Result<Value> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse()?;

        let mut last_value = Value::Nil;
        for stmt in statements {
            last_value = self.execute_statement(&stmt)?;
        }

        Ok(last_value)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<Value> {
            match stmt {
                Stmt::Expression(expr) => self.evaluate_expression(expr),
                Stmt::Assignment { name, value } => {
                    let val = self.evaluate_expression(value)?;
                    self.variable_stack.last_mut().unwrap().insert(name.clone(), val.clone());
                    Ok(val)
                }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                if condition_value.is_truthy() {
                    self.execute_block(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute_block(else_branch)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::While { condition, body } => {
                let mut last_value = Value::Nil;
                while self.evaluate_expression(condition)?.is_truthy() {
                    last_value = self.execute_block(body)?;
                }
                Ok(last_value)
            }
            Stmt::Block(statements) => self.execute_block(statements),
            Stmt::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(Value::Nil)
            }
            Stmt::Return(expr) => {
                // Special handling: propagate return value up
                let val = self.evaluate_expression(expr)?;
                Err(anyhow!("__RETURN__:{:?}", val))
            }
            Stmt::TryCatch { try_block, catch_block, error_var } => {
                // Execute try block
                match self.execute_block(try_block) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        // Optionally bind error to variable
                        if let Some(var) = error_var {
                            self.variable_stack.last_mut().unwrap().insert(var.clone(), Value::String(format!("{}", e)));
                        }
                        // Execute catch block
                        self.execute_block(catch_block)
                    }
                }
            },
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value> {
        // Lazy evaluation: only evaluate statements as needed (e.g., for early return)
        let mut last_value = Value::Nil;
        for stmt in statements {
            match self.execute_statement(stmt) {
                Ok(v) => last_value = v,
                Err(e) => {
                    let msg = format!("{}", e);
                    if msg.starts_with("__RETURN__:") {
                        last_value = Value::Nil; // Value will be handled by caller
                        break;
                    } else {
                        return Err(anyhow!(msg));
                    }
                }
            }
        }
        Ok(last_value)
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Identifier(name) => self
                .variable_stack.last().unwrap()
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow!(IqraError {
                    kind: "متغير غير معرف".to_string(),
                    message_ar: format!("المتغير غير معرف: {}", name),
                    message_en: format!("Undefined variable: {}", name),
                    suggestion: Some("تأكد من تعريف المتغير قبل استخدامه".to_string()),
                    line: None,
                })),
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                self.evaluate_unary_op(operator, &operand_val)
            }
            Expr::Call { name, args } => {
                let arg_values: Result<Vec<Value>> =
                    args.iter().map(|arg| self.evaluate_expression(arg)).collect();
                let arg_values = arg_values?;
                if self.functions.contains_key(name) {
                    return self.call_user_function(name, &arg_values);
                } else {
                    return self.call_builtin(name, &arg_values);
                }
            }
            Expr::List(elements) => {
                let values: Result<Vec<Value>> =
                    elements.iter().map(|elem| self.evaluate_expression(elem)).collect();
                Ok(Value::List(values?))
            }
            Expr::Index { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                let idx_val = self.evaluate_expression(index)?;
                self.evaluate_index(&obj_val, &idx_val)
            }
        }
    }

    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err(anyhow!(IqraError {
                    kind: "جمع غير صالح".to_string(),
                    message_ar: "معاملات غير صالحة للجمع".to_string(),
                    message_en: "Invalid operands for addition".to_string(),
                    suggestion: Some("تأكد أن الطرفين أرقام أو نصوص".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Subtract => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(anyhow!(IqraError {
                    kind: "طرح غير صالح".to_string(),
                    message_ar: "معاملات غير صالحة للطرح".to_string(),
                    message_en: "Invalid operands for subtraction".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Multiply => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(anyhow!(IqraError {
                    kind: "ضرب غير صالح".to_string(),
                    message_ar: "معاملات غير صالحة للضرب".to_string(),
                    message_en: "Invalid operands for multiplication".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Divide => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!(IqraError {
                            kind: "قسمة على صفر".to_string(),
                            message_ar: "القسمة على صفر".to_string(),
                            message_en: "Division by zero".to_string(),
                            suggestion: Some("تأكد أن المقسوم عليه ليس صفراً".to_string()),
                            line: None,
                        }))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(anyhow!(IqraError {
                    kind: "قسمة غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للقسمة".to_string(),
                    message_en: "Invalid operands for division".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Modulo => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!(IqraError {
                            kind: "قسمة باقية على صفر".to_string(),
                            message_ar: "القسمة الباقية على صفر".to_string(),
                            message_en: "Modulo by zero".to_string(),
                            suggestion: Some("تأكد أن المقسوم عليه ليس صفراً".to_string()),
                            line: None,
                        }))
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(anyhow!(IqraError {
                    kind: "قسمة باقية غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للقسمة الباقية".to_string(),
                    message_en: "Invalid operands for modulo".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOp::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err(anyhow!(IqraError {
                    kind: "مقارنة غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للمقارنة".to_string(),
                    message_en: "Invalid operands for comparison".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(anyhow!(IqraError {
                    kind: "مقارنة غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للمقارنة".to_string(),
                    message_en: "Invalid operands for comparison".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err(anyhow!(IqraError {
                    kind: "مقارنة غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للمقارنة".to_string(),
                    message_en: "Invalid operands for comparison".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(anyhow!(IqraError {
                    kind: "مقارنة غير صالحة".to_string(),
                    message_ar: "معاملات غير صالحة للمقارنة".to_string(),
                    message_en: "Invalid operands for comparison".to_string(),
                    suggestion: Some("استخدم أرقام فقط".to_string()),
                    line: None,
                })),
            },
            BinaryOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value> {
        match op {
            UnaryOp::Not => Ok(Value::Bool(!operand.is_truthy())),
            UnaryOp::Minus => match operand {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(anyhow!(IqraError {
                    kind: "سالب أحادي غير صالح".to_string(),
                    message_ar: "معامل غير صالح للسالب الأحادي".to_string(),
                    message_en: "Invalid operand for unary minus".to_string(),
                    suggestion: Some("استخدم رقم فقط".to_string()),
                    line: None,
                })),
            },
        }
    }

    fn evaluate_index(&self, object: &Value, index: &Value) -> Result<Value> {
        match (object, index) {
            (Value::List(list), Value::Number(n)) => {
                let idx = *n as usize;
                list.get(idx).cloned().ok_or_else(|| anyhow!(IqraError {
                    kind: "فهرسة خارج النطاق".to_string(),
                    message_ar: format!("الفهرس خارج النطاق: {}", idx),
                    message_en: format!("Index out of bounds: {}", idx),
                    suggestion: Some("تأكد من أن الفهرس ضمن حدود القائمة".to_string()),
                    line: None,
                }))
            }
            (Value::Map(map), Value::String(key)) => {
                map.get(key).cloned().ok_or_else(|| anyhow!(IqraError {
                    kind: "مفتاح غير موجود".to_string(),
                    message_ar: format!("المفتاح غير موجود: {}", key),
                    message_en: format!("Key not found: {}", key),
                    suggestion: Some("تأكد من وجود المفتاح في القاموس".to_string()),
                    line: None,
                }))
            }
            _ => Err(anyhow!(IqraError {
                kind: "عملية فهرسة غير صالحة".to_string(),
                message_ar: "عملية فهرسة غير صالحة".to_string(),
                message_en: "Invalid indexing operation".to_string(),
                suggestion: Some("استخدم قائمة أو قاموس مع فهرس مناسب".to_string()),
                line: None,
            })),
        }
    }

    pub fn call_builtin(&mut self, name: &str, args: &[Value]) -> Result<Value> {
        match name {
            // Arabic and English print functions
            "اطبع" | "print" => {
                if args.is_empty() {
                    println!();
                } else {
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                }
                Ok(Value::Nil)
            }

            // List functions
            "list" | "قائمة" => Ok(Value::List(args.to_vec())),

            "list_len" | "طول_القائمة" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة طول_القائمة تتوقع وسيطاً واحداً".to_string(),
                        message_en: "list_len expects 1 argument".to_string(),
                        suggestion: Some("استخدم قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => Ok(Value::Number(list.len() as f64)),
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "دالة طول_القائمة تتوقع قائمة".to_string(),
                        message_en: "list_len expects a list".to_string(),
                        suggestion: Some("تأكد أن الوسيط هو قائمة".to_string()),
                        line: None,
                    })),
                }
            }

            "get" | "عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة عنصر تتوقع وسيطين".to_string(),
                        message_en: "get expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قائمة وفهرس".to_string()),
                        line: None,
                    }));
                }
                self.evaluate_index(&args[0], &args[1])
            }

            "append" | "أضف" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة أضف تتوقع وسيطين".to_string(),
                        message_en: "append expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قائمة وقيمة للإضافة".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        let mut new_list = list.clone();
                        new_list.push(args[1].clone());
                        Ok(Value::List(new_list))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "دالة أضف تتوقع قائمة كوسيط أول".to_string(),
                        message_en: "append expects a list as first argument".to_string(),
                        suggestion: Some("تأكد أن الوسيط الأول هو قائمة".to_string()),
                        line: None,
                    })),
                }
            }

            "remove" | "احذف" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة احذف تتوقع وسيطين".to_string(),
                        message_en: "remove expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قائمة وقيمة للحذف".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        let mut new_list = Vec::new();
                        for item in list {
                            if item != &args[1] {
                                new_list.push(item.clone());
                            }
                        }
                        Ok(Value::List(new_list))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "دالة احذف تتوقع قائمة كوسيط أول".to_string(),
                        message_en: "remove expects a list as first argument".to_string(),
                        suggestion: Some("تأكد أن الوسيط الأول هو قائمة".to_string()),
                        line: None,
                    })),
                }
            }

            "contains" | "يحتوي" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة يحتوي تتوقع وسيطين".to_string(),
                        message_en: "contains expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قائمة وقيمة للبحث".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => Ok(Value::Bool(list.contains(&args[1]))),
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "دالة يحتوي تتوقع قائمة كوسيط أول".to_string(),
                        message_en: "contains expects a list as first argument".to_string(),
                        suggestion: Some("تأكد أن الوسيط الأول هو قائمة".to_string()),
                        line: None,
                    })),
                }
            }

            // Map functions
            "map" | "قاموس" => {
                if args.len() % 2 != 0 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة قاموس تتوقع عدد زوجي من الوسائط".to_string(),
                        message_en: "map expects an even number of arguments".to_string(),
                        suggestion: Some("استخدم أزواج مفتاح/قيمة".to_string()),
                        line: None,
                    }));
                }
                let mut map = HashMap::new();
                for chunk in args.chunks(2) {
                    if let Value::String(key) = &chunk[0] {
                        map.insert(key.clone(), chunk[1].clone());
                    } else {
                        return Err(anyhow!(IqraError {
                            kind: "نوع مفتاح غير صحيح".to_string(),
                            message_ar: "مفاتيح القاموس يجب أن تكون نصوصاً".to_string(),
                            message_en: "map keys must be strings".to_string(),
                            suggestion: Some("تأكد أن جميع المفاتيح نصوص".to_string()),
                            line: None,
                        }));
                    }
                }
                Ok(Value::Map(map))
            }

            "map_get" | "جلب_عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة جلب_عنصر تتوقع وسيطين".to_string(),
                        message_en: "map_get expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قاموس ومفتاح".to_string()),
                        line: None,
                    }));
                }
                self.evaluate_index(&args[0], &args[1])
            }

            "map_set" | "تعيين_عنصر" => {
                if args.len() != 3 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة تعيين_عنصر تتوقع 3 وسائط".to_string(),
                        message_en: "map_set expects 3 arguments".to_string(),
                        suggestion: Some("استخدم قاموس، مفتاح، وقيمة".to_string()),
                        line: None,
                    }));
                }
                match (&args[0], &args[1]) {
                    (Value::Map(map), Value::String(key)) => {
                        let mut new_map = map.clone();
                        new_map.insert(key.clone(), args[2].clone());
                        Ok(Value::Map(new_map))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "map_set تتوقع قاموس ومفتاح نصي".to_string(),
                        message_en: "map_set expects a map and string key".to_string(),
                        suggestion: Some("تأكد أن الوسيط الأول قاموس والثاني نص".to_string()),
                        line: None,
                    })),
                }
            }

            "map_remove" | "حذف_عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "دالة حذف_عنصر تتوقع وسيطين".to_string(),
                        message_en: "map_remove expects 2 arguments".to_string(),
                        suggestion: Some("استخدم قاموس ومفتاح".to_string()),
                        line: None,
                    }));
                }
                match (&args[0], &args[1]) {
                    (Value::Map(map), Value::String(key)) => {
                        let mut new_map = map.clone();
                        new_map.remove(key);
                        Ok(Value::Map(new_map))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "map_remove تتوقع قاموس ومفتاح نصي".to_string(),
                        message_en: "map_remove expects a map and string key".to_string(),
                        suggestion: Some("تأكد أن الوسيط الأول قاموس والثاني نص".to_string()),
                        line: None,
                    })),
                }
            }

            // Type and conversion functions
            "type" | "نوع" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "type تتوقع وسيطاً واحداً".to_string(),
                        message_en: "type expects 1 argument".to_string(),
                        suggestion: Some("استخدم قيمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                Ok(Value::String(args[0].type_name().to_string()))
            }

            "to_number" | "إلى_رقم" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "إلى_رقم تتوقع وسيطاً واحداً".to_string(),
                        message_en: "to_number expects 1 argument".to_string(),
                        suggestion: Some("استخدم قيمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::String(s) => {
                        let ascii_str = s
                            .chars()
                            .map(|ch| match ch {
                                '٠' => '0',
                                '١' => '1',
                                '٢' => '2',
                                '٣' => '3',
                                '٤' => '4',
                                '٥' => '5',
                                '٦' => '6',
                                '٧' => '7',
                                '٨' => '8',
                                '٩' => '9',
                                _ => ch,
                            })
                            .collect::<String>();

                        ascii_str
                            .parse::<f64>()
                            .map(Value::Number)
                            .map_err(|_| anyhow!(IqraError {
                                kind: "تحويل غير صالح".to_string(),
                                message_ar: format!("لا يمكن تحويل '{}' إلى رقم", s),
                                message_en: format!("Cannot convert '{}' to number", s),
                                suggestion: Some("تأكد أن النص يمثل رقماً صحيحاً".to_string()),
                                line: None,
                            }))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "لا يمكن تحويل القيمة إلى رقم".to_string(),
                        message_en: "Cannot convert to number".to_string(),
                        suggestion: Some("استخدم نصاً أو رقماً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "to_string" | "إلى_نص" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "إلى_نص تتوقع وسيطاً واحداً".to_string(),
                        message_en: "to_string expects 1 argument".to_string(),
                        suggestion: Some("استخدم قيمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                Ok(Value::String(format!("{}", args[0])))
            }

            "is_number" | "رقم؟" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "رقم؟ تتوقع وسيطاً واحداً".to_string(),
                        message_en: "is_number expects 1 argument".to_string(),
                        suggestion: Some("استخدم قيمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                Ok(Value::Bool(matches!(args[0], Value::Number(_))))
            }

            "is_string" | "نص؟" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "نص؟ تتوقع وسيطاً واحداً".to_string(),
                        message_en: "is_string expects 1 argument".to_string(),
                        suggestion: Some("استخدم قيمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                Ok(Value::Bool(matches!(args[0], Value::String(_))))
            }

            "len" | "طول" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "طول تتوقع وسيطاً واحداً".to_string(),
                        message_en: "len expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً أو قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
                    Value::List(l) => Ok(Value::Number(l.len() as f64)),
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "طول يتوقع نصاً أو قائمة".to_string(),
                        message_en: "len expects a string or list".to_string(),
                        suggestion: Some("استخدم نصاً أو قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            // Math functions
            "sum" | "جمع" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "جمع تتوقع وسيطاً واحداً".to_string(),
                        message_en: "sum expects 1 argument".to_string(),
                        suggestion: Some("استخدم قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        let mut total = 0.0;
                        for item in list {
                            if let Value::Number(n) = item {
                                total += n;
                            } else {
                                return Err(anyhow!(IqraError {
                                    kind: "نوع عنصر غير صحيح".to_string(),
                                    message_ar: "جمع يتوقع قائمة أرقام فقط".to_string(),
                                    message_en: "sum expects a list of numbers".to_string(),
                                    suggestion: Some("تأكد أن جميع العناصر أرقام".to_string()),
                                    line: None,
                                }));
                            }
                        }
                        Ok(Value::Number(total))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "جمع يتوقع قائمة".to_string(),
                        message_en: "sum expects a list".to_string(),
                        suggestion: Some("استخدم قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "average" | "متوسط" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "متوسط يتوقع وسيطاً واحداً".to_string(),
                        message_en: "average expects 1 argument".to_string(),
                        suggestion: Some("استخدم قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Ok(Value::Number(0.0));
                        }
                        let mut total = 0.0;
                        for item in list {
                            if let Value::Number(n) = item {
                                total += n;
                            } else {
                                return Err(anyhow!(IqraError {
                                    kind: "نوع عنصر غير صحيح".to_string(),
                                    message_ar: "متوسط يتوقع قائمة أرقام فقط".to_string(),
                                    message_en: "average expects a list of numbers".to_string(),
                                    suggestion: Some("تأكد أن جميع العناصر أرقام".to_string()),
                                    line: None,
                                }));
                            }
                        }
                        Ok(Value::Number(total / list.len() as f64))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "متوسط يتوقع قائمة".to_string(),
                        message_en: "average expects a list".to_string(),
                        suggestion: Some("استخدم قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "max" | "أكبر" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "أكبر تتوقع وسيطاً واحداً".to_string(),
                        message_en: "max expects 1 argument".to_string(),
                        suggestion: Some("استخدم قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err(anyhow!(IqraError {
                                kind: "قائمة فارغة".to_string(),
                                message_ar: "أكبر تتوقع قائمة غير فارغة".to_string(),
                                message_en: "max expects a non-empty list".to_string(),
                                suggestion: Some("استخدم قائمة فيها عناصر".to_string()),
                                line: None,
                            }));
                        }
                        let mut max_val = f64::NEG_INFINITY;
                        for item in list {
                            if let Value::Number(n) = item {
                                if *n > max_val {
                                    max_val = *n;
                                }
                            } else {
                                return Err(anyhow!(IqraError {
                                    kind: "نوع عنصر غير صحيح".to_string(),
                                    message_ar: "أكبر يتوقع قائمة أرقام فقط".to_string(),
                                    message_en: "max expects a list of numbers".to_string(),
                                    suggestion: Some("تأكد أن جميع العناصر أرقام".to_string()),
                                    line: None,
                                }));
                            }
                        }
                        Ok(Value::Number(max_val))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "أكبر يتوقع قائمة".to_string(),
                        message_en: "max expects a list".to_string(),
                        suggestion: Some("استخدم قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "min" | "أصغر" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "أصغر تتوقع وسيطاً واحداً".to_string(),
                        message_en: "min expects 1 argument".to_string(),
                        suggestion: Some("استخدم قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err(anyhow!(IqraError {
                                kind: "قائمة فارغة".to_string(),
                                message_ar: "أصغر تتوقع قائمة غير فارغة".to_string(),
                                message_en: "min expects a non-empty list".to_string(),
                                suggestion: Some("استخدم قائمة فيها عناصر".to_string()),
                                line: None,
                            }));
                        }
                        let mut min_val = f64::INFINITY;
                        for item in list {
                            if let Value::Number(n) = item {
                                if *n < min_val {
                                    min_val = *n;
                                }
                            } else {
                                return Err(anyhow!(IqraError {
                                    kind: "نوع عنصر غير صحيح".to_string(),
                                    message_ar: "أصغر يتوقع قائمة أرقام فقط".to_string(),
                                    message_en: "min expects a list of numbers".to_string(),
                                    suggestion: Some("تأكد أن جميع العناصر أرقام".to_string()),
                                    line: None,
                                }));
                            }
                        }
                        Ok(Value::Number(min_val))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "أصغر يتوقع قائمة".to_string(),
                        message_en: "min expects a list".to_string(),
                        suggestion: Some("استخدم قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            // String functions
            "word_count" | "عدد_الكلمات" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "عدد_الكلمات تتوقع وسيطاً واحداً".to_string(),
                        message_en: "word_count expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً واحداً فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(s) => {
                        let count = s.split_whitespace().count();
                        Ok(Value::Number(count as f64))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "عدد_الكلمات تتوقع نصاً".to_string(),
                        message_en: "word_count expects a string".to_string(),
                        suggestion: Some("استخدم نصاً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "reverse" | "عكس" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "عكس تتوقع وسيطاً واحداً".to_string(),
                        message_en: "reverse expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً أو قائمة واحدة فقط".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(s) => {
                        let reversed: String = s.chars().rev().collect();
                        Ok(Value::String(reversed))
                    }
                    Value::List(list) => {
                        let mut reversed = list.clone();
                        reversed.reverse();
                        Ok(Value::List(reversed))
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "عكس يتوقع نصاً أو قائمة".to_string(),
                        message_en: "reverse expects a string or list".to_string(),
                        suggestion: Some("استخدم نصاً أو قائمة فقط".to_string()),
                        line: None,
                    })),
                }
            }

            // Date functions
            "today" | "تاريخ_اليوم" => {
                if let Some(ref cached) = self.today_cache {
                    Ok(Value::String(cached.clone()))
                } else {
                    use chrono::Local;
                    let today = Local::now().format("%Y-%m-%d").to_string();
                    self.today_cache = Some(today.clone());
                    Ok(Value::String(today))
                }
            }

            // System functions
            "system" | "نفذ_أمر" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "نفذ_أمر تتوقع وسيطاً واحداً".to_string(),
                        message_en: "system expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً يمثل الأمر".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(cmd) => match self.system_executor.exec(cmd) {
                        Ok(output) => Ok(Value::String(output.trim().to_string())),
                        Err(e) => Err(anyhow!(IqraError {
                            kind: "فشل تنفيذ أمر النظام".to_string(),
                            message_ar: format!("فشل تنفيذ الأمر: {}", e),
                            message_en: format!("System command failed: {}", e),
                            suggestion: Some("تأكد من صحة الأمر وصلاحيات التنفيذ".to_string()),
                            line: None,
                        })),
                    },
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "نفذ_أمر يتوقع نصاً يمثل الأمر".to_string(),
                        message_en: "system expects a string command".to_string(),
                        suggestion: Some("استخدم نصاً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "system_with_io" | "نفذ_أمر_بمدخل" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "نفذ_أمر_بمدخل تتوقع وسيطين".to_string(),
                        message_en: "system_with_io expects 2 arguments".to_string(),
                        suggestion: Some("استخدم نصين: الأمر والمدخل".to_string()),
                        line: None,
                    }));
                }
                match (&args[0], &args[1]) {
                    (Value::String(cmd), Value::String(input)) => {
                        match self.system_executor.exec_with_io(cmd, input) {
                            Ok(output) => Ok(Value::String(output.trim().to_string())),
                            Err(e) => Err(anyhow!(IqraError {
                                kind: "فشل تنفيذ أمر النظام".to_string(),
                                message_ar: format!("فشل تنفيذ الأمر بمدخل: {}", e),
                                message_en: format!("System command failed: {}", e),
                                suggestion: Some("تأكد من صحة الأمر والمدخل وصلاحيات التنفيذ".to_string()),
                                line: None,
                            })),
                        }
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "نفذ_أمر_بمدخل يتوقع نصين".to_string(),
                        message_en: "system_with_io expects string arguments".to_string(),
                        suggestion: Some("استخدم نصين فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "read_file" | "اقرأ_ملف" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "اقرأ_ملف تتوقع وسيطاً واحداً".to_string(),
                        message_en: "read_file expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً يمثل المسار".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(path) => match self.system_executor.read_file(path) {
                        Ok(content) => Ok(Value::String(content)),
                        Err(e) => Err(anyhow!(IqraError {
                            kind: "فشل قراءة الملف".to_string(),
                            message_ar: format!("فشل قراءة الملف: {}", e),
                            message_en: format!("Failed to read file: {}", e),
                            suggestion: Some("تأكد من صحة المسار وصلاحيات القراءة".to_string()),
                            line: None,
                        })),
                    },
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "اقرأ_ملف يتوقع نصاً يمثل المسار".to_string(),
                        message_en: "read_file expects a string path".to_string(),
                        suggestion: Some("استخدم نصاً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "write_file" | "اكتب_ملف" => {
                if args.len() != 2 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "اكتب_ملف تتوقع وسيطين".to_string(),
                        message_en: "write_file expects 2 arguments".to_string(),
                        suggestion: Some("استخدم نصين: المسار والمحتوى".to_string()),
                        line: None,
                    }));
                }
                match (&args[0], &args[1]) {
                    (Value::String(path), Value::String(content)) => {
                        match self.system_executor.write_file(path, content) {
                            Ok(success) => Ok(Value::Bool(success)),
                            Err(e) => Err(anyhow!(IqraError {
                                kind: "فشل كتابة الملف".to_string(),
                                message_ar: format!("فشل كتابة الملف: {}", e),
                                message_en: format!("Failed to write file: {}", e),
                                suggestion: Some("تأكد من صحة المسار وصلاحيات الكتابة".to_string()),
                                line: None,
                            })),
                        }
                    }
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "اكتب_ملف يتوقع نصين".to_string(),
                        message_en: "write_file expects string arguments".to_string(),
                        suggestion: Some("استخدم نصين فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "list_files" | "قائمة_ملفات" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "قائمة_ملفات تتوقع وسيطاً واحداً".to_string(),
                        message_en: "list_files expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً يمثل المسار".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(path) => match self.system_executor.list_files(path) {
                        Ok(files) => {
                            let file_values: Vec<Value> =
                                files.into_iter().map(Value::String).collect();
                            Ok(Value::List(file_values))
                        }
                        Err(e) => Err(anyhow!(IqraError {
                            kind: "فشل جلب قائمة الملفات".to_string(),
                            message_ar: format!("فشل جلب قائمة الملفات: {}", e),
                            message_en: format!("Failed to list files: {}", e),
                            suggestion: Some("تأكد من صحة المسار وصلاحيات القراءة".to_string()),
                            line: None,
                        })),
                    },
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "قائمة_ملفات تتوقع نصاً يمثل المسار".to_string(),
                        message_en: "list_files expects a string path".to_string(),
                        suggestion: Some("استخدم نصاً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "env_var" | "متغير_بيئة" => {
                if args.len() != 1 {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "متغير_بيئة تتوقع وسيطاً واحداً".to_string(),
                        message_en: "env_var expects 1 argument".to_string(),
                        suggestion: Some("استخدم نصاً يمثل اسم المتغير".to_string()),
                        line: None,
                    }));
                }
                match &args[0] {
                    Value::String(name) => match self.system_executor.get_env_var(name) {
                        Some(value) => Ok(Value::String(value)),
                        None => Ok(Value::Nil),
                    },
                    _ => Err(anyhow!(IqraError {
                        kind: "نوع وسيط غير صحيح".to_string(),
                        message_ar: "متغير_بيئة يتوقع نصاً يمثل اسم المتغير".to_string(),
                        message_en: "env_var expects a string name".to_string(),
                        suggestion: Some("استخدم نصاً فقط".to_string()),
                        line: None,
                    })),
                }
            }

            "system_info" | "معلومات_النظام" => {
                if !args.is_empty() {
                    return Err(anyhow!(IqraError {
                        kind: "عدد وسائط غير صحيح".to_string(),
                        message_ar: "معلومات_النظام لا تتوقع وسائط".to_string(),
                        message_en: "system_info expects no arguments".to_string(),
                        suggestion: Some("لا تستخدم وسائط مع هذه الدالة".to_string()),
                        line: None,
                    }));
                }
                if let Some(ref cached) = self.system_info_cache {
                    let map_values: HashMap<String, Value> =
                        cached.clone().into_iter().map(|(k, v)| (k, Value::String(v))).collect();
                    Ok(Value::Map(map_values))
                } else {
                    match self.system_executor.system_info() {
                        Ok(info) => {
                            self.system_info_cache = Some(info.clone());
                            let map_values: HashMap<String, Value> =
                                info.into_iter().map(|(k, v)| (k, Value::String(v))).collect();
                            Ok(Value::Map(map_values))
                        }
                        Err(e) => Err(anyhow!(IqraError {
                            kind: "فشل جلب معلومات النظام".to_string(),
                            message_ar: format!("فشل جلب معلومات النظام: {}", e),
                            message_en: format!("Failed to get system info: {}", e),
                            suggestion: Some("تأكد من صلاحيات النظام".to_string()),
                            line: None,
                        })),
                    }
                }
            }

            _ => Err(anyhow!(IqraError {
                kind: "دالة غير معرفة".to_string(),
                message_ar: format!("دالة غير معرفة: {}", name),
                message_en: format!("Unknown function: {}", name),
                suggestion: Some("تأكد من كتابة اسم الدالة بشكل صحيح".to_string()),
                line: None,
            })),
        }
    }
}
