# Language Features

Interactive examples for each language feature. Click tabs to see what the compiler accepts and rejects.

<script setup>
// 1 — Value constraint levels
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

// 2 — Closed structs
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
  { label: 'Extra field not in type', expect: 'fail', code:
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

// 3 — Open & anonymous structs
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
  { label: 'Anonymous: one field', expect: 'pass', code:
`tag = Single { userId String }` },
  { label: 'Pair: two typed fields', expect: 'pass', code:
`coords = Pair { count Int, label String }` },
  { label: 'Single: too many fields', expect: 'fail', code:
`broken = Single {
    name  String
    extra Int
}` },
]

// 4 — List cardinality
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

// 5 — Literal unions
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
  { label: 'Lowercase not matched', expect: 'fail', code:
`ep = Endpoint {
    method "get"
    path   "/users"
}` },
]

// 6 — Named-type unions
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

// 7 — Intersection types
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

// 8 — Optional fields
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

// 9 — Reference types
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

// 10 — Field origins
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

// 11 — @constraint
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

// 12 — @assoc associated values
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

// 13 — @out annotation
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
</script>

## Value Constraint Levels

Three forms control how tightly a field's value is fixed at type-definition time vs. instance time vs. runtime.

| Form | Constraint | Meaning |
|------|-----------|---------|
| `String` | Open | Any value at runtime |
| `Concrete<String>` | Instance-fixed | Instance author picks one value |
| `"hello"` | Type-fixed | Only this exact value is valid |

<TypeExample :typeCode="tcConstraints" :instances="iConstraints" />

## Closed Structs

A struct with named fields requires **exactly** those fields — no extras, no missing required ones.

<TypeExample :typeCode="tcClosed" :instances="iClosed" />

## Open and Anonymous Structs

`{...}` accepts any fields. `{_ T}` accepts exactly one field of any name with type `T`. `{_ T, _ U}` accepts exactly two typed fields.

<TypeExample :typeCode="tcOpen" :instances="iOpen" />

## List Cardinality

List types carry cardinality constraints: `[]T` (any), `[N]T` (exact), `[N..]T` (min), `[N..M]T` (range), `[..M]T` (max).

<TypeExample :typeCode="tcList" :instances="iList" />

## Literal Unions

A union of string (or integer) literals — the instance value must match one of the alternatives exactly.

<TypeExample :typeCode="tcLiteralUnion" :instances="iLiteralUnion" />

## Named-Type Unions

A union of named types — instances select a branch by writing the type name.

<TypeExample :typeCode="tcNamedUnion" :instances="iNamedUnion" />

## Intersection Types

`A & B` merges both struct shapes. Every field from both sides is required. When both sides declare the same field, the right side wins.

<TypeExample :typeCode="tcIntersection" :instances="iIntersection" />

## Optional Fields

`field? Type` marks a field as optional in the type declaration — instances may omit it.

<TypeExample :typeCode="tcOptional" :instances="iOptional" />

## Reference Types

`&T` is a reference to an existing binding of type `T`. No data flows through references — the compiler checks the binding exists and has the right type.

<TypeExample :typeCode="tcRef" :instances="iRef" />

## Field Origins

When `@source` is in effect, each field in a struct must be traceable to the declared sources. Three origin forms override implicit name-matching:

| Form | Meaning |
|------|---------|
| `field Type*` | Generated at runtime — exempt from source checks |
| `field Type = path` | Copied from a source field path |
| `field Type = compute(paths…)` | Derived from multiple source fields |

<TypeExample :typeCode="tcOrigins" :instances="iOrigins" />

## `@constraint` Expressions

`@constraint` attaches a boolean predicate to a type. Every instance must satisfy it at compile time.

<TypeExample :typeCode="tcConstraint" :instances="iConstraint" />

## `@assoc` Associated Values

`@assoc [T]` allows instances to carry named bindings of type `T` in angle brackets. The `.assoc(t)` predicate in `@constraint` expressions tests membership.

<TypeExample :typeCode="tcAssoc" :instances="iAssoc" />

## `@out` Annotation

`@out` marks a field as an **output** — data flows out from it, not into it. It is exempt from `@source` checks and can be referenced as a source by downstream fields.

<TypeExample :typeCode="tcOut" :instances="iOut" />
