---
layout: home
hero:
  name: ilk / kli
  text: Two-level data modeling
  tagline: Where data provenance matters
  actions:
    - theme: brand
      text: ilk Schema Language
      link: /ilk-spec
    - theme: alt
      text: kli Domain Model
      link: /kli-spec
features:
  - title: Data Flow Validation
    details: Traditional schemas validate shape. ilk/kli validates data flow — where does each field come from?
  - title: Two-Level System
    details: ilk (.ilk) defines abstract vocabulary with @source constraints. kli (.kli) instantiates it with concrete entities.
  - title: Not Runtime Data
    details: Neither level holds runtime data. A .kli file is a domain model — a catalog of named entities and structures.
---

## Quick Example

### API Endpoint with Data Flow

<div class="code-compare">

```ilk
// we define the vocabulary for our API schema in .ilk

HttpMethod GET | POST | PUT | DELETE

DbMethod {
    name    Concrete<String>
    args    {...}

    @out // this field won't be check for @source constraints
    returns {...}
}

HttpResponse {
    status Concrete<Int>
    body {...}
}

Endpoint {
    // ensure that all fields used in path template are declared as params
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

@main
Api {
    endpoints []Endpoint
}
```

```kli
// then implement it in .kli-spec

findUser = DbMethod {
    name    "users.findById"
    args    {userId Uuid}

    returns {id Uuid, name String}
}

getUser = Endpoint {
    path   "/users/{id}" // template variable "id" is actually declared in params
    method GET // one of the HttpMethod literals
    params {id Uuid}

    db findUser & {
        // because field name doen't match,
        // we have to explicitly tell the validator where to find userId
        userId = params.id
    }

    // uses data from params and db.returns to construct responses
    responses [
        HttpResponse {
            status 200
            body {
                id   Uuid   = db.returns.id
                name String = db.returns.name
            }
        }
        HttpResponse {
            status 404
            body {
                error  "User not found"
                userId Uuid = params.id
            }
        }
    ]
}

Api {
    endpoints [getUser]
}
```

</div>

## Quick Reference

### Base Types
`*` `Uuid` `String` `Int` `Float` `Bool` `Date` `Timestamp` `Money`

### Value Constraints
| ilk | kli | meaning |
|-----|-----|---------|
| `String` | `String` | open - any value at runtime |
| `Concrete<String>` | `"hello"` | kli-fixed - author picks one |
| `"hello"` | `"hello"` | schema-fixed - exact match |

### Structs
```ilk
{_}              // exactly 1 field (= {_ Any})
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
