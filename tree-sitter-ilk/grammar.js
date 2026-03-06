module.exports = grammar({
  name: 'ilk',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat($._top_level),

    _top_level: $ => choice($.block, $.type_def),

    // @assoc [Tag]
    // Tag {1 _ Type} | Concrete<String>
    type_def: $ => seq(
      repeat($.annotation),
      $.type_ident,
      optional($._type_body),
      repeat(seq(choice('|', '&'), $._type_expr))
    ),

    _type_body: $ => choice(
      $.type_args,
      $.wildcard_object
    ),

    _type_expr: $ => choice(
      $.type_ident,
      $.wildcard_object,
      $.concrete_type,
      $.inline_object
    ),

    block: $ => seq(
      $.type_ident,
      optional($.name),
      '{', repeat($._block_item), '}'
    ),

    name: $ => $.ident,

    _block_item: $ => choice($.field, $.nested_block, $.constraint, $.source),

    field: $ => seq(
      repeat($.annotation),
      $.ident,
      optional('?'),
      optional('*'),
      $._field_value
    ),

    _field_value: $ => choice(
      $.type_ident,
      $.wildcard,
      $.list_type
    ),

    nested_block: $ => seq(
      $.ident,
      $.wildcard_object
    ),

    constraint: $ => seq('@constraint', $.expr),

    source: $ => seq(
      '@source',
      '[', sep1($.ident, ','), ']',
      optional(seq('for', '[', sep1($.ident, ','), ']'))
    ),

    annotation: $ => seq('@', $.ident, optional(seq('[', sep1(choice($.ident, $.type_ident), ','), ']'))),

    wildcard: $ => '_',
    wildcard_object: $ => seq('{', choice('*', '0', '1', $.number), '_', $.type_ident, '}'),
    concrete_type: $ => seq('Concrete', '<', $.type_ident, '>'),
    list_type: $ => seq('[', ']', $.type_ident),
    inline_object: $ => seq('{', sep1(seq($.ident, $.type_ident), ','), '}'),

    type_args: $ => seq('<', sep1($.type_ident, ','), '>'),

    expr: $ => choice(
      $.call_expr,
      $.binary_expr,
      $.unary_expr,
      $.member_expr,
      $.lambda_expr,
      $.ident,
      seq('(', $.expr, ')')
    ),

    call_expr: $ => seq($.ident, '(', optional($.expr_list), ')'),
    binary_expr: $ => prec.left(1, seq($.expr, choice('&&', '||', '==', '!=', '>', '<', '>=', '<='), $.expr)),
    unary_expr: $ => prec(2, seq('!', $.expr)),
    member_expr: $ => prec.left(3, seq($.expr, '.', $.ident, optional(seq('(', optional($.expr_list), ')')))),
    lambda_expr: $ => seq($.ident, '=>', $.expr),
    expr_list: $ => sep1($.expr, ','),

    type_ident: $ => /[A-Z][a-zA-Z0-9_]*/,
    ident: $ => /[a-z][a-zA-Z0-9_]*/,
    number: $ => /[0-9]+/,

    comment: $ => choice(
      /\/\/[^\n]*/,
      /\/\*[^*]*\*+([^/*][^*]*\*+)*\//
    ),
  }
});

function sep1(rule, sep) {
  return seq(rule, repeat(seq(sep, rule)));
}
