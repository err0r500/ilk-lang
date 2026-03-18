<script setup>
// Value constraint levels
const tcConstraints = `type Config = {
    name     String           // open — any string at runtime
    mode!    Concrete<String> // instance-fixed — instance picks one value
    version! 1                // type-fixed — must be exactly 1
}`
const iConstraints = [
  { label: 'Valid', expect: 'pass', code:
`cfg = Config {
    name    String
    mode    "production"
    version 1
}` },
  { label: 'Any Concrete value', expect: 'pass', code:
`cfg = Config {
    mode    "development"
    version 1
}` },
  { label: 'Wrong type-fixed value', expect: 'fail', code:
`cfg = Config {
    mode    "prod"
    version 2
}` },
  { label: 'Open type for Concrete field', expect: 'fail', code:
`cfg = Config {
    mode    String
    version 1
}` },
]

// Struct types — closed
const tcClosed = `type Point = {
    x! Int
    y! Int
}`
const iClosed = [
  { label: 'Valid', expect: 'pass', code:
`p = Point {
    x Int
    y Int
}` },
  { label: 'Missing required field', expect: 'fail', code:
`p = Point {
    x Int
}` },
  { label: 'Extra field', expect: 'fail', code:
`p = Point {
    x Int
    y Int
    z Int
}` },
  { label: 'Wrong field type', expect: 'fail', code:
`p = Point {
    x String
    y Int
}` },
]

// Struct types — open & anonymous
const tcOpen = `type Flexible = {...}

type Single = {_ String}

type Pair = {_ Int, _ String}`
const iOpen = [
  { label: 'Open: any fields', expect: 'pass', code:
`data = Flexible {
    name  String
    count Int
    flag  Bool
}` },
  { label: 'Anonymous: one field of any name', expect: 'pass', code:
`tag = Single { userId String }` },
  { label: 'Pair: two typed fields', expect: 'pass', code:
`coords = Pair { count Int, label String }` },
  { label: 'Single: too many fields', expect: 'fail', code:
`broken = Single {
    name  String
    extra Int
}` },
]

// List cardinality
const tcList = `type Item = Concrete<String>

type Bag = {
    contents! [1..3]Item
}`
const iList = [
  { label: 'One item', expect: 'pass', code:
`item = Item "apple"
bag  = Bag { contents [item] }` },
  { label: 'Three items (max)', expect: 'pass', code:
`i1 = Item "apple"
i2 = Item "cherry"
i3 = Item "peach"
bag = Bag { contents [i1, i2, i3] }` },
  { label: 'Empty — below minimum', expect: 'fail', code:
`bag = Bag { contents [] }` },
  { label: 'Four items — above maximum', expect: 'fail', code:
`i1 = Item "a"
i2 = Item "b"
i3 = Item "c"
i4 = Item "d"
bag = Bag { contents [i1, i2, i3, i4] }` },
]

// Reference types
const tcRef = `type Color = Concrete<String>

type Theme = {
    primary!  &Color
    secondary &Color
    all!      []&Color
}`
const iRef = [
  { label: 'Valid references', expect: 'pass', code:
`red   = Color "red"
blue  = Color "blue"
green = Color "green"

t = Theme {
    primary   red
    secondary blue
    all       [red, blue, green]
}` },
  { label: 'Without optional secondary', expect: 'pass', code:
`red = Color "red"

t = Theme {
    primary red
    all     [red]
}` },
  { label: 'Undefined binding', expect: 'fail', code:
`t = Theme {
    primary phantom
    all     []
}` },
  { label: 'Wrong type', expect: 'fail', code:
`type Font = Concrete<String>
serif = Font "serif"

t = Theme {
    primary serif
    all     [serif]
}` },
]

