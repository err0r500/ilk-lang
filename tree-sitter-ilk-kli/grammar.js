/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  UNION: 1,
  INTERSECTION: 2,
  OR: 3,
  AND: 4,
  COMPARISON: 5,
  NOT: 6,
  FIELD_ACCESS: 7,
  ASSOC_CALL: 8,
};

module.exports = grammar({
  name: "ilk_kli",

  extras: ($) => [/\s/, $.comment],

  conflicts: ($) => [[$.binding_ref, $.variant_value]],

  rules: {
    source_file: ($) => repeat($._definition),

    _definition: ($) => choice($.block, $.binding),

    // ===================
    // Comments
    // ===================
    comment: ($) => token(seq("//", /.*/)),

    // ===================
    // Identifiers & Literals
    // ===================
    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    string: ($) => /"([^"\\]|\\.)*"/,

    number: ($) => /-?[0-9]+/,

    boolean: ($) => choice("true", "false"),

    // ===================
    // Base Types (ilk)
    // ===================
    base_type: ($) =>
      choice(
        "*",
        "Uuid",
        "String",
        "Int",
        "Float",
        "Bool",
        "Date",
        "Timestamp",
        "Money"
      ),

    // ===================
    // Type Expressions (ilk)
    // ===================
    type_expr: ($) =>
      choice(
        $.union_type,
        $.intersection_type,
        $._primary_type
      ),

    _primary_type: ($) =>
      choice(
        $.base_type,
        $.concrete_type,
        $.list_type,
        $.reference_type,
        $.struct_type,
        $.named_type,
        $.literal_type
      ),

    named_type: ($) => $.identifier,

    literal_type: ($) => choice($.string, $.number, $.boolean),

    concrete_type: ($) =>
      seq("Concrete", "<", $.type_expr, ">"),

    list_type: ($) => seq("[", optional($.cardinality), "]", $._primary_type),

    cardinality: ($) =>
      choice(
        seq($.number, "..", $.number), // [N..M]
        seq($.number, ".."),           // [N..]
        seq("..", $.number),           // [..M]
        $.number                       // [N]
      ),

    reference_type: ($) => seq("&", $.identifier),

    struct_type: ($) =>
      seq(
        "{",
        optional(choice(
          "...",                  // open struct {…}
          $.anonymous_fields,     // {_ Type, _ Type}
          $.ilk_field_list       // closed struct {x Int, y String}
        )),
        "}"
      ),

    anonymous_fields: ($) =>
      seq($.anonymous_field, repeat(seq(choice(",", /\r?\n/), $.anonymous_field))),

    anonymous_field: ($) => seq("_", optional($._primary_type)),

    ilk_field_list: ($) =>
      seq($.ilk_field, repeat(seq(choice(",", /\r?\n/), $.ilk_field))),

    ilk_field: ($) =>
      seq(
        repeat($.annotation),
        field("name", $.identifier),
        optional("?"),
        field("type", $.type_expr)
      ),

    union_type: ($) =>
      prec.left(PREC.UNION, seq($.type_expr, "|", $.type_expr)),

    intersection_type: ($) =>
      prec.right(PREC.INTERSECTION, seq($.type_expr, "&", $.type_expr)),

    // ===================
    // Annotations
    // ===================
    annotation: ($) =>
      seq(
        "@",
        choice(
          "main",
          "out",
          seq("assoc", "[", $.annotation_args, "]"),
          seq("source", "[", $.source_args, "]"),
          seq("constraint", $.constraint_expr),
          seq("doc", $.string)
        )
      ),

    annotation_args: ($) =>
      seq($.identifier, repeat(seq(choice(",", /\r?\n/), $.identifier))),

    source_args: ($) =>
      seq($.source_path, repeat(seq(choice(",", /\r?\n/), $.source_path))),

    source_path: ($) => seq($.identifier, repeat(seq(".", $.identifier))),

    // ===================
    // Constraint Expressions (ilk)
    // ===================
    constraint_expr: ($) =>
      choice(
        $.constraint_or,
        $.constraint_and,
        $.constraint_not,
        $.constraint_comparison,
        $._constraint_primary
      ),

    constraint_or: ($) =>
      prec.left(PREC.OR, seq($.constraint_expr, "||", $.constraint_expr)),

    constraint_and: ($) =>
      prec.left(PREC.AND, seq($.constraint_expr, "&&", $.constraint_expr)),

    constraint_not: ($) =>
      prec.right(PREC.NOT, seq("!", $.constraint_expr)),

    constraint_comparison: ($) =>
      prec.left(
        PREC.COMPARISON,
        seq(
          $.constraint_expr,
          choice("==", "!=", "<", "<=", ">", ">=", "in"),
          $.constraint_expr
        )
      ),

    _constraint_primary: ($) =>
      choice(
        $.constraint_call,
        $.constraint_field_access,
        $.constraint_var,
        $.constraint_literal,
        $.constraint_paren
      ),

    constraint_paren: ($) => seq("(", $.constraint_expr, ")"),

    constraint_call: ($) =>
      choice(
        // forall(col, x => body)
        seq(
          choice("forall", "exists", "unique"),
          "(",
          $.constraint_expr,
          ",",
          $.identifier,
          "=>",
          $.constraint_expr,
          ")"
        ),
        // count(col)
        seq("count", "(", $.constraint_expr, ")"),
        // templateVars(str)
        seq("templateVars", "(", $.constraint_expr, ")"),
        // keys(struct)
        seq("keys", "(", $.constraint_expr, ")"),
        // e.assoc(t) - needs high precedence to bind tighter than field access
        prec.left(PREC.ASSOC_CALL, seq($.constraint_expr, ".", "assoc", "(", $.constraint_expr, ")"))
      ),

    constraint_field_access: ($) =>
      prec.left(PREC.FIELD_ACCESS, seq($.constraint_expr, ".", $.identifier)),

    constraint_var: ($) => $.identifier,

    constraint_literal: ($) => choice($.boolean, $.number, $.string),

    // ===================
    // Blocks (ilk definitions)
    // ===================
    // Block: Name TypeExpr (no = sign)
    block: ($) =>
      seq(
        repeat($.annotation),
        field("name", $.identifier),
        field("body", $.type_expr)
      ),

    // ===================
    // Bindings (kli definitions)
    // ===================
    // Binding: name = TypeName value
    binding: ($) =>
      seq(
        optional(seq("@doc", $.string)),
        field("name", $.identifier),
        "=",
        field("type", $.identifier),
        optional($.assoc_list),
        field("value", $.value)
      ),

    assoc_list: ($) => seq("<", $.identifier, repeat(seq(",", $.identifier)), ">"),

    // ===================
    // Values (kli)
    // ===================
    value: ($) =>
      choice(
        $.struct_value,
        $.list_value,
        $.type_ref,
        $.literal_value,
        $.binding_ref,
        $.variant_value
      ),

    type_ref: ($) => $.base_type,

    literal_value: ($) => choice($.string, $.number, $.boolean),

    binding_ref: ($) => $.identifier,

    struct_value: ($) =>
      seq("{", optional($.value_field_list), "}"),

    value_field_list: ($) =>
      seq($.value_field, repeat(seq(choice(",", /\r?\n/), $.value_field))),

    value_field: ($) =>
      seq(
        optional(seq("@doc", $.string)),
        field("name", $.identifier),
        optional("?"),
        field("value", $.value),
        optional($.field_origin)
      ),

    field_origin: ($) =>
      choice(
        "*",                                           // generated
        seq("=", $.origin_path),                       // mapped
        seq("=", "compute", "(", $.origin_paths, ")") // computed
      ),

    origin_path: ($) => seq($.identifier, repeat(seq(".", $.identifier))),

    origin_paths: ($) => seq($.origin_path, repeat(seq(",", $.origin_path))),

    list_value: ($) =>
      seq("[", optional($.list_elements), "]"),

    list_elements: ($) =>
      seq($.list_element, repeat(seq(choice(",", /\r?\n/), $.list_element))),

    list_element: ($) =>
      choice(
        // Refinement: binding & { origins }
        $.refinement,
        // Plain value or reference
        $.value
      ),

    refinement: ($) => seq($.identifier, "&", $.struct_value),

    variant_value: ($) => seq($.identifier, $.value),
  },
});
