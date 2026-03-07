## Overview

ilk is a **two-level data modeling system**:

| Level | File | Purpose |
|---|---|---|
| Schema | `.ilk` | Declares the abstract vocabulary of a domain: which concepts exist, what shape they have, what constraints apply. |
| Domain model | `.kli` | Instantiates that vocabulary for a specific domain: names the concrete entities (events, commands, tags…) and their exact shapes. |

A `.kli` file is validated against the block marked `@main` in its linked `.ilk` file.

Neither level describes runtime data records. A `.kli` file is not a JSON document or
a database row — it is a domain model that says *which named entities exist* and *what
they look like*. Runtime values (actual UUIDs, strings, timestamps) live outside both
levels, in whatever system consumes the validated domain model.

Type compatibility is checked **structurally** (by shape, not by name).

This document specifies the **ilk schema language**. For the kli domain-model language see `kli-spec.md`.

---

## Comments

Single-line comments only, using `//`:

```ilk
// this is a comment
Tag {_} | Concrete<String> // inline comment
```

---

## Base types

| Token | Description |
|-------|-------------|
| `Any` | Meta-type: stands for any built-in or user-defined type. Used in struct cardinality notation and type-level positions. |
| `Uuid` | UUID value |
| `String` | UTF-8 string |
| `Int` | Integer |
| `Float` | Floating-point number |
| `Bool` | Boolean (`true` / `false`) |
| `Date` | Calendar date |
| `Timestamp` | Point in time |
| `Money` | Monetary amount |

`Any` is not a value type — it is a placeholder meaning "any type" and appears in
struct cardinality notation, schema-level declarations, and any position where a
specific type is intentionally left open.

---

## Concrete types

`T` and `Concrete<T>` both accept values of type `T`, but carry different intent:

| Form | Meaning |
|---|---|
| `String`, `Int`, … | A **runtime** value — determined dynamically by the system consuming the domain model |
| `Concrete<String>`, `Concrete<Int>`, … | A **domain-model constant** — a specific value chosen in the kli file and fixed for all instances |

```ilk
// label is a specific constant string chosen in kli
label Concrete<String>
```

```kli
label "webhook"   // the constant string, pinned in the domain model
```

The validator does not enforce a particular literal; it records that the field holds
a domain-constant. The distinction documents intent: `Concrete<T>` fields belong to
the model definition, `T` fields belong to runtime data.

Identifier-only union variants (no body, no quotes) are shorthand for named empty blocks:

```ilk
HttpVerb Get | Post | Put | Delete
```

```kli
verb Get   // write the variant name directly
```

---

## Struct types

Structs have named fields. The schema can constrain how many fields are required using
**anonymous-field shorthand** where `_` is a placeholder for "a field of any name":

```ilk
{_}      // exactly 1 field of any name and type
{_, _}   // exactly 2 fields of any names and types
{...}    // zero or more fields of any names and types
```

For structs with known field names, list them explicitly:

```ilk
{id Uuid, name String}   // exactly these two fields
```

Fields are separated by **newlines** (or commas inline):

```kli
{hello Int}

{
    hello Int
    goodbye String
}
```

**Constraints:**
- `{}` (zero fields) is invalid — use the absence of a struct instead.
- Mixed anonymous/named structs are not allowed; use **intersection types** (`&`) for that pattern.

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
    fields {...}
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

### Optional fields

Appending `?` to a field name marks it as optional — the field may be absent in a kli
binding without failing validation.

```ilk
User {
    id     Uuid
    name   String
    email? String   // may be absent
}
```

Required fields (no `?`) must be present in every kli instance of that block.

---

## Intersection types

`A & B` produces a type whose instances must satisfy **both** `A` and `B`.
All fields from both sides are merged into a single struct.

```ilk
Event {...} & {timestamp Int}
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
// open struct plus a fixed "id" field
Entity {...} & {id Uuid}
```

### Conflict rule

When both sides of `&` name the same field, the **right side wins** — its type overrides
the left side's declaration for that field. This makes intersection behave like a mixin:
the right operand refines the left.

```ilk
// {...} may include a timestamp field of any type;
// & {timestamp Int} pins it to Int — right side wins
Event {...} & {timestamp Int}
```

If both sides are concrete (neither is `{...}`) and declare the same field name with
**incompatible types**, it is a schema error.

```ilk
// error: both sides explicitly name "id" with different types
Bad {id Uuid} & {id String}
```

---

## Union types

`A | B` means a value must satisfy **exactly one** of the alternatives. Every branch must
be a **named type** — either a user-defined block or a built-in scalar type
(`String`, `Int`, `Concrete<T>`, etc.). Inline anonymous struct expressions (`{...}`) are
not valid as union branches; declare a named block first.

