# ILK Compiler Source

Data modeling language compiler: validates both type declarations and runtime instances.

## Pipeline

```
Source → Parse → Resolve → Validate → Output
```

## Files

| File | Purpose |
|------|---------|
| `main.rs` | CLI: check, watch, parse, json commands |
| `lib.rs` | Compiler API, orchestrates pipeline |
| `parser.rs` | chumsky-based parser → AST |
| `ast.rs` | Type declarations + instance values |
| `resolve.rs` | Symbol table, type refs, cycle detection |
| `span.rs` | Source location tracking |
| `error.rs` | Diagnostic infrastructure |
| `validate/` | 3-phase validation |

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
```

## See Also

- [ARCHITECTURE.md](./ARCHITECTURE.md) - detailed architecture docs
- [../CLAUDE.md](../CLAUDE.md) - language reference
