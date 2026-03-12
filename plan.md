# ilk/kli Compiler Plan

## Goal

Rust compiler (lexer, parser, validator) for ilk/kli using chumsky 0.10 + ariadne 0.4

## Module Structure

```
src/
├── lib.rs / main.rs
├── span.rs           # Spanned<T> wrapper
├── error.rs          # ariadne error reporting
├── ilk/
│   ├── ast.rs        # TypeExpr, StructType, Annotation, ConstraintExpr
│   ├── parser.rs     # chumsky combinators
│   └── resolve.rs    # TypeEnv, forward refs, cycles
├── kli/
│   ├── ast.rs        # Binding, KliValue, FieldOrigin
│   └── parser.rs
└── validate/
    ├── structural.rs # type matching, cardinality
    ├── source.rs     # @source data flow
    └── constraint.rs # @constraint evaluation
```

## Implementation Order (Test-First)

### Phase 1: Shared infra

**Implement:** `Spanned<T>`, error types, ariadne integration

**Tests:**
```rust
#[test] fn spanned_preserves_range() // Spanned::new("x", 0..1).span == 0..1
#[test] fn error_renders_with_ariadne() // smoke test: error -> report -> no panic
```

---

### Phase 2: ilk parser

#### 2.1 Base types & literals
**Tests:**
```rust
// base types
parse("*") => TypeExpr::Wildcard
parse("String") => TypeExpr::Base(String)
parse("Int") => TypeExpr::Base(Int)
// ... Uuid, Float, Bool, Date, Timestamp, Money

// concrete
parse("Concrete<String>") => TypeExpr::Concrete(Base(String))
parse("Concrete<Int>") => TypeExpr::Concrete(Base(Int))

// literals
parse("\"hello\"") => TypeExpr::LitString("hello")
parse("42") => TypeExpr::LitInt(42)
parse("-5") => TypeExpr::LitInt(-5)
parse("true") => TypeExpr::LitBool(true)
parse("false") => TypeExpr::LitBool(false)
```

#### 2.2 Structs
**Tests:**
```rust
parse("{}") => Struct(Closed, [])
parse("{...}") => Struct(Open, [])
parse("{_}") => Struct(Anonymous([None]))  // 1 field, any type
parse("{_ String}") => Struct(Anonymous([Some(String)]))
parse("{_, _}") => Struct(Anonymous([None, None]))
parse("{x Int}") => Struct(Closed, [Field("x", Int)])
parse("{x Int, y String}") => Struct(Closed, [Field("x", Int), Field("y", String)])
// multiline
parse("{\n  x Int\n  y String\n}") => same as above
```

#### 2.3 Lists
**Tests:**
```rust
parse("[]Event") => List(Any, Named("Event"))
parse("[3]Tag") => List(Exact(3), Named("Tag"))
parse("[1..]Tag") => List(AtLeast(1), Named("Tag"))
parse("[..10]Tag") => List(AtMost(10), Named("Tag"))
parse("[2..5]Tag") => List(Range(2,5), Named("Tag"))
```

#### 2.4 References
**Tests:**
```rust
parse("&Event") => Reference("Event")
parse("[]&Event") => List(Any, Reference("Event"))
```

#### 2.5 Union & Intersection
**Tests:**
```rust
// union
parse("A | B") => Union([Named("A"), Named("B")])
parse("A | B | C") => Union([Named("A"), Named("B"), Named("C")])

// intersection
parse("A & B") => Intersection(Named("A"), Named("B"))
parse("{...} & {x Int}") => Intersection(Struct(Open), Struct(Closed, [x]))

// precedence: & binds tighter than |
parse("A | B & C") => Union([Named("A"), Intersection(Named("B"), Named("C"))])
parse("A & B | C") => Union([Intersection(Named("A"), Named("B")), Named("C")])
```

#### 2.6 Blocks
**Tests:**
```rust
parse("Foo {x Int}") => Block("Foo", Struct(...))
parse("Foo String") => Block("Foo", Base(String))
parse("Foo A | B") => Block("Foo", Union(...))
parse("Foo {...} & {id Uuid}") => Block("Foo", Intersection(...))
```

#### 2.7 Annotations
**Tests:**
```rust
parse("@main\nFoo {...}") => Block with annotations=[Main]
parse("@assoc [Tag]\nEvent {...}") => Block with annotations=[Assoc(["Tag"])]
parse("@assoc [Tag, Other]\nE {...}") => Assoc(["Tag", "Other"])
parse("@source [fields]\nemits []Event") => Field with annotations=[Source(["fields"])]
parse("@source [fields, body]\ndb {...}") => Source(["fields", "body"])
parse("@source [db.returns]\nresp {...}") => Source with path segments
parse("@out\nreturns {...}") => Field with annotations=[Out]
```

