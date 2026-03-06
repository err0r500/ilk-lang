module.exports = grammar({
  name: 'kli',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat($.definition),

    // name = Type<assoc> { ... }
    definition: $ => seq(
      $.ident,
      '=',
      $.type_ident,
      optional($.associations),
      optional($._body)
    ),

    associations: $ => seq('<', sep1($.ident, ','), '>'),

    _body: $ => choice(
      $.block_body,
      $.string
    ),

    block_body: $ => seq('{', repeat($._item), '}'),

    _item: $ => choice($.field, $.nested_block),

    field: $ => seq(
      $.ident,
      optional('?'),
      optional('*'),
      $._value
    ),

    nested_block: $ => seq(
      $.ident,
      optional($.associations),
      '{', repeat($._item), '}'
    ),

    _value: $ => choice(
      $.string,
      $.number,
      $.boolean,
      $.ident,
      $.type_ident,
      $.list,
      $.type_refinement
    ),

    string: $ => /"([^"\\]|\\.)*"/,
    number: $ => /-?[0-9]+(\.[0-9]+)?/,
    boolean: $ => choice('true', 'false'),

    list: $ => seq('[', optional($._list_content), ']'),
    _list_content: $ => sep1($._list_item, ','),
    _list_item: $ => choice(
      $.ident,
      $.type_ident,
      $.inline_object,
      $.type_refinement
    ),

    inline_object: $ => seq('{', sep1($.object_field, ','), '}'),
    object_field: $ => seq($.ident, $.list),

    type_refinement: $ => seq(
      $.ident,
      '{',
      repeat(seq($.ident, $.type_ident, optional('?'), optional('*'))),
      '}'
    ),

    type_ident: $ => /[A-Z][a-zA-Z0-9_]*/,
    ident: $ => /[a-z][a-zA-Z0-9_]*/,

    comment: $ => choice(
      /\/\/[^\n]*/,
      /\/\*[^*]*\*+([^/*][^*]*\*+)*\//
    ),
  }
});

function sep1(rule, sep) {
  return seq(rule, repeat(seq(sep, rule)));
}
