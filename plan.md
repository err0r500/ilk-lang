# Plan: Merge ilk/kli into Single Language

## Goal

Collapse two-level system (.ilk schema + .kli instance) into one language with type declarations and instance bindings.

## New Syntax

// Type declarations (formerly ilk)
type Event = {...} & {timestamp Int}
type Tag = Parametrized | Unique
type Parametrized = {_ String}
type Unique = Concrete<String>

@assoc [Tag]
type Event = {...} & {timestamp Int}

type Command = {
    fields {...}

    @source [fields]
    emits []Event
}

// Instances (formerly kli)
userIdTag = Parametrized {userId String}
simpleTag = Unique "simple-tag"

userRegistered = Event<userIdTag> {
    id String
    name String
}

registerUser = Command {
    fields { id String, name String }
    emits [userRegistered & { timestamp Int* }]
}

// Entry point
@main
board = Board {
    commands [registerUser]
}

// Imports (new feature)
import "./base-types.ilk"
import "./common-tags.ilk" as tags  // namespaced: tags.SomeType

Key Changes
┌───────────────────┬───────────────────┐
│      Before       │       After       │
├───────────────────┼───────────────────┤
│ .ilk + .kli files │ Single .ilk file  │
├───────────────────┼───────────────────┤
│ Block { body }    │ type Block = body │
├───────────────────┼───────────────────┤
│ @main on type     │ @main on instance │
├───────────────────┼───────────────────┤
│ No imports        │ import "path"     │
├───────────────────┼───────────────────┤
│ Separate parsers  │ Single parser     │
└───────────────────┴───────────────────┘
What Stays Same

- Concrete<T> syntax
- @source, @out, @constraint, @assoc // on types only
- Struct/list/union/intersection syntax
- Field origins (Type*, = path, = compute())
- All constraint functions

## Implementation Steps

### Phase 1: Parser Merge

Files: src/parser.rs (new unified), delete src/ilk/parser.rs + src/kli/parser.rs

1. New top-level items:
  - type Name = TypeExpr (type declaration)
  - name = TypeName body (instance binding)
  - import "path" (import statement)
2. Annotations attach to either types or instances

### Phase 2: AST Unification

Files: src/ast.rs (unified)

enum Item {
    TypeDecl { name, annotations, body: TypeExpr },
    Instance { name, type_name, assocs, body: Value },
    Import { path, alias },
}

struct File {
    items: Vec<Item>,
}

### Phase 3: Resolution

File: src/resolve.rs

1. Collect all type declarations → TypeEnv
2. Resolve imports (load + merge TypeEnvs)
3. Find @main instance (not type)

### Phase 4: Validation

Files: src/validate/*.rs (minimal changes)

- Structural validation: same logic, different AST traversal
- Source validation: unchanged
- Constraint validation: unchanged

### Phase 5: Update Specs

- Merge ilk-spec.md + kli-spec.md into single spec.md
- Update examples

Files to Modify
┌───────────────────┬────────────────────────────┐
│       File        │           Action           │
├───────────────────┼────────────────────────────┤
│ src/ilk/parser.rs │ Delete                     │
├───────────────────┼────────────────────────────┤
│ src/kli/parser.rs │ Delete                     │
├───────────────────┼────────────────────────────┤
│ src/ilk/ast.rs    │ Delete                     │
├───────────────────┼────────────────────────────┤
│ src/kli/ast.rs    │ Delete                     │
├───────────────────┼────────────────────────────┤
│ src/parser.rs     │ New - unified parser       │
├───────────────────┼────────────────────────────┤
│ src/ast.rs        │ New - unified AST          │
├───────────────────┼────────────────────────────┤
│ src/resolve.rs    │ Modify - handle imports    │
├───────────────────┼────────────────────────────┤
│ src/lib.rs        │ Modify - single file entry │
├───────────────────┼────────────────────────────┤
│ src/validate/*.rs │ Modify - use new AST       │
├───────────────────┼────────────────────────────┤
│ ilk-spec.md       │ Rewrite as unified spec    │
├───────────────────┼────────────────────────────┤
│ kli-spec.md       │ Delete                     │
└───────────────────┴────────────────────────────┘

## Design Decisions

- File extension: .ilk
- Exports: All types auto-exported (no @export needed)
- Library files: Files without @main are pure type libraries

## Verification

1. Convert existing examples to new syntax
2. Run all existing tests
3. Test import resolution with multi-file setup
