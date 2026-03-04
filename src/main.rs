use ariadne::{Color, Label, Report, ReportKind, Source};
use eml2::{parse_meta, parse_schema, validate_sources};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <meta.eml> <schema.eml>", args[0]);
        process::exit(1);
    }

    let meta_path = &args[1];
    let schema_path = &args[2];

    let meta_src = fs::read_to_string(meta_path).unwrap_or_else(|e| {
        eprintln!("Failed to read meta: {}", e);
        process::exit(1);
    });

    let schema_src = fs::read_to_string(schema_path).unwrap_or_else(|e| {
        eprintln!("Failed to read schema: {}", e);
        process::exit(1);
    });

    let meta = match parse_meta(&meta_src) {
        Ok(s) => s,
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, meta_path, err.span.start)
                    .with_message(&err.message)
                    .with_label(
                        Label::new((meta_path, err.span.clone()))
                            .with_color(Color::Red)
                            .with_message(&err.message),
                    )
                    .finish()
                    .eprint((meta_path, Source::from(&meta_src)))
                    .unwrap();
            }
            process::exit(1);
        }
    };

    let schema = match parse_schema(&schema_src) {
        Ok(i) => i,
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, schema_path, err.span.start)
                    .with_message(&err.message)
                    .with_label(
                        Label::new((schema_path, err.span.clone()))
                            .with_color(Color::Red)
                            .with_message(&err.message),
                    )
                    .finish()
                    .eprint((schema_path, Source::from(&schema_src)))
                    .unwrap();
            }
            process::exit(1);
        }
    };

    let errors = validate_sources(&meta, &schema);

    if errors.is_empty() {
        println!("OK");
    } else {
        for err in &errors {
            Report::build(ReportKind::Error, schema_path, err.span.start)
                .with_message(&err.message)
                .with_label(
                    Label::new((schema_path, err.span.clone()))
                        .with_color(Color::Red)
                        .with_message(&err.message),
                )
                .finish()
                .eprint((schema_path, Source::from(&schema_src)))
                .unwrap();
        }
        process::exit(1);
    }
}
