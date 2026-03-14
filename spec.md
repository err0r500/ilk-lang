## Overview

ilk is a **single-file data modeling language**. A `.ilk` file contains both
**type declarations** — the abstract vocabulary of a domain (which concepts exist, what
shape they have, what constraints apply) — and **instance bindings** — the concrete
entities that exist in a specific domain (which named events, commands, tags, etc.).

A `.ilk` file is not a data file. It does not hold runtime values like actual UUIDs or
timestamps. Think of it as a catalog: types define what an *Event* is in the abstract;
instance bindings say "in *my* system, the specific events are `userRegistered` and
`orderPlaced`."

Runtime records (actual field values) live downstream.

Type compatibility is checked **structurally** (by shape, not by name).

---

## Comments

Single-line comments only, using `//`:

```ilk
// this is a comment
userIdTag = Parametrized {userId String} // inline comment
```

---

## Base types

| Token | Description |
|-------|-------------|
| `*` | Wildcard — matches any type. Usable as a field type or in struct cardinality notation. |
| `Uuid` | UUID value |
| `String` | UTF-8 string |
| `Int` | Integer |
| `Float` | Floating-point number |
| `Bool` | Boolean (`true` / `false`) |
| `Date` | Calendar date |
| `Timestamp` | Point in time |
| `Money` | Monetary amount |

`*` can be used as a field type (any concrete type or value is accepted) or in struct
cardinality notation like `{_}` (shorthand for `{_ *}`).

---

## Value constraint levels

Three forms express how tightly a field's value is constrained:

| Form | Constraint | Meaning |
|------|------------|---------|
| `String`, `Int`, … | Open | Any value of that type; provided at runtime |
| `Concrete<String>`, `Concrete<Int>`, … | Instance-fixed | Instance declares **one specific** value; the type does not prescribe which |
| `"hello"`, `42`, `true`, … | Type-fixed | Only this exact value is valid |

```ilk
// open: any string at runtime
name String

// instance-fixed: instance picks one specific string (e.g. "webhook")
label Concrete<String>

// type-fixed: must be exactly this integer
version 1
```

In instance bindings:

```ilk
name    String      // satisfies String — open type, value provided at runtime
label   "webhook"   // satisfies Concrete<String> — instance author's chosen value
version 1           // satisfies literal 1 — must match exactly
```

Both `Concrete<T>` and type-fixed literals look the same in instances (`"value"`), but
validation differs:
- `Concrete<T>` — any literal of type T is valid; instance author chooses
- Type-fixed literal — must match exactly

**Types must match exactly.** Instances must use the same type as declared — no subtyping
on constraint levels:
- Type `String` → instance must use `String`, not `Uuid` or `"hello"`
- Type `Timestamp` → instance must use `Timestamp`, not a literal

The three levels form a tightening progression: `String` leaves the value fully open,
`Concrete<String>` lets the instance author fix it to one value, and `"hello"` forecloses
the choice in the type itself.

> **Future consideration:** Variance annotations (`+T` covariant, `-T` contravariant) could
> allow controlled narrowing/widening of constraint levels. Currently all levels are invariant.

Literal types are most useful in union positions:

```ilk
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"
```

Identifier-only union variants (no body, no quotes) are named blocks with empty bodies:

```ilk
type HttpVerb = Get | Post | Put | Delete
```

---

## Type declarations

Type declarations define named types. The `type` keyword introduces a declaration:

```ilk
type Name = TypeExpr
```

Type names start with a capital letter. Declarations may be annotated:

```ilk
type Parametrized = {_ String}
type Unique = Concrete<String>
type Tag = Parametrized | Unique

@assoc [Tag]
type Event = {...} & {timestamp Int}

type Command = {
    fields {...}

    @source [fields]
    emits []Event
}
```

Annotations appear on the line immediately before the declaration they annotate.

---

## Struct types

Structs have named fields. The **anonymous-field shorthand** uses `_` as a placeholder
for "a field of any name":

