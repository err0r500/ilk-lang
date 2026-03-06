## Overview

ilk is a two-level language system:

- **`.ilk` files** define schemas — the shapes, types, and constraints that instances must satisfy.
- **`.kli` files** define instances — concrete values that conform to an `.ilk` schema.

A `.kli` file is validated against the block marked `@main` in its linked `.ilk` file.
Type compatibility is checked **structurally** (by shape, not by name).

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

## Value literals

| Type | kli literal example |
|------|---------------------|
| `String` | `"hello world"` |
| `Int` | `42` |
| `Float` | `3.14` |
| `Bool` | `true` / `false` |
| `Uuid` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `Date` | `"2024-01-31"` (ISO 8601) |
| `Timestamp` | `"2024-01-31T12:00:00Z"` (ISO 8601) |
| `Money` | `"19.99 USD"` (amount + ISO 4217 currency code) |

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

## Generated fields (`T*`)

Appending `*` to a type inside a struct marks that field as **generated** — its value
is produced automatically and is not expected to come from any declared source.

This is required when `@source` is in effect on the enclosing construct and a field
cannot be traced back to the listed sources.

```ilk
// emits declares userRegistered events; timestamp is generated (not sourced from fields)
@source [fields]
emits []Event
```

```kli
emits [userRegistered {timestamp Int*}]
```

The `*` tells the validator: "do not check provenance for this field — it will be
generated at runtime."

---

## Bindings (kli only)

A binding assigns a name to an instance. Bindings are:
- **Top-level only** — not nested inside other constructs
- **Unordered** — order does not matter for validation
- **Unique** — each name may be declared at most once

```
name = TypeName body
```

```kli
userIdTag      = Tag {userId String}
userRegistered = Event<userIdTag, commonTag> {
    id   String
    name String
}
```

Names follow standard identifier rules (may start with lowercase or uppercase).

---

## Associated values (`@assoc`)

`@assoc [T]` on a block declaration means that kli instances of that block may carry
**associated values** — references to instances of type `T`.

In kli, associated values are listed in angle brackets after the type name:

```ilk
// ilk
@assoc [Tag]
Event {* Type} & {timestamp Int}
```

```kli
// kli — Event instance associated with two Tag instances
userRegistered = Event<userIdTag, commonTag> {
    id   String
    name String
}
```

These are **not generic type parameters**. They are runtime references to named kli
bindings of the declared associated type.

The `.assoc(t)` predicate is available in constraint expressions on any block that
carries `@assoc`.

---

## Annotations

Annotations appear on the line immediately before the declaration they annotate.

| Annotation | Valid target | Meaning |
|---|---|---|
| `@main` | block | Entry point — the kli file must satisfy this block |
| `@assoc [T]` | block | Instances may carry associated values of type `T` |
| `@source [fields]` | field / list decl | Values must originate from the named field list |
| `@source [f] for [g]` | field / list decl | Only fields tagged by `g` must be sourced from `f` |
| `@constraint <expr>` | block body | Boolean predicate that must hold for every instance |

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
traceable to one of the fields listed. Fields that cannot be sourced must be
marked as **generated** (`T*`).

`@source [fields] for [tags]` narrows the sourcing requirement: only the tag-related
subset (identified by `tags`) must come from `fields`.

```ilk
Command {
    fields {* Type}

    @source [fields]
    emits []Event

    @source [fields] for [tags]
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

## Full example walk-through

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

// Command: fields drive emits (with timestamp auto-generated) and tag-subset of query
Command {
    fields {* Type}

    @source [fields]
    emits []Event

    @source [fields] for [tags]
    query []QueryItem
}

// Board is the entry point
@main
Board {
    commands []Command
}
```

### Instance (`dcb-board-instance-valid.kli`)

```kli
// Tag bindings
userIdTag  = Tag {userId String}
userNameTag = Tag {name String}
commonTag  = Tag {x String}
simpleTag  = Tag "simple-tag"

// Event bindings with their associated tags
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id   String
    name String
}

other = Event<commonTag, simpleTag> {
    hello String
}

// Command binding
registerUser = Command {
    fields {
        id   String
        name String
        x    String
    }

    // timestamp is generated (Int*) because it is not in fields
    emits [userRegistered {timestamp Int*}]

    // tags subset of query must be sourced from fields; commonTag.x comes from fields.x
    query [
        {
            eventTypes [userRegistered, other]
            tags       [commonTag]
        }
    ]
}
```
