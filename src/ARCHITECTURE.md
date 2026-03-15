# ILK Compiler Architecture

## Overview

Single-pass compiler for data modeling.
Unified AST handles both type declarations and instances (merged from old ilk + kli).

## Pipeline Detail

```
                    ┌─ PARSE (parser.rs)
                    │   chumsky combinators → AST
                    │   Output: File { TypeDecl, Instance, Import }
                    ↓
                    ├─ RESOLVE (resolve.rs)
                    │   1. Collect types into symbol table
                    │   2. Collect instances, track @main
                    │   3. Validate type refs (no undefined)
                    │   4. Detect cycles via DFS
                    │   Output: TypeEnv (symbol table)
                    ↓
            ┌─ VALIDATE (validate/)
            │
            ├─ STRUCTURAL - type conformance
            │   TypeRef vs Base, literals, structs, lists
            │   Cardinality, references, unions, intersections
            │
            ├─ SOURCE - @source paths
            │   Validates field origin chains exist
            │
            └─ CONSTRAINTS - @constraint exprs
                Boolean algebra + collection ops
                forall, exists, unique, count, etc.
```

## Module Details

### ast.rs

**Type-Level** (declarations):
- `BaseType` - Uuid, String, Int, Float, Bool, Date, Timestamp, Money, Wildcard
- `TypeExpr` - recursive: Base, Named, Ref(&T), List, Struct, Union, Intersection, Concrete<T>
- `Field` - name, optional?, type, annotations
- `StructKind` - Closed, Open{...}, Anonymous{_}
- `Annotation` - @main, @assoc, @source, @out, @constraint, @doc
- `ConstraintExpr` - boolean exprs, field access, collection ops

**Instance-Level** (runtime):
- `Value` - TypeRef, literals, Struct, List, Variant, BindingRef
- `InstanceField` - field + origin tracking (Generated/Mapped/Computed)
- `ListElement` - Value, BindingRef, Refinement

**Top-Level**:
- `TypeDecl` - type X = TypeExpr
- `Instance` - x = Type<assocs> Value
- `Import` - import "path" [as alias]
- `File` - collection with helper iterators

### parser.rs

Layers (bottom-up):
1. **Primitives**: ident, ws, sep
2. **Type parsers**: base_type, literal types, concrete, reference, cardinality, list, struct, constraint_expr, type_expr (combines with | and &)
3. **Value parsers**: literals, type_ref, binding_ref, field_origin, instance_field/struct/list, value
4. **Top-level**: type_decl, instance, import, file

### resolve.rs

`TypeEnv` = symbol table:
- `types: HashMap<String, S<TypeDecl>>`
- `instances: HashMap<String, S<Instance>>`
- `main_instance: Option<String>`

Phases:
1. Collect types (error on dups)
2. Collect instances (track @main, error on dups/multiple @main)
3. Validate type refs exist
4. Check cycles (DFS with visited/in_stack)

### validate/structural.rs

`validate_value_against_type()` - recursive matching:
- TypeRef vs Base (String → String)
- Literal vs Concrete
- Schema-fixed equality
- Struct: field names, counts, nested
- List: cardinality + elements
- References: BindingRef name resolution
- Unions: try each variant
- Intersections: both sides (special for open structs)

### validate/source.rs

Validates @source annotation paths:
- @source[fields] → check 'fields' exists in struct
- Follows nested struct chains
- Respects FieldOrigin (Generated skips check)

### validate/constraint.rs

`EvalValue` - runtime during eval: Bool, Int, String, List, Set, Struct, BindingRef

Constraint ops:
- Boolean: true, false, &&, ||, !
- Comparison: ==, !=, <, <=, >, >=, in
- Collections: forall, exists, unique, count
- Access: x.y.z, e.assoc(type)
- String: templateVars(str), keys(struct)

## Key Patterns

1. **Spanned AST** - every node carries source span via `S<T>`
2. **Two-level types** - declarations vs instances
3. **Symbol table** - TypeEnv as immutable ref to validators
4. **Diagnostic accumulation** - collect all errors, no early exit
5. **Parser combinators** - chumsky for composable parsing
6. **Annotation-driven** - @main, @source, @constraint modulate behavior

## Error Flow

| Phase | Example | Recovery |
|-------|---------|----------|
| Parse | Unexpected token | Fail fast |
| Resolve | Unknown type | Fail |
| Validate | Type mismatch | Accumulate |

All errors become `Diagnostic { span, message, severity, path }` → rendered via `ariadne`.

## Extending

- **New syntax**: parser.rs (parser) + ast.rs (AST node)
- **New validation**: validate/ module + call from lib.rs
- **New constraint op**: ConstraintExpr variant + constraint.rs handler
- **New annotation**: ast.rs Annotation variant + relevant validator
