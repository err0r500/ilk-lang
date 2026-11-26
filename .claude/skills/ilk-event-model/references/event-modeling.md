# Event Modeling with vertical slices, mapped onto ilk

Event Modeling describes a system as a **timeline** of facts (events). A small
set of patterns connects those events to users and other systems. Each pattern
instance is a **vertical slice**: a unit you can build and test on its own,
going from UI or API down to storage.

## From prose to building blocks

| Prose signal | Building block | ilk |
|--|--|--|
| "The user can / wants to / submits / clicks…" | Command | `ChangeSlice.command` |
| "…is created / was approved / has been paid…" (anything later steps rely on) | Event | `TaggedEvent` in `events.ilk` |
| "The user sees / lists / gets a dashboard of…" | View (read model) | `ViewSlice.readModel` |
| "Only if / at most / must not / unless…" | Business rule | `Err:` scenario on the deciding command |
| "When X happens, the system automatically…" | Automation | `AutomationSlice` |
| "We receive X from the payment provider / ERP…" | Translation (external input) | `TranslationSlice` |
| Roles ("member", "admin", a background policy) | Actor | `Actor` in `actors.ilk` |
| Other systems ("Stripe", "the warehouse ERP") | External system | `ExternalSystem` in `actors.ilk` |
| Nouns that rules are scoped to ("per cart", "per member") | Consistency identifier | `Tag` in `tags.ilk` |

## Steps

1. **Brainstorm events.** List every state change the business cares about,
   in the past tense. Over-collect, then merge duplicates and drop technical
   events (`RowInserted`, `EmailQueued`).
2. **Order them into a timeline** along the main journey. Branches and
   failure paths come later, as scenarios.
3. **Attach a command to each event**: which actor intent produces it? One
   command may emit several events (cart: `AddItem` emits `CartCreated` +
   `ItemAdded` the first time). One event should be emitted by one command.
4. **Attach views** wherever an actor needs information to decide what to do
   next. Each view lists exactly the events it reads.
5. **Cut slices.** One human command = one ChangeSlice. One read model = one
   ViewSlice. One (trigger event → command) = one AutomationSlice. One
   inbound message type = one TranslationSlice.
6. **Write scenarios** (Given/When/Then) for every slice:
   - Change: `given` = past events, `when` = command input, `then` = emitted events **or** `{error "..."}`.
   - View: `given` = past events, `query` = request input, `expect` = read-model state.
7. **Check information completeness.** Every field must come from somewhere:
   endpoint → command fields → event fields → read-model fields. ilk enforces
   the first two arrows. You check the last one by hand.

## Slices in ilk

### ChangeSlice (command)

```
endpoint (params/body/auth) ──@source──▶ command.fields ──@source──▶ emits[] events
                                             │
                                             └──▶ query[] (events read to decide)
```

- `command.fields`: the full input to the decision. Each field is either
  implicitly matched by name against `endpoint.params`/`body`/`auth`,
  explicitly mapped (`shopperId Uuid = endpoint.auth.userId`), or generated
  (`loanId Uuid*`, `dueDate Date*`).
- `command.query`: the events the command must load to enforce its rules.
  This is the **consistency boundary** (DCB style). Each `QueryItem` lists
  `eventTypes` + `tags`. Include exactly what the rules need, no more.
- `command.emits`: every event the command can produce. Each event's fields
  must be traceable to `command.fields`, so name command fields the same as
  the event fields.
- `scenarios`: `OK:` and `Err:` cases (see below).

### ViewSlice (read model)

- `endpoint`: how the view is requested. The path template vars must be in `params`.
- `query`: which events feed the projection, filtered by tags. A tag can be
  bound to the request (`memberIdTag & {memberId Uuid = endpoint.auth.userId}`).
- `readModel`: the shape returned (`{...}` is open). Use list-typed fields for
  collections: `loans []{ loanId Uuid, dueDate Date }`.
- `scenarios`: `given` events → `query` input → `expect` state. Use `@doc`
  on expected derived values to state the formula (`@doc "sum(items, i => i.price * i.quantity)"`).

### Board / contexts / chapters

`main.ilk` holds a single `@main board = Board {...}`. Group slices into
`contexts` (bounded contexts), and inside each context into `chapters` (one
step of a user journey). Each chapter's `flow` lists slices in timeline order,
alternating command → view the way a user would actually move through them.

## Scenario-writing rules

- Name scenarios `"OK: …"`, `"Err: …"` (change) and `"Ex: …"` (view examples).
- One business rule → at least one `Err:` scenario whose `given` sets up the
  violating history. Write the error as a short domain message:
  `{error "book not available"}`.
- Idempotent no-ops: `then []` (cart: removing an already-removed item).
- Pin only the values that matter, by refining the event with
  `event & {fields {cartId "abc"}}`. Leave the rest implicit.
- Correlate ids across `given` and `when` (same `"b1"`, `"m1"` literals) so
  the scenario is readable as a story.
- Every `given` event must appear in some `query` item's `eventTypes`. If a
  scenario needs an event the command doesn't load, the **query** is
  incomplete. Don't drop the scenario.

### AutomationSlice ("when X happens, the system automatically…")

```
trigger (&TaggedEvent) ──@source [trigger.fields]──▶ command.fields ──▶ emits[]
```

- `trigger`: the single event type that wakes the processor. The command
  input must come from that event's `fields` (by name, `= trigger.fields.x`,
  `compute(trigger.fields.x)`, or `*`). So the trigger event must carry
  everything the automation needs. This is a strong argument for capturing
  decision-time facts in events (guideline 7).
- `actor`: a system actor (`lendingPolicyActor`), not a human.
- `command.query`: the history the processor checks before acting (avoid
  duplicates, enforce policies).
- `scenarios` use `AutomationScenario`: `given` (history, must be queried),
  `when` = the trigger event, optionally refined
  (`bookReturned & {fields {daysOverdue 4}}`), and `then` (emitted events, `[]`
  for "nothing to do", or an error).
- If the automation reacts to several event types, write one AutomationSlice
  per trigger.

### TranslationSlice (messages coming from another system)

```
message ({...}) ──@source [message]──▶ command.fields ──▶ emits[] (our events)
```

- `from`: a reference to an `ExternalSystem` binding (declared in `actors.ilk`).
- `channel`: free text saying how the message arrives (`"webhook POST /webhooks/payments"`).
- `message`: the external payload **in the other system's vocabulary**. Map
  it explicitly into our language in `command.fields`
  (`paymentRef String = message.reference`). This mapping is the whole point
  of the slice: external names must not leak into our events.
- External delivery is at-least-once, so always add an idempotency scenario
  (duplicate message → `then []`).
- `scenarios` use `ChangeScenario`, but `when` keys are **message** keys.

Use a TranslationSlice whenever the spec says "we receive / are notified /
sync from X". Don't model it as a ChangeSlice with a fake endpoint.
