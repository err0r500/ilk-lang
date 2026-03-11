# ilk/kli Language Reference

Two-level data modeling: `.ilk` (schema) validates `.kli` (domain model).

Full specs: [ilk-spec.md](ilk-spec.md) | [kli-spec.md](kli-spec.md)

## Quick Reference

### Base Types
`*` `Uuid` `String` `Int` `Float` `Bool` `Date` `Timestamp` `Money`

### Value Constraints
| ilk | kli | meaning |
|-----|-----|---------|
| `String` | `String` | open - any value at runtime |
| `Concrete<String>` | `"hello"` | kli-fixed - author picks one |
| `"hello"` | `"hello"` | schema-fixed - exact match |

Constraint levels must match exactly. kli cannot narrow open→concrete (would change runtime contract).

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
&Event           // reference to binding (no data flow)
```

### Unions
```ilk
HttpMethod "GET" | "POST" | "PUT"   // literal union
Status Pending | Active | Archived  // identifier union
Response Success | Error            // block union
```

### Annotations
| Annotation | Target | Purpose |
|------------|--------|---------|
| `@main` | block | entry point for .kli validation |
| `@assoc [T]` | block | instances carry refs to T |
| `@source [fields]` | field/list | data provenance constraint |
| `@out` | field | output field - exempt from @source, can be referenced |
| `@constraint expr` | block | boolean predicate |
| `@doc "..."` | field (kli) | implementation hint |

### @source & Field Origins (kli)
```kli
timestamp Int*                    // generated - no check
customerId Uuid = fields.userId   // mapped from path
total Int = compute(a, b)         // computed from multiple
```
Paths resolve from enclosing block root.

### Constraint Functions
`forall(col, x => body)` `exists(col, x => body)` `unique(col, x => expr)` `count(col)` `e.assoc(t)` `templateVars(str)` `keys(struct)`

### Constraint Operators
`&&` `||` `!` `==` `!=` `in` `<` `<=` `>` `>=`

## Syntax Rules
- Comments: `//`
- Separators: newlines (or commas inline for structs/lists)
- Optional fields (kli only): `email? String` — can't be relied upon by @source
- Intersection: `A & B` (right side wins on conflict)
- Bindings (kli): `name = Type body`
- Associations (kli): `Event<tag1, tag2> {...}`

## Example Pattern: API Endpoint
```ilk
Endpoint {
    @constraint forall(templateVars(path), v => v in keys(params))
    path    Concrete<String>
    method  "GET" | "POST"
    params? {...}
    body?   {...}

    @source [params, body]
    db DbMethod

    response {
        @source [db.returns]
        body {...}
    }
}
```
