## Review Findings

### Status
- ~~#2 Intersection Conflict Rule~~ — **FIXED** (right side wins unconditionally)
- ~~#10 Empty Struct `{}`~~ — **FIXED** (now allowed)

---

## ilk Concerns

### P1: Struct Cardinality Ambiguity
`{_}` vs `{String}` distinction unclear:
- `{_}` = exactly 1 field of any name and **type**
- `{String}` = one field of any name, type String (inferred from `Parametrized {String}`)

The `{_}` shorthand for `{_ Any}` conflates two concepts: anonymous fields and the `Any` type. Is `{_ String}` valid? Not documented.

**Fixes:**
1. Document `{_ T}` explicitly as valid syntax
2. Clarify `{T}` is shorthand for `{_ T}`
3. Or: remove `{_}` entirely, require `{_ Any}` always

### P2: `templateVars()` / `keys()` Typing
```ilk
@constraint forall(templateVars(path), v => v in keys(params))
```
`templateVars` returns `Set<String>`, `keys` returns `Set<String>`. But `path` is `Concrete<String>` — does `templateVars` work on open `String`? Probably not meaningful at validation time.

**Fixes:**
1. Require `Concrete<String>` argument for `templateVars()`
2. Add type signatures to constraint function docs
3. Document that these only make sense at kli validation time

### P3: Union Branch Restriction
> Inline anonymous struct expressions ({...}) are not valid as union branches

Forces naming everything. Why not allow inline structs with structural discrimination?

**Fixes:**
1. Keep as-is — forces explicit naming (simplicity)
2. Allow inline structs, discriminate by field shape
3. Allow only if fields are disjoint (unambiguous)

### P4: Reference Covariance
```ilk
&S <: &T when S <: T
```
References are read-only — covariance is correct. `[]&Event` with list covariance OK since lists aren't mutable at runtime.

**Fixes:** None needed — just documenting the reasoning would help.

---

## kli Concerns

### P1: @source Path Resolution Ambiguity
> Source paths are resolved from the enclosing block root

But kli example shows:
```kli
db insertUser & {
    name String = body.name  // body is sibling field, not db's field
}
```
This works because refinement inherits parent scope? Not explicitly stated.

**Fixes:**
1. Document scope inheritance explicitly: refinement sees parent scope
2. Add examples showing nested vs sibling resolution
3. Define precedence: local fields > parent scope > error

### P2: `compute()` Escape Hatch
```kli
id Uuid = compute(fields.id)  // fields.id is String, narrowing to Uuid
```
Bypasses type safety entirely. No constraint on what `compute()` can do.

**Fixes:**
1. Document as intentional escape hatch, runtime-validated only
2. Add optional type annotation: `compute<Uuid>(fields.id)`
3. Require explicit unsafe marker: `@unsafe compute(...)`

### P3: Optional Fields Semantics Mismatch
kli has `email? String`, ilk has no equivalent but says "all fields optional by default."

- ilk optional = may be absent
- kli `?` = downstream cannot rely on it

These are different concepts.

**Fixes:**
1. Clarify ilk "optional" means schema doesn't require presence
2. Rename kli `?` to something like `@unreliable` or `@volatile`
3. Or: align semantics — kli `?` also means "may be absent"

### P4: `@assoc` vs Fields Redundancy
```ilk
@assoc [Tag]
Event {...}
```
vs
```ilk
Event {
    tags []&Tag
    ...
}
```
Both express "Event relates to Tags." When to use which?

**Fixes:**
1. Document: `@assoc` for instance-time binding, fields for schema-time
2. `@assoc` enables `.assoc(t)` predicate — document this as primary distinction
3. Add guidance: use `@assoc` when cardinality varies per instance

---

## Summary by Priority

| Priority | Issue | Layer | Status |
|----------|-------|-------|--------|
| — | Intersection conflicts | ilk | ✅ Fixed |
| — | Empty struct `{}` | ilk | ✅ Fixed |
| P1 | @source path resolution | kli | Open |
| P1 | Struct cardinality `{_}` | ilk | Open |
| P2 | `compute()` typing | kli | Open |
| P2 | `templateVars`/`keys` types | ilk | Open |
| P3 | Optional `?` semantics | kli | Open |
| P3 | Union branch restriction | ilk | Open |
| P4 | `@assoc` vs fields | kli | Open |
| P4 | Reference covariance | ilk | OK (doc only) |
