use clap::{Parser, Subcommand};
use ilk::error::Diagnostic;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "ilk")]
#[command(about = "ilk compiler and validator")]
struct Cli {
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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            run_check(&file);
        }
        Commands::Watch { file } => {
            run_watch(&file);
        }
        Commands::Parse { file } => {
            run_parse(&file);
        }
    }
}

fn run_check(file: &PathBuf) {
    match ilk::validate_file(file) {
        Ok(()) => {
            println!("Validation passed");
            std::process::exit(0);
        }
        Err(errors) => {
            print_errors(&errors, file);
            std::process::exit(1);
        }
    }
}

fn run_watch(file: &PathBuf) {
    println!("Watching {}", file.display());

    // Initial validation
    run_validation(file);

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

                println!("\n--- Re-validating ---");
                run_validation(file);
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }
}

fn run_validation(file: &PathBuf) {
    match ilk::validate_file(file) {
        Ok(()) => {
            println!("Validation passed");
        }
        Err(errors) => {
            print_errors(&errors, file);
        }
    }
}

fn run_parse(file: &PathBuf) {
    let src = std::fs::read_to_string(file).expect("Failed to read file");

    match ilk::parser::parse(&src, file) {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err.to_report(&src));
            }
            std::process::exit(1);
        }
    }
}

fn print_errors(errors: &[Diagnostic], file: &PathBuf) {
    let src = std::fs::read_to_string(file).unwrap_or_default();

    for err in errors {
        eprintln!("{}", err.to_report(&src));
    }
}
