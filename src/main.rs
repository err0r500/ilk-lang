use ariadne::{Color, Label, Report, ReportKind, Source};
use eml2::{parse_instance, parse_schema, validate_sources};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <schema.eml> <instance.eml>", args[0]);
        process::exit(1);
    }

    let schema_path = &args[1];
    let instance_path = &args[2];

    let schema_src = fs::read_to_string(schema_path).unwrap_or_else(|e| {
        eprintln!("Failed to read schema: {}", e);
        process::exit(1);
    });

    let instance_src = fs::read_to_string(instance_path).unwrap_or_else(|e| {
        eprintln!("Failed to read instance: {}", e);
        process::exit(1);
    });

    let schema = match parse_schema(&schema_src) {
        Ok(s) => s,
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

    let instance = match parse_instance(&instance_src) {
        Ok(i) => i,
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, instance_path, err.span.start)
                    .with_message(&err.message)
                    .with_label(
                        Label::new((instance_path, err.span.clone()))
                            .with_color(Color::Red)
                            .with_message(&err.message),
                    )
                    .finish()
                    .eprint((instance_path, Source::from(&instance_src)))
                    .unwrap();
            }
            process::exit(1);
        }
    };

    let errors = validate_sources(&schema, &instance);

    if errors.is_empty() {
        println!("OK");
    } else {
        for err in &errors {
            Report::build(ReportKind::Error, instance_path, err.span.start)
                .with_message(&err.message)
                .with_label(
                    Label::new((instance_path, err.span.clone()))
                        .with_color(Color::Red)
                        .with_message(&err.message),
                )
                .finish()
                .eprint((instance_path, Source::from(&instance_src)))
                .unwrap();
        }
        process::exit(1);
    }
}