#### 2.8 Constraints
**Tests:**
```rust
// expressions
parse_constraint("true") => Bool(true)
parse_constraint("x") => Var("x")
parse_constraint("x.field") => FieldAccess(Var("x"), "field")

// functions
parse_constraint("forall(col, x => body)") => ForAll("col", "x", body)
parse_constraint("exists(tags, t => t.active)") => Exists(...)
parse_constraint("unique(items, i => i.id)") => Unique(...)
parse_constraint("count(tags)") => Count("tags")
parse_constraint("e.assoc(t)") => Assoc(Var("e"), Var("t"))
parse_constraint("templateVars(path)") => TemplateVars(Var("path"))
parse_constraint("keys(params)") => Keys(Var("params"))

// operators
parse_constraint("a && b") => And(a, b)
parse_constraint("a || b") => Or(a, b)
parse_constraint("!a") => Not(a)
parse_constraint("a == b") => Eq(a, b)
parse_constraint("a != b") => Ne(a, b)
parse_constraint("x in set") => In(x, set)
parse_constraint("count(x) >= 1") => Ge(Count("x"), Int(1))

// complex
parse_constraint("forall(tags, t => forall(events, e => e.assoc(t)))")
parse_constraint("forall(templateVars(path), v => v in keys(params))")
```

#### 2.9 Full ilk file
**Tests:**
```rust
// parse examples/dcb-board-spec.ilk => success, correct AST structure
// parse examples/api-spec.ilk (from spec) => success
```

---

### Phase 3: ilk resolution

**Tests:**
```rust
// block collection
resolve("Foo {x Int}") => TypeEnv with blocks={"Foo": ...}
resolve("@main\nFoo {}") => TypeEnv with main_block=Some("Foo")

// forward refs
resolve("A B\nB {x Int}") => A resolves to B's type

// cycles
resolve("A B\nB A") => Error::CyclicReference

// multiple @main
resolve("@main\nA {}\n@main\nB {}") => Error::MultipleMain
```

---

### Phase 4: kli parser

#### 4.1 Bindings
**Tests:**
```rust
parse_kli("foo = Type {}") => Binding("foo", "Type", [], Struct)
parse_kli("foo = Type<a, b> {}") => Binding("foo", "Type", ["a","b"], Struct)
parse_kli("foo = Type \"value\"") => Binding("foo", "Type", [], LitString)
```

#### 4.2 Field values
**Tests:**
```rust
// open types
parse_field("x String") => Field("x", TypeRef("String"), None)
parse_field("x Int") => Field("x", TypeRef("Int"), None)

// literals
parse_field("x \"hello\"") => Field("x", LitString("hello"), None)
parse_field("x 42") => Field("x", LitInt(42), None)

// nested
parse_field("x {a Int, b String}") => Field("x", Struct(...))
```

#### 4.3 Field origins
**Tests:**
```rust
parse_field("x Int*") => Field with origin=Generated
parse_field("x Int = fields.id") => Field with origin=Mapped(["fields","id"])
parse_field("x Int = a.b.c") => origin=Mapped(["a","b","c"])
parse_field("x Int = compute(a, b)") => origin=Computed([["a"],["b"]])
parse_field("x Int = compute(a.x, b.y)") => Computed([["a","x"],["b","y"]])
```

#### 4.4 Optional fields
**Tests:**
```rust
parse_field("x? String") => Field("x", optional=true, ...)
parse_field("email? String = fields.email") => optional=true, origin=Mapped
```

#### 4.5 List refinements
**Tests:**
```rust
parse_list("[a, b]") => List([BindingRef("a"), BindingRef("b")])
parse_list("[a & {x Int*}]") => List([Refinement("a", [Field("x", Int, Generated)])])
parse_list("[a & {x Int*, y String = fields.y}]") => Refinement with 2 origins
```

#### 4.6 Full kli file
**Tests:**
```rust
// parse examples/dcb-board-instance-valid.kli => success
// parse examples/api-instance.kli (from spec) => success
```

---

### Phase 5: Structural validation

**Tests:**
```rust
// type matching
validate(kli:"String", ilk:"String") => Ok
validate(kli:"String", ilk:"Int") => Err(TypeMismatch)
validate(kli:"\"x\"", ilk:"Concrete<String>") => Ok
validate(kli:"\"x\"", ilk:"String") => Err (literal vs open)
validate(kli:"\"x\"", ilk:"\"x\"") => Ok (exact match)
validate(kli:"\"x\"", ilk:"\"y\"") => Err(LiteralMismatch)

// structs
validate(kli:"{x Int}", ilk:"{x Int}") => Ok
validate(kli:"{x Int, y Bool}", ilk:"{x Int}") => Err(ExtraField)
validate(kli:"{x Int}", ilk:"{...}") => Ok (open accepts any)
validate(kli:"{x Int, y Bool}", ilk:"{...} & {x Int}") => Ok

// anonymous structs
validate(kli:"{a String}", ilk:"{_}") => Ok (1 field)
validate(kli:"{a Int, b Int}", ilk:"{_}") => Err(WrongFieldCount)
validate(kli:"{a String}", ilk:"{_ Int}") => Err(TypeMismatch on field)

// lists
validate(kli:"[a,b,c]", ilk:"[]T") => Ok
validate(kli:"[a,b]", ilk:"[3]T") => Err(CardinalityMismatch)
validate(kli:"[a]", ilk:"[1..]T") => Ok
validate(kli:"[]", ilk:"[1..]T") => Err
validate(kli:"[a,b,c,d,e,f]", ilk:"[..5]T") => Err

// unions
validate(kli:"VariantA {...}", ilk:"VariantA | VariantB") => Ok
validate(kli:"VariantC {...}", ilk:"VariantA | VariantB") => Err(UnknownVariant)

// references
validate(kli:"existingBinding", ilk:"&Event") => Ok (if existingBinding: Event)
validate(kli:"missing", ilk:"&Event") => Err(UnknownBinding)
validate(kli:"wrongType", ilk:"&Event") => Err(TypeMismatch)
```

