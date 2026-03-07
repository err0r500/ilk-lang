## Overview

A `.kli` file is a **domain model** — it names the specific entities that exist in a
domain (which events, which commands, which tags…) and declares their exact shapes.
It is validated against the block marked `@main` in its linked `.ilk` schema file.

A `.kli` file is **not** a data file. It does not contain runtime values like actual
UUIDs or timestamps. Think of it as a catalog: the `.ilk` defines what an *Event* is in
the abstract; the `.kli` says "in *my* system, the specific events are `userRegistered`
(with fields `id` and `name`) and `orderPlaced` (with field `orderId`)."

Runtime records (actual field values) live downstream of both levels.

This document specifies the **kli domain-model language**. For the ilk schema language see
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
userIdTag      = TagField {userId String}    // satisfies TagField branch of Tag
simpleTag      = Tag "simple-tag"            // satisfies Concrete<String> branch of Tag
userRegistered = Event<userIdTag> {
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

### Named block branches (discriminated)

When union branches are user-defined block types, write the variant name — it is always
sufficient; the schema context provides the union type:

```kli
// schema: Status = Started | Finished
current = Started { at "2024-01-31T12:00:00Z" }

// or inline in a list:
history [
    Started  { at "2024-01-01T00:00:00Z" }
    Finished { at "2024-01-31T12:00:00Z" }
]
```

### Built-in scalar branches

When a union has a built-in scalar branch (`String`, `Concrete<String>`, `Int`, etc.),
write the literal directly — the syntax identifies the branch:

```kli
// schema: TagField = {_}
//         Tag = TagField | Concrete<String>
userIdTag = TagField {userId String}   // TagField branch: named block with one field
simpleTag = Tag "simple-tag"           // Concrete<String> branch: string literal
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

### Computed (`Type = compute(path, ...)`)

```kli
amount Int = compute(fields.quantity, fields.unitAmount)
```

The value is derived from multiple source fields. Paths are comma-separated dot-paths
inside `(...)`. At least one path is required. All path roots must satisfy the same
`@source` constraint as mapped fields.

### Precedence

When `@source` is in effect, the validator resolves each field in priority order:

1. `Type*` — exempt; skip provenance check
2. `Type = path` or `Type = compute(paths)` — explicit origin; validate path roots
3. No origin form — implicit; structural name-match against the source fields

---

## Inline binding refinements

When `@source` is in effect on a list, a list element may be written as a binding reference
followed by a struct body. The struct body supplies **origin annotations** for specific
fields of the referenced binding:

```kli
emits [userRegistered {
    timestamp Int*               // Generated — exempt from source check
    id        String             // implicit: matched by name to fields.id
}]
```

Rules:
- The struct body contains only origin-annotated fields (`Type*`, `Type = path`, `Type = compute(...)`), or fields with no annotation (explicit implicit match).
- Fields not mentioned fall back to implicit name-matching against the source.
- The refinement may not name fields that do not exist in the binding's declared type.
- This syntax is only valid within `@source`-constrained list declarations.

---

## Anonymous struct instantiation

When a field or list element has an unambiguous expected type from the schema, the type
name may be omitted and an anonymous struct `{ ... }` supplied directly. Structural typing
validates that the struct matches the expected type:

```kli
// schema: query []QueryItem
// QueryItem type name omitted — struct matches structurally
query [
    {
        eventTypes [userRegistered, other]
        tags       [commonTag]
    }
]
```

This is only valid when the expected element type is a single concrete block type
(unambiguous from context). For union-typed lists, write the branch name explicitly.

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
// Tag bindings — TagField = {_}, Tag = TagField | Concrete<String>
userIdTag   = TagField {userId String}    // TagField branch: one-field struct
userNameTag = TagField {name String}
commonTag   = TagField {x String}
simpleTag   = Tag "simple-tag"            // Concrete<String> branch: string constant

// Event bindings with their associated tags
// Event carries @assoc [Tag] — supply tags in angle brackets
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

    // @source [fields] is in effect on emits.
    // Inline refinement annotates field origins for userRegistered:
    // - timestamp is generated (not in fields, so marked Int*)
    // - id is matched by name implicitly (fields.id)
    // - name is matched by name implicitly (fields.name)
    emits [userRegistered {
        timestamp Int*
        id        String         // implicit: matched by name to fields.id
    }]

    // @source [fields] for [tags] is in effect on query:
    // only the tags field of each QueryItem is source-constrained.
    // QueryItem type name omitted — anonymous struct matches structurally.
    query [
        {
            eventTypes [userRegistered, other]
            tags       [commonTag]
        }
    ]
}
```