// Literal unions
const tcLiteralUnion = `type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"

type Endpoint = {
    method! HttpMethod
    path!   Concrete<String>
}`
const iLiteralUnion = [
  { label: 'GET', expect: 'pass', code:
`ep = Endpoint {
    method "GET"
    path   "/users"
}` },
  { label: 'DELETE', expect: 'pass', code:
`ep = Endpoint {
    method "DELETE"
    path   "/users/{id}"
}` },
  { label: 'PATCH — not in union', expect: 'fail', code:
`ep = Endpoint {
    method "PATCH"
    path   "/users/{id}"
}` },
  { label: 'Lowercase — not matched', expect: 'fail', code:
`ep = Endpoint {
    method "get"
    path   "/users"
}` },
]

// Named-type unions
const tcNamedUnion = `type Pending  = { queue Concrete<String> }
type Active   = { since Timestamp }
type Archived = { at Timestamp }

type Status = Pending | Active | Archived

type Job = {
    name!   Concrete<String>
    status! Status
}`
const iNamedUnion = [
  { label: 'Pending branch', expect: 'pass', code:
`j = Job {
    name   "my-job"
    status Pending { queue "default" }
}` },
  { label: 'Active branch', expect: 'pass', code:
`j = Job {
    name   "my-job"
    status Active { since Timestamp }
}` },
  { label: 'Archived branch', expect: 'pass', code:
`j = Job {
    name   "my-job"
    status Archived { at Timestamp }
}` },
  { label: 'Unknown variant', expect: 'fail', code:
`j = Job {
    name   "my-job"
    status Cancelled { at Timestamp }
}` },
]

// Intersection types
const tcIntersection = `// Open struct extended with a required id field
type Entity = {...} & {id Uuid}

// Right side wins when both sides name the same field
type Stamped = {timestamp String} & {timestamp Int}`
const iIntersection = [
  { label: 'Entity: id + extra fields', expect: 'pass', code:
`e = Entity {
    id   Uuid
    name String
}` },
  { label: 'Entity: only required id', expect: 'pass', code:
`e = Entity {
    id Uuid
}` },
  { label: 'Entity: missing required id', expect: 'fail', code:
`e = Entity {
    name String
}` },
  { label: 'Stamped: right side wins (Int)', expect: 'pass', code:
`s = Stamped {
    timestamp Int
}` },
]

// @assoc associated values
const tcAssoc = `type Tag = Concrete<String>

@assoc [Tag]
type Event = {...}

type Log = {
    events! []&Event
}`
const iAssoc = [
  { label: 'Events with tags', expect: 'pass', code:
`urgentTag = Tag "urgent"
userTag   = Tag "user"

login  = Event<urgentTag, userTag> { action String }
logout = Event<userTag>            { action String }

log = Log { events [login, logout] }` },
  { label: 'Event without tags', expect: 'pass', code:
`ping = Event { action String }
log  = Log { events [ping] }` },
  { label: 'Mixed: tagged and untagged', expect: 'pass', code:
`urgentTag = Tag "urgent"
alert = Event<urgentTag> { action String }
info  = Event            { action String }

log = Log { events [alert, info] }` },
  { label: 'Wrong type for associated value', expect: 'fail', code:
`type Category = Concrete<String>
work = Category "work"

click = Event<work> { action String }
log   = Log { events [click] }` },
]

// @source — basic tracing
const tcSource = `type Form = {
    inputs {...}

    @source [inputs]
    output {...}
}`
const iSource = [
  { label: 'Field name matches source', expect: 'pass', code:
`f = Form {
    inputs { name String }
    output { name String }
}` },
  { label: 'Multiple source fields', expect: 'pass', code:
`f = Form {
    inputs { id Uuid, name String }
    output { id Uuid, name String }
}` },
  { label: 'Field not in source', expect: 'fail', code:
`f = Form {
    inputs { id Uuid }
    output { id Uuid, extra Float }
}` },
  { label: 'Empty output', expect: 'pass', code:
`f = Form {
    inputs { id Uuid }
    output {}
}` },
]

