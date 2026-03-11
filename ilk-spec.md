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
TagField {_} // inline comment on a block declaration
```

---

## Base types

| Token | Description |
|-------|-------------|
| `Any` | Accepts any type. Usable as a field type or in struct cardinality notation. |
| `Uuid` | UUID value |
| `String` | UTF-8 string |
| `Int` | Integer |
| `Float` | Floating-point number |
| `Bool` | Boolean (`true` / `false`) |
| `Date` | Calendar date |
| `Timestamp` | Point in time |
| `Money` | Monetary amount |

`Any` can be used as a field type (kli may instantiate with any concrete type or
value) or in struct cardinality notation like `{_}` (shorthand for `{_ Any}`).

---

## Value constraint levels

Three forms express how tightly the schema constrains a field's value:

| ilk form | Constraint | Meaning |
|---|---|---|
| `String`, `Int`, … | Open | kli accepts **any** value of that type |
| `Concrete<String>`, `Concrete<Int>`, … | kli-fixed | kli declares **one specific** value; the schema does not prescribe which one |
| `"hello"`, `42`, `true`, … | Schema-fixed | Only this exact value is valid; kli cannot change it |

```ilk
// open: kli may supply any string
name String

// kli-fixed: kli picks one specific string (e.g. "webhook")
label Concrete<String>

// schema-fixed: must be exactly this integer — kli cannot override it
version 1
```

```kli
name    "alice"            // any string accepted — open constraint
label   Concrete "webhook" // one specific string, chosen by the kli author — tagged
version 1                  // must match the schema literal exactly
```

The three levels form a tightening progression: `String` leaves the value fully open,
`Concrete<String>` lets the kli author fix it to one value, and `"hello"` forecloses
the choice in the schema itself.

**Types must match exactly.** kli must use the same type as ilk — no subtyping:
- ilk `String` → kli must use `String`, not `Uuid` or `"hello"`
- ilk `Timestamp` → kli must use `Timestamp`, not a literal

ilk defines structure; kli instantiates it without narrowing the runtime contract.

> **Future consideration:** Variance annotations (`+T` covariant, `-T` contravariant) could
> allow controlled narrowing/widening of constraint levels. Currently all levels are invariant.

Literal types are most useful in union positions:

```ilk
HttpMethod "GET" | "POST" | "PUT" | "DELETE"
```

```kli
method "POST"   // matched by literal syntax
```

Identifier-only union variants (no body, no quotes) remain available as shorthand for
named empty blocks when a symbolic name is preferred over a string literal:

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
{_}              // exactly 1 field of any name and type (= {_ Any})
{_ String}       // exactly 1 field of any name, type String
{_, _}           // exactly 2 fields of any names and types
{_ Int, _ String}  // exactly 2 fields with specific types
{}               // zero fields (empty struct)
{...}            // zero or more fields of any names and types
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

## Reference types

`&T` — a reference to a binding of type `T`.

Reference types express a pointer to an existing binding without instantiating it or flowing data through it. The validator checks that the referenced binding exists and is of the correct type.

```ilk
&Event      // reference to an Event binding
[]&Event    // list of references to Event bindings
```

**Validation rules:**
- The kli value must be an unquoted binding name
- The binding must exist in the kli file
- The binding must be of type `T` (or a subtype)
- No data flows through references — `@source` checks do not apply

Use references when you need to identify *which* binding, not instantiate it:

```ilk
Query {
    eventTypes []&Event   // which event types to filter (references)
    tags       []Tag      // actual tag instances (data may flow for Parametrized)
}
```

Contrast with instance types where structural rules apply:
- Open fields (`String`, `{...}`) require data, `@source` checked
- Concrete fields (`Concrete<T>`, literals) need no data

---

## Subtyping

Type compatibility in ilk follows structural subtyping with the following rules:

### Struct subtyping

Closed structs require exact field match — no width subtyping:

```ilk
{x Int}           // requires exactly {x Int}, no extra fields
{...}             // accepts any struct (zero or more fields)
{...} & {x Int}   // accepts any struct with at least {x Int}
```

Width subtyping is only available via the open struct pattern (`{...} & {...}`).

### List subtyping (covariant)

Lists are covariant in their element type:

```ilk
[]Event           // accepts list of Event or Event subtypes
```

### Reference subtyping (covariant)

References are covariant — `&S` is a subtype of `&T` when `S` is a subtype of `T`:

```ilk
&Event            // accepts reference to Event or Event subtype
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

