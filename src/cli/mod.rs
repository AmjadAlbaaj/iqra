use crate::lang::runtime::Runtime;
use anyhow::Result;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::fs;

pub fn run_repl() -> Result<()> {
    println!("مرحباً بك في اقرأ - Welcome to Iqra");
    println!("اكتب 'خروج' أو 'exit' للخروج - Type 'خروج' or 'exit' to quit");
    println!("اكتب 'خطوة' أو 'step' لتفعيل التنفيذ التفاعلي - Type 'خطوة' or 'step' for interactive step-by-step mode");

    let mut rl = Editor::<(), DefaultHistory>::new()?;
    let mut runtime = Runtime::new();
    let mut step_mode = false;
    let mut step_lines: Vec<String> = Vec::new();
    let mut step_index = 0;

    loop {
        let prompt = if step_mode {
            "اقرأ (خطوة)> "
        } else {
            "اقرأ> "
        };
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Handle exit commands in both Arabic and English
                if line == "خروج" || line == "exit" || line == "quit" || line == "إنهاء" {
                    println!("وداعاً! - Goodbye!");
                    break;
                }

                // Step mode activation
                if !step_mode && (line == "خطوة" || line == "step") {
                    println!("تم تفعيل وضع التنفيذ التفاعلي خطوة بخطوة!\nأدخل البرنامج كاملاً، ثم استخدم 'التالي' أو 'next' للتنفيذ خطوة خطوة.");
                    println!("اكتب 'إعادة' أو 'restart' لإعادة التنفيذ، 'إنهاء' أو 'exit' للخروج من الوضع.");
                    let program = rl.readline("أدخل البرنامج:")?;
                    step_lines = program
                        .lines()
                        .map(|l| l.trim().to_string())
                        .filter(|l| !l.is_empty())
                        .collect();
                    step_index = 0;
                    step_mode = true;
                    continue;
                }

                if step_mode {
                    if line == "التالي" || line == "next" {
                        if step_index < step_lines.len() {
                            let current_line = &step_lines[step_index];
                            println!("السطر الحالي [{}]: {}", step_index + 1, current_line);
                            match runtime.execute(current_line) {
                                Ok(result) => {
                                    println!("الناتج: {}", result);
                                }
                                Err(e) => {
                                    eprintln!("خطأ - Error: {}", e);
                                }
                            }
                            println!("المتغيرات الحالية:");
                            for (k, v) in runtime.get_variables() {
                                println!("{} = {}", k, v);
                            }
                            step_index += 1;
                            if step_index == step_lines.len() {
                                println!("تم تنفيذ جميع الأسطر!\nAll lines executed!");
                                step_mode = false;
                            }
                        } else {
                            println!("لا توجد أسطر متبقية - No more lines.");
                            step_mode = false;
                        }
                        continue;
                    }
                    if line == "إعادة" || line == "restart" {
                        runtime = Runtime::new();
                        step_index = 0;
                        println!("تمت إعادة التنفيذ - Execution restarted.");
                        continue;
                    }
                    if line == "إنهاء" || line == "exit" {
                        step_mode = false;
                        println!("تم الخروج من وضع التنفيذ التفاعلي - Exited step mode.");
                        continue;
                    }
                    println!("استخدم 'التالي' أو 'next' للتنفيذ، 'إعادة' أو 'restart' لإعادة التنفيذ، 'إنهاء' أو 'exit' للخروج.");
                    continue;
                }

                let _ = rl.add_history_entry(line);

                match runtime.execute(line) {
                    Ok(result) => {
                        if !result.is_nil() {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("خطأ - Error: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("تم المقاطعة - Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("وداعاً! - Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("خطأ في القراءة - Read error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let mut runtime = Runtime::new();
    let result = runtime.execute(&content)?;

    if !result.is_nil() {
        println!("{}", result);
    }

    Ok(())
}
