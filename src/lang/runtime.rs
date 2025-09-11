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
    variables: HashMap<String, Value>,
    system_executor: Box<dyn SystemExecutor>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self { variables: HashMap::new(), system_executor: Box::new(DefaultSystemExecutor) }
    }

    pub fn new_with_executor(executor: Box<dyn SystemExecutor>) -> Self {
        Self { variables: HashMap::new(), system_executor: executor }
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
                self.variables.insert(name.clone(), val.clone());
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
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value> {
        let mut last_value = Value::Nil;
        for stmt in statements {
            last_value = self.execute_statement(stmt)?;
        }
        Ok(last_value)
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow!("Undefined variable: {}", name)),
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
                self.call_builtin(name, &arg_values)
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
                _ => Err(anyhow!("Invalid operands for addition")),
            },
            BinaryOp::Subtract => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(anyhow!("Invalid operands for subtraction")),
            },
            BinaryOp::Multiply => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(anyhow!("Invalid operands for multiplication")),
            },
            BinaryOp::Divide => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(anyhow!("Invalid operands for division")),
            },
            BinaryOp::Modulo => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!("Modulo by zero"))
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(anyhow!("Invalid operands for modulo")),
            },
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOp::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err(anyhow!("Invalid operands for comparison")),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(anyhow!("Invalid operands for comparison")),
            },
            BinaryOp::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err(anyhow!("Invalid operands for comparison")),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(anyhow!("Invalid operands for comparison")),
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
                _ => Err(anyhow!("Invalid operand for unary minus")),
            },
        }
    }

    fn evaluate_index(&self, object: &Value, index: &Value) -> Result<Value> {
        match (object, index) {
            (Value::List(list), Value::Number(n)) => {
                let idx = *n as usize;
                list.get(idx).cloned().ok_or_else(|| anyhow!("Index out of bounds: {}", idx))
            }
            (Value::Map(map), Value::String(key)) => {
                map.get(key).cloned().ok_or_else(|| anyhow!("Key not found: {}", key))
            }
            _ => Err(anyhow!("Invalid indexing operation")),
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
                    return Err(anyhow!("list_len expects 1 argument"));
                }
                match &args[0] {
                    Value::List(list) => Ok(Value::Number(list.len() as f64)),
                    _ => Err(anyhow!("list_len expects a list")),
                }
            }

            "get" | "عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!("get expects 2 arguments"));
                }
                self.evaluate_index(&args[0], &args[1])
            }

            "append" | "أضف" => {
                if args.len() != 2 {
                    return Err(anyhow!("append expects 2 arguments"));
                }
                match &args[0] {
                    Value::List(list) => {
                        let mut new_list = list.clone();
                        new_list.push(args[1].clone());
                        Ok(Value::List(new_list))
                    }
                    _ => Err(anyhow!("append expects a list as first argument")),
                }
            }

            "remove" | "احذف" => {
                if args.len() != 2 {
                    return Err(anyhow!("remove expects 2 arguments"));
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
                    _ => Err(anyhow!("remove expects a list as first argument")),
                }
            }

            "contains" | "يحتوي" => {
                if args.len() != 2 {
                    return Err(anyhow!("contains expects 2 arguments"));
                }
                match &args[0] {
                    Value::List(list) => Ok(Value::Bool(list.contains(&args[1]))),
                    _ => Err(anyhow!("contains expects a list as first argument")),
                }
            }

            // Map functions
            "map" | "قاموس" => {
                if args.len() % 2 != 0 {
                    return Err(anyhow!("map expects an even number of arguments"));
                }
                let mut map = HashMap::new();
                for chunk in args.chunks(2) {
                    if let Value::String(key) = &chunk[0] {
                        map.insert(key.clone(), chunk[1].clone());
                    } else {
                        return Err(anyhow!("map keys must be strings"));
                    }
                }
                Ok(Value::Map(map))
            }

            "map_get" | "جلب_عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!("map_get expects 2 arguments"));
                }
                self.evaluate_index(&args[0], &args[1])
            }

            "map_set" | "تعيين_عنصر" => {
                if args.len() != 3 {
                    return Err(anyhow!("map_set expects 3 arguments"));
                }
                match (&args[0], &args[1]) {
                    (Value::Map(map), Value::String(key)) => {
                        let mut new_map = map.clone();
                        new_map.insert(key.clone(), args[2].clone());
                        Ok(Value::Map(new_map))
                    }
                    _ => Err(anyhow!("map_set expects a map and string key")),
                }
            }

            "map_remove" | "حذف_عنصر" => {
                if args.len() != 2 {
                    return Err(anyhow!("map_remove expects 2 arguments"));
                }
                match (&args[0], &args[1]) {
                    (Value::Map(map), Value::String(key)) => {
                        let mut new_map = map.clone();
                        new_map.remove(key);
                        Ok(Value::Map(new_map))
                    }
                    _ => Err(anyhow!("map_remove expects a map and string key")),
                }
            }

            // Type and conversion functions
            "type" | "نوع" => {
                if args.len() != 1 {
                    return Err(anyhow!("type expects 1 argument"));
                }
                Ok(Value::String(args[0].type_name().to_string()))
            }

            "to_number" | "إلى_رقم" => {
                if args.len() != 1 {
                    return Err(anyhow!("to_number expects 1 argument"));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::String(s) => {
                        // Convert Arabic digits to ASCII digits first
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
                            .map_err(|_| anyhow!("Cannot convert '{}' to number", s))
                    }
                    _ => Err(anyhow!("Cannot convert to number")),
                }
            }

            "to_string" | "إلى_نص" => {
                if args.len() != 1 {
                    return Err(anyhow!("to_string expects 1 argument"));
                }
                Ok(Value::String(format!("{}", args[0])))
            }

            "is_number" | "رقم؟" => {
                if args.len() != 1 {
                    return Err(anyhow!("is_number expects 1 argument"));
                }
                Ok(Value::Bool(matches!(args[0], Value::Number(_))))
            }

            "is_string" | "نص؟" => {
                if args.len() != 1 {
                    return Err(anyhow!("is_string expects 1 argument"));
                }
                Ok(Value::Bool(matches!(args[0], Value::String(_))))
            }

            "len" | "طول" => {
                if args.len() != 1 {
                    return Err(anyhow!("len expects 1 argument"));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
                    Value::List(l) => Ok(Value::Number(l.len() as f64)),
                    _ => Err(anyhow!("len expects a string or list")),
                }
            }

            // Math functions
            "sum" | "جمع" => {
                if args.len() != 1 {
                    return Err(anyhow!("sum expects 1 argument"));
                }
                match &args[0] {
                    Value::List(list) => {
                        let mut total = 0.0;
                        for item in list {
                            if let Value::Number(n) = item {
                                total += n;
                            } else {
                                return Err(anyhow!("sum expects a list of numbers"));
                            }
                        }
                        Ok(Value::Number(total))
                    }
                    _ => Err(anyhow!("sum expects a list")),
                }
            }

            "average" | "متوسط" => {
                if args.len() != 1 {
                    return Err(anyhow!("average expects 1 argument"));
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
                                return Err(anyhow!("average expects a list of numbers"));
                            }
                        }
                        Ok(Value::Number(total / list.len() as f64))
                    }
                    _ => Err(anyhow!("average expects a list")),
                }
            }

            "max" | "أكبر" => {
                if args.len() != 1 {
                    return Err(anyhow!("max expects 1 argument"));
                }
                match &args[0] {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err(anyhow!("max expects a non-empty list"));
                        }
                        let mut max_val = f64::NEG_INFINITY;
                        for item in list {
                            if let Value::Number(n) = item {
                                if *n > max_val {
                                    max_val = *n;
                                }
                            } else {
                                return Err(anyhow!("max expects a list of numbers"));
                            }
                        }
                        Ok(Value::Number(max_val))
                    }
                    _ => Err(anyhow!("max expects a list")),
                }
            }

            "min" | "أصغر" => {
                if args.len() != 1 {
                    return Err(anyhow!("min expects 1 argument"));
                }
                match &args[0] {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err(anyhow!("min expects a non-empty list"));
                        }
                        let mut min_val = f64::INFINITY;
                        for item in list {
                            if let Value::Number(n) = item {
                                if *n < min_val {
                                    min_val = *n;
                                }
                            } else {
                                return Err(anyhow!("min expects a list of numbers"));
                            }
                        }
                        Ok(Value::Number(min_val))
                    }
                    _ => Err(anyhow!("min expects a list")),
                }
            }

            // String functions
            "word_count" | "عدد_الكلمات" => {
                if args.len() != 1 {
                    return Err(anyhow!("word_count expects 1 argument"));
                }
                match &args[0] {
                    Value::String(s) => {
                        let count = s.split_whitespace().count();
                        Ok(Value::Number(count as f64))
                    }
                    _ => Err(anyhow!("word_count expects a string")),
                }
            }

            "reverse" | "عكس" => {
                if args.len() != 1 {
                    return Err(anyhow!("reverse expects 1 argument"));
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
                    _ => Err(anyhow!("reverse expects a string or list")),
                }
            }

            // Date functions
            "today" | "تاريخ_اليوم" => {
                use chrono::Local;
                let today = Local::now().format("%Y-%m-%d").to_string();
                Ok(Value::String(today))
            }

            // System functions
            "system" | "نفذ_أمر" => {
                if args.len() != 1 {
                    return Err(anyhow!("system expects 1 argument"));
                }
                match &args[0] {
                    Value::String(cmd) => match self.system_executor.exec(cmd) {
                        Ok(output) => Ok(Value::String(output.trim().to_string())),
                        Err(e) => Err(anyhow!("System command failed: {}", e)),
                    },
                    _ => Err(anyhow!("system expects a string command")),
                }
            }

            "system_with_io" | "نفذ_أمر_بمدخل" => {
                if args.len() != 2 {
                    return Err(anyhow!("system_with_io expects 2 arguments"));
                }
                match (&args[0], &args[1]) {
                    (Value::String(cmd), Value::String(input)) => {
                        match self.system_executor.exec_with_io(cmd, input) {
                            Ok(output) => Ok(Value::String(output.trim().to_string())),
                            Err(e) => Err(anyhow!("System command failed: {}", e)),
                        }
                    }
                    _ => Err(anyhow!("system_with_io expects string arguments")),
                }
            }

            "read_file" | "اقرأ_ملف" => {
                if args.len() != 1 {
                    return Err(anyhow!("read_file expects 1 argument"));
                }
                match &args[0] {
                    Value::String(path) => match self.system_executor.read_file(path) {
                        Ok(content) => Ok(Value::String(content)),
                        Err(e) => Err(anyhow!("Failed to read file: {}", e)),
                    },
                    _ => Err(anyhow!("read_file expects a string path")),
                }
            }

            "write_file" | "اكتب_ملف" => {
                if args.len() != 2 {
                    return Err(anyhow!("write_file expects 2 arguments"));
                }
                match (&args[0], &args[1]) {
                    (Value::String(path), Value::String(content)) => {
                        match self.system_executor.write_file(path, content) {
                            Ok(success) => Ok(Value::Bool(success)),
                            Err(e) => Err(anyhow!("Failed to write file: {}", e)),
                        }
                    }
                    _ => Err(anyhow!("write_file expects string arguments")),
                }
            }

            "list_files" | "قائمة_ملفات" => {
                if args.len() != 1 {
                    return Err(anyhow!("list_files expects 1 argument"));
                }
                match &args[0] {
                    Value::String(path) => match self.system_executor.list_files(path) {
                        Ok(files) => {
                            let file_values: Vec<Value> =
                                files.into_iter().map(Value::String).collect();
                            Ok(Value::List(file_values))
                        }
                        Err(e) => Err(anyhow!("Failed to list files: {}", e)),
                    },
                    _ => Err(anyhow!("list_files expects a string path")),
                }
            }

            "env_var" | "متغير_بيئة" => {
                if args.len() != 1 {
                    return Err(anyhow!("env_var expects 1 argument"));
                }
                match &args[0] {
                    Value::String(name) => match self.system_executor.get_env_var(name) {
                        Some(value) => Ok(Value::String(value)),
                        None => Ok(Value::Nil),
                    },
                    _ => Err(anyhow!("env_var expects a string name")),
                }
            }

            "system_info" | "معلومات_النظام" => {
                if !args.is_empty() {
                    return Err(anyhow!("system_info expects no arguments"));
                }
                match self.system_executor.system_info() {
                    Ok(info) => {
                        let map_values: HashMap<String, Value> =
                            info.into_iter().map(|(k, v)| (k, Value::String(v))).collect();
                        Ok(Value::Map(map_values))
                    }
                    Err(e) => Err(anyhow!("Failed to get system info: {}", e)),
                }
            }

            _ => Err(anyhow!("Unknown function: {}", name)),
        }
    }
}
