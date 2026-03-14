# ilk Language Reference

Single-file data modeling: `.ilk` files contain both type declarations and instances.

## Quick Reference

### Base Types
`*` `Uuid` `String` `Int` `Float` `Bool` `Date` `Timestamp` `Money`

### Value Constraints
| type | instance | meaning |
|------|----------|---------|
| `String` | `String` | open - any value at runtime |
| `Concrete<String>` | `"hello"` | concrete - author picks literal |
| `"hello"` | `"hello"` | schema-fixed - exact match |

### Type Declarations
```ilk
type Foo = {x Int}                    // type declaration
type Bar = {...} & {id Uuid}          // intersection
type Status = Pending | Active        // union
```

### Structs
```ilk
{_}              // exactly 1 field (= {_ *})
{_ String}       // exactly 1 field, type String
{_, _}           // exactly 2 fields
{_ Int, _ String}  // exactly 2 fields with specific types
{...}            // any fields
{id Uuid}        // specific fields
{...} & {id Uuid} // any fields + required id
```

### Lists & References
```ilk
[]Event          // 0+ events
[3]Tag           // exactly 3
[1..]Tag         // 1+ tags
[2..5]Tag        // 2 to 5 tags
[..10]Tag        // 0 to 10 tags
&Event           // reference to instance (no data flow)
```

### Unions
```ilk
type HttpMethod = "GET" | "POST" | "PUT"   // literal union
type Status = Pending | Active | Archived  // identifier union
type Response = Success | Error            // block union
```

### Annotations
| Annotation | Target | Purpose |
|------------|--------|---------|
| `@main` | instance | entry point for validation |
| `@assoc [T]` | type | instances carry refs to T |
| `@source [fields]` | field/list | data provenance constraint |
| `@out` | field | output field - exempt from @source |
| `@constraint expr` | type | boolean predicate |
| `@doc "..."` | field | implementation hint |

### Instances
```ilk
tag1 = Tag {userId String}            // instance declaration
event = Event<tag1> {id String}       // instance with associations
@main board = Board {commands [...]}  // entry point
```

### Field Origins (instances)
```ilk
timestamp Int*                    // generated - no check
customerId Uuid = fields.userId   // mapped from path
total Int = compute(a, b)         // computed from multiple
```

### Imports
```ilk
import "./base-types.ilk"
import "./common-tags.ilk" as tags  // namespaced
```

### Constraint Functions
`forall(col, x => body)` `exists(col, x => body)` `unique(col, x => expr)` `count(col)` `e.assoc(t)` `templateVars(str)` `keys(struct)`

### Constraint Operators
`&&` `||` `!` `==` `!=` `in` `<` `<=` `>` `>=`

## Syntax Rules
- Comments: `//`
- Separators: newlines (or commas inline for structs/lists)
- Optional fields: `email? String`
- Intersection: `A & B` (right side wins on conflict)

## Example
```ilk
type Tag = {_ String}

@assoc [Tag]
type Event = {...} & {timestamp Int}

type Command = {
    fields {...}
    @source [fields]
    emits []Event
}

type Board = { commands []Command }

tag1 = Tag {userId String}
ev1 = Event<tag1> {id String}

cmd = Command {
    fields {id String, userId String}
    emits [ev1 & {timestamp Int*}]
}

@main
board = Board { commands [cmd] }
```