---

### Phase 6: @source validation

**Tests:**
```rust
// implicit name match
validate_source(
  sources: ["fields"],
  field: "id String",  // no origin
  context: {fields: {id: String}}
) => Ok (matched by name)

validate_source(
  sources: ["fields"],
  field: "customerId String",  // no match
  context: {fields: {id: String}}
) => Err(NoSourceForField)

// explicit mapping
validate_source(
  sources: ["fields"],
  field: "customerId String = fields.id"
) => Ok

validate_source(
  sources: ["fields"],
  field: "x Int = other.id"  // root not in sources
) => Err(SourceNotFound)

// generated exemption
validate_source(
  sources: ["fields"],
  field: "timestamp Int*"
) => Ok (exempt)

// concrete exemption
validate_source(
  sources: ["fields"],
  field: "status 404"  // literal = Concrete
) => Ok (exempt)

// compute
validate_source(
  sources: ["fields"],
  field: "total Int = compute(fields.qty, fields.price)"
) => Ok

// subtype check
validate_source(
  field: "id String = fields.uuid",
  source_type: Uuid, target_type: String
) => Ok (Uuid <: String)

validate_source(
  field: "id Uuid = fields.str",
  source_type: String, target_type: Uuid
) => Err (String </: Uuid, needs compute)

// optional field
validate_source(
  field: "email String = fields.email",
  source: {email?: String}
) => Err(RequiredFromOptional)

validate_source(
  field: "email? String = fields.email",
  source: {email?: String}
) => Ok (both optional)
```

---

### Phase 7: @constraint evaluation

**Tests:**
```rust
// forall
eval("forall(tags, t => true)", {tags: [a,b,c]}) => true
eval("forall(tags, t => false)", {tags: [a]}) => false
eval("forall(tags, t => true)", {tags: []}) => true (vacuous)

// exists
eval("exists(tags, t => t.active)", {tags: [{active:true}, {active:false}]}) => true
eval("exists(tags, t => t.active)", {tags: [{active:false}]}) => false

// unique
eval("unique(items, i => i.id)", {items: [{id:1}, {id:2}]}) => true
eval("unique(items, i => i.id)", {items: [{id:1}, {id:1}]}) => false

// count
eval("count(tags) >= 1", {tags: [a]}) => true
eval("count(tags) >= 1", {tags: []}) => false

// assoc
eval("e.assoc(t)", e=Event<t>, t=someTag) => true
eval("e.assoc(t)", e=Event<other>, t=someTag) => false
eval("forall(tags, t => e.assoc(t))", e=Event<t1,t2>, tags=[t1,t2]) => true
eval("forall(tags, t => e.assoc(t))", e=Event<t1>, tags=[t1,t2]) => false

// templateVars
eval("templateVars(\"/users/{id}\")", _) => {"id"}
eval("templateVars(\"/users/{id}/posts/{postId}\")", _) => {"id", "postId"}

// keys
eval("keys(params)", {params: {id: _, name: _}}) => {"id", "name"}

// in
eval("v in keys(params)", v="id", params={id:_}) => true
eval("v in keys(params)", v="missing", params={id:_}) => false

// complex
eval("forall(templateVars(path), v => v in keys(params))",
     {path: "/users/{id}", params: {id: _}}) => true
eval("forall(templateVars(path), v => v in keys(params))",
     {path: "/users/{id}", params: {}}) => false
```

---

### Integration tests

**Tests:**
```rust
// valid example
validate_file("examples/dcb-board-spec.ilk", "examples/dcb-board-instance-valid.kli") => Ok

// invalid cases (create test fixtures)
validate_file("test/schema.ilk", "test/missing-field.kli") => Err with helpful message
validate_file("test/schema.ilk", "test/bad-source.kli") => Err with source hint
validate_file("test/schema.ilk", "test/constraint-fail.kli") => Err with constraint info
```

---

## Key Rules

- & binds tighter than |
- Intersection: right wins on conflict
- &T refs exempt from @source
- Concrete<T> = any literal, schema-fixed = exact match
- Anonymous {_} = exactly 1, {...} = any count
- Required field cannot map to optional source

## Critical Files

- ilk-spec.md - full ilk syntax/semantics
- kli-spec.md - full kli syntax/semantics
- examples/dcb-board-spec.ilk - reference ilk
- examples/dcb-board-instance-valid.kli - reference kli