### Field presence

All fields in ilk are **optional by default** — the schema defines shape and structure,
not presence requirements. Presence is enforced by data flow: if a `@source` mapping
requires a field, validation catches its absence.

```ilk
User {
    id    Uuid
    name  String
    email String   // all fields optional by default
}
```

This design separates concerns: ilk defines *what fields can exist*, kli defines
*what fields do exist*, and `@source` validation ensures data flow consistency.

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

This rule applies unconditionally — the right side wins even when both sides are
concrete with different types:

```ilk
// OK: right side wins — result has {id String}
{id Uuid} & {id String}
```

### Reference intersections

Reference types (`&T`) cannot participate in intersections. References point to bindings,
while intersections merge struct shapes — the two concepts do not combine meaningfully.

```ilk
// error: reference cannot be intersected
Bad &Event & {priority Int}
```

---

## Union types

`A | B` means a value must satisfy **exactly one** of the alternatives. All branches must
be **named blocks** — user-defined block types, including:
- Struct blocks: `Success { value Bool }`
- Identifier-only variants (empty named blocks): `Get`, `Post`, …
- Type aliases: `Unique Concrete<String>`

Inline anonymous struct expressions (`{...}`) are not valid as union branches; declare a
named block first. `Concrete<T>` and other scalar types used as branches must likewise be
wrapped in a named alias so kli can discriminate them by name.

