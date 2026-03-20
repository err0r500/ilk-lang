import { StreamLanguage } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

const ilkLang = StreamLanguage.define({
    token(stream) {
        // Comments
        if (stream.match("//")) {
            stream.skipToEnd();
            return "comment";
        }

        // Strings
        if (stream.match(/"[^"]*"/)) {
            return "string";
        }

        // Annotations
        if (stream.match(/@(main|assoc|source|constraint|doc|out)\b/)) {
            return "meta";
        }

        // Numbers
        if (stream.match(/\b\d+\b/)) {
            return "number";
        }

        // Base types
        if (
            stream.match(
                /\b(Any|Uuid|String|Int|Float|Bool|Date|Timestamp|Money)\b/,
            )
        ) {
            return "typeName";
        }

        // Concrete keyword
        if (stream.match(/\bConcrete\b/)) {
            return "keyword";
        }

        // type keyword
        if (stream.match(/\btype\b/)) {
            return "keyword";
        }

        // Constraint keywords
        if (
            stream.match(
                /\b(all|exists|unique|count|keys|templateVars|assoc|compute)\b/,
            )
        ) {
            return "keyword";
        }

        // Operators
        if (stream.match(/&&|\|\||==|!=|<=|>=|=>|[&|*=!<>]/)) {
            return "operator";
        }

        // in operator
        if (stream.match(/\bin\b/)) {
            return "operator";
        }

        // Type names (PascalCase)
        if (stream.match(/\b[A-Z][a-zA-Z0-9]*\b/)) {
            return "typeName";
        }

        // Brackets
        if (stream.match(/[\{\}\[\]\(\)<>]/)) {
            return "bracket";
        }

        // Variable names starting with $
        if (stream.match(/\$[a-zA-Z_][a-zA-Z0-9_.]*/)) {
            return "variableName";
        }

        // Identifiers
        if (stream.match(/[a-zA-Z_][a-zA-Z0-9_]*/)) {
            return "variableName";
        }

        // Skip whitespace and other chars
        stream.next();
        return null;
    },
});

export { ilkLang };
