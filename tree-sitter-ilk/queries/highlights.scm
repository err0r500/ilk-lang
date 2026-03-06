(type_ident) @type
(ident) @variable
(comment) @comment
(number) @number
(wildcard) @constant

"@constraint" @keyword
"extends" @keyword
"Concrete" @type.builtin

["&" "|"] @operator
["?" "*"] @punctuation.special
["{" "}" "[" "]" "<" ">" "(" ")"] @punctuation.bracket
["," "."] @punctuation.delimiter
["=>" "&&" "||" "==" "!=" "!" ">" "<" ">=" "<="] @operator

(annotation "@" @attribute)
(annotation (ident) @attribute)
