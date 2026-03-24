; Comments
(comment) @comment

; Base types
(base_type) @type.builtin

; Type identifiers
(named_type (identifier) @type.builtin)

; Field names
(field name: (identifier) @variable.member)
(value_field name: (identifier) @variable.member)

; Type/instance names
(type_decl name: (identifier) @type.definition)
(instance name: (identifier) @variable.definition)
(instance type: (identifier) @type.builtin)

; Annotations
"@" @attribute
"main" @attribute
"out" @attribute
"source" @attribute
"constraint" @attribute
"doc" @attribute
(main_annotation) @attribute
(doc_annotation) @attribute

; Constraint functions
"all" @function.builtin
"exists" @function.builtin
"unique" @function.builtin
"count" @function.builtin
"templateVars" @function.builtin
"keys" @function.builtin
"compute" @function.builtin

; Keywords
"type" @keyword
"import" @keyword
"as" @keyword
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
"-" @operator
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