```ilk
{_}              // exactly 1 field of any name and type (= {_ *})
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

```ilk
{
    id   Uuid
    name String
}
```

**Constraint:** Mixed anonymous/named structs are not allowed; use **intersection types**
(`&`) for that pattern.

---

## List types

| Syntax | Meaning |
|--------|---------|
| `[]T` | 0+ elements |
| `[N]T` | exactly N elements |
| `[N..]T` | N+ elements |
| `[N..M]T` | N to M elements (inclusive) |
| `[..M]T` | 0 to M elements |

```ilk
[]Event       // zero or more Event values
[3]Tag        // exactly 3 Tag values
[1..]Tag      // at least 1 Tag
[2..5]Tag     // 2 to 5 Tags
[..10]Tag     // up to 10 Tags
```

List values in instances are separated by **commas** (or newlines):

```ilk
[userRegistered, other]

[
    userRegistered
    other
]
```

---

## Reference types

`&T` — a reference to a binding of type `T`.

Reference types point to an existing binding without instantiating it or flowing data
through it. The validator checks that the referenced binding exists and is of the correct type.

```ilk
&Event      // reference to an Event binding
[]&Event    // list of references to Event bindings
```

**Validation rules:**
- The instance value must be an unquoted binding name
- The binding must exist in the file
- The binding must be of type `T` (or a subtype)
- No data flows through references — `@source` checks do not apply

---

## Refinable type references

`-T` — a **refinable** reference to a binding of type `T`. The `-` prefix signals that
the instance may refine the binding with concrete values using `& { ... }` syntax.

```ilk
type Scenario = {
    name  Concrete<String>
    given []-Event    // list of refinable Event references
}
```

In instance bindings, a refinable reference may be refined:

```ilk
scenarios [
    {
        name  "happy path"
        given [userRegistered & {id "123"}, userRegistered]
    }
]
```

Without the `-` prefix, providing concrete values in a refinement is an error. With `-`,
the validator allows concrete literals for open fields in the refinement struct.

---

## Union types

`A | B` means a value must satisfy **exactly one** of the alternatives. All branches must
be **named types** — user-defined type names, including:
- Struct block types: `type Success = { value Bool }`
- Identifier-only variants (empty named types): `Get`, `Post`, …
- Type aliases: `type Unique = Concrete<String>`

Inline anonymous struct expressions (`{...}`) are **not** valid as union branches. Declare
a named type first. `Concrete<T>` and other scalar types used as branches must likewise be
wrapped in a named alias so instances can discriminate them by name.

```ilk
type Success = { value Bool }
type Error   = { message String }
type Response = Success | Error
```

In instances, the branch is identified by the variant name:

```ilk
result = Success { value Bool }
// or
result = Error { message String }
```

Identifier-only variants (no body) are named types with empty bodies:

```ilk
type Status = Pending | Active | Archived
```

```ilk
status Active
```

### Discriminated unions

Since all union branches are named types, every union is discriminated. When a field or
binding expects a union type, the instance value identifies which variant it is by name.

```ilk
type Started  = { at Timestamp }
type Finished = { at Timestamp }
type Status   = Started | Finished   // two branches with the same shape — unambiguous by name
```

Scalar branches (`String`, `Concrete<T>`, `Int`, etc.) and literal branches (`"GET"`, `42`,
etc.) are matched by the syntax of the instance value itself — no variant name is written:

```ilk
// Parametrized wraps exactly one field of any name, constrained to type String.
// Anonymous struct expressions are not valid as union branches — a named type is required.
type Parametrized = {_ String}
// Unique is a named alias for Concrete<String>, making the branch discriminable by name.
type Unique = Concrete<String>
// Tag is either a Parametrized struct or a Unique concrete string.
type Tag = Parametrized | Unique
```

In instances:

```ilk
userIdTag = Parametrized {userId String}   // Parametrized branch: named struct, one String field
simpleTag = Unique "simple-tag"            // Unique branch: instance picks a concrete string
```

---

## Intersection types

`A & B` produces a type whose instances must satisfy **both** `A` and `B`. All fields
from both sides are merged into a single struct.

```ilk
type Event = {...} & {timestamp Int}
```

Use intersection instead of mixed-cardinality structs:

```ilk
// open struct plus a required id field
type Entity = {...} & {id Uuid}
```

### Conflict rule

When both sides of `&` name the same field, the **right side wins** — its type overrides
the left side's declaration for that field:

```ilk
// {...} may include a timestamp field of any type;
// & {timestamp Int} pins it to Int — right side wins
type Event = {...} & {timestamp Int}
```

```ilk
// OK: right side wins — result has {id String}
{id Uuid} & {id String}
```

### Reference intersections

Reference types (`&T`) cannot participate in intersections. References point to bindings,
while intersections merge struct shapes — the two concepts do not combine meaningfully.

```ilk
// error: reference cannot be intersected
type Bad = &Event & {priority Int}
```

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

## Annotations

Annotations appear on the line immediately before the declaration they annotate.

| Annotation | Valid target | Meaning |
|---|---|---|
| `@main` | instance binding | Entry point — the file is validated starting from this instance |
| `@assoc [T]` | type declaration | Instances may carry associated values of type `T` |
| `@source [S, …]` | field / list decl | Values must originate from one of the named source fields |
| `@out` | field | Field is an output — exempt from `@source` checks, can be referenced by other `@source` |
| `@constraint <expr>` | type body | Boolean predicate that must hold for every instance |
| `@doc "..."` | declaration / field | Implementation hint preserved in AST; not stripped during parsing |

### `@main`

Exactly one instance binding per `.ilk` file may be marked `@main`. Validation starts
from this instance.

```ilk
@main
board = Board {
    commands [registerUser]
}
```

### `@assoc [T]`

Declares that instances of this type may carry associated values — named bindings of type
`T` — listed in angle brackets at the binding site. See [Associated values](#associated-values).

```ilk
@assoc [Tag]
type Event = {...} & {timestamp Int}
```

### `@source`

`@source [S, …]` on a declaration means every value in that construct must be traceable
to one of the named source fields. Multiple sources may be listed, comma-separated.

**Dot-path sources:** Source entries may be dot-separated paths to reach nested fields:

```ilk
@source [db.returns]   // fields must trace to db.returns.*
body {...}
```

Source paths are resolved from the enclosing type root, not relative to the annotation's
position.

The validator resolves each field in an instance struct in priority order:

1. `Concrete<T>` value or type-fixed literal — exempt (author-chosen, not runtime data)
2. `Type*` — exempt (generated)
3. `Type = path` / `Type = compute(paths)` — explicit origin; path root must be in the source list
4. No origin form — implicit; matched by structural name against the source fields (one level deep)

**On a list declaration** — each element's fields are checked against the sources.

**On a plain struct field** — the field's own struct element is checked directly: every
sub-field of that struct must be traceable to the named sources.

**Reference types (`&T`) are exempt** — references point to bindings rather than
instantiating them, so no data flows and `@source` validation does not apply.

```ilk
type Command = {
    fields {...}

    @source [fields]
    emits []Event       // each Event element's fields must trace to Command.fields

    @source [fields]
    summary {...}       // summary struct's own fields must trace to Command.fields

    query []QueryItem   // no @source — no provenance constraint
}
```

#### Inline binding refinements

When `@source` is in effect on a list, a list element may be written as a binding
reference followed by `& { ... }` — mirroring intersection syntax. The struct body
supplies **origin annotations** for specific fields of the referenced binding:

```ilk
emits [userRegistered & {
    timestamp Int*               // Generated — exempt from source check
    id        String             // implicit: matched by name to fields.id
}]
```

Rules:
- The struct body contains origin-annotated fields (`Type*`, `Type = path`,
  `Type = compute(...)`), or fields with no annotation (implicit name-match).
- Fields not mentioned fall back to implicit name-matching against the source.
- The refinement may not name fields that do not exist in the binding's declared type.
- This syntax is only valid within `@source`-constrained list declarations.

#### Subtyping rules for `@source`

Direct field mapping (implicit or explicit `= path`) requires the source type to be a
**subtype** of the target type. Narrowing mappings require `compute()`.

| Mapping | Syntax | Type rule | Example |
|---|---|---|---|
| Author-chosen | `field "hello"` / `Concrete<T>` value | n/a | no source check |
| Generated | `field Type*` | n/a | no source check |
| Direct (implicit) | `field Type` | source ≤ target | `Uuid` → `String` ✓ |
| Direct (explicit) | `field Type = path` | source ≤ target | `Uuid` → `String` ✓ |
| Narrowing | `field Type = compute(...)` | any (runtime) | `String` → `Uuid` ✓ |

```ilk
// OK: fields.id (Uuid) can map to Event.id (String) — Uuid <: String
type Command = {
    fields {id Uuid}
    @source [fields]
    emits []Event
}

