---
name: ilk-event-model
description: Turn a human-written spec (prose requirements, user stories, business rules, workshop notes) into a compilable, well-formed ilk event model, using Event Modeling, vertical slices (command, view, automation, translation), a flow graph between slices, and Given/When/Then scenarios. Use when asked to model, formalize, or "make compilable" a feature/domain spec in ilk, to write ChangeSlice/ViewSlice/TaggedEvent instances, or to extend an existing ilk board with new slices. Output is validated with `ilk check` until it passes.
---

# ilk event model from a human spec

You turn prose into a **compilable** ilk model: tagged events, vertical slices,
scenarios, and a `@main` board with the flow graph. The model is done only when
`ilk check main.ilk` prints `Validation passed`.

Passing means the model is **well-formed** (rules R1–R8, listed at the top of
`assets/types.ilk` and explained in `references/ilk-cheatsheet.md`): every field
has a provenance, every precondition can happen earlier in the flow, every
output is consumed, and every rule is tested. Each rule has an explicit escape
hatch. Use one only as a *decision* you can justify, never to silence an error
you don't understand. Silence is never accepted.

The blueprint is `examples/cart/` in the ilk-lang repo. Copy its structure.

## Reference files — read before writing

| File | Read when |
|--|--|
| `references/event-modeling.md` | Always. How to get from prose to events, slices, scenarios. |
| `references/event-schema-guidelines.md` | Always, before writing `events.ilk`. |
| `references/ilk-cheatsheet.md` | Before writing any `.ilk`, and whenever `ilk check` fails. Has the rules the meta-model enforces and a compiler-error → fix table. |
| `references/worked-example.md` | To see a full spec → model run end to end. |
| `assets/types.ilk` | The meta-model. Copy it **unchanged** into the output folder. Its header comment and `@doc` lines explain what each meta means. Read them once. |

## Workflow

### 0. Locate the compiler

```bash
command -v ilk || ls target/release/ilk target/debug/ilk 2>/dev/null
```

If it's missing and you're in the ilk-lang repo, run `cargo build --release` and use
`./target/release/ilk`. Never declare the model done without running the checker.

### 1. Read the spec and write an interpretation table (no ilk yet)

Pull out the following and write it down in your reply or a scratch file:

- **Actors**: who acts (humans and systems)
- **Commands**: intents, phrased imperatively (`BorrowBook`)
- **Events**: facts, in the past tense (`BookBorrowed`)
- **Views / read models**: what someone needs to *see* to decide or act
- **Business rules**: split them into *requires* (data the command needs from
  the past: "the loan's due date", "the shopper's email") and *invariants* (a
  rule over history: "at most 3 books"). Each becomes at least one `Err:` scenario
  (R6/R7); each invariant names the rejecting scenarios that violate it (`violatedBy`)
- **Identifiers**: the entities that consistency decisions depend on; these become tags
- **Open questions / assumptions**: anything the spec doesn't settle

Don't invent business rules. When the spec is silent, pick the simplest
reading, write it down as an assumption, and keep going. Don't block on it.

### 2. Lay out the timeline and cut it into slices

Order the events along the user journey. Then cut it into vertical slices:

- **ChangeSlice**: UI/endpoint → command → event(s). One command per slice.
- **ViewSlice**: event(s) → read model → endpoint. One read model per slice.
- **AutomationSlice**: trigger event → command → event(s), with no human. One trigger per slice.
- **TranslationSlice**: external system message → command → our event(s). One message type per slice.

Then draw the **flow graph**: for each slice, which slices can come next
(screen → command, command → view, event → automation…). It may branch and
loop. Group slices into **chapters** (themes) inside **contexts** (bounded
contexts); grouping carries no order. Details are in `references/event-modeling.md`.

For each state-change slice, also decide:
- `requires`: the **attributes** it needs from past events (`["dueDate"]`, `["userEmail"]`),
  whichever event provides them. R3b checks that on every path from every start, some slice
  emits an event carrying each of them; R6 wants a rejecting scenario where none is there.
  To require that something happened, require an attribute only that event carries
  (only catalogued books have an `isbn`).
