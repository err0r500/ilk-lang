# ILK Compiler Source

Data modeling language compiler: validates both type declarations and runtime instances.

## Pipeline

```
Source → Parse → Resolve → Validate → Output
```

## Files

| File | Purpose |
|------|---------|
| `main.rs` | CLI: check, watch, parse, json, format, emit, lsp commands |
| `lib.rs` | Compiler API, orchestrates pipeline, import loading |
| `parser/` | chumsky-based parser → AST (`common`, `types`, `values`, `items`) |
| `ast.rs` | Type declarations + instance values |
| `resolve.rs` | Symbol table, type refs, cycle detection |
| `span.rs` | Source location tracking |
| `error.rs` | Diagnostic infrastructure |
| `emit_schema.rs` | `@main` instances → shape document (type-name leaves) |
| `emit_jsonschema.rs` | `@main` instances → valid JSON Schema (draft 2020-12) |
| `formatter.rs` | Canonical source formatting (`ilk format`) |
| `validate/` | 3-phase validation |
| `lsp/` | Language server (diagnostics, hover, completions, goto-def) |
| `wasm.rs` | WASM bindings for the playground (`wasm` feature) |

## Validation Phases

1. **structural.rs** - type conformance (values match types)
2. **source.rs** - @source annotation path resolution
3. **constraint.rs** - @constraint expression evaluation

## Quick Start

```bash
cargo run -- check file.ilk      # validate
cargo run -- watch file.ilk      # continuous validation
cargo run -- parse file.ilk      # debug AST
cargo run -- json file.ilk       # AST as JSON
cargo run -- format file.ilk     # canonical formatting
cargo run -- emit file.ilk       # @main instances → shape document
cargo run -- emit --json-schema file.ilk   # @main instances → JSON Schema (draft 2020-12)
```

## See Also

- [ARCHITECTURE.md](./ARCHITECTURE.md) - detailed architecture docs
- [../CLAUDE.md](../CLAUDE.md) - language reference