// @out annotation
const tcOut = `type DbQuery = {
    name Concrete<String>
    args {...}

    @out
    returns {...}
}

type Endpoint = {
    params {...}

    @source [params]
    db DbQuery

    @source [db.returns]
    response {...}
}`
const iOut = [
  { label: 'Valid data flow', expect: 'pass', code:
`ep = Endpoint {
    params { id Uuid }
    db {
        name    "users.findById"
        args    { id Uuid = params.id }
        returns { name String, email String }
    }
    response {
        name  String
        email String
    }
}` },
  { label: 'Different fields from returns', expect: 'pass', code:
`ep = Endpoint {
    params { userId Uuid }
    db {
        name    "users.getProfile"
        args    { id Uuid = params.userId }
        returns { username String, bio String }
    }
    response {
        username String
        bio      String
    }
}` },
  { label: 'Response field not in returns', expect: 'fail', code:
`ep = Endpoint {
    params { id Uuid }
    db {
        name    "users.find"
        args    { id Uuid = params.id }
        returns { name String }
    }
    response {
        name    String
        unknown Int
    }
}` },
  { label: 'Args field not from params', expect: 'fail', code:
`ep = Endpoint {
    params { id Uuid }
    db {
        name    "users.find"
        args    { query String }
        returns { name String }
    }
    response {
        name String
    }
}` },
]

// @constraint
const tcConstraint = `type Tag = Concrete<String>

type List = {
    @constraint count(items) >= 2
    @constraint count(items) <= 4
    items! []Tag
}`
const iConstraint = [
  { label: '2 items (minimum)', expect: 'pass', code:
`a = Tag "alpha"
b = Tag "beta"
list = List { items [a, b] }` },
  { label: '4 items (maximum)', expect: 'pass', code:
`a = Tag "alpha"
b = Tag "beta"
c = Tag "gamma"
d = Tag "delta"
list = List { items [a, b, c, d] }` },
  { label: '1 item — below minimum', expect: 'fail', code:
`a = Tag "alone"
list = List { items [a] }` },
  { label: '5 items — above maximum', expect: 'fail', code:
`a = Tag "a"
b = Tag "b"
c = Tag "c"
d = Tag "d"
e = Tag "e"
list = List { items [a, b, c, d, e] }` },
]

// Field origins
const tcOrigins = `type Request = {
    params {...}
    body   {...}

    @source [params, body]
    result {...}
}`
const iOrigins = [
  { label: 'Implicit name matching', expect: 'pass', code:
`r = Request {
    params { id Uuid, name String }
    body   {}
    result {
        id   Uuid
        name String
    }
}` },
  { label: 'Explicit mapping (= path)', expect: 'pass', code:
`r = Request {
    params { userId Uuid }
    body   { userName String }
    result {
        id   Uuid   = params.userId
        name String = body.userName
    }
}` },
  { label: 'Generated (*) and computed', expect: 'pass', code:
`r = Request {
    params { x Int }
    body   { y Int }
    result {
        reqId Uuid*
        total Int = compute(params.x, body.y)
    }
}` },
  { label: 'Unmapped field — no source', expect: 'fail', code:
`r = Request {
    params { id Uuid }
    body   {}
    result {
        id      Uuid
        unknown Float
    }
}` },
]

// Optional fields
const tcOptional = `type Profile = {
    id!      Uuid
    name!    String
    email?   String
    website? String
}`
const iOptional = [
  { label: 'All fields', expect: 'pass', code:
`p = Profile {
    id      Uuid
    name    String
    email   String
    website String
}` },
  { label: 'Only required fields', expect: 'pass', code:
`p = Profile {
    id   Uuid
    name String
}` },
  { label: 'Some optional fields', expect: 'pass', code:
`p = Profile {
    id    Uuid
    name  String
    email String
}` },
  { label: 'Missing required name', expect: 'fail', code:
`p = Profile {
    id    Uuid
    email String
}` },
]
</script>

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
userIdTag = Tag {userId String} // inline comment
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

