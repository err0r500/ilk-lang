# ilk / kli Spec Review

Expert review of `ilk-spec.md`, `kli-spec.md`, and the accompanying example files.
Issues are grouped by severity. Each entry states the problem, the resolution applied
(or the open question remaining), and which files were changed.

---

## Critical inconsistencies (spec vs examples)

### 1. `@main` vs `@root` — **Fixed**

`ilk-spec.md` defines the entry-point annotation as `@main`.
`examples/dcb-board-spec.ilk` used `@root`.

**Fix:** Changed `@root` → `@main` in `examples/dcb-board-spec.ilk`. `@main` is
canonical; `@root` is removed.

---

### 2. Anonymous struct cardinality syntax — **Fixed**

`ilk-spec.md` uses `{_}` / `{...}` as field-cardinality shorthands.
`examples/dcb-board-spec.ilk` used a completely different `{1 Type}` / `{* Type}` form.

**Resolution:** Keep spec syntax (`{_}` / `{...}`). Changed:

| Old (example) | New (spec-canonical) |
|---|---|
| `Tag {1 Type}` | `TagField {_}` (plus named-block change, see §3) |
| `Event {* Type} & ...` | `Event {...} & ...` |
| `fields {* Type}` | `fields {...}` |

Note: `{_}` is equivalent in intent to `{1 Any}` (exactly one field of any name and
type); `{...}` is equivalent to `{* Any}`. The `Any` type entry in the base-types table
already documents this role.

---

### 3. Anonymous struct in union branches — **Fixed**

`ilk-spec.md` states: *"Inline anonymous struct expressions (`{...}`) are not valid as
union branches; declare a named block first."* Both the spec's own examples and the
example files then used `Tag {_} | Concrete<String>` — a direct contradiction of the rule.

**Resolution (per author decision):** The rule stands. Anonymous struct expressions —
including cardinality shorthands like `{_}` — are **not** valid as union branches.
A named block must be declared first.

Changes applied:
- `examples/dcb-board-spec.ilk`: introduced `TagField {_}`, changed union to
  `Tag TagField | Concrete<String>`.
- `ilk-spec.md` discriminated-unions section: updated inline example to show the
  two-line named-block form.
- `ilk-spec.md` full example: same update.
- `kli-spec.md` bindings and union-values sections: updated to `TagField {userId String}`
  as the branch binding.
- `examples/dcb-board-instance-valid.kli`: updated all `Tag {…}` bindings to
  `TagField {…}`.

---

## Missing / under-specified features

### 4. `@source [S] for [F]` qualifier — **Removed**

`examples/dcb-board-spec.ilk` contained `@source [fields] for [tags]` on the `query`
list. This qualifier did not appear anywhere in `ilk-spec.md` and was removed from the
spec and all examples per author decision. The `query` field now carries no `@source`
constraint.

---

### 5. Inline binding refinements — **Documented**

Both `kli-spec.md` (full example) and `examples/dcb-board-instance-valid.kli` use the
pattern `[bindingRef { field-origin-annotations }]` — appending a struct body to a
binding reference within a `@source`-constrained list. This syntax was not explained
anywhere.

**Fix:** Added an "Inline binding refinements" section to `kli-spec.md` explaining:
- Purpose: override or make explicit the origin annotation for specific fields of a
  referenced binding.
- Rules: the struct body may only name fields that exist in the binding's declared type;
  unmentioned fields fall back to implicit name-matching.
- Scope: only valid within `@source`-constrained list declarations.

Also documented in `ilk-spec.md` under the `@source` section (kli-side behaviour).

---

### 6. Anonymous struct instantiation in typed list contexts — **Documented**

The kli example supplies `{ eventTypes [...], tags [...] }` (no type name) in a
`[]QueryItem` list. The mechanism — structural typing allowing the type name to be omitted
when context is unambiguous — was not documented.

**Fix:** Added an "Anonymous struct instantiation" section to `kli-spec.md` clarifying:
- When the expected element type is a single unambiguous block type, the type name may
  be omitted.
- For union-typed lists, the branch name must be written explicitly.

---

## Design clarity issues

### 7. `String` (runtime) vs `Concrete<String>` in kli

