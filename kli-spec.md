## Overview

A `.kli` file provides a **concrete instance** that is validated against the block marked
`@main` in its linked `.ilk` schema file. Validation is structural: the instance must
satisfy the shape and constraints declared in the schema.

This document specifies the **kli instance language**. For the ilk schema language see
`ilk-spec.md`.

---

## Value literals

| Type | kli literal |
|------|-------------|
| `String` | `"hello world"` |
| `Int` | `42` |
| `Float` | `3.14` |
| `Bool` | `true` / `false` |
| `Uuid` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `Date` | `"2024-01-31"` (ISO 8601) |
| `Timestamp` | `"2024-01-31T12:00:00Z"` (ISO 8601) |
| `Money` | `"19.99 USD"` (amount + ISO 4217 currency code) |

---

## Bindings

A binding assigns a name to a typed instance:

```
name = TypeName body
```

Bindings are:
- **Top-level only** — not nested inside other constructs
- **Unordered** — order does not matter for validation
- **Unique** — each name may be declared at most once

```kli
userIdTag      = Tag {userId String}
userRegistered = Event<userIdTag, commonTag> {
    id   String
    name String
}
```

Names follow standard identifier rules and may start with lowercase or uppercase.

---

## Struct values

A struct value is a `{ ... }` block of named fields separated by **newlines**:

```kli
{hello Int}

{
    hello Int
    goodbye String
}
```

Each field is a `name value` pair. The value is a literal, a reference to a binding,
or another nested struct/list.

---

## List values

A list value is `[ ... ]` with elements separated by **commas** or **newlines**:

```kli
[userRegistered, other]

[
    userRegistered
    other
]
```

All elements must conform to the same type declared in the schema.

---

## Block values

A block instance supplies all required fields declared in the schema block:

```kli
registerUser = Command {
    fields {
        id   String
        name String
    }
    emits [userRegistered]
}
```

---

## Associated values

When a block type is declared with `@assoc [T]` in the schema, kli instances of that
block may carry associated values — named bindings of type `T` — listed in angle brackets
immediately after the type name:

```kli
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id   String
    name String
}
```

The angle-bracket list is **not** a generic type parameter. It is a runtime set of
references to other named bindings. The `.assoc(t)` predicate in `@constraint` expressions
tests membership in this set.

---

## Union values

### Structural unions

When a union's branches are all anonymous type expressions (e.g. `{1 Type} | Concrete<String>`),
the branch is determined by shape — no extra syntax is needed:

```kli
userIdTag = Tag {userId String}   // satisfies {1 Type}
simpleTag = Tag "simple-tag"      // satisfies Concrete<String>
```

### Discriminated unions

When a union's branches are all **named block types**, the variant name must be written
explicitly because shape alone cannot discriminate.

**At a binding site**, write the union type followed by the variant name:

```
name = UnionType VariantName body
```

```kli
current = Status Started { at "2024-01-31T12:00:00Z" }
//         ^^^^^^ union type (matches the schema field type)
//                ^^^^^^^ variant name (discriminant)
//                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ variant body
```

**Inline inside a list or struct field**, the union type is inferred from schema context —
only the variant name is needed:

```kli
// schema declares: history []Status
history [
    Started  { at "2024-01-01T00:00:00Z" }
    Finished { at "2024-01-31T12:00:00Z" }
]
```

---

## Field origins

When `@source [fields]` is in effect on a schema declaration, every field in a kli struct
refinement must be traceable to the listed sources. By default the validator matches by
structural name. Three **optional** origin annotations override that:

### Generated (`Type*`)

```kli
timestamp Int*
```

The field value is auto-produced at runtime. Provenance is **not** checked even when
`@source` is in effect.

### Mapped (`Type = path`)

```kli
customerId Uuid = fields.userId
nestedId   Uuid = fields.user.address.id
```

The value is copied from a source field identified by a **dot-path** walked from the
enclosing block. Use this when the kli field name differs from the source, or when the
source is nested.

The root segment of the path must be one of the sources named in `@source`.

### Computed (`Type = @computed_from [path, ...]`)

```kli
amount Int = @computed_from [fields.quantity, fields.unitAmount]
```

The value is derived from multiple source fields. Paths are comma-separated dot-paths
inside `[...]`. At least one path is required. All path roots must satisfy the same
`@source` constraint as mapped fields.

### Precedence

When `@source` is in effect, the validator resolves each field in priority order:

1. `Type*` — exempt; skip provenance check
2. `Type = path` or `Type = @computed_from [paths]` — explicit origin; validate path roots
3. No origin form — implicit; structural name-match against the source fields

---

## Separator rules (summary)

| Context | Separator |
|---|---|
| Struct fields (`{ ... }`) | Newlines |
| List values (`[ ... ]`) | Commas or newlines |

---

## Full example

### Instance (`dcb-board-instance-valid.kli`)

For the corresponding ilk schema see `ilk-spec.md`.

```kli
// Tag bindings — structural union: shape determines branch
userIdTag   = Tag {userId String}    // satisfies {1 Type}
userNameTag = Tag {name String}
commonTag   = Tag {x String}
simpleTag   = Tag "simple-tag"       // satisfies Concrete<String>

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

    // @source [fields] is in effect on emits:
    // - timestamp is generated (not in fields)
    // - id is mapped by name (fields.id matches)
    emits [userRegistered {
        timestamp Int*
        id        String         // implicit: matched by name to fields.id
    }]

    // query has no @source — no provenance constraint
    query [
        {
            eventTypes [userRegistered, other]
            tags       [commonTag]
        }
    ]
}
```
