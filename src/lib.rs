pub mod ast;
pub mod error;
pub mod parser;
pub mod schema;
pub mod validation;

pub use ast::*;
pub use error::ParseError;
pub use parser::instance::parse_instance;
pub use parser::schema::parse_schema;
pub use schema::*;
pub use validation::{validate_sources, ValidationError};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_parse_example_schema() {
        let input = include_str!("../examples/meta.eml");
        let result = parse_schema(input);
        assert!(result.is_ok(), "schema parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_example_instance() {
        let input = include_str!("../examples/schema-valid.eml");
        let result = parse_instance(input);
        assert!(result.is_ok(), "instance parse failed: {:?}", result.err());
    }

    #[test]
    fn test_validate_example_files() {
        let schema = parse_schema(include_str!("../examples/meta.eml")).unwrap();
        let instance = parse_instance(include_str!("../examples/schema-valid.eml")).unwrap();
        let errors = validate_sources(&schema, &instance);
        assert_eq!(errors.len(), 0, "expected no error: {:?}", errors);
    }

    #[test]
    fn test_source_validation_pass() {
        let schema = parse_schema(
            r#"
event { _ type }
command {
    fields {_ type}
    @source [fields]
    emits []event
}
"#,
        )
        .unwrap();

        let instance = parse_instance(
            r#"
event UserCreated {
    id string
    name string
}
command CreateUser {
    fields {
        id string
        name string
    }
    emits [UserCreated]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_source_validation_fail_missing() {
        let schema = parse_schema(
            r#"
event { _ type }
command {
    fields {_ type}
    @source [fields]
    emits []event
}
"#,
        )
        .unwrap();

        let instance = parse_instance(
            r#"
event UserCreated {
    id string
    name string
    timestamp int
}
command CreateUser {
    fields {
        id string
        name string
    }
    emits [UserCreated]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("timestamp"));
    }

    #[test]
    fn test_source_validation_generated_ok() {
        let schema = parse_schema(
            r#"
event { _ type }
command {
    fields {_ type}
    @source [fields]
    emits []event
}
"#,
        )
        .unwrap();

        let instance = parse_instance(
            r#"
event UserCreated {
    id string
    timestamp int
}
command CreateUser {
    fields {
        id string
    }
    emits [UserCreated{timestamp int*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }
}
