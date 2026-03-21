---
layout: home
hero:
  name: ilk
  text: Model your Domain, type check your data flow
  tagline: Example concrete usecase is declare a json-schema with higher-level constraints it has to respect
  actions:
    - theme: brand
      text: Language Specification
      link: /ilk-spec
    - theme: brand
      text: Playground
      link: /playground
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

## Try it

<script setup>
const ex1 = {
  type: `type HttpResponse = {
    status! Concrete<Int> // required field (note the "!")
    body {...} // optional open field
}`,
  instances: [
    { label: 'Valid', expect: 'pass', code: `success = HttpResponse {
    status 200
    body {
        message String
        timestamp Timestamp
    }
}` },
    { label: 'Optional by default', expect: 'pass', code: `success = HttpResponse {
    status 200
}` },
    { label: 'Not an Int', expect: 'fail', code: `broken = HttpResponse {
    status "not a number"
}` },
    { label: 'Missing required status', expect: 'fail', code: `broken = HttpResponse {
      body {
          message String
      }
}` }
  ]
}

const ex2 = {
  type: `type HttpResponse = {
    status! Concrete<Int>
    body {...}
}

type Endpoint = {
    params  {...}
    body    {...}

    @source [params, body]
    response HttpResponse
}`,
  instances: [
    { label: 'Valid mappings', expect: 'pass', code: `getUser = Endpoint {
    params { id Uuid }
    body   { name String }

    response {
        status 200
        body {
           name   String // resolved implicitly from field names in @source
           userId Uuid = params.id // needs explicit mapping (different field names)
        }
    }
}` },
    { label: 'Missing mapping', expect: 'fail', code: `getUser = Endpoint {
    params { id Uuid }
    body   { name String }

    response {
        status 200
        body {
            userId Uuid
            name   String
        }
    }
}` }
  ]
}
</script>

<TypeExample :example="ex1" />

## The idea in 30 seconds

A `.ilk` file has two sections: **type declarations** and **instance bindings**.

- **Types** define abstract shapes — what fields exist, what types they have, what constraints apply.
- **Instances** name the concrete entities in *your* domain — not runtime values, but a typed catalog of the things your system knows about.

```ilk
// Type declaration
type HttpResponse = {
    status! Concrete<Int> // required (!), Concrete<...> means instances have to declare a value
    body {...} // open : any fields allowed, no constraints
}

// Instances
creationSuccess = HttpResponse {
    status 201
    body {uid Uuid}
}

notFound = HttpResponse {
    status 404
}

userProfile = HttpResponse {
    status 200
    body {
        uid Uuid
        name String
        admin Bool
        email String
    }
}
```

## The key feature: data flow

Traditional schemas validate *shape*. ilk also validates *where data comes from*.

`@source` declares which upstream fields may supply data to a downstream field.
Every open field must be explicitly mapped — the compiler checks it.

<TypeExample :example="ex2" />

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