// ERROR: fields.id (String) cannot narrow to Event.id (Uuid) — String </: Uuid
type Command = {
    fields {id String}
    @source [fields]
    emits []Event  // Event.id is Uuid — fails, needs compute()
}

// OK: narrowing via compute() — runtime validation
type Command = {
    fields {id String}
    @source [fields]
    emits []Event & {
        id Uuid = compute(fields.id)  // explicit narrowing
    }
}
```

### `@out`

`@out` marks a field as **output** — data flows OUT from this field rather than into it.
Fields marked `@out` are exempt from `@source` checks and can be referenced in `@source`
lists of other fields.

```ilk
type DbMethod = {
    name    Concrete<String>
    args    {...}

    @out
    returns {...}  // data provided by DB, not consumed
}

type Endpoint = {
    @source [params, body]
    db DbMethod

    @source [db.returns]   // can reference @out field
    response {...}
}
```

Without `@out`, a field like `returns` would appear to be missing a `@source` constraint.
The annotation disambiguates intentional output fields from accidental omissions.

### `@constraint`

An inline boolean predicate that every instance of the enclosing type must satisfy.
Uses the constraint expression language (see [Constraint expression language](#constraint-expression-language)).

```ilk
type QueryItem = {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))

    eventTypes []Event
    tags       []Tag
}
```

### `@doc`

`@doc "..."` attaches documentation to the following declaration or field. Unlike `//`
comments which are stripped during parsing, `@doc` annotations are preserved in the AST
and emitted by tooling.