- `invariants`: the business rules it enforces, in one short sentence each

### 3. Write the files, in this order

```
<model-dir>/
  types.ilk        # copy of assets/types.ilk, unchanged
  tags.ilk         # one Tag per consistency identifier
  actors.ilk       # one Actor per role + one ExternalSystem per outside system
  events.ilk       # every TaggedEvent
  cmd<Name>.ilk    # one ChangeSlice per file
  view<Name>.ilk   # one ViewSlice per file
  auto<Name>.ilk   # one AutomationSlice per file
  trans<Name>.ilk  # one TranslationSlice per file
  main.ilk         # imports + @main Board: contexts/chapters, flows graph, openEndedStreams
```

Every slice file starts with the same four imports (types, events, tags,
actors). `main.ilk` imports `types.ilk` and every slice file.

Naming:
- Binding names are camelCase: `bookBorrowed`, `borrowBook`, `viewMyLoans`
- The event `type` is the PascalCase form of the binding name: `"BookBorrowed"`
- Slice `name` is PascalCase: `"BorrowBook"`

Descriptions: every `TaggedEvent`, `ChangeSlice`, `ViewSlice`, `Actor`, and
the `Board` gets a `description`. It is free text (`Concrete<String>`), so the
type checker ignores it. Its purpose is to let the next reader, human or
agent, understand an element at a glance and trace it back to the spec.
- **Slices**: start with `Spec: ` and quote the spec sentence(s) this slice
  implements, close to word for word. Multi-line strings are fine. This is
  the traceability link from the model back to the source text.
- **Events**: one sentence stating the business fact in the past tense
  (`"A member borrowed a book; the loan runs until dueDate."`).
- **Actors / Board / Context / Chapter**: one short line.
- **Scenarios**: optional. Use one when the `name` alone doesn't say which rule
  is being tested.
- Strings have **no escape sequences**. Never put `"` inside a description.
  Replace quotes in the spec with `'`.

### 4. Check after every file, and fix one error at a time

```bash
ilk check <model-dir>/main.ilk
```

Check each slice file on its own too (`ilk check cmdX.ilk`); that gives
shorter feedback. Fix the **first** error, then re-run, because later errors
are often caused by earlier ones. For a constraint failure, read the `where:`
block: it names the scenario, event, or key that broke the rule. Use the error
table in `references/ilk-cheatsheet.md` to find the fix.

Never get to green by weakening the model: don't delete scenarios, empty out
`emits`, edit `types.ilk`, or turn real fields into `*` to silence provenance
errors. Fix the data flow instead. Usually that means renaming a field so it
matches, adding an explicit `= path` mapping, or adding the missing query item.

### 5. Self-review (the compiler doesn't check these)

- [ ] Every command has an `OK:` scenario. (R6/R7 already guarantee an `Err:` per required event and invariant, if you declared them all.)
- [ ] Every TranslationSlice has a duplicate-message scenario (`then []`).
- [ ] Every sentence of the spec is quoted in at least one slice `description`. A sentence quoted nowhere is either an unmodelled requirement or an open question.
- [ ] `starts` lists every way into the system (anonymous pages included), not only the happy path.
- [ ] Every escape hatch you used (the board's `unconsumedEvents`, `openEndedStreams`, or a `Type*` generator) is a real decision. Mention each in the report.
- [ ] Every event is emitted by some slice. An event nobody emits is a missing slice, usually a translation from another system.
- [ ] Event schemas pass the checklist at the end of `references/event-schema-guidelines.md`.
- [ ] Optionally normalize layout with `ilk format file.ilk > tmp && mv tmp file.ilk` (it prints to stdout).

### 6. Report back

Keep the report short:
- the file list
- a slice table: chapter / slice / kind / command or read model / events
- the assumptions you made
- open questions for the spec author
- the escape hatches used, with a reason each
- slices you added that the spec only implies (the rules often force these: see the worked example)
