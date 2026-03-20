use clap::{Parser, Subcommand};
use ilk::error::Diagnostic;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "ilk")]
#[command(about = "ilk compiler and validator")]
struct Cli {
    /// Output in JSON format for tooling integration
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an ilk file
    Check {
        /// Path to the ilk file
        file: PathBuf,
    },
    /// Watch file and re-validate on changes
    Watch {
        /// Path to the ilk file
        file: PathBuf,
    },
    /// Parse a file and dump the AST
    Parse {
        /// Path to the file to parse
        file: PathBuf,
    },
    /// Output the compiled AST as JSON
    Json {
        /// Path to the ilk file
        file: PathBuf,
        /// Pretty-print the JSON output
        #[arg(long)]
        pretty: bool,
    },
    /// Start LSP server (stdio)
    Lsp,
    /// Format an ilk file
    Format {
        /// Path to the ilk file
        file: PathBuf,
    },
    /// Emit types and @main instances as JSON
    Emit {
        /// Path to the ilk file
        file: PathBuf,
        /// Pretty-print the JSON output
        #[arg(long)]
        pretty: bool,
    },
}

#[derive(Serialize)]
struct JsonOutput {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    diagnostics: Vec<Diagnostic>,
}

impl JsonOutput {
    fn success() -> Self {
        Self {
            success: true,
            message: Some("Validation passed".to_string()),
            diagnostics: Vec::new(),
        }
    }

    fn error(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            success: false,
            message: None,
            diagnostics,
        }
    }

    fn print(&self) {
        println!("{}", serde_json::to_string(self).unwrap());
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            run_check(&file, cli.json);
        }
        Commands::Watch { file } => {
            run_watch(&file, cli.json);
        }
        Commands::Parse { file } => {
            run_parse(&file, cli.json);
        }
        Commands::Json { file, pretty } => {
            run_json(&file, pretty);
        }
        Commands::Lsp => {
            tokio::runtime::Runtime::new()
                .expect("Failed to create tokio runtime")
                .block_on(ilk::lsp::run());
        }
        Commands::Format { file } => {
            run_format(&file);
        }
        Commands::Emit { file, pretty } => {
            run_emit(&file, pretty);
        }
    }
}

fn run_format(file: &PathBuf) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    match ilk::parser::parse(&src, file) {
        Ok(ast) => {
            let formatted = ilk::formatter::format(&ast, &src);
            print!("{}", formatted);
        }
        Err(errors) => {
            print_errors(&errors, file);
            std::process::exit(1);
        }
    }
}

fn run_check(file: &PathBuf, json: bool) {
    match ilk::validate_file(file) {
        Ok(()) => {
            if json {
                JsonOutput::success().print();
            } else {
                println!("Validation passed");
            }
            std::process::exit(0);
        }
        Err(errors) => {
            if json {
                JsonOutput::error(errors).print();
            } else {
                print_errors(&errors, file);
            }
            std::process::exit(1);
        }
    }
}

fn run_watch(file: &PathBuf, json: bool) {
    if !json {
        println!("Watching {}", file.display());
    }

    // Initial validation
    run_validation(file, json);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )
    .expect("Failed to create watcher");

    watcher
        .watch(file, RecursiveMode::NonRecursive)
        .expect("Failed to watch file");

    loop {
        match rx.recv() {
            Ok(_) => {
                // Debounce - wait a bit for more events
                std::thread::sleep(Duration::from_millis(100));
                while rx.try_recv().is_ok() {}

                let start = std::time::Instant::now();
                if !json {
                    let now = chrono::Local::now();
                    println!("\n--- Re-validating at {} ---", now.format("%H:%M:%S"));
                }
                run_validation(file, json);
                if !json {
                    println!("Completed in {:?}", start.elapsed());
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }
}

fn run_validation(file: &PathBuf, json: bool) {
    match ilk::validate_file(file) {
        Ok(()) => {
            if json {
                JsonOutput::success().print();
            } else {
                println!("Validation passed");
            }
        }
        Err(errors) => {
            if json {
                JsonOutput::error(errors).print();
            } else {
                print_errors(&errors, file);
            }
        }
    }
}

fn run_parse(file: &PathBuf, json: bool) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    match ilk::parser::parse(&src, file) {
        Ok(ast) => {
            if json {
                // For parse, just output success with the debug AST as message
                println!(
                    "{}",
                    serde_json::json!({
                        "success": true,
                        "ast": format!("{:#?}", ast)
                    })
                );
            } else {
                println!("{:#?}", ast);
            }
        }
        Err(errors) => {
            if json {
                JsonOutput::error(errors).print();
            } else {
                for err in errors {
                    eprintln!("{}", err.to_report(&src));
                }
            }
            std::process::exit(1);
        }
    }
}

fn run_json(file: &PathBuf, pretty: bool) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    match ilk::parse(&src, file) {
        Ok(ast) => {
            let output = if pretty {
                serde_json::to_string_pretty(&ast).unwrap()
            } else {
                serde_json::to_string(&ast).unwrap()
            };
            println!("{}", output);
        }
        Err(errors) => {
            for err in &errors {
                eprintln!("{}", err.to_report(&src));
            }
            std::process::exit(1);
        }
    }
}

fn run_emit(file: &PathBuf, pretty: bool) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    let ast = match ilk::parser::parse(&src, file) {
        Ok(ast) => ast,
        Err(errors) => {
            print_errors(&errors, file);
            std::process::exit(1);
        }
    };

    let env = match ilk::resolve::resolve(&ast, file) {
        Ok(env) => env,
        Err(errors) => {
            print_errors(&errors, file);
            std::process::exit(1);
        }
    };

    let output = ilk::emit_schema::emit_schema(&ast, &env);
    let json_str = if pretty {
        serde_json::to_string_pretty(&output).unwrap()
    } else {
        serde_json::to_string(&output).unwrap()
    };
    println!("{}", json_str);
}

fn print_errors(errors: &[Diagnostic], file: &PathBuf) {
    let src = std::fs::read_to_string(file).unwrap_or_default();

    for err in errors {
        eprintln!("{}", err.to_report(&src));
    }
}
