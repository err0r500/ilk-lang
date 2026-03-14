 Refinable Type References Feature

 Summary

 Add - prefix to type references (-Event, -Bla) that allows refinement with concrete values at instance level.

 Syntax

 Type level:
 type Bla = {_ String}

 type Something = {
     bla -Bla        // refinable type reference
     events []-Event // refinable in list context too
 }

 Instance level:
 hello = Bla {name String}

 something = Something {
     bla hello & {name "hello"}  // refine to concrete
 }

 Changes

 1. AST (src/ast.rs)

 Add refinable flag to type expressions that can hold references:

 // Option A: Add Refinable wrapper
 TypeExpr::Refinable(Box<S<TypeExpr>>)  // wraps Named, List element types

 // Option B: Add flag to Named
 TypeExpr::Named(String, bool)  // 2nd param = refinable

 // Option C: New variant
 TypeExpr::RefinableRef(String)  // -TypeName

 Recommend Option C - cleanest for parsing.

 2. Parser (src/parser.rs)

 Type parsing:
 - Add refinable_type() parser: - followed by ident
 - Returns TypeExpr::RefinableRef(name)
 - Add to type_expr() choices

 List type:
 - []-Event = List(card, RefinableRef("Event"))
 - Already works if RefinableRef is a valid type atom

 3. Validation (src/validate/structural.rs)

 When validating refinement (ref & {fields}):
 1. Check if expected type is RefinableRef
 2. If refinable: allow concrete literals for open types
 3. If not refinable: error on concrete values for open types

 4. Source Validation (src/validate/source.rs)

 - May need updates for @source tracking with refinements

 Files to Modify

 - src/ast.rs - Add RefinableRef variant
 - src/parser.rs - Parse -TypeName syntax
 - src/validate/structural.rs - Check refinable flag when validating refinements

 Verification

 1. Parse -Event and []-Event
 2. Validate refinement with concrete value passes when - present
 3. Validate fails without - but concrete refinement provided
 4. cargo test