In kli, the branch is always identified by the variant name (see [Discriminated unions](#discriminated-unions)).

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

Scalar branches (`String`, `Concrete<T>`, `Int`, etc.) and literal branches (`"GET"`, `42`,
etc.) are matched by the syntax of the kli value itself — no variant name is written:

```ilk
// Anonymous struct not valid as union branch — declare a named block first:
Parametrized {_ String}          // one field of any name, type String
Unique Concrete<String>          // named alias — Concrete<String> as a discriminable branch
Tag Parametrized | Unique        // Parametrized branch = named struct; Unique branch = concrete string
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
after the type name at the binding site. The angle brackets are omitted entirely when no
associations are supplied:

```kli
// with associations
userRegistered = Event<userIdTag, commonTag> {
    id   String
    name String
}

// no associations — angle brackets absent
other = Event {
    hello String
}
```

The angle brackets are **not** generics. They are a list of references to named kli
bindings of the declared associated type. `Event<>` (empty brackets) is not valid — omit
them instead.

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
| `@source [S, …]` | field / list decl | Values must originate from one of the named source fields |
| `@constraint <expr>` | block body | Boolean predicate that must hold for every instance |
| `@doc "..."` | declaration / field (kli only) | Implementation hint preserved in AST; see `kli-spec.md` |

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

`@source [S, …]` on a declaration means every value in that construct must be traceable
to one of the named source fields. Multiple sources may be listed, comma-separated.

**Dot-path sources:** Source entries may be dot-separated paths to reach nested fields:

```ilk
@source [db.returns]   // fields must trace to db.returns.*
body {...}
```

Source paths are resolved from the enclosing block root, not relative to the annotation's position.

The validator resolves each field in a kli struct refinement in priority order:
1. `Type*` — exempt (generated)
2. `Type = path` / `Type = compute(paths)` — explicit origin; path root must be in the source list
3. No origin form — implicit; matched by structural name against the source fields

**On a list declaration** — each element's fields are checked against the sources.

**On a plain struct field** — the field's own struct element is checked directly: every
sub-field of that struct must be traceable to the named sources.

**Reference types (`&T`) are exempt** — since references point to bindings rather than instantiating them, no data flows and `@source` validation does not apply.

```ilk
Command {
    fields {...}

    @source [fields]
    emits []Event       // each Event element's fields must trace to Command.fields

    @source [fields]
    summary {...}       // summary struct's own fields must trace to Command.fields

    query []QueryItem
}
```

#### Inline binding refinements (kli side)

When `@source` is in effect on a list, kli may refine an existing binding's field origins
using `& { ... }` after the binding reference — mirroring ilk intersection syntax:

```kli
emits [userRegistered & {
    timestamp Int*               // Generated — exempt from source check
    id        String             // implicit: matched by name to fields.id
}]
```

The refinement struct contains only **origin annotations** for specific fields of the
referenced binding. Fields not mentioned are resolved by the default implicit name-match.
The refinement may not add fields that do not exist in the binding's declared type.

#### Subtyping rules for `@source`

Direct field mapping (implicit or explicit `= path`) requires the source type to be a
**subtype** of the target type. Narrowing mappings require `compute()`.

| Mapping | Syntax | Type rule | Example |
|---|---|---|---|
| Direct (implicit) | `field Type` | source ≤ target | `Uuid` → `String` ✓ |
| Direct (explicit) | `field Type = path` | source ≤ target | `Uuid` → `String` ✓ |
| Narrowing | `field Type = compute(...)` | any (runtime) | `String` → `Uuid` ✓ |
| Generated | `field Type*` | n/a | no source check |

**Why no variance annotations needed:** Data flow direction is declared via `@source`.
The sound rule (source ≤ target) is implicit. Narrowing requires the explicit `compute()`
escape hatch, signaling runtime validation.

```ilk
// OK: fields.id (Uuid) can map to Event.id (String) — Uuid <: String
Command {
    fields {id Uuid}
    @source [fields]
    emits []Event
}

// ERROR: fields.id (String) cannot narrow to Event.id (Uuid) — String </: Uuid
Command {
    fields {id String}
    @source [fields]
    emits []Event  // Event.id is Uuid — fails, needs compute()
}

// OK: narrowing via compute() — runtime validation
Command {
    fields {id String}
    @source [fields]
    emits []Event & {
        id Uuid = compute(fields.id)  // explicit narrowing
    }
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
| `exists(col, x => body)` | True if `body` holds for at least one element `x` in collection `col` |
| `unique(col, x => expr)` | True if `expr` yields distinct values for all elements in `col` |
| `count(col)` | Number of elements in collection `col` |
| `e.assoc(t)` | True if instance `e` has `t` as one of its associated values. Available only when `e`'s type carries `@assoc [T]` and `t` is of type `T`. |
| `templateVars(str)` | Extracts `{var}` placeholders from a string template as a set of names |
| `keys(struct)` | Returns the set of field names in a struct |

### Operators

| Operator | Meaning |
|---|---|
| `&&` | Logical and |
| `\|\|` | Logical or |
| `!` | Logical not |
| `==`, `!=` | Equality, inequality |
| `in` | Set membership (`x in set`) |
| `<`, `<=`, `>`, `>=` | Numeric comparison |

Examples:

```ilk
@constraint exists(eventTypes, e => e.assoc(urgentTag))
@constraint unique(eventTypes, e => e.name)
@constraint count(eventTypes) >= 1
@constraint count(tags) <= 5
```

User-defined predicates are not currently supported.

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
// Parametrized wraps exactly one field of any name, constrained to type String.
// Anonymous struct expressions are not valid as union branches — a named block is required.
Parametrized {_ String}
// Unique is a named alias for Concrete<String>, making the branch discriminable in kli.
Unique Concrete<String>
// Tag is either a Parametrized struct or a Unique concrete string.
Tag Parametrized | Unique

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

---

## API endpoint example

### Schema (`api-spec.ilk`)

```ilk
// HTTP method union
HttpMethod "GET" | "POST" | "PUT" | "DELETE"

// Database method abstraction
DbMethod {
    name    Concrete<String>
    args    {...}
    returns {...}
}

// API endpoint with data flow constraints
Endpoint {
    @constraint forall(templateVars(path), v => v in keys(params))
    path    Concrete<String>
    method  HttpMethod
    params {...}
    body   {...}

    @source [params, body]
    db DbMethod

    response {
        status Concrete<Int>
        @source [db.returns]
        body {...}
    }
}

@main
Api {
    endpoints []Endpoint
}
```

Data flows through the endpoint:
- `params`/`body` → `@source` → `db.args`
- `db.returns` → `@source` → `response.body`

For the corresponding kli instance see `kli-spec.md`.