<TypeExample :typeCode="tcConstraints" :instances="iConstraints" />

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

<TypeExample :typeCode="tcClosed" :instances="iClosed" />

<TypeExample :typeCode="tcOpen" :instances="iOpen" />

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

<TypeExample :typeCode="tcList" :instances="iList" />

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

<TypeExample :typeCode="tcRef" :instances="iRef" />

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

`A | B` means a value must satisfy **exactly one** of the alternatives. Each branch is a
type expression: a named type reference, an anonymous struct expression (`{_ String}`,
`{...}`, etc.), a `Concrete<T>`, a scalar base type, or a literal.

```ilk
type Success = { value Bool }
type Error   = { message String }
type Response = Success | Error
```

Unions with named-type branches — in instances, the branch is identified by the variant name:

```ilk
result = Success { value Bool }
// or
result = Error { message String }
```

Identifier-only variants (named types with empty bodies) need no body in instances:

```ilk
type Status = Pending | Active | Archived
```

```ilk
status Active
```

Anonymous struct branches are valid — the branch is matched structurally:

```ilk
// Tag is either a {_ String} anonymous struct or a Concrete<String>
type Tag = {_ String} | Concrete<String>
```

In instances, the branch is chosen by the shape of the value:

```ilk
userIdTag = Tag {userId String}   // {_ String} branch — a single-field struct
simpleTag = Tag "simple-tag"      // Concrete<String> branch — a literal string
```

Literal branches (`"GET"`, `42`, etc.) and `Concrete<T>` branches are matched by the
syntax of the instance value. Named type branches are matched by writing the type name.

Literal unions are useful for enumerated string/int values:

```ilk
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"
type Verb       = "POST" | "DELETE" | "PUT" | "PATCH"
```

### Discriminated unions

For named-type branches, every union is discriminated by name. When two branches have the
same shape, the name distinguishes them:

```ilk
type Started  = { at Timestamp }
type Finished = { at Timestamp }
type Status   = Started | Finished   // same shape — unambiguous by name
```

For anonymous struct branches or scalar/literal branches, the instance value's syntactic
form is sufficient to discriminate: `{...}` is a struct, `"literal"` is a string, etc.

<TypeExample :typeCode="tcLiteralUnion" :instances="iLiteralUnion" />

<TypeExample :typeCode="tcNamedUnion" :instances="iNamedUnion" />

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

<TypeExample :typeCode="tcIntersection" :instances="iIntersection" />

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

<TypeExample :typeCode="tcAssoc" :instances="iAssoc" />

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

<TypeExample :typeCode="tcSource" :instances="iSource" />

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

<TypeExample :typeCode="tcOut" :instances="iOut" />

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

<TypeExample :typeCode="tcConstraint" :instances="iConstraint" />

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

<TypeExample :typeCode="tcOrigins" :instances="iOrigins" />

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

### Named type branches

When a union branch is a named type, write the type name in the instance value:

```ilk
// type: type Status = Started | Finished
current = Started { at Timestamp }

// inline in a list:
history [
    Started  { at Timestamp }
    Finished { at Timestamp }
]
```

### Anonymous struct branches

When a union branch is an anonymous struct type (`{_ String}`, `{...}`, etc.), supply a
struct value directly — no type name prefix:

```ilk
// type Tag = {_ String} | Concrete<String>
userIdTag = Tag {userId String}   // {_ String} branch — struct value
simpleTag = Tag "simple-tag"      // Concrete<String> branch — literal
```

### Literal and `Concrete<T>` branches

Literal branches and `Concrete<T>` branches are matched by the value's syntax — a string
literal satisfies a `Concrete<String>` branch, an integer literal satisfies a `Concrete<Int>`
branch, etc.:

