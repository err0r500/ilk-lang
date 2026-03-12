use clap::{Parser, Subcommand};
use ilk::error::Diagnostic;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "ilk")]
#[command(about = "ilk/kli compiler and validator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a kli file against an ilk schema
    Check {
        /// Path to the ilk schema file
        schema: PathBuf,
        /// Path to the kli instance file
        instance: PathBuf,
    },
    /// Watch files and re-validate on changes
    Watch {
        /// Path to the ilk schema file
        schema: PathBuf,
        /// Path to the kli instance file
        instance: PathBuf,
    },
    /// Parse a file and dump the AST
    Parse {
        /// Path to the file to parse
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { schema, instance } => {
            run_check(&schema, &instance);
        }
        Commands::Watch { schema, instance } => {
            run_watch(&schema, &instance);
        }
        Commands::Parse { file } => {
            run_parse(&file);
        }
    }
}

fn run_check(schema: &PathBuf, instance: &PathBuf) {
    match ilk::validate_files(schema, instance) {
        Ok(()) => {
            println!("Validation passed");
            std::process::exit(0);
        }
        Err(errors) => {
            print_errors(&errors, schema, instance);
            std::process::exit(1);
        }
    }
}

fn run_watch(schema: &PathBuf, instance: &PathBuf) {
    println!("Watching {} and {}", schema.display(), instance.display());

    // Initial validation
    run_validation(schema, instance);

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
        .watch(schema, RecursiveMode::NonRecursive)
        .expect("Failed to watch schema");
    watcher
        .watch(instance, RecursiveMode::NonRecursive)
        .expect("Failed to watch instance");

    loop {
        match rx.recv() {
            Ok(_) => {
                // Debounce - wait a bit for more events
                std::thread::sleep(Duration::from_millis(100));
                while rx.try_recv().is_ok() {}

                println!("\n--- Re-validating ---");
                run_validation(schema, instance);
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }
}

fn run_validation(schema: &PathBuf, instance: &PathBuf) {
    match ilk::validate_files(schema, instance) {
        Ok(()) => {
            println!("Validation passed");
        }
        Err(errors) => {
            print_errors(&errors, schema, instance);
        }
    }
}

fn run_parse(file: &PathBuf) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "ilk" => match ilk::ilk::parse_ilk(&src, file) {
            Ok(ast) => {
                println!("{:#?}", ast);
            }
            Err(errors) => {
                for err in errors {
                    eprintln!("{}", err.to_report(&src));
                }
                std::process::exit(1);
            }
        },
        "kli" => match ilk::kli::parse_kli(&src, file) {
            Ok(ast) => {
                println!("{:#?}", ast);
            }
            Err(errors) => {
                for err in errors {
                    eprintln!("{}", err.to_report(&src));
                }
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown file extension: {}", ext);
            std::process::exit(1);
        }
    }
}

fn print_errors(errors: &[Diagnostic], schema: &PathBuf, instance: &PathBuf) {
    let ilk_src = std::fs::read_to_string(schema).unwrap_or_default();
    let kli_src = std::fs::read_to_string(instance).unwrap_or_default();

    for err in errors {
        let src = if err.file == *schema {
            &ilk_src
        } else {
            &kli_src
        };
        eprintln!("{}", err.to_report(src));
    }
}
