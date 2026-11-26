# ilk cheat sheet for event models

This covers only the subset you need for the event-model meta-model in
`assets/types.ilk`. Full language spec: `docs/ilk-spec.md` in the ilk-lang repo.

## Syntax you will use

```ilk
import "./types.ilk"                       // relative path, flat namespace

memberIdTag = Tag {memberId Uuid}          // binding = MetaName body
memberActor = Actor {name "member"}        // Concrete<String> → a literal

bookBorrowed = TaggedEvent {
    type "BookBorrowed"                    // Concrete<String>
    description "A member borrowed a book."  // free text, ignored by checks
    fields {                               // open struct {...}: fields DECLARE types
        loanId   Uuid
        lines    []{ sku Uuid, qty Int }   // typed list field
        note?    String                    // optional at runtime
    }
    tags [bookIdTag, memberIdTag]          // tags flow from fields by name
}
```

| Form (inside instance structs) | Meaning |
|--|--|
| `name Type` | Field; under `@source` it is matched by name against the sources |
| `name Type = a.b.c` | Explicit mapping, exact type match required |
| `name Type = compute(a.x, b.y)` | Derived or converted; types not checked; add `@doc "formula"` |
| `name Type*` | Generated at runtime (new ids, system dates) |
| `name? Type` | Optional; a required target can't map from an optional source |
| `binding & { ... }` | Refine a referenced binding (scenario values, tag remapping) |
| `@doc "..."` | Hint on the next field (one per field in instances) |
| `[a, b]` / newline-separated | List values; struct fields are separated by newlines or commas |

Base types: `Bool Int Float String Uuid Date Timestamp Money`.

String literals may span several lines, but there are **no escape sequences**
(`\"` and `\n` are parse errors). Keep `"` out of descriptions.

`description` (optional, `Concrete<String>`) exists on TaggedEvent, Actor,
ChangeSlice, ViewSlice, ChangeScenario, ViewScenario, Context, Chapter, and
Board. Concrete values are exempt from `@source`, so descriptions never
affect data-flow checks.
HTTP methods are bare identifiers: `method POST`.

## Rules the meta-model enforces (what `ilk check` verifies)

**Endpoint**
- Every `{var}` in `path` must be a key of `params`.
- `method`, `path`, `responses` are required (`responses []` is allowed).

**ChangeSlice**
- `command.fields` ← `@source [endpoint.params, endpoint.body, endpoint.auth]`: every field
  must be matched by name, explicitly mapped, or generated (`*`).
- A field name present in more than one of params/body/auth → **ambiguous**; map it explicitly.
- `command.emits` ← `@source [fields]`: **every field of every emitted event** must be
  traceable to `command.fields` (by name, unless refined in the emits list).
- `command.query` ← `@source [fields]`: tag fields used in queries must be traceable to command fields.
- Scenarios: every `given` event ∈ some `query[].eventTypes`; every key of `when` ∈
  `command.fields`; `then` is either `{error "..."}` or a list of events, each ∈ `emits`.

**ViewSlice**
- `query` ← `@source [endpoint.params, endpoint.body, endpoint.auth]`: tag fields must be
  traceable to the endpoint, or explicitly mapped
  (`memberIdTag & {memberId Uuid = endpoint.auth.userId}`).
- Scenarios: every `given` event ∈ some `query[].eventTypes`.
- `readModel` is **not** provenance-checked. Check it yourself.

**QueryItem**
- Every tag in `tags` must appear in the `tags` of **every** event listed in `eventTypes`.
  So group events by the tags they share. Use several query items when they differ.

**TaggedEvent**
- `tags` ← `@source [fields]`: each tag's field must exist in `fields` with the same name
  and type, or be remapped: `productIdTag & {sku Uuid = fields.productId}`.

**Scenarios / refinements**
- A refinement may only name fields that exist on the binding: `itemAdded & {fields {cartId "abc"}}`.

**Main**
- Only one `@main` per file. Every slice must be imported into `main.ilk` before it can appear in a `flow`.

## Compiler errors → likely fix

| Error | Usual cause | Fix |
|--|--|--|
| `No source found for field 'X'` at an `emits` entry | Emitted event has field `X` that the command doesn't have | Add `X` to `command.fields` (sourced from the endpoint, or `X Type*` if generated) |
| `No source found for field 'X'` in `command.fields` | Field not in endpoint params/body/auth | Add it to the endpoint, map explicitly (`= endpoint.auth.userId`), or mark it generated |
| `No source found for field 'X'` at a `tags` entry (event or query) | Tag field name ≠ event/command field name | Rename the field to match (preferred) or refine the tag `tagX & {X Type = fields.y}` |
| `Ambiguous source for 'X'` | Same name in two of params/body/auth | `X Type = endpoint.params.X` |
| `Type mismatch: source 'a.b' is T1 but field is T2` | Same concept typed differently | Unify types (guideline 5), or `compute()` if it really is a conversion |
| `Unknown field in nested refinement: X` | Scenario/tag refinement names a field the event doesn't have | Fix the field name, or add the field to the event if it belongs there |
| `Constraint failed for inline HttpEndpoint` | Path `{var}` not in `params` | Add the var to `params` |
| `Constraint failed for inline QueryItem` | A query tag isn't on every event in that item | Split the query item, or tag the event |
| `Constraint failed for instance 'slice'` | One of the scenario rules | Read `where:` → `event in [...] = false` means the given event isn't queried or the `then` event isn't emitted; `k in [...] = false` means the `when` key isn't a command field |
| `Undefined reference` / unknown binding | Missing import or typo | Add the `import`, fix the name |

Constraint failures print a `where:` block with the bound variables, for example:

```
Error: Constraint failed for instance 'borrowBook'
  where:
    s = {name: OK: borrow an available book, ..., then: [bookReturned], ...}
    event = bookReturned
    bookReturned in [bookBorrowed] = false      ← bookReturned isn't in emits
```

## CLI

```bash
ilk check main.ilk          # validate (exit code 1 on error)
ilk check cmdBorrowBook.ilk # a slice file validates on its own (it imports what it needs)
ilk format file.ilk         # prints the formatted file to stdout (does NOT rewrite it)
ilk emit main.ilk           # JSON of types + the @main instance
```
