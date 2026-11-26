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

`description` (optional, `Concrete<String>`) exists on every slice, scenario,
event, Actor, ExternalSystem, Context, Chapter, and Board. Concrete values are exempt from `@source`, so descriptions never
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
- `command.emits` ← `@source [fields, query.eventTypes.fields]` (R2): every field of every
  emitted event comes from `command.fields` first, otherwise from an attribute of an event
  the command reads (never from earlier occurrences of the emitted event itself), or is
  generated (`Type*`). Fields a refinement in `emits` doesn't mention are checked too.
- `command.query` ← `@source [fields]`: tag fields used in queries must be traceable to command fields.
- Scenarios: every `given` event ∈ some `query[].eventTypes`; every key of `when` ∈
  `command.fields`; `then` is either `{error "..."}` or a list of events, each ∈ `emits`.

**ViewSlice**
- `query` ← `@source [endpoint.params, endpoint.body, endpoint.auth]`: tag fields must be
  traceable to the endpoint, or explicitly mapped
  (`memberIdTag & {memberId Uuid = endpoint.auth.userId}`).
- Scenarios: every `given` event ∈ some `query[].eventTypes`.
- `readModel` ← `@source [query.eventTypes.fields]` (R1): every field is an attribute of a
  queried event (by name, `= query.eventTypes.fields.x`), or derived with
  `= compute(query.eventTypes.fields.a, ...)` plus an `@doc` giving the formula. Write
  collections as a list holding one struct, `loans [{loanId Uuid, dueDate Date}]`: its
  fields are then checked one by one. (`loans []{...}` would need a `loans` field on an event.)
- `actor` is required (R5).

**AutomationSlice**
- `command.fields` ← `@source [trigger.fields]`: every field comes from the trigger event
  (by name, `= trigger.fields.x`, `compute(trigger.fields.x)`) or is generated.
- Scenarios (`AutomationScenario`): `when` must be the trigger event (it may be refined);
  `given` ∈ query; `then` ∈ emits, or Error.

**TranslationSlice**
- `from` must reference an `ExternalSystem` binding.
- `command.fields` ← `@source [message]`; nested paths are allowed (`= message.metadata.fineId`).
- Scenarios: `when` keys ∈ `keys(message)`; `given` ∈ query; `then` ∈ emits, or Error.

**QueryItem**
- Every tag in `tags` must appear in the `tags` of **every** event listed in `eventTypes`.
  So group events by the tags they share. Use several query items when they differ.

**TaggedEvent**
- `tags` ← `@source [fields]`: each tag's field must exist in `fields` with the same name
  and type, or be remapped: `productIdTag & {sku Uuid = fields.productId}`.

**Scenarios / refinements**
- A refinement may only name fields that exist on the binding: `itemAdded & {fields {cartId "abc"}}`.
- Refinements work in `given` and in an automation's `when`, but **not inside `then`**
  (`then` is typed `[]-TaggedEvent | Error`, and refining inside that union fails with
  `Refinement on non-named type`). List plain event names in `then`, and use the scenario
  `description` to state the expected values.

**Main**
- Only one `@main` per file. Every slice must be imported into `main.ilk`: the board
  rules apply to every declared slice and event (`instances(...)`), listed or not.

## Well-formedness rules (R1–R8) and their escape hatches

| Rule | Checked where | Escape hatch (a decision, not a silencer) |
|--|--|--|
| R1 read-model fields are sourced | ViewSlice `readModel` | `= compute(...)` with `@doc` |
| R2 emitted attributes have an origin | Command `emits` | `Type*` generator (ids, clocks, policy values) with `@doc` |
| R3 preconditions are reachable: each `given` event and automation trigger is emitted by a slice that can precede it in `flows` | Board | add the producing slice and the edge |
| R3b each `requires` attribute is emitted (by any event) on **every** path from every start (`starts`) to the slice | Board | change the flow (route through the producer) or the start points |
| R4 commands are triggered: entered from a ViewSlice in `flows` | Board | list the command in `starts` (issued from a static page) |
| R5 read models are consumed: `actor` is required | ViewSlice | — (use the automation's system actor for a todo list) |
| R5 events are consumed (read by a query or a trigger) | Board | `unconsumedEvents [{event cartSubmitted, reason "terminal"}]` (or `"audit"`) |
| R6 each `requires` attribute is read by the slice (queried events, or trigger) and has a rejecting scenario where no `given` event carries it (not needed when the trigger carries it) | slice | `requires []` when nothing is required |
| R7 each invariant `{rule, violatedBy [scenario names]}` names existing rejecting scenarios | slice | `invariants []` |
| R8 each stream (Tag) is in `closedStreams` (its closing events must carry the tag) | Board | list it in `openEndedStreams` |

`requires`, `invariants`, `scenarios` (state-change slices) and `flows`, `starts`,
`unconsumedEvents`, `closedStreams`, `openEndedStreams` (Board) are **required**, so
writing `[]` is itself an explicit statement.

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
| `Reference type mismatch: x is type A, expected B` | Wrong kind of binding (for example an `Actor` given as `from`) | Reference a binding of meta `B` |
| `Refinement on non-named type` | `event & {...}` inside `then` | Use the plain event name in `then` |
| `Missing required field: requires` / `invariants` / `flows` / `openEndedStreams` | Undeclared | Declare it, even as `[]` |
| `Constraint failed … R1`–`R8: …` | A well-formedness rule | The label names the rule; `where:` names the slice/event/tag. See the table above |
| `No source found for field 'X'` in a `readModel` | No queried event has `X` (R1) | Rename to the event's name for that concept, query the event that carries it, or derive it with `compute()` |

Constraint failures print the rule's label (when it has one) and a `where:` block with
the bound variables, for example:

```
Error: Constraint failed for instance 'borrowBook'
  where:
    s = {name: OK: borrow an available book, ..., then: [bookReturned], ...}
    event = bookReturned
    bookReturned in [bookBorrowed] = false      ← bookReturned isn't in emits

Error: Constraint failed for instance 'board': R3: each automation trigger is produced by a slice that precedes the automation in flows
  where:
    a = issueLateFine                            ← no edge leads from the emitter of its trigger
```

## CLI

```bash
ilk check main.ilk          # validate (exit code 1 on error)
ilk check cmdBorrowBook.ilk # a slice file validates on its own (it imports what it needs)
ilk format file.ilk         # prints the formatted file to stdout (does NOT rewrite it)
ilk emit main.ilk           # JSON of types + the @main instance
```
