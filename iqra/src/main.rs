use clap::{Parser, Subcommand};
use std::io::IsTerminal;
use tracing::{debug, info};

/// Simple CLI for iqra project
#[derive(Parser, Debug)]
#[command(author, version, about = "Iqra CLI", long_about = None)]
struct Cli {
    /// Override log level (e.g. info, debug, trace)
    #[arg(long, env = "IQRA_LOG", default_value = "info", help = "Log level (info|debug|trace)")]
    log_level: String,

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

#[derive(Subcommand, Debug)]
enum Commands {
    /// اطبع تحية باسم محدد
    Greet {
        /// Name to greet
        #[arg(short, long, default_value = "world")]
        name: String,
    },
    /// اعرض معلومات النسخة
    Version,
    /// شغّل برنامج نصي بلغة اقرأ من سلسلة مباشرة
    Run {
        #[arg(short, long, help = "Source code inline")]
        code: Option<String>,
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
    /// REPL تفاعلي للغة اقرأ
    Repl,
}

fn main() {
    let cli = Cli::parse();
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(cli.log_level.as_str())
        .with_target(false)
        .compact()
        .init();

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
        Commands::Run { code, json } => {
            use iqra::diagnostics::{error_as_json, render_error_with_opts};
            use iqra::lang::{Runtime, lex, parse};
            let colorize = match cli.color.as_str() {
                "always" => true,
                "never" => false,
                _ => std::io::stderr().is_terminal(),
            };
            let Some(source) = code else {
                eprintln!("الخيار --code مطلوب حالياً");
                std::process::exit(1);
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
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    if cli.error_json {
                        eprintln!("{}", error_as_json(&source, &e));
                    } else {
                        eprintln!("{}", render_error_with_opts(&source, &e, colorize));
                    }
                    std::process::exit(1);
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
            println!("اكتب :help للمساعدة، :q للخروج");
            while let Ok(line) = rl.readline(">> ") {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed == ":q" || trimmed == ":quit" {
                    break;
                }
                if trimmed == ":help" {
                    println!(
                        "{}",
                        iqra::diagnostics::render_warning(
                            ":q أو :quit للخروج | :help للمساعدة",
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