```ilk
@doc "multiply qty * unitPrice"
totalAmount Int = compute(fields.qty, fields.unitPrice)

@doc "generate UUID v4 at runtime"
correlationId Uuid*
```

Use `@doc` to provide implementation hints — transformation semantics, generation
strategy, domain context for AI or human implementers. Multiple `@doc` annotations on the
same element concatenate.

---

## Field origins

When `@source` is in effect on a declaration, each field in an instance struct must be
provably traceable to the listed sources. Three **origin annotations** override default
implicit resolution:

| Form | Meaning |
|---|---|
| `fieldName Type*` | **Generated** — value is auto-produced at runtime; provenance not checked |
| `fieldName Type = path` | **Mapped** — value copied from a dot-path in a source field |
| `fieldName Type = compute(path, ...)` | **Computed** — derived from multiple source fields |

### Generated (`Type*`)

```ilk
timestamp Int*
```

The field value is auto-produced at runtime. Provenance is not checked even when
`@source` is in effect.

### Mapped (`Type = path`)

```ilk
customerId Uuid = fields.userId
nestedId   Uuid = fields.user.address.id
```

The value is copied from a source field identified by a **dot-path** walked from the
enclosing type. The root segment must be one of the sources named in `@source`.

### Computed (`Type = compute(path, ...)`)

```ilk
amount Int = compute(fields.quantity, fields.unitAmount)
```

The value is derived from multiple source fields. Paths are comma-separated dot-paths.
At least one path is required. All path roots must satisfy the same `@source` constraint
as mapped fields. Use `compute()` for narrowing mappings (e.g. `String` → `Uuid`) that
require runtime validation.

---

## Instance bindings

A binding assigns a name to a typed instance:

```ilk
name = TypeName body
```

Bindings are:
- **Top-level only** — not nested inside other constructs
- **Unordered** — order does not matter for validation
- **Unique** — each name may be declared at most once

Names follow standard identifier rules and may start with lowercase or uppercase.

```ilk
userIdTag      = Parametrized {userId String}
simpleTag      = Unique "simple-tag"
userRegistered = Event<userIdTag> {
    id   String
    name String
}
```

The `@main` annotation on a binding designates the file's entry point:

```ilk
@main
board = Board {
    commands [registerUser]
}
```

---

## Struct values

A struct value is a `{ ... }` block of named fields separated by **newlines**:

```ilk
{hello Int}

{
    hello   Int
    goodbye String
}
```

Each field is a `name value` pair. The value is a type name, a literal, a reference to a
binding, or another nested struct/list.

---

## List values

A list value is `[ ... ]` with elements separated by **commas** or **newlines**:

```ilk
[userRegistered, other]

[
    userRegistered
    other
]
```

All elements must conform to the type declared in the schema.

**List cardinality validation:**

| Type form | Valid instance |
|-----------|----------------|
| `[]T` | any count |
| `[N]T` | exactly N |
| `[N..]T` | N or more |
| `[N..M]T` | N to M (inclusive) |
| `[..M]T` | 0 to M |

---

## Union values

### Named type branches (discriminated)

When union branches are user-defined named types, write the variant name — it is always
sufficient; the type context provides the union type:

```ilk
// type: type Status = Started | Finished
current = Started { at Timestamp }

// or inline in a list:
history [
    Started  { at Timestamp }
    Finished { at Timestamp }
]
```

### `Concrete<T>` branches

When a union has a `Concrete<T>` branch, wrap it in a named alias so it is discriminable
by name:

```ilk
// type Parametrized = {_ String}
// type Unique = Concrete<String>
// type Tag = Parametrized | Unique
userIdTag = Parametrized {userId String}   // Parametrized branch: named type, one String field
simpleTag = Unique "simple-tag"            // Unique branch: instance picks a concrete string
```

---

## Reference values

When a field has type `&T` (reference to T), the instance value is an unquoted binding name:

```ilk
// type: eventTypes []&Event
eventTypes [cartCreated, itemAdded]
```

The binding must exist in the file and be of type `T`. References are not strings —
`"cartCreated"` (quoted) would not satisfy `&Event`. No data flows through references.

---

## Associated values

When a type is declared with `@assoc [T]`, instances of that type may carry associated
values — named bindings of type `T` — listed in angle brackets immediately after the type
name. When there are no associated values, the angle brackets are omitted entirely
(`Event<>` is **not** valid):

```ilk
// with associations
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id   String
    name String
}

// no associations — angle brackets absent
other = Event {
    hello String
}
```

The angle-bracket list is **not** a generic type parameter. It is a set of references to
named bindings. The `.assoc(t)` predicate in `@constraint` expressions tests membership
in this set.

---

## Optional fields

Appending `?` to a field name marks it as optional — downstream mappings cannot rely on
its presence:

```ilk
fields {
    id    String
    email? String   // optional — may be absent, don't rely on it
}
```

**Validation rule:** A required target field cannot map to an optional source field via
`@source`:

```ilk
// ERROR: required field relies on optional source
emits [userRegistered & {
    email String = fields.email   // fields.email is optional
}]

// OK: optional target can map to optional source
emits [userRegistered & {
    email? String = fields.email  // both optional
}]

// OK: use compute() for explicit handling
emits [userRegistered & {
    email String = compute(fields.email)  // runtime handles absence
}]
```

---

## Anonymous struct instantiation

When a field or list element has an unambiguous expected type from the schema, the type
name may be omitted and an anonymous struct `{ ... }` supplied directly. Structural typing
validates that the struct matches the expected type:

```ilk
// type: query []QueryItem
// QueryItem type name omitted — struct matches structurally
query [
    {
        eventTypes [userRegistered, other]
        tags       [commonTag]
    }
]
```

This is only valid when the expected element type is a single unambiguous named type
(not a union). For union-typed lists, write the branch name explicitly.

---

## Imports

A file may import types from another `.ilk` file:

```ilk
import "./base-types.ilk"
import "./common-tags.ilk" as tags   // namespaced: tags.SomeType
```

All types in a file are automatically exported — no explicit export annotation needed.
Files without a `@main` instance are pure type libraries.

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
| Type body entries | Newlines |
| List values (`[ ... ]`) | Commas or newlines |
| Annotation arguments (`[...]`) | Commas or newlines |

---

## Full example

### DCB board (`dcb-board.ilk`)

