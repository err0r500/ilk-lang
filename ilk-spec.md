## Overview

ilk is a two-level language system:

- **`.ilk` files** define schemas — the shapes, types, and constraints that instances must satisfy.
- **`.kli` files** define instances — concrete values that conform to an `.ilk` schema.

A `.kli` file is validated against the block marked `@main` in its linked `.ilk` file.
Type compatibility is checked **structurally** (by shape, not by name).

This document specifies the **ilk schema language**. For the kli instance language see `kli-spec.md`.

---

## Comments

Single-line comments only, using `//`:

```ilk
// this is a comment
Tag {1 Type} | Concrete<String> // inline comment
```

---

## Base types

| Token | Description |
|-------|-------------|
| `Type` | Meta-type: stands for any built-in or user-defined type. Used in struct cardinality notation. |
| `Uuid` | UUID value |
| `String` | UTF-8 string |
| `Int` | Integer |
| `Float` | Floating-point number |
| `Bool` | Boolean (`true` / `false`) |
| `Date` | Calendar date |
| `Timestamp` | Point in time |
| `Money` | Monetary amount |

`Type` is not a value type — it is a placeholder meaning "any type" and only appears in
struct cardinality notation and schema-level declarations.

---

## Concrete types

`Concrete<T>` constrains a field to a single fixed value of type `T`.

```ilk
// ilk: field must be the string "hello"
label Concrete<String>
```

```kli
label "hello"
```

Identifier-based enum variants (no quotes) can be expressed directly as union members:

```ilk
// ilk
HttpVerb Get | Post | Put | Delete
```

```kli
// kli — just write the chosen variant
verb Get
```

---

## Struct types

Format: `{cardinality Type}`

`Type` here is the meta-type meaning "each field may be of any type".

```ilk
{1 Type}   // exactly 1 field
{2 Type}   // exactly 2 fields
{* Type}   // zero or more fields (any number)
```

Fields are separated by **newlines**. Each field is a `name Type` pair:

```kli
{hello Int}

{
    hello Int
    goodbye String
}

{
    hello Int
    goodbye String
    score Float
    active Bool
}
```

**Constraints:**
- `{0 Type}` is invalid — use the absence of a struct instead.
- Mixed-cardinality structs (e.g. fixed fields alongside `{* Type}`) are not allowed;
  use **intersection types** (`&`) for that pattern.

---

## List types

`[]T` — a list of zero or more values, all of the same type `T`.
`[N]T` — a list of exactly `N` values of type `T` (N is a positive integer).

Unlike structs, list elements have **no keys** — they are positional and uniform.

```ilk
[]Event       // zero or more Event values
[3]Tag        // exactly 3 Tag values
```

List values in `.kli` are separated by **commas** (or newlines):

```kli
[userRegistered, other]

[
    userRegistered
    other
]
```

---

## Block (user-defined types)

Names start with a capital letter: `User`, `Product`, `Command`, etc.

A block body is a set of named field declarations, separated by **newlines**:

```ilk
Command {
    fields {* Type}
    emits  []Event
}
```

```kli
Command {
    fields {
        name  String
        age   Int
        email String
    }
    emits [userRegistered]
}
```

---

## Intersection types

`A & B` produces a type whose instances must satisfy **both** `A` and `B`.
All fields from both sides are merged into a single struct.

```ilk
Event {* Type} & {timestamp Int}
```

```kli
// valid — has arbitrary fields plus the required timestamp
Event {
    name      String
    other     Bool
    timestamp Int
}
```

Use intersection instead of mixed-cardinality structs:

```ilk
// {* Type} plus a fixed "id" field
Entity {* Type} & {id Uuid}
```

---

## Union types

`A | B` means a value must satisfy **exactly one** of the alternatives.

