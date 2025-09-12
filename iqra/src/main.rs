use clap::{Parser, Subcommand, ValueEnum};
use std::io::IsTerminal;
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;

/// Simple CLI for iqra project
#[derive(Parser, Debug)]
#[command(author, version, about = "Iqra CLI", long_about = None)]
struct Cli {
    /// Override log level (e.g. info, debug, trace)
    #[arg(long, env = "IQRA_LOG", default_value = "info", help = "Log level (info|debug|trace)")]
    log_level: String,

    /// Log format (text or json)
    #[arg(long, env = "IQRA_LOG_FORMAT", value_enum, default_value_t = LogFormat::Text, help = "Log format (text|json)")]
    log_format: LogFormat,

    /// Quiet mode (overrides log level to error)
    #[arg(long, global = true, help = "الوضع الهادئ (أخطاء فقط)")]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
    /// Output errors as JSON (for non-REPL run mode)
    #[arg(long, global = true, help = "صيغة أخطاء JSON")]
    error_json: bool,
    /// التحكم بالألوان (auto|always|never)
    #[arg(
        long,
        global = true,
        default_value = "auto",
        help = "التحكم بالألوان (auto|always|never)"
    )]
    color: String,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// اطبع تحية باسم محدد
    #[command(visible_alias = "حيي")]
    Greet {
        /// Name to greet
        #[arg(short, long, default_value = "world")]
        name: String,
    },
    /// اعرض معلومات النسخة
    #[command(visible_alias = "نسخة")]
    Version,
    /// شغّل برنامج نصي بلغة اقرأ من سلسلة مباشرة
    #[command(visible_alias = "شغل")]
    Run {
        #[arg(short, long, visible_alias = "كود", help = "Source code inline")]
        code: Option<String>,
        /// Execute from a file path
        #[arg(long, visible_alias = "ملف", help = "Execute from file path")]
        file: Option<String>,
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
    /// تحقق من صحة الشيفرة (تحليل لغوي/نحوي فقط)
    #[command(visible_alias = "تحقق")]
    Check {
        #[arg(short, long, visible_alias = "كود", help = "Source code inline")]
        code: Option<String>,
        #[arg(long, visible_alias = "ملف", help = "Check from file path")]
        file: Option<String>,
        #[arg(long, help = "Output JSON result")]
        json: bool,
    },
    /// REPL تفاعلي للغة اقرأ
    #[command(visible_alias = "تفاعلي")]
    Repl,
}

