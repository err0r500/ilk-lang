## Overview

The **evolution tracking system** gives domain model authors a Terraform-style
workflow for managing how `.kli` files change over time. It answers the question:
*"If I change my domain model, what breaks, and what is safe?"*

The system has two artefacts:

| Artefact | Purpose |
|---|---|
| `.kli` | The live domain model, edited by the author |
| `.kli.lock` | A machine-generated snapshot of the last approved model state |

The lock file is the "known good" baseline. Every time the author modifies the
`.kli` file they can run `ilk check` to compare the current model against the
baseline and see a structured compatibility report — analogous to `terraform plan`.
When the changes are intentional the author runs `ilk snapshot` to advance the
baseline — analogous to `terraform apply`.

Both files should be committed to version control. The `.kli` file is hand-authored;
the `.kli.lock` file is machine-generated and should never be edited by hand.

---

## Concepts

### Snapshot

A snapshot is a normalized, machine-readable representation of a validated `.kli`
file at a specific point in time. It captures every binding's name, kind, shape,
and relationships (associations, emits, query dependencies).

The snapshot does **not** capture comments, whitespace, or formatting — only
the semantic content that affects compatibility.

### Compatibility check

A compatibility check compares the current `.kli` file against the snapshot in
`.kli.lock`. Each difference is classified as either:

- **Non-breaking** — existing consumers of the domain model continue to work.
  Safe to apply without acknowledgment.
- **Breaking** — at least one consumer of the domain model would need to change.
  Requires an explicit `--allow-breaking` flag when advancing the snapshot.

### Consumer

A *consumer* is any system that reads events, sends commands, or queries using
the bindings defined in the `.kli` file. The compatibility rules are written from
the consumer's perspective: "would a consumer built against the old snapshot still
work correctly against the new model?"

---

## The `.kli.lock` file

### Location

The lock file lives alongside the `.kli` file, with the same base name:

```
board.kli
board.kli.lock
```

### Format

The lock file is a JSON document. It is generated entirely by the `ilk snapshot`
command and must never be edited by hand.

```json
{
  "kli_file": "board.kli",
  "ilk_file": "dcb-board-spec.ilk",
  "ilk_hash": "sha256:<hex>",
  "snapshotted_at": "2026-03-09T10:00:00Z",
  "bindings": {
    "<name>": { ... }
  }
}
```

| Field | Description |
|---|---|
| `kli_file` | Relative path to the `.kli` source file |
| `ilk_file` | Relative path to the `.ilk` schema the model was validated against |
| `ilk_hash` | SHA-256 of the `.ilk` file contents at snapshot time |
| `snapshotted_at` | ISO 8601 UTC timestamp of when the snapshot was generated |
| `bindings` | Map of binding name → binding record (see below) |

### Binding records

Each entry in `bindings` is one of the following kinds, determined by the schema
type the binding was validated against.

#### Event binding

```json
"userRegistered": {
  "kind": "event",
  "stability": "stable",
  "fields": {
    "id":   { "type": "String",  "optional": false },
    "name": { "type": "String",  "optional": false }
  },
  "associations": ["userIdTag", "userNameTag", "commonTag"]
}
```

