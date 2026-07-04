use clap::{Parser, Subcommand};
use ilk::error::Diagnostic;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::path::{Path, PathBuf};
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
        /// Emit JSON schema on successful validation
        #[arg(long)]
        emit: bool,
        /// Emit a valid JSON Schema (draft 2020-12) instead of the shape document
        #[arg(long)]
        json_schema: bool,
        /// Pretty-print the emitted JSON output
        #[arg(long)]
        pretty: bool,
        /// Write emitted output to this file (rewritten each cycle) instead of stdout
        #[arg(long, short)]
        output: Option<PathBuf>,
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
        /// Emit a valid JSON Schema (draft 2020-12) instead of the shape document
        #[arg(long)]
        json_schema: bool,
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
        Commands::Watch {
            file,
            emit,
            json_schema,
            pretty,
            output,
        } => {
            run_watch(
                &file,
                cli.json,
                emit,
                json_schema,
                pretty,
                output.as_deref(),
            );
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
        Commands::Emit {
            file,
            json_schema,
            pretty,
        } => {
            run_emit(&file, json_schema, pretty);
        }
    }
}

fn run_format(file: &Path) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    match ilk::parser::parse(&src, file) {
        Ok(ast) => {
            let formatted = ilk::formatter::format(&ast, &src);
            print!("{}", formatted);
        }
        Err(errors) => {
            print_errors(&errors);
            std::process::exit(1);
        }
    }
}

fn run_check(file: &Path, json: bool) {
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
                print_errors(&errors);
            }
            std::process::exit(1);
        }
    }
}

fn run_watch(
    file: &Path,
    json: bool,
    emit: bool,
    json_schema: bool,
    pretty: bool,
    output: Option<&Path>,
) {
    // Status banners are only for plain watch; when emitting to stdout, stdout
    // must stay clean (machine-read JSON). Writing to a file keeps stdout free.
    let banners = !json && (!emit || output.is_some());
    if banners {
        println!("Watching {}", file.display());
    }

    // Initial validation
    run_validation(file, json, emit, json_schema, pretty, output);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                // Only react to .ilk source changes. This ignores writes to the
                // --output file (which lives in the watched dir) that would
                // otherwise trigger an infinite re-validation loop.
                let touches_ilk = event
                    .paths
                    .iter()
                    .any(|p| p.extension().is_some_and(|e| e == "ilk"));
                if touches_ilk {
                    let _ = tx.send(event);
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )
    .expect("Failed to create watcher");

    // Watch the main file
    watcher
        .watch(file, RecursiveMode::NonRecursive)
        .expect("Failed to watch file");

    // Also watch the directory for imported files
    if let Some(dir) = file.parent() {
        let _ = watcher.watch(dir, RecursiveMode::Recursive);
    }

    loop {
        match rx.recv() {
            Ok(_) => {
                // Debounce - wait a bit for more events
                std::thread::sleep(Duration::from_millis(100));
                while rx.try_recv().is_ok() {}

                let start = std::time::Instant::now();
                if banners {
                    let now = chrono::Local::now();
                    println!("\n--- Re-validating at {} ---", now.format("%H:%M:%S"));
                }
                run_validation(file, json, emit, json_schema, pretty, output);
                if banners {
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

fn run_validation(
    file: &Path,
    json: bool,
    emit: bool,
    json_schema: bool,
    pretty: bool,
    out_file: Option<&Path>,
) {
    let canonical = match file.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Cannot resolve path: {}", e);
            return;
        }
    };
    let mut compiler = ilk::Compiler::new();

    match compiler.load_file(&canonical) {
        Ok(_) => {
            if let Err(errors) = compiler.validate(&canonical) {
                if json {
                    JsonOutput::error(errors).print();
                } else {
                    print_errors(&errors);
                }
                return;
            }
            if emit {
                let ast = compiler.get_file(&canonical).unwrap();
                let env = compiler.get_env(&canonical).unwrap();
                let output = if json_schema {
                    ilk::emit_jsonschema::emit_json_schema(ast, env)
                } else {
                    ilk::emit_schema::emit_schema(ast, env)
                };
                let json_str = if pretty {
                    serde_json::to_string_pretty(&output).unwrap()
                } else {
                    serde_json::to_string(&output).unwrap()
                };
                if let Some(out_path) = out_file {
                    // Truncate and rewrite each cycle so the file holds only the
                    // latest schema (a shell `>` redirect truncates only once).
                    if let Err(e) = std::fs::write(out_path, format!("{}\n", json_str)) {
                        eprintln!("Failed to write {}: {}", out_path.display(), e);
                    }
                } else {
                    println!("{}", json_str);
                }
            } else if json {
                JsonOutput::success().print();
            } else {
                println!("Validation passed");
            }
        }
        Err(errors) => {
            if json {
                JsonOutput::error(errors).print();
            } else {
                print_errors(&errors);
            }
        }
    }
}

fn run_parse(file: &Path, json: bool) {
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
                print_errors(&errors);
            }
            std::process::exit(1);
        }
    }
}

fn run_json(file: &Path, pretty: bool) {
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
            print_errors(&errors);
            std::process::exit(1);
        }
    }
}

fn run_emit(file: &Path, json_schema: bool, pretty: bool) {
    let canonical = file.canonicalize().expect("Cannot resolve path");
    let mut compiler = ilk::Compiler::new();

    if let Err(errors) = compiler.load_file(&canonical) {
        print_errors(&errors);
        std::process::exit(1);
    }

    if let Err(errors) = compiler.validate(&canonical) {
        print_errors(&errors);
        std::process::exit(1);
    }

    let ast = compiler.get_file(&canonical).unwrap();
    let env = compiler.get_env(&canonical).unwrap();

    let output = if json_schema {
        ilk::emit_jsonschema::emit_json_schema(ast, env)
    } else {
        ilk::emit_schema::emit_schema(ast, env)
    };
    let json_str = if pretty {
        serde_json::to_string_pretty(&output).unwrap()
    } else {
        serde_json::to_string(&output).unwrap()
    };
    println!("{}", json_str);
}

fn print_errors(errors: &[Diagnostic]) {
    for err in errors {
        let src = std::fs::read_to_string(&err.file).unwrap_or_default();
        eprintln!("{}", err.to_report(&src));
    }
}