fn main() {
    let cli = Cli::parse();
    init_windows_console();
    init_tracing(&cli);

    match cli.command.unwrap_or(Commands::Greet { name: "world".into() }) {
        Commands::Greet { name } => {
            debug!(user_name = %name, "Generating greeting");
            let greet = iqra::make_greeting(&name);
            println!("{greet}");
            info!("Done");
        }
        Commands::Version => {
            println!("iqra version {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Run { code, file, json } => {
            use iqra::diagnostics::{error_as_json, render_error_with_opts};
            use iqra::lang::{Runtime, lex, parse};
            let colorize = match cli.color.as_str() {
                "always" => true,
                "never" => false,
                _ => std::io::stderr().is_terminal(),
            };
            let source = if let Some(c) = code {
                c
            } else if let Some(path) = file {
                match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        exit_with_error_text(&format!("تعذر قراءة الملف '{}': {}", path, e), 2);
                        unreachable!();
                    }
                }
            } else {
                // Fallback: read entire stdin
                use std::io::{self, Read};
                let mut buf = String::new();
                if let Err(e) = io::stdin().read_to_string(&mut buf) {
                    exit_with_error_text(&format!("تعذر قراءة الإدخال القياسي: {}", e), 2);
                }
                if buf.trim().is_empty() {
                    exit_with_error_text("يرجى تمرير --code أو --file أو تزويد stdin", 2);
                }
                buf
            };
            match lex(&source).and_then(|t| parse(&t).map(|stmts| (t, stmts))) {
                Ok((_toks, stmts)) => {
                    let mut rt = Runtime::new();
                    match rt.exec(&stmts) {
                        Ok(exec_out) => {
                            if json {
                                println!("{}", serde_json::to_string_pretty(&exec_out).unwrap());
                            } else {
                                for v in exec_out.printed {
                                    if colorize {
                                        println!(
                                            "{}",
                                            iqra::diagnostics::render_success(
                                                &format!("{v}"),
                                                true
                                            )
                                        );
                                    } else {
                                        println!("{v}");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if cli.error_json {
                                eprintln!("{}", error_as_json(&source, &e));
                            } else {
                                eprintln!("{}", render_error_with_opts(&source, &e, colorize));
                            }
                            std::process::exit(4);
                        }
                    }
                }
                Err(e) => {
                    if cli.error_json {
                        eprintln!("{}", error_as_json(&source, &e));
                    } else {
                        eprintln!("{}", render_error_with_opts(&source, &e, colorize));
                    }
                    std::process::exit(3);
                }
            }
        }
        Commands::Check { code, file, json } => {
            use iqra::diagnostics::{error_as_json, render_error_with_opts};
            use iqra::lang::{lex, parse};
            let colorize = match cli.color.as_str() {
                "always" => true,
                "never" => false,
                _ => std::io::stderr().is_terminal(),
            };
            let source = if let Some(c) = code {
                c
            } else if let Some(path) = file {
                match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        exit_with_error_text(&format!("تعذر قراءة الملف '{}': {}", path, e), 2);
                        unreachable!();
                    }
                }
            } else {
                exit_with_error_text("يرجى تمرير --code أو --file", 2);
                unreachable!();
            };
            match lex(&source).and_then(|t| parse(&t).map(|_stmts| ())) {
                Ok(()) => {
                    if json {
                        println!("{}", serde_json::json!({"status":"ok"}));
                    } else {
                        println!("تم التحقق بنجاح");
                    }
                }
                Err(e) => {
                    if cli.error_json {
                        eprintln!("{}", error_as_json(&source, &e));
                    } else {
                        eprintln!("{}", render_error_with_opts(&source, &e, colorize));
                    }
                    std::process::exit(3);
                }
            }
        }
        Commands::Repl => {
            use iqra::diagnostics::render_error_with_opts;
            use iqra::lang::keywords::KEYWORDS;
            use iqra::lang::{Runtime, lex, parse};
            use rustyline::Editor;
            use rustyline::completion::{Completer, Pair};
            use rustyline::highlight::Highlighter;
            use rustyline::hint::Hinter;
            use rustyline::history::FileHistory;
            use rustyline::validate::{ValidationContext, ValidationResult, Validator};
            use rustyline::{Context, Helper};
            #[derive(Default)]
            struct IqraCompleter;
            impl Completer for IqraCompleter {
                type Candidate = Pair;
                fn complete(
                    &self,
                    line: &str,
                    pos: usize,
                    _ctx: &Context<'_>,
                ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
                    let start = line[..pos].rfind(|c: char| c.is_whitespace()).map_or(0, |i| i + 1);
                    let word = &line[start..pos];
                    let mut pairs = vec![];
                    for &kw in KEYWORDS {
                        if kw.starts_with(word) {
                            pairs.push(Pair {
                                display: kw.to_string(),
                                replacement: kw.to_string(),
                            });
                        }
                    }
                    Ok((start, pairs))
                }
            }
            impl Helper for IqraCompleter {}
            impl Hinter for IqraCompleter {
                type Hint = String;
                fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
                    None
                }
            }
            impl Highlighter for IqraCompleter {}
            impl Validator for IqraCompleter {
                fn validate(
                    &self,
                    _ctx: &mut ValidationContext,
                ) -> rustyline::Result<ValidationResult> {
                    Ok(ValidationResult::Valid(None))
                }
            }
            let colorize = std::io::stdout().is_terminal();
            let mut rl = Editor::<IqraCompleter, FileHistory>::new().expect("repl");
            rl.set_helper(Some(IqraCompleter));
            let mut rt = Runtime::new();
            println!("اكتب :help أو :مساعدة للمساعدة، :q أو :خروج للخروج");
            while let Ok(line) = rl.readline(">> ") {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed == ":q" || trimmed == ":quit" || trimmed == ":خروج" {
                    break;
                }
                if trimmed == ":help" || trimmed == ":مساعدة" {
                    println!(
                        "{}",
                        iqra::diagnostics::render_warning(
                            ":q أو :quit أو :خروج للخروج | :help أو :مساعدة للمساعدة",
                            colorize
                        )
                    );
                    continue;
                }
                rl.add_history_entry(trimmed).ok();
                match lex(trimmed).and_then(|t| parse(&t).map(|stmts| (t, stmts))) {
                    Ok((_toks, stmts)) => match rt.exec(&stmts) {
                        Ok(out) => {
                            for v in out.printed {
                                if colorize {
                                    println!(
                                        "{}",
                                        iqra::diagnostics::render_success(&format!("{v}"), true)
                                    );
                                } else {
                                    println!("{v}");
                                }
                            }
                        }
                        Err(e) => {
                            println!("{}", render_error_with_opts(trimmed, &e, colorize));
                        }
                    },
                    Err(e) => {
                        println!("{}", render_error_with_opts(trimmed, &e, colorize));
                    }
                }
            }
        }
    }
}

fn init_tracing(cli: &Cli) {
    let level = if cli.quiet { "error" } else { cli.log_level.as_str() };
    let filter = EnvFilter::new(level);
    let fmt = tracing_subscriber::fmt().with_env_filter(filter).with_target(false);
    match cli.log_format {
        LogFormat::Text => fmt.compact().init(),
        LogFormat::Json => fmt.json().init(),
    }
}

fn exit_with_error_text(msg: &str, code: i32) {
    eprintln!("{} (ERRCODE={})", msg, code);
    std::process::exit(code);
}

#[cfg(windows)]
fn init_windows_console() {
    use windows_sys::Win32::System::Console::{
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_ERROR_HANDLE,
        STD_OUTPUT_HANDLE, SetConsoleCP, SetConsoleMode, SetConsoleOutputCP,
    };
    unsafe {
        // Prefer UTF-8 code page for I/O
        let _ = SetConsoleOutputCP(65001);
        let _ = SetConsoleCP(65001);
        // Enable ANSI escape sequences for colors on stdout and stderr
        for handle_kind in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
            let h = GetStdHandle(handle_kind);
            if !h.is_null() {
                let mut mode: u32 = 0;
                if GetConsoleMode(h, &mut mode) != 0 {
                    let _ = SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
                }
            }
        }
    }
}

#[cfg(not(windows))]
fn init_windows_console() {}