In kli, the right way to write a union value depends on whether the branch is a
user-defined block or a built-in scalar:

- **User-defined block branches** require writing the variant name (see [Discriminated unions](#discriminated-unions)).
- **Built-in scalar branches** are matched by syntax (string literals, int literals, etc.).

```ilk
// named block branches — variant name required in kli
Success { value Bool }
Error   { message String }
Response Success | Error
```

```kli
result = Success { value true }
// or
result = Error { message "not found" }
```

Identifier-only variants (no body) are named blocks with empty bodies:

```ilk
Status Pending | Active | Archived
```

```kli
status Active
```

---

## Discriminated unions

Since all union branches are named types, every union is discriminated. When a field or
binding expects a union type, the kli value identifies which variant it is by name.

Named block branches have identical syntax in any position — the variant name is always
sufficient; the schema context provides the union type.

```ilk
Started  { at Timestamp }
Finished { at Timestamp }
Status   Started | Finished   // two named branches with the same shape — unambiguous by name
```

Built-in scalar branches (`String`, `Concrete<T>`, `Int`, etc.) are matched by the
syntax of the kli value itself (a string literal, int literal, etc.), so no variant name
is written for those branches:

```ilk
Tag {_} | Concrete<String>   // {_} branch = TagField struct; Concrete<String> branch = string literal
```

See `kli-spec.md` for full kli syntax for each case.

---

## Field origins (`T*`, `= path`, `= compute(...)`)

When `@source` is in effect on a list or field declaration, each field in a kli struct
refinement must be provably traceable to the listed sources. By default the validator
matches by structural name. Three optional **origin annotations** override that resolution:

| Form | Meaning |
|---|---|
| `fieldName Type*` | **Generated** — value is auto-produced; provenance not checked |
| `fieldName Type = path` | **Mapped** — value copied from a dot-path in a source field |
| `fieldName Type = compute(path, ...)` | **Computed** — derived from multiple source fields |

Origin annotations are kli-side only. They appear in struct refinements in `.kli` files.
For full syntax and rules see `kli-spec.md`.

---

## Associated values (`@assoc`)

Associations are an out-of-band relationship mechanism — they attach named references to
an instance without making those references part of the instance's fields. They are used
to express metadata relationships like "this event is tagged with these domain tags."

The mechanism has three parts that work together:

**1. Schema declaration** — `@assoc [T]` on a block means instances may carry references
to bindings of type `T`:

```ilk
@assoc [Tag]
Event {...} & {timestamp Int}
```

**2. Kli supply** — in kli, associated references are listed in angle brackets immediately
after the type name at the binding site:

```kli
userRegistered = Event<userIdTag, commonTag> {
    id   String
    name String
}
```

The angle brackets are **not** generics. They are a list of references to named kli
bindings of the declared associated type.

**3. Constraint access** — the `.assoc(t)` predicate in `@constraint` expressions tests
whether a specific reference is in an instance's association set:

```ilk
QueryItem {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))
    eventTypes []Event
    tags       []Tag
}
```

The predicate `e.assoc(t)` is only available on a variable whose type carries `@assoc [T]`,
and `t` must be of type `T`.

---

## Annotations

Annotations appear on the line immediately before the declaration they annotate.

| Annotation | Valid target | Meaning |
|---|---|---|
| `@main` | block | Entry point — the kli file must satisfy this block |
| `@assoc [T]` | block | Instances may carry associated values of type `T` |
| `@source [fields]` | field / list decl | Values must originate from the named field list |
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
traceable to one of the fields listed.

The validator resolves each field in a kli struct refinement in priority order:
1. `Type*` — exempt (generated)
2. `Type = path` / `Type = compute(paths)` — explicit origin; path root must be in the source list
3. No origin form — implicit; matched by structural name against the source fields

```ilk
Command {
    fields {...}

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

One rule everywhere: **newlines, or commas where elements fit on one line**.

| Context | Separator |
|---|---|
| Struct fields (`{ ... }`) | Newlines (or commas inline) |
| Block body entries | Newlines |
| List values (`[ ... ]`) | Commas or newlines |
| Annotation arguments (`[...]`) | Commas or newlines |

---

## Full example

### Schema (`dcb-board-spec.ilk`)

```ilk
// Tag is either a single-field struct of any type, or a concrete string
Tag {_} | Concrete<String>

// Event has any number of fields plus a timestamp; instances may carry Tag values
@assoc [Tag]
Event {...} & {timestamp Int}

// QueryItem: every event in eventTypes must be associated with every tag in tags
QueryItem {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))

    eventTypes []Event
    tags       []Tag
}

// Command: fields drive emits (timestamp auto-generated); query has no source constraint
Command {
    fields {...}

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