`String` is defined as runtime-owned (the consuming system provides the value). Yet both
the kli-spec value-literals table and the full example show string literals for `String`
fields, with comments like `// runtime value — supplied by the consumer`. This is
potentially confusing: if the consuming system provides the value, why does it appear as a
literal in a kli file?

**Assessment:** The spec intent is that kli files define the *shape* of entities (field
names and types) even for runtime-typed fields. The string literals in examples are
illustrative placeholders showing what a runtime string looks like — they are not fixed
domain constants. The distinction is:
- `Concrete<String>` in kli: the value is fixed and meaningful (e.g. `"webhook"` as a
  label identifier).
- `String` in kli: the literal is a shape declaration (type annotation), not a value
  commitment; runtime provides the actual string.

**Recommendation (not yet applied):** Add a clarifying note to the value-ownership table
or the kli-spec overview explaining that `String`/`Int`/etc. field declarations in kli
describe *shape*, and any literal shown is illustrative. A future syntax like `_` or `?`
as a runtime-value placeholder (instead of a mock literal) would remove the ambiguity.

---

### 8. `@source` implicit name-matching depth

The spec says fields with no origin annotation are "matched by structural name against the
source fields." It does not specify the depth of this matching:

- Is `id` in an event matched to `fields.id` (one level into `fields`)?
- Would `id` in an event match `fields.user.id` (nested) automatically?

**Recommendation (not yet applied):** Clarify in `ilk-spec.md` and `kli-spec.md` that
implicit matching is **one level deep** — a field name is matched against the direct
members of the named source struct. Paths deeper than one level require an explicit
`= path` annotation.

---

### 9. Separator rules — minor inconsistency

`ilk-spec.md` separator table: struct fields allow "Newlines (or commas inline)."
`kli-spec.md` separator table: struct fields — "Newlines" only (no mention of commas).

The example `examples/dcb-board-instance-valid.kli` uses commas in list contexts but not
in struct bodies, consistent with the kli rule.

**Recommendation (not yet applied):** Decide whether kli struct fields allow inline
commas. If yes, update `kli-spec.md`'s separator table. If kli is intentionally stricter
(newlines only in structs), add an explicit note to `ilk-spec.md` distinguishing the two
languages on this point.

---

## Constraint language gaps

The current constraint expression language provides only `forall` and `.assoc(t)`,
composed with `&&` / `||`. Several common predicates are absent:

| Missing | Notes |
|---|---|
| `exists(col, x => body)` | Existential counterpart to `forall` |
| `count(col)` | Cardinality check |
| Comparison operators (`==`, `!=`, `<`, `>`) | Needed for numeric/string constraints |
| Negation (`!expr`) | Simple complement |
| Field access in predicate body (`x.field`) | Walk into struct elements |

The spec notes "Additional built-ins may be added as the language evolves," but gives no
roadmap.

**Recommendation (not yet applied):** Add a "Planned additions" subsection listing at
minimum `exists` and equality comparison (`==`), so implementors can reserve syntax and
leave extension hooks without breaking changes.

---

## Summary of changes applied

| File | Changes |
|---|---|
| `examples/dcb-board-spec.ilk` | `@root` → `@main`; `{1 Type}` → `{_}`, `{* Type}` → `{...}`; introduced `TagField {_}` named block; updated `Tag`, `Event`, `fields`; removed `@source [fields] for [tags]` |
| `examples/dcb-board-instance-valid.kli` | `Tag {…}` bindings → `TagField {…}`; improved inline comments |
| `ilk-spec.md` | Comments section example updated; discriminated-unions section updated to named-block form; `@source` section: added `for [F]` qualifier docs and inline binding refinement docs; full example updated |
| `kli-spec.md` | Bindings example updated; union-values section updated; added "Inline binding refinements" section; added "Anonymous struct instantiation" section; full example updated |

## Open items (require further design decisions)

1. **§7** — Clarify kli semantics of `String` literals (shape vs runtime placeholder)
2. **§8** — Specify `@source` implicit name-matching depth (one level vs recursive)
3. **§9** — Decide whether kli struct fields allow inline commas
4. **Constraint language** — Add `exists`, comparisons, and roadmap note
