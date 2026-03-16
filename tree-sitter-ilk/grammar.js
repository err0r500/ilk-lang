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

// Helper: comma-or-newline separated list (1+)
function commaSep1(rule) {
  return seq(rule, repeat(seq(optional(","), rule)), optional(","));
}

module.exports = grammar({
  name: "ilk",

  extras: ($) => [/\s/, $.comment],

  conflicts: ($) => [[$.binding_ref, $.variant_value]],

  rules: {
    source_file: ($) => repeat($._definition),

    _definition: ($) => choice($.type_decl, $.instance, $.import_stmt),

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
    // Base Types
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
    // Type Expressions
    // _type_expr is anonymous (hidden node) - no wrapper in CST
    // ===================
    _type_expr: ($) =>
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
        $.refinable_ref,
        $.struct_type,
        $.named_type,
        $.literal_type
      ),

    named_type: ($) => $.identifier,

    literal_type: ($) => choice($.string, $.number, $.boolean),

    concrete_type: ($) =>
      seq("Concrete", "<", $._type_expr, ">"),

    list_type: ($) => seq("[", optional($.cardinality), "]", $._primary_type),

    cardinality: ($) =>
      choice(
        seq($.number, "..", $.number), // [N..M]
        seq($.number, ".."),           // [N..]
        seq("..", $.number),           // [..M]
        $.number                       // [N]
      ),

    reference_type: ($) => seq("&", $.identifier),

    // -TypeName: a type reference that allows value-level refinement
    refinable_ref: ($) => seq("-", $.identifier),

    struct_type: ($) =>
      seq(
        "{",
        optional(choice(
          "...",                // open struct {…}
          $.anonymous_fields,  // {_ Type, _ Type}
          $.field_list         // closed struct {x Int, y String}
        )),
        "}"
      ),

    anonymous_fields: ($) => commaSep1($.anonymous_field),

    anonymous_field: ($) => seq("_", optional($._primary_type)),

    field_list: ($) => commaSep1($.field),

    field: ($) =>
      seq(
        repeat($.annotation),
        field("name", $.identifier),
        optional(choice("?", "!")),
        field("type", $._type_expr)
      ),

    union_type: ($) =>
      prec.left(PREC.UNION, seq($._type_expr, "|", $._type_expr)),

    intersection_type: ($) =>
      prec.right(PREC.INTERSECTION, seq($._type_expr, "&", $._type_expr)),

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
          seq("constraint", $._constraint_expr),
          seq("doc", $.string)
        )
      ),

    annotation_args: ($) => commaSep1($.identifier),

    source_args: ($) => commaSep1($.source_path),

    source_path: ($) => seq($.identifier, repeat(seq(".", $.identifier))),

    // ===================
    // Constraint Expressions
    // _constraint_expr is anonymous (hidden node) - no wrapper in CST
    // ===================
    _constraint_expr: ($) =>
      choice(
        $.constraint_or,
        $.constraint_and,
        $.constraint_not,
        $.constraint_comparison,
        $._constraint_primary
      ),

    constraint_or: ($) =>
      prec.left(PREC.OR, seq($._constraint_expr, "||", $._constraint_expr)),

    constraint_and: ($) =>
      prec.left(PREC.AND, seq($._constraint_expr, "&&", $._constraint_expr)),

    constraint_not: ($) =>
      prec.right(PREC.NOT, seq("!", $._constraint_expr)),

    constraint_comparison: ($) =>
      prec.left(
        PREC.COMPARISON,
        seq(
          $._constraint_expr,
          choice("==", "!=", "<", "<=", ">", ">=", "in"),
          $._constraint_expr
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

    constraint_paren: ($) => seq("(", $._constraint_expr, ")"),

    constraint_call: ($) =>
      choice(
        // forall(col, x => body)
        seq(
          choice("forall", "exists", "unique"),
          "(",
          $._constraint_expr,
          ",",
          $.identifier,
          "=>",
          $._constraint_expr,
          ")"
        ),
        // count(col)
        seq("count", "(", $._constraint_expr, ")"),
        // templateVars(str)
        seq("templateVars", "(", $._constraint_expr, ")"),
        // keys(struct)
        seq("keys", "(", $._constraint_expr, ")"),
        // e.assoc(t)
        prec.left(PREC.ASSOC_CALL, seq($._constraint_expr, ".", "assoc", "(", $._constraint_expr, ")"))
      ),

    constraint_field_access: ($) =>
      prec.left(PREC.FIELD_ACCESS, seq($._constraint_expr, ".", $.identifier)),

    constraint_var: ($) => $.identifier,

    constraint_literal: ($) => choice($.boolean, $.number, $.string),

    // ===================
    // Top-Level Items
    // ===================

    // type Name = TypeExpr
    type_decl: ($) =>
      seq(
        repeat($.annotation),
        "type",
        field("name", $.identifier),
        "=",
        field("body", $._type_expr)
      ),

    // [@main] [@doc "..."] name = TypeName<assocs?> value
    instance: ($) =>
      seq(
        optional($.main_annotation),
        optional($.doc_annotation),
        field("name", $.identifier),
        "=",
        field("type", $.identifier),
        optional($.assoc_list),
        field("value", $._value)
      ),

    main_annotation: ($) => "@main",

    doc_annotation: ($) => seq("@doc", $.string),

    assoc_list: ($) => seq("<", commaSep1($.identifier), ">"),

    // import "path" [as alias]
    import_stmt: ($) =>
      seq(
        "import",
        field("path", $.string),
        optional(seq("as", field("alias", $.identifier)))
      ),

    // ===================
    // Values (instance-level)
    // _value is anonymous (hidden node) - no wrapper in CST
    // ===================
    _value: ($) =>
      choice(
        $.struct_value,
        $.assoc_struct_value,
        $.list_value,
        $.type_ref,
        $.literal_value,
        $.binding_ref,
        $.variant_value
      ),

    assoc_struct_value: ($) =>
      prec(1, seq($.assoc_list, $.struct_value)),

    type_ref: ($) => $.base_type,

    literal_value: ($) => choice($.string, $.number, $.boolean),

    binding_ref: ($) => $.identifier,

    struct_value: ($) =>
      seq("{", optional($.value_field_list), "}"),

    value_field_list: ($) => commaSep1($.value_field),

    value_field: ($) =>
      seq(
        optional($.doc_annotation),
        field("name", $.identifier),
        optional("?"),
        field("value", $._value),
        optional($.field_origin)
      ),

    field_origin: ($) =>
      choice(
        "*",                                            // generated
        seq("=", $.origin_path),                        // mapped
        seq("=", "compute", "(", $.origin_paths, ")")   // computed
      ),

    origin_path: ($) =>
      choice(
        seq($.identifier, repeat(seq(".", $.identifier))),
        seq("$assoc", repeat1(seq(".", $.identifier)))
      ),

    origin_paths: ($) => commaSep1($.origin_path),

    list_value: ($) =>
      seq("[", optional($.list_elements), "]"),

    list_elements: ($) => commaSep1($.list_element),

    list_element: ($) =>
      choice(
        $.refinement,
        $._value
      ),

    refinement: ($) => seq($.identifier, "&", $.struct_value),

    variant_value: ($) => seq($.identifier, $._value),
  },
});
