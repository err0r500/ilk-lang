---
layout: home
hero:
  name: ilk
  text: Single-file data modeling
  tagline: Where data provenance matters
  actions:
    - theme: brand
      text: Language Specification
      link: /spec
features:
  - title: Data Flow Validation
    details: Traditional schemas validate shape. ilk validates data flow — where does each field come from?
  - title: Single File
    details: Type declarations and instance bindings live in one .ilk file. Types define the abstract vocabulary; instances name the concrete entities.
  - title: Not Runtime Data
    details: A .ilk file is a domain model — a catalog of named entities and structures. Runtime values live downstream.
---

## Quick Example

### API Endpoint with Data Flow

```ilk
// Type declarations

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"

type DbMethod = {
    name    Concrete<String>
    args    {...}

    @out // data flows out — exempt from @source checks
    returns {...}
}

type HttpResponse = {
    status Concrete<Int>
    body   {...}
}

type Endpoint = {
    // ensure all template variables are declared in params
    @constraint forall(templateVars(path), v => v in keys(params))
    path    Concrete<String>
    method  HttpMethod
    params  {...}
    body    {...}

    @source [params, body]
    db DbMethod

    @source [params, body, db.returns]
    responses []HttpResponse
}

type Api = {
    endpoints []Endpoint
}


// Instance bindings

findUser = DbMethod {
    name    "users.findById"
    args    {userId Uuid}
    returns {id Uuid, name String}
}

getUser = Endpoint {
    path   "/users/{id}"
    method "GET"
    params {id Uuid}

    // field name differs from source — explicit mapping required
    db findUser & {
        userId Uuid = params.id
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
            body {
                error  "User not found"
                userId Uuid = params.id
            }
        }
    ]
}

@main
api = Api {
    endpoints [getUser]
}
```

## Quick Reference

### Base Types
`*` `Uuid` `String` `Int` `Float` `Bool` `Date` `Timestamp` `Money`

### Value Constraints
| type | instance | meaning |
|------|----------|---------|
| `String` | `String` | open - any value at runtime |
| `Concrete<String>` | `"hello"` | instance-fixed - author picks one |
| `"hello"` | `"hello"` | type-fixed - exact match |

### Structs
```ilk
{_}              // exactly 1 field (= {_ *})
{_ String}       // exactly 1 field, type String
{_, _}           // exactly 2 fields
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
| `@out` | field | output field - exempt from @source, can be referenced |
| `@constraint expr` | type | boolean predicate |
| `@doc "..."` | field | implementation hint |
