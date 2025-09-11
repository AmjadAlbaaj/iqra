use crate::lang::runtime::Runtime;
use anyhow::Result;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::fs;

pub fn run_repl() -> Result<()> {
    println!("مرحباً بك في اقرأ - Welcome to Iqra");
    println!("اكتب 'خروج' أو 'exit' للخروج - Type 'خروج' or 'exit' to quit");

    let mut rl = Editor::<(), DefaultHistory>::new()?;
    let mut runtime = Runtime::new();

    loop {
        let readline = rl.readline("اقرأ> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Handle exit commands in both Arabic and English
                if line == "خروج" || line == "exit" || line == "quit" {
                    println!("وداعاً! - Goodbye!");
                    break;
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