| Key | Description |
|---|---|
| `kind` | Always `"event"` |
| `stability` | `"stable"`, `"experimental"`, or `"deprecated"` (see [Stability](#stability-annotations)) |
| `fields` | Map of field name → `{ type, optional }`. `type` is the ilk type name as a string. `optional` is `true` when the field is marked `?` in ilk. |
| `associations` | Ordered list of tag binding names supplied via `<...>` in the kli file |

#### Command binding

```json
"registerUser": {
  "kind": "command",
  "stability": "stable",
  "fields": {
    "id":   { "type": "String", "optional": false },
    "name": { "type": "String", "optional": false },
    "x":    { "type": "String", "optional": false }
  },
  "emits": ["userRegistered"],
  "query_events": ["userRegistered", "other"],
  "query_tags":   ["commonTag"]
}
```

| Key | Description |
|---|---|
| `kind` | Always `"command"` |
| `fields` | The `fields {...}` block contents — the command's input shape |
| `emits` | Ordered list of event binding names the command emits |
| `query_events` | Union of all `eventTypes` across the command's `query` list |
| `query_tags` | Union of all `tags` across the command's `query` list |

#### Tag binding

```json
"userIdTag": {
  "kind": "tag",
  "stability": "stable",
  "variant": "Parametrized",
  "fields": {
    "userId": { "type": "String", "optional": false }
  }
}
```

```json
"simpleTag": {
  "kind": "tag",
  "stability": "stable",
  "variant": "Unique",
  "value": "simple-tag"
}
```

| Key | Description |
|---|---|
| `kind` | Always `"tag"` |
| `variant` | The union branch name from the schema (e.g. `"Parametrized"`, `"Unique"`) |
| `fields` | Present when `variant` is a struct branch; the struct's fields |
| `value` | Present when `variant` is a `Concrete<T>` alias; the concrete string value |

---

## Compatibility rules

The table below defines what constitutes a **breaking** versus **non-breaking**
change when diffing a new `.kli` state against the existing `.kli.lock`.

Stability modifiers are applied after classification (see [Stability](#stability-annotations)).

### Events

| Change | Classification |
|---|---|
| Remove an event binding | **Breaking** |
| Rename an event binding | **Breaking** (equivalent to remove + add) |
| Remove a field from an event | **Breaking** |
| Change a field's type | **Breaking** |
| Change a required field to optional | Non-breaking |
| Change an optional field to required | **Breaking** |
| Add a new required field | **Breaking** |
| Add a new optional field | Non-breaking |
| Remove a tag from an event's association list | **Breaking** |
| Add a tag to an event's association list | Non-breaking |
| Add a new event binding | Non-breaking |

### Commands

| Change | Classification |
|---|---|
| Remove a command binding | **Breaking** |
| Rename a command binding | **Breaking** |
| Remove a required field from `fields` | **Breaking** |
| Remove an optional field from `fields` | Non-breaking |
| Change a field's type in `fields` | **Breaking** |
| Change a required field to optional in `fields` | Non-breaking |
| Change an optional field to required in `fields` | **Breaking** |
| Add a new required field to `fields` | **Breaking** |
| Add a new optional field to `fields` | Non-breaking |
| Remove an event from `emits` | **Breaking** |
| Add an event to `emits` | Non-breaking |
| Change `query` events or tags | Non-breaking |
| Add a new command binding | Non-breaking |

### Tags

| Change | Classification |
|---|---|
| Remove a tag binding | **Breaking** |
| Rename a tag binding | **Breaking** |
| Change a Parametrized tag's field name | **Breaking** |
| Change a Parametrized tag's field type | **Breaking** |
| Change a Unique tag's concrete value | **Breaking** |
| Change a tag's variant (e.g. Parametrized → Unique) | **Breaking** |
| Add a new tag binding | Non-breaking |

### Schema changes

If the `.ilk` file referenced by the lock has changed (its hash differs), `ilk check`
reports this first. Schema changes can make previously valid `.kli` bindings invalid,
or silently change their meaning. The author must re-validate the `.kli` against the
new schema before snapshotting.

| Change | Classification |
|---|---|
| `.ilk` file changed (hash mismatch) | Requires re-validation |
| `.ilk` file unchanged, `.kli` re-validated | Normal diff proceeds |

---

## Stability annotations

Stability annotations let authors signal which parts of the domain model are settled
and which are still in flux. They appear in the `.kli` file on the line immediately
before a binding.

```kli
@stable
userRegistered = Event<userIdTag, userNameTag> {
    id   String
    name String
}

@experimental
draftOrderPlaced = Event {
    draftId String
}

@deprecated
legacyEvent = Event {
    data String
}
```

| Annotation | Meaning |
|---|---|
| `@stable` (default) | Binding is stable. Breaking changes require `--allow-breaking`. |
| `@experimental` | Binding is in flux. Breaking changes produce warnings but do not block `ilk snapshot`. |
| `@deprecated` | Binding is scheduled for removal. `ilk check` always warns that it will be removed. A future removal is **non-breaking** if the binding was marked `@deprecated` in the snapshot. |

When no annotation is present, the binding is treated as `@stable`.

A binding that was `@experimental` in the snapshot and is now `@stable` in the new
`.kli` file is a non-breaking "graduation". The inverse (stable → experimental) is
treated as a **breaking** change because it removes the stability guarantee.

---

## Workflow

### Initial snapshot

After creating and validating a new `.kli` file for the first time, generate its
lock file:

```
ilk snapshot board.kli
```

This writes `board.kli.lock`. Commit both files.

### Checking for changes

After modifying `board.kli`, compare it against the snapshot:

```
ilk check board.kli
```

Output:

```
ilk check board.kli

  Schema:  dcb-board-spec.ilk (unchanged)
  Changes: 3 non-breaking, 1 breaking

  NON-BREAKING
  + event  orderPlaced        added
  ~ event  userRegistered     field "email" added (optional)
  ~ tag    commonTag          new association on userRegistered

  BREAKING
  ~ event  userRegistered     field "name" removed

  Run `ilk snapshot --allow-breaking board.kli` to advance the snapshot.
```

The exit code is `0` for non-breaking-only changes, `1` for any breaking change.

### Advancing the snapshot

If all changes are non-breaking:

```
ilk snapshot board.kli
```

If there are breaking changes, an explicit flag is required:

```
ilk snapshot --allow-breaking board.kli
```

Without the flag, `ilk snapshot` refuses to write the lock file when breaking
changes are present and prints the same diff as `ilk check`.

### CI integration

A recommended CI gate:

```sh
ilk check board.kli   # exits 1 if breaking changes are present
```

This prevents accidental breaking changes from being merged without a conscious
`--allow-breaking` decision recorded in the commit.

---

## Full example

### Starting state (`board.kli`)

```kli
userIdTag   = Parametrized {userId String}
userNameTag = Parametrized {name String}
commonTag   = Parametrized {x String}
simpleTag   = Unique "simple-tag"

userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id   String
    name String
}

other = Event<commonTag, simpleTag> {
    hello String
}

registerUser = Command {
    fields {
        id   String
        name String
        x    String
    }
    emits [userRegistered & {timestamp Int*}]
    query [
        {
            eventTypes [userRegistered, other]
            tags       [commonTag]
        }
    ]
}
```

After `ilk snapshot board.kli`, the lock captures this model as the baseline.

### Non-breaking evolution

Add a new optional field to `userRegistered` and a new event:

```kli
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id      String
    name    String
    email?  String    // new optional field
}

@experimental
orderPlaced = Event {
    orderId String
}
```

`ilk check board.kli` reports:

```
  NON-BREAKING
  ~ event  userRegistered   field "email" added (optional)
  + event  orderPlaced      added (experimental)
```

`ilk snapshot board.kli` succeeds without flags.

### Breaking evolution

Remove the `name` field from `userRegistered`:

```kli
userRegistered = Event<userIdTag, userNameTag, commonTag> {
    id    String
    email String
}
```

`ilk check board.kli` reports:

```
  BREAKING
  ~ event  userRegistered   field "name" removed
  ~ event  userRegistered   field "email" type changed: String (optional) → String (required)
```

`ilk snapshot board.kli` exits with an error. The author must run:

```
ilk snapshot --allow-breaking board.kli
```

The flag makes the decision explicit and auditable in git history.

### Deprecation then removal

Mark a binding deprecated in one snapshot, remove it in a later one:

```kli
// snapshot 1: mark deprecated
@deprecated
legacyEvent = Event {
    data String
}
```

```kli
// snapshot 2: remove — non-breaking because it was @deprecated
// (legacyEvent binding is gone)
```

Because `legacyEvent` was recorded as `deprecated` in the snapshot, its removal
is classified as **non-breaking** and does not require `--allow-breaking`.

---

## Design notes

### What the lock does not capture

The lock captures semantic structure, not presentation. The following are explicitly
excluded:

- Comments
- Whitespace and indentation
- Field ordering within a struct (fields are keyed by name)
- Binding ordering in the file (bindings are keyed by name)
- Origin annotations (`Type*`, `= path`, `= compute(...)`) — these are provenance
  metadata, not part of the observable interface

### Relationship to the `.ilk` schema

The `.kli.lock` records the hash of the `.ilk` schema it was validated against. If
the schema changes, `ilk check` warns before showing the binding diff. The schema
change may itself introduce or mask compatibility issues, so schema updates and
binding updates should be snapshotted separately when possible.

### Multiple `.kli` files

Each `.kli` file has its own `.kli.lock`. There is no global state file. If multiple
domain models share an `.ilk` schema, each is tracked independently.