When all branches are **named block types**, the union is *discriminated* and the kli
variant must be named explicitly (see [Discriminated unions](#discriminated-unions)).
When all branches are anonymous type expressions, the branch is selected by shape.

```ilk
Response {success Bool} | {error String}
```

```kli
// one valid instance
Response {
    success Bool
}
```

```kli
// another valid instance
Response {
    error String
}
```

Union members may also be concrete identifier variants (enum-style):

```ilk
Status Pending | Active | Archived
```

```kli
status Active
```

---

## Discriminated unions

When all branches of a union are **named block types**, the union is a **discriminated union**.
The validator cannot pick a branch by shape alone — two named block types may have identical
fields. The variant name is the discriminant.

### Ilk rules

- A union is discriminated when every branch is a named block type (declared elsewhere in the
  same `.ilk` file).
- A union is **structural** when every branch is an anonymous type expression
  (`{N Type}`, `Concrete<T>`, primitive type, etc.).
- **Mixed unions** — some branches named, some anonymous — are not valid.

```ilk
// discriminated: both branches are named block types
Started  { at Timestamp }
Finished { at Timestamp }
Status   Started | Finished

// structural: both branches are anonymous expressions (unchanged from current behaviour)
Tag {1 Type} | Concrete<String>
```

For kli syntax for discriminated and structural unions, see `kli-spec.md`.

---

## Field origins (`T*`, `= path`, `= @computed_from`)

When `@source` is in effect on a list or field declaration, each field in a kli struct
refinement must be provably traceable to the listed sources. By default the validator
matches by structural name. Three optional **origin annotations** override that resolution:

| Form | Meaning |
|---|---|
| `fieldName Type*` | **Generated** — value is auto-produced; provenance not checked |
| `fieldName Type = path` | **Mapped** — value copied from a dot-path in a source field |
| `fieldName Type = @computed_from [path, ...]` | **Computed** — derived from multiple source fields |

Origin annotations are kli-side only. They appear in struct refinements in `.kli` files.
For full syntax and rules see `kli-spec.md`.

---

## Associated values (`@assoc`)

`@assoc [T]` on a block declaration means that kli instances of that block may carry
**associated values** — references to other named kli instances of type `T`.

```ilk
@assoc [Tag]
Event {* Type} & {timestamp Int}
```

Associated values are **not generic type parameters**. In kli, they are listed in angle
brackets at the binding site: `Event<tag1, tag2> { ... }`. See `kli-spec.md`.

The `.assoc(t)` predicate is available in `@constraint` expressions on any block that
carries `@assoc`.

---

## Annotations

Annotations appear on the line immediately before the declaration they annotate.

| Annotation | Valid target | Meaning |
|---|---|---|
| `@main` | block | Entry point — the kli file must satisfy this block |
| `@assoc [T]` | block | Instances may carry associated values of type `T` |
| `@source [fields]` | field / list decl | Values must originate from the named field list |
| `@constraint <expr>` | block body | Boolean predicate that must hold for every instance |
| `@AnnotationName` | field / list decl | User-defined annotation with declared validation effects (see [User-defined annotations](#user-defined-annotations)) |

### `@main`

Exactly one block per `.ilk` file may be marked `@main`. The `.kli` file is validated
as an instance of this block.

```ilk
@main
Board {
    commands []Command
}
```

### `@source`

`@source [fields]` on a declaration means every value in that construct must be
traceable to one of the fields listed.

The validator resolves each field in a kli struct refinement in priority order:
1. `Type*` — exempt (generated)
2. `Type = path` / `Type = @computed_from [paths]` — explicit origin; path root must be in the source list
3. No origin form — implicit; matched by structural name against the source fields

```ilk
Command {
    fields {* Type}

    @source [fields]
    emits []Event

    query []QueryItem
}
```

### `@constraint`

An inline boolean predicate that every instance of the enclosing block must satisfy.
Uses the constraint expression language (see below).

```ilk
QueryItem {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))

    eventTypes []Event
    tags       []Tag
}
```

---

## User-defined annotations

Ilk authors may define their own annotations and declare what effect they have on validation.
This allows domain-specific annotation vocabulary without adding new built-in keywords.

### Definition syntax

```ilk
annotation AnnotationName {
    effect
    effect
    ...
}
```

The body lists one or more **effects** from the closed effects vocabulary (see below).
Annotation names follow the same PascalCase convention as block type names.

### Effects vocabulary

| Effect | Meaning |
|---|---|
| `suppresses source_check` | The annotated field's value does not need a declared `@source`. The validator skips provenance checking for this field even when `@source` is in effect on the enclosing block. |

Additional effects may be added as new validation dimensions are introduced.
User-defined predicates or arbitrary code are not valid effects.

### Applying a user-defined annotation

A defined annotation is applied with `@AnnotationName`, exactly like a built-in annotation,
on the line immediately before the field it annotates:

```ilk
annotation Output {
    suppresses source_check
}

Command {
    fields {* Type}

    @source [fields]
    emits []Event

    query []QueryItem
}

QueryItem {
    @Output
    eventTypes []Event   // source_check suppressed — not subject to @source

    tags []Tag
}
```

### Semantics

- Annotation definitions are **top-level declarations**, like block type definitions.
  They are not valid inside block bodies.
- The same annotation may be applied to multiple fields across multiple blocks.
- A field may carry **at most one** user-defined annotation.
- User-defined annotations and built-in annotations may not be stacked on the same field.
- Kli files never define or use annotations — they are an ilk-only concept.

---

## Constraint expression language

A minimal expression language for `@constraint` predicates.

### Built-in functions

| Expression | Meaning |
|---|---|
| `forall(col, x => body)` | True if `body` holds for every element `x` in collection `col` |
| `e.assoc(t)` | True if instance `e` has `t` as one of its associated values. Available only when `e`'s type carries `@assoc [T]` and `t` is of type `T`. |

Predicates compose with `&&` (and) and `||` (or). Additional built-ins may be added
as the language evolves; user-defined predicates are not currently supported.

---

## Separator rules (summary)

| Context | Separator |
|---|---|
| Struct fields (`{ ... }`) | Newlines |
| Block body entries | Newlines |
| List values (`[ ... ]`) | Commas or newlines |
| Annotation arguments (`[...]`) | Commas |

---

## Full example

### Schema (`dcb-board-spec.ilk`)

```ilk
// Tag is either a single-field struct of any type, or a concrete string
Tag {1 Type} | Concrete<String>

// Event has any number of fields plus a timestamp; instances may carry Tag values
@assoc [Tag]
Event {* Type} & {timestamp Int}

// QueryItem: every event in eventTypes must be associated with every tag in tags
QueryItem {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))

    eventTypes []Event
    tags       []Tag
}

// Command: fields drive emits (timestamp auto-generated); query has no source constraint
Command {
    fields {* Type}

    @source [fields]
    emits []Event

    query []QueryItem
}

// Board is the entry point
@main
Board {
    commands []Command
}
```

For the corresponding kli instance see `kli-spec.md`.
