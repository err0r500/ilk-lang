# Event schema guidelines

Events are the long-lived contract of the system. Once stored, they are
immutable and get re-read for years. Design them more carefully than
anything else in the model.

## Naming

1. **Past tense, in domain language, subject first**: `BookBorrowed`,
   `CartSubmitted`, `PriceChanged`. Never imperative (`BorrowBook` is a
   command), and never technical (`LoanRowInserted`).
2. **Name the intent, not CRUD.** Avoid `XCreated/XUpdated/XDeleted` as a
   catch-all. Split `CustomerUpdated` into `CustomerAddressChanged`,
   `CustomerEmailVerified`, … Each event should answer "what happened, and why
   does the business care?". `Created` is fine when creation *is* the
   business fact (`CartCreated`).
3. **Use one consistent casing**: binding `bookBorrowed`, `type "BookBorrowed"`.
   (The cart example has `type "itemAdded"`. Don't copy that.)

3b. **Describe each event in one sentence** (`description`): the business
    fact, in the past tense, in domain language. If you can't write that
    sentence without "and", it is probably two events.

## Content

4. **Carry the identifier of every entity the event concerns.** Rules,
   queries, and tags are scoped by these ids. An `ItemAdded` without `cartId`
   can't be found again.
5. **One concept = one field name and one type, everywhere.** If the product id
   is `sku Uuid` in one event, it is `sku Uuid` in every event, command, tag,
   and view. (The cart uses `sku` in `inventoryChanged` but `productId` in
   `priceChanged`, which forces a tag refinement. Avoid that.) In ilk this
   also makes implicit `@source` matching work without explicit mappings.
6. **Record facts, not derived state.** No totals, counters, `isAvailable`
   flags, or "new state" snapshots. Those belong in read models. Derive
   `totalPrice` in the view and document it with `@doc`.
7. **But do capture the decision-time values** a later reader couldn't
   reconstruct: the price at the moment of adding to the cart, the due date
   that was set, the reason given. If it might change later elsewhere, copy it
   into the event.
8. **Keep events small and focused.** Include what the fact needs and what
   downstream consumers need to react without calling back. Don't embed whole
   entities.
9. **Avoid optional-field sprawl.** If an event has several fields that are
   only present "sometimes", it is probably two different events.
10. **Leave out envelope metadata.** Event id, `occurredAt`/recorded
    timestamp, correlation/causation ids, schema version, and stream
    position belong to the store's envelope, not to `fields`. Add a time field
    only when it is a **business** time (`dueDate`, `returnedOn`, `effectiveFrom`).
11. **Generate values in the command, not the event.** New ids and
    system-chosen dates are `Type*` fields on `command.fields`. They then flow
    by name into the emitted events.

## Types

| Concept | Use | Avoid |
|--|--|--|
| Entity identifiers | `Uuid` | `String`, `Int` |
| Monetary amounts | `Money` | `Int`, `Float` (the cart uses `Int`: don't copy that) |
| Calendar day (due date, birthday) | `Date` | `Timestamp`, `String` |
| Instant something business-relevant happened | `Timestamp` | `String` |
| Counts, quantities | `Int` | `Float` |
| Measurements, rates | `Float` | `Int` |
| External codes (ISBN, email, SKU code) | `String` | |
| Closed sets (status, reason code) | `String` + `@doc "one of: A, B, C"` on the field | encoding the state machine in one event |
| Collections | typed lists `lines []{ sku Uuid, qty Int }` | `String` blobs |

## Tags (consistency boundaries)

12. **Tag each event with the identifiers that decisions will query it by.**
    A tag `{memberId Uuid}` on `BookBorrowed` lets `BorrowBook` load "all loans
    of this member" to enforce "max 3 loans".
13. **Tag field names must exist in the event's `fields`**, with the same
    type. If they don't, either rename the event field (preferred, see rule 5)
    or refine the tag: `productIdTag & {sku Uuid = fields.productId}`.
14. **Use few tags, on purpose.** One per entity that bounds a rule. Over-tagging
    makes nothing more consistent. It only adds noise.

## Evolution

15. **Never change the meaning of an existing event.** Add a new event type
    (`BookBorrowedV2` only as a last resort; a better name is preferred)
    instead of repurposing fields.

## Checklist (run it on `events.ilk`)

- [ ] Every name is past tense and in domain language; `type` = PascalCase(binding)
- [ ] Every event has a one-sentence `description` of the business fact
- [ ] No `…Updated`/`…Changed` catch-all that hides several intents
- [ ] Every event has the id(s) of the entities it is about
- [ ] Same concept → same field name + type across all events
- [ ] No derived/aggregate fields (totals, counts, flags)
- [ ] Decision-time values captured (prices, dates chosen, reasons)
- [ ] No envelope metadata in `fields`
- [ ] `Money` for amounts, `Uuid` for ids, `Date` vs `Timestamp` chosen deliberately
- [ ] Every tag's field exists in the event with the same name and type
- [ ] Every event is emitted by exactly one command (or marked external)
