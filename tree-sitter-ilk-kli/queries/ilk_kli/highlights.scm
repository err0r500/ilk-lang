; Comments
(comment) @comment

; Base types
(base_type) @type.builtin

; Type identifiers
(named_type (identifier) @type)

; Field names
(ilk_field name: (identifier) @variable.member)
(value_field name: (identifier) @variable.member)

; Block/binding names
(block name: (identifier) @type.definition)
(binding name: (identifier) @variable.definition)

; Annotations
"@" @attribute
"main" @attribute
"out" @attribute
"assoc" @attribute
"source" @attribute
"constraint" @attribute
"doc" @attribute

; Constraint functions
"forall" @function.builtin
"exists" @function.builtin
"unique" @function.builtin
"count" @function.builtin
"templateVars" @function.builtin
"keys" @function.builtin
"compute" @function.builtin

; Keywords
"Concrete" @keyword

; Operators
"|" @operator
"&" @operator
"=" @operator
"*" @operator
"=>" @operator
"&&" @operator
"||" @operator
"!" @operator
"==" @operator
"!=" @operator
"<" @operator
"<=" @operator
">" @operator
">=" @operator
"in" @keyword.operator
".." @operator

; Literals
(string) @string
(number) @number
(boolean) @boolean

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"," @punctuation.delimiter
"." @punctuation.delimiter
"?" @punctuation.special

; Special struct markers
"..." @punctuation.special
"_" @variable.parameter
