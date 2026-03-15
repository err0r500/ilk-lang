---
layout: home
hero:
  name: ilk
  text: Domain modeling in one file
  tagline: Define your types. Name your entities. Trace your data.
  actions:
    - theme: brand
      text: Language Specification
      link: /spec
    - theme: alt
      text: Interactive Examples
      link: /index.html
features:
  - icon: 🗂️
    title: Types and instances together
    details: One .ilk file holds both type declarations (what shapes exist) and instance bindings (the named entities in your domain). No separate schema files — the catalog lives with the model.
  - icon: 🔍
    title: Data provenance, not just shape
    details: "@source declares where a field's data must come from. Every response field must be traced back to a declared source — params, request body, or DB result. The compiler catches missing mappings."
  - icon: ✅
    title: Compile-time validation
    details: Structure, data flow, and domain constraints are all checked before runtime. If a field is untraced, a path is invalid, or a constraint fails, you get a clear error at compile time.
---

## The idea in 30 seconds

A `.ilk` file has two sections: **type declarations** and **instance bindings**.

- **Types** define abstract shapes — what fields exist, what types they have, what constraints apply.
- **Instances** name the concrete entities in *your* domain — not runtime values, but a typed catalog of the things your system knows about.

```ilk
// Type declaration
type User = {
    id    Uuid
    name  String
    role  "admin" | "guest"
}

// Instance bindings — the users that exist in this domain
adminUser = User {
    id   Uuid
    name String
    role "admin"   // type-fixed: must be exactly "admin"
}

guestUser = User {
    id   Uuid
    name String
    role "guest"
}
```

## The key feature: data flow

Traditional schemas validate *shape*. ilk also validates *where data comes from*.

`@source` declares which upstream fields may supply data to a downstream field.
Every open field must be explicitly mapped — the compiler checks it.

```ilk
type Endpoint = {
    params  {...}
    body    {...}

    @source [params, body]       // db args must come from the request
    db      DbMethod

    @source [params, body, db.returns]   // responses may use db results too
    responses []HttpResponse
}

// In an instance, every field assignment is checked against @source
getUser = Endpoint {
    params {id Uuid}
    body   {}

    db findUserDb & {
        userId Uuid = params.id    // ✓ params is in @source
    }

    responses [
        {
            status 200             // Concrete<Int> — exempt from @source
            body {
                id   Uuid   = db.returns.id     // ✓
                name String = db.returns.name   // ✓
            }
        }
        {
            status 404
            body {message "User not found"}     // ✓ literal — exempt
        }
    ]
}
```

## Quick reference

### Value constraint levels

| Type form | Instance form | Meaning |
|-----------|---------------|---------|
| `String` | `String` | Open — any value at runtime |
| `Concrete<String>` | `"webhook"` | Instance-fixed — instance author picks one |
| `"POST"` | `"POST"` | Type-fixed — must match exactly |

### Structs
```ilk
{_}              // exactly 1 field of any name/type
{_ String}       // exactly 1 field, type String
{...}            // any fields (open)
{id Uuid}        // specific named fields
{...} & {id Uuid} // any fields + required id
```

### Lists
```ilk
[]Event      // 0+ elements
[3]Tag       // exactly 3
[1..]Tag     // 1 or more
[2..5]Tag    // 2 to 5
[..10]Tag    // up to 10
&Event       // reference to a binding (no data flow)
[]&Event     // list of references
```

### Unions
```ilk
type HttpMethod = "GET" | "POST" | "PUT"   // literal union
type Status = Pending | Active | Archived  // identifier union
type Tag = {_ String} | Concrete<String>   // mixed
```

### Annotations
| Annotation | Purpose |
|------------|---------|
| `@main` | Entry point — compiler validates from here |
| `@source [fields]` | Data provenance: values must trace back to *fields* |
| `@out` | Output field — exempt from `@source`, can be referenced by callers |
| `@assoc [T]` | Instances carry typed associations (used with `@constraint`) |
| `@constraint expr` | Boolean predicate validated at compile time |
| `@doc "…"` | Implementation hint, preserved in AST |