```ilk
// Type declarations

// Parametrized wraps exactly one field of any name, constrained to type String.
// Anonymous struct expressions are not valid as union branches — a named type is required.
type Parametrized = {_ String}
// Unique is a named alias for Concrete<String>, making the branch discriminable by name.
type Unique = Concrete<String>
// Tag is either a Parametrized struct or a Unique concrete string.
type Tag = Parametrized | Unique

// Event has any fields plus a required timestamp; instances may carry Tag associations.
@assoc [Tag]
type Event = {...} & {timestamp Int}

// QueryItem: every event in eventTypes must be associated with every tag in tags.
type QueryItem = {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))

    eventTypes []Event
    tags       []Tag
}

// Scenario: a named test scenario with a list of refinable Event references.
type Scenario = {
    name  Concrete<String>
    given []-Event
}

// Command: fields drive emits; query and scenarios have no source constraint.
type Command = {
    fields {...}

    query     []QueryItem
    scenarios []Scenario

    @source [fields]
    emits []Event
}

// Board is the entry point.
type Board = {
    commands []Command
}


// Instance bindings

userIdTag   = Parametrized {userId String}
userNameTag = Parametrized {name String}
commonTag   = Parametrized {x String}
simpleTag   = Unique "simple-tag"

// Event bindings with their associated tags.
// Event carries @assoc [Tag] — supply tags in angle brackets.
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id   String
    name String
}

other = Event<commonTag, simpleTag> {
    bla String
}

// Command binding.
registerUser = Command {
    fields {
        id    String
        hello String
        x     String
        bla   String
    }

    // @source [fields] is in effect on emits.
    // other.bla traces implicitly to fields.bla.
    emits [other]

    // query has no @source — no provenance constraint.
    // QueryItem type name omitted — anonymous struct matches structurally.
    query [
        {
            eventTypes [userRegistered, other]
            tags       [commonTag]
        }
    ]

    // Refinable Event references — & {field "literal"} allowed because []-Event.
    scenarios [
        {
            name  "happy path"
            given [userRegistered & {id "123"}, userRegistered]
        }
    ]
}

@main
board = Board {
    commands [registerUser]
}
```

---

## API endpoint example

### Schema and instances (`api.ilk`)

```ilk
// Type declarations

// HTTP method union — literal branches matched by value syntax
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"

// Database method abstraction — returns is an output field
type DbMethod = {
    name    Concrete<String>
    args    {...}

    @out
    returns {...}
}

// Response: each endpoint declares one or more possible responses
type Response = {
    status Concrete<Int>
    body   {...}
}

// API endpoint with data flow constraints
type Endpoint = {
    @constraint forall(templateVars(path), v => v in keys(params))
    path    Concrete<String>
    method  HttpMethod
    params  {...}
    body    {...}

    @source [params, body]
    db DbMethod

    @source [params, body, db.returns]
    responses []Response
}

@main
type Api = {
    endpoints []Endpoint
}


// Instance bindings

insertUser = DbMethod {
    name    "users.insert"
    args    {name String, email String}
    returns {id Uuid, name String, email String}
}

findUser = DbMethod {
    name    "users.findById"
    args    {id Uuid}
    returns {id Uuid, name String, email String}
}

createUser = Endpoint {
    path   "/users"
    method "POST"
    body   {name String, email String}

    // @source [params, body] in effect — db.args traces to body
    db insertUser & {
        name  String = body.name
        email String = body.email
    }

    // status codes and static messages are Concrete<T> — exempt from @source
    responses [
        {
            status 201
            body {
                id    Uuid   = db.returns.id
                name  String = db.returns.name
                email String = db.returns.email
            }
        }
        {
            status 422
            body { message "Validation failed" }
        }
    ]
}

getUser = Endpoint {
    path   "/users/{id}"
    method "GET"
    params {id Uuid}

    db findUser & {
        id Uuid = params.id
    }

    responses [
        {
            status 200
            body {
                id   Uuid   = db.returns.id
                name String = db.returns.name
            }
        }
        {
            status 404
            body { message "User not found" }
        }
    ]
}

@main
api = Api {
    endpoints [createUser, getUser]
}
```

Data flows through each endpoint:
- `params`/`body` → `@source` → `db.args`
- `db.returns` → `@source` → `responses[*].body` (for data-carrying responses)
- `Concrete<Int>` status codes and `Concrete<String>` error messages are author-chosen and
  exempt from `@source`