```ilk
// type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"
method "GET"

// type Verb = "POST" | "DELETE" | "PUT" | "PATCH"
verb "DELETE"
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

`?` appended to a field name marks it as optional. The semantics differ between type
declarations and instance bindings.

### Optional in type declarations

`field? Type` in a type declaration means instances are not required to provide this field:

```ilk
type User = {
    id    Uuid
    name  String
    email? String   // instances may omit email
}
```

A missing optional type field does not cause a validation error. When present, it must
match the declared type.

### Optional in instance bindings

`field? value` in an instance binding marks the field as conditionally present at runtime.
Downstream `@source` checks treat an optional source field as unreliable:

```ilk
fields {
    id    String
    email? String   // may be absent at runtime
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

<TypeExample :typeCode="tcOptional" :instances="iOptional" />

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

// Tag: either a single-field String struct or a concrete string literal.
// Anonymous struct and Concrete<T> branches are valid directly in the union.
type Tag = {_ String} | Concrete<String>

// Event has any fields plus a required timestamp; instances carry Tag associations.
@assoc [Tag]
type Event = {...} & {timestamp Int}

// QueryItem: every event in eventTypes must be associated with every tag in tags.
type QueryItem = {
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))
    eventTypes []&Event   // list of references to Event bindings
    tags []Tag
}

// Command: fields drive both query and emits via @source.
type Command = {
    fields {...}

    @source [fields]
    query []QueryItem

    @source [fields]
    emits []Event
}

type HttpResponse = {
    status Concrete<Int>
    body {...}
}

type HttpEndpoint = {
    method "GET" | "POST" | "DELETE" | "PUT" | "PATCH"

    @constraint forall(templateVars(path), v => v in keys(params))
    path Concrete<String>
    params {...}
    body {...}

    responses []HttpResponse
}

// ChangeSlice bundles an endpoint and a command; endpoint drives command via @source.
type ChangeSlice = {
    name Concrete<String>
    endpoint HttpEndpoint

    @source [endpoint]
    command Command

    scenarios []CommandScenario
}

// CommandScenario: a BDD-style test scenario.
type CommandScenario = {
    name Concrete<String>

    given []-Event   // refinable Event references for preconditions
    when Command
    then []-Event    // refinable Event references for postconditions
}

// Board is the entry point.
type Board = {
    changes []ChangeSlice
}


// Instance bindings

// Tag bindings — {_ String} branch: single-field structs
userIdTag   = Tag {userId String}
userNameTag = Tag {name String}

// Event bindings with their associated tags.
userRegistered = Event<userIdTag, userNameTag> {
    id   Uuid
    name String
    ts   Timestamp
}

// ChangeSlice instance bundling endpoint + command + scenarios.
registerUser = ChangeSlice {
    name "registerUser"

    // HttpEndpoint value — type name omitted (anonymous struct matches structurally)
    endpoint {
        path   "/users/{id}"
        method "POST"
        params { id Uuid }
        body   { name String }
    }

    // Command value — @source [endpoint] is in effect.
    // fields are mapped explicitly from endpoint.params and endpoint.body.
    command {
        fields {
            id   Uuid   = endpoint.params.id
            name String = endpoint.body.name
        }

        // @source [fields] — eventTypes is []&Event (references, no @source check)
        query [
            {
                eventTypes [userRegistered],
                tags [userIdTag & {userId Uuid = fields.id}]   // refinable tag
            },
            {
                eventTypes [userRegistered],
                tags [userNameTag & {name String = fields.name}]
            }
        ]

        // @source [fields] — ts is generated, other fields trace implicitly.
        emits [userRegistered & {ts Timestamp*}]
    }

    scenarios [
        {
            name  "happy path",
            given [userRegistered & {id 123}, userRegistered]
            when  {}
            then  [userRegistered & {id "123"}]
        }
    ]
}

@main
board = Board {
    changes [registerUser]
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
