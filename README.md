# Ilk lang : Model your Domain, type check your data flow

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

```ilk
type Endpoint = {
    params  {...}
    body    {...}

    @source [params, body]   // responses use data from params and body
    responses []HttpResponse
}

// In an instance, every field assignment is checked against @source
getUser = Endpoint {
    params { id Uuid }
    body   {
        name String
    }

    responses [
        {
            status 200             // Concrete<Int> — exempt from @source
            body {
                userId  Uuid   = params.id   // mapping
                name    String = body.name   // mapping
            }
        }
        {
            status 404
            body {
                message "User not found"  // ✓ literal — exempt
                timestamp Timestamp*      // generated field (*)
            }
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

| `@constraint expr` | Boolean predicate validated at compile time |
| `@doc "…"` | Implementation hint, preserved in AST |


## Run the CLI

```sh
Usage: ilk [OPTIONS] <COMMAND>

Commands:
  check   Validate an ilk file
  watch   Watch file and re-validate on changes
  parse   Parse a file and dump the AST
  json    Output the compiled AST as JSON
  lsp     Start LSP server (stdio)
  format  Format an ilk file
  emit    Emit types and @main instances as JSON
  help    Print this message or the help of the given subcommand(s)
```
