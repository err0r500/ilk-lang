use crate::ast::*;
use crate::span::S;

pub struct Formatter<'a> {
    output: String,
    indent: usize,
    comments: &'a [Comment],
    comment_idx: usize,
    last_pos: usize,
    source: &'a str,
}

impl<'a> Formatter<'a> {
    pub fn new(comments: &'a [Comment], source: &'a str) -> Self {
        Self {
            output: String::new(),
            indent: 0,
            comments,
            comment_idx: 0,
            last_pos: 0,
            source,
        }
    }

    pub fn format(file: &File, source: &str) -> String {
        let mut f = Formatter::new(&file.comments, source);
        f.format_file(file);
        f.output
    }

    /// Count blank lines in source between two positions
    fn count_blank_lines_between(&self, start: usize, end: usize) -> usize {
        if start >= end || start >= self.source.len() {
            return 0;
        }
        let slice = &self.source[start..end.min(self.source.len())];
        let newline_count = slice.chars().filter(|&c| c == '\n').count();
        // blank lines = newlines - 1 (one newline = no blank line)
        newline_count.saturating_sub(1)
    }

    /// Emit blank lines (capped at 1) based on original source
    fn emit_preserved_blank_lines(&mut self, next_pos: usize) {
        let blank_lines = self.count_blank_lines_between(self.last_pos, next_pos);
        let capped = blank_lines.min(1);
        for _ in 0..capped {
            self.writeln();
        }
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn writeln(&mut self) {
        self.output.push('\n');
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
    }

    /// Emit comments that appear before the given position
    fn emit_comments_before(&mut self, pos: usize) {
        while self.comment_idx < self.comments.len() {
            let c = &self.comments[self.comment_idx];
            if c.span.start < pos {
                // Check if we need a newline before this comment
                if !self.output.is_empty() && !self.output.ends_with('\n') {
                    self.writeln();
                }
                self.write_indent();
                self.write(&c.text);
                self.writeln();
                self.comment_idx += 1;
            } else {
                break;
            }
        }
    }

    /// Emit remaining comments at end of file
    fn emit_remaining_comments(&mut self) {
        while self.comment_idx < self.comments.len() {
            let c = &self.comments[self.comment_idx];
            if !self.output.is_empty() && !self.output.ends_with('\n') {
                self.writeln();
            }
            self.write(&c.text);
            self.writeln();
            self.comment_idx += 1;
        }
    }

    fn format_file(&mut self, file: &File) {
        let mut prev_import = false;
        let mut first = true;
        for item in &file.items {
            self.emit_comments_before(item.span.start);

            let is_import = matches!(&item.node, Item::Import(_));
            if !first {
                if prev_import && is_import {
                    // No blank line between consecutive imports
                } else {
                    // Two blank lines between top-level items
                    self.writeln();
                    self.writeln();
                }
            }
            first = false;
            prev_import = is_import;

            self.format_item(item);
            self.last_pos = item.span.end;
        }
        self.emit_remaining_comments();
    }

    fn format_item(&mut self, item: &S<Item>) {
        match &item.node {
            Item::Import(imp) => self.format_import(imp),
            Item::TypeDecl(td) => self.format_type_decl(td),
            Item::Instance(inst) => self.format_instance(inst),
        }
    }

    fn format_import(&mut self, imp: &Import) {
        self.write("import \"");
        self.write(&imp.path.node);
        self.write("\"");
        if let Some(alias) = &imp.alias {
            self.write(" as ");
            self.write(&alias.node);
        }
        self.writeln();
    }

    fn format_type_decl(&mut self, td: &TypeDecl) {
        for ann in &td.annotations {
            self.format_annotation(ann);
            self.writeln();
        }
        self.write("type ");
        self.write(&td.name.node);
        self.write(" = ");
        self.format_type_expr(&td.body);
        self.writeln();
    }

    fn format_annotation(&mut self, ann: &S<Annotation>) {
        match &ann.node {
            Annotation::Main => self.write("@main"),
            Annotation::Out => self.write("@out"),
            Annotation::Doc(s) => {
                self.write("@doc \"");
                self.write(s);
                self.write("\"");
            }
            Annotation::Assoc(names) => {
                self.write("@assoc [");
                for (i, n) in names.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(&n.node);
                }
                self.write("]");
            }
            Annotation::Source(paths) => {
                self.write("@source [");
                for (i, p) in paths.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_source_path(&p.node);
                }
                self.write("]");
            }
            Annotation::Constraint(expr) => {
                self.write("@constraint ");
                self.format_constraint_expr(expr);
            }
        }
    }

    fn format_source_path(&mut self, p: &SourcePath) {
        match p {
            SourcePath::Simple(s) => self.write(s),
            SourcePath::Dotted(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        self.write(".");
                    }
                    self.write(part);
                }
            }
        }
    }

    fn format_constraint_expr(&mut self, expr: &S<ConstraintExpr>) {
        match &expr.node {
            ConstraintExpr::Bool(b) => self.write(if *b { "true" } else { "false" }),
            ConstraintExpr::Int(n) => self.write(&n.to_string()),
            ConstraintExpr::Var(v) => self.write(v),
            ConstraintExpr::FieldAccess(e, f) => {
                self.format_constraint_expr(e);
                self.write(".");
                self.write(f);
            }
            ConstraintExpr::All(col, var, body) => {
                self.write("all(");
                self.format_constraint_expr(col);
                self.write(", ");
                self.write(var);
                self.write(" => ");
                self.format_constraint_expr(body);
                self.write(")");
            }
            ConstraintExpr::Exists(col, var, body) => {
                self.write("exists(");
                self.format_constraint_expr(col);
                self.write(", ");
                self.write(var);
                self.write(" => ");
                self.format_constraint_expr(body);
                self.write(")");
            }
            ConstraintExpr::Unique(col, var, body) => {
                self.write("unique(");
                self.format_constraint_expr(col);
                self.write(", ");
                self.write(var);
                self.write(" => ");
                self.format_constraint_expr(body);
                self.write(")");
            }
            ConstraintExpr::Count(col) => {
                self.write("count(");
                self.format_constraint_expr(col);
                self.write(")");
            }
            ConstraintExpr::Assoc(e, arg) => {
                self.format_constraint_expr(e);
                self.write(".assoc(");
                self.format_constraint_expr(arg);
                self.write(")");
            }
            ConstraintExpr::TemplateVars(e) => {
                self.write("templateVars(");
                self.format_constraint_expr(e);
                self.write(")");
            }
            ConstraintExpr::Keys(e) => {
                self.write("keys(");
                self.format_constraint_expr(e);
                self.write(")");
            }
            ConstraintExpr::And(l, r) => {
                self.format_constraint_expr(l);
                self.write(" && ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Or(l, r) => {
                self.format_constraint_expr(l);
                self.write(" || ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Not(e) => {
                self.write("!");
                self.format_constraint_expr(e);
            }
            ConstraintExpr::Eq(l, r) => {
                self.format_constraint_expr(l);
                self.write(" == ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Ne(l, r) => {
                self.format_constraint_expr(l);
                self.write(" != ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Lt(l, r) => {
                self.format_constraint_expr(l);
                self.write(" < ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Le(l, r) => {
                self.format_constraint_expr(l);
                self.write(" <= ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Gt(l, r) => {
                self.format_constraint_expr(l);
                self.write(" > ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::Ge(l, r) => {
                self.format_constraint_expr(l);
                self.write(" >= ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::In(l, r) => {
                self.format_constraint_expr(l);
                self.write(" in ");
                self.format_constraint_expr(r);
            }
            ConstraintExpr::IsType(e, type_name) => {
                self.write("isType(");
                self.format_constraint_expr(e);
                self.write(", ");
                self.write(type_name);
                self.write(")");
            }
        }
    }

    fn format_type_expr(&mut self, expr: &S<TypeExpr>) {
        match &expr.node {
            TypeExpr::Base(b) => self.format_base_type(b),
            TypeExpr::Concrete(inner) => {
                self.write("Concrete<");
                self.format_type_expr(inner);
                self.write(">");
            }
            TypeExpr::LitString(s) => {
                self.write("\"");
                self.write(s);
                self.write("\"");
            }
            TypeExpr::LitInt(n) => self.write(&n.to_string()),
            TypeExpr::LitBool(b) => self.write(if *b { "true" } else { "false" }),
            TypeExpr::Named(n) => self.write(n),
            TypeExpr::RefinableRef(n) => {
                self.write("-");
                self.write(n);
            }
            TypeExpr::Reference(n) => {
                self.write("&");
                self.write(n);
            }
            TypeExpr::List(card, inner) => {
                self.write("[");
                self.format_cardinality(card);
                self.write("]");
                self.format_type_expr(inner);
            }
            TypeExpr::Struct(kind) => self.format_struct_kind(kind),
            TypeExpr::Union(variants) => {
                for (i, v) in variants.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.format_type_expr(v);
                }
            }
            TypeExpr::Intersection(l, r) => {
                self.format_type_expr(l);
                self.write(" & ");
                self.format_type_expr(r);
            }
        }
    }

    fn format_base_type(&mut self, b: &BaseType) {
        let s = match b {
            BaseType::Wildcard => "*",
            BaseType::Uuid => "Uuid",
            BaseType::String => "String",
            BaseType::Int => "Int",
            BaseType::Float => "Float",
            BaseType::Bool => "Bool",
            BaseType::Date => "Date",
            BaseType::Timestamp => "Timestamp",
            BaseType::Money => "Money",
        };
        self.write(s);
    }

    fn format_cardinality(&mut self, c: &Cardinality) {
        match c {
            Cardinality::Any => {}
            Cardinality::Exact(n) => self.write(&n.to_string()),
            Cardinality::AtLeast(n) => {
                self.write(&n.to_string());
                self.write("..");
            }
            Cardinality::AtMost(n) => {
                self.write("..");
                self.write(&n.to_string());
            }
            Cardinality::Range(a, b) => {
                self.write(&a.to_string());
                self.write("..");
                self.write(&b.to_string());
            }
        }
    }

    fn format_struct_kind(&mut self, kind: &StructKind) {
        match kind {
            StructKind::Open(fields) if fields.is_empty() => {
                self.write("{...}");
            }
            StructKind::Open(fields) => {
                self.write("{...} & {");
                let multiline = fields.len() > 1;
                self.format_fields(fields);
                if multiline {
                    self.write_indent();
                }
                self.write("}");
            }
            StructKind::Closed(fields) if fields.is_empty() => {
                self.write("{}");
            }
            StructKind::Closed(fields) => {
                self.write("{");
                let multiline = fields.len() > 1;
                self.format_fields(fields);
                if multiline {
                    self.write_indent();
                }
                self.write("}");
            }
            StructKind::Anonymous(fields) => {
                self.write("{");
                for (i, f) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write("_");
                    if let Some(t) = f {
                        self.write(" ");
                        self.format_type_expr(t);
                    }
                }
                self.write("}");
            }
        }
    }

    fn format_fields(&mut self, fields: &[S<Field>]) {
        if fields.is_empty() {
            return;
        }

        let multiline = fields.len() > 1;

        if multiline {
            self.writeln();
            self.indent += 1;
        }

        for (i, field) in fields.iter().enumerate() {
            if multiline {
                // Preserve blank lines between fields
                if i > 0 {
                    self.emit_preserved_blank_lines(field.span.start);
                }
                self.emit_comments_before(field.span.start);
                self.write_indent();
            }

            for ann in &field.node.annotations {
                self.format_annotation(ann);
                if multiline {
                    self.writeln();
                    self.write_indent();
                } else {
                    self.write(" ");
                }
            }

            self.write(&field.node.name.node);
            if !field.node.optional {
                self.write("!");
            }
            self.write(" ");
            self.format_type_expr(&field.node.ty);

            if multiline {
                self.writeln();
                self.last_pos = field.span.end;
            } else if i < fields.len() - 1 {
                self.write(", ");
            }
        }

        if multiline {
            self.indent -= 1;
        }
    }

    fn format_instance(&mut self, inst: &Instance) {
        for ann in &inst.annotations {
            self.format_annotation(ann);
            self.writeln();
        }

        if let Some(doc) = &inst.doc {
            self.write("@doc \"");
            self.write(doc);
            self.write("\"");
            self.writeln();
        }

        self.write(&inst.name.node);
        self.write(" = ");
        self.write(&inst.type_name.node);

        if !inst.assocs.is_empty() {
            self.write("<");
            for (i, a) in inst.assocs.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&a.node);
            }
            self.write(">");
        }

        self.write(" ");
        self.format_value(&inst.body);
        self.writeln();
    }

    fn format_value(&mut self, val: &S<Value>) {
        match &val.node {
            Value::TypeRef(s) => self.write(s),
            Value::LitString(s) => {
                self.write("\"");
                self.write(s);
                self.write("\"");
            }
            Value::LitInt(n) => self.write(&n.to_string()),
            Value::LitBool(b) => self.write(if *b { "true" } else { "false" }),
            Value::BindingRef(s) => self.write(s),
            Value::Struct(fields) => self.format_instance_struct(fields),
            Value::List(elems) => self.format_instance_list(elems),
            Value::Variant(name, body) => {
                self.write(name);
                self.write(" ");
                self.format_value(body);
            }
            Value::Refinement(name, assocs, fields) => {
                self.write(name);
                if !assocs.is_empty() {
                    self.write(" <");
                    for (i, a) in assocs.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.write(&a.node);
                    }
                    self.write(">");
                }
                self.write(" & ");
                self.format_refinement_fields(fields);
            }
        }
    }

    fn format_instance_struct(&mut self, fields: &[S<InstanceField>]) {
        if fields.is_empty() {
            self.write("{}");
            return;
        }

        let multiline = fields.len() > 1;

        self.write("{");

        if multiline {
            self.writeln();
            self.indent += 1;
        }

        for (i, field) in fields.iter().enumerate() {
            if multiline {
                // Preserve blank lines between fields
                if i > 0 {
                    self.emit_preserved_blank_lines(field.span.start);
                }
                self.emit_comments_before(field.span.start);
                self.write_indent();
            }

            if let Some(doc) = &field.node.doc {
                self.write("@doc \"");
                self.write(doc);
                self.write("\" ");
                if multiline {
                    self.writeln();
                    self.write_indent();
                }
            }

            self.write(&field.node.name.node);
            if field.node.optional {
                self.write("?");
            }
            if !field.node.assocs.is_empty() {
                self.write(" <");
                for (i, a) in field.node.assocs.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(&a.node);
                }
                self.write(">");
            }
            self.write(" ");
            self.format_value(&field.node.value);
            self.format_field_origin(&field.node.origin);

            if multiline {
                self.writeln();
                self.last_pos = field.span.end;
            } else if i < fields.len() - 1 {
                self.write(", ");
            }
        }

        if multiline {
            self.indent -= 1;
            self.write_indent();
        }

        self.write("}");
    }

    fn format_field_origin(&mut self, origin: &FieldOrigin) {
        match origin {
            FieldOrigin::None => {}
            FieldOrigin::Generated => self.write("*"),
            FieldOrigin::Mapped(path) => {
                self.write(" = ");
                self.write(&path.join("."));
            }
            FieldOrigin::Computed(paths) => {
                self.write(" = compute(");
                for (i, p) in paths.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(&p.join("."));
                }
                self.write(")");
            }
        }
    }

    fn format_instance_list(&mut self, elems: &[S<ListElement>]) {
        if elems.is_empty() {
            self.write("[]");
            return;
        }

        let multiline = elems.len() > 1;

        self.write("[");

        if multiline {
            self.writeln();
            self.indent += 1;
        }

        for (i, elem) in elems.iter().enumerate() {
            if multiline {
                // Preserve blank lines between elements
                if i > 0 {
                    self.emit_preserved_blank_lines(elem.span.start);
                }
                self.emit_comments_before(elem.span.start);
                self.write_indent();
            }

            match &elem.node {
                ListElement::Value(v) => {
                    let tmp = S {
                        node: v.clone(),
                        span: elem.span.clone(),
                    };
                    self.format_value(&tmp);
                }
                ListElement::BindingRef(s) => self.write(s),
                ListElement::Refinement(name, assocs, fields) => {
                    self.write(name);
                    if !assocs.is_empty() {
                        self.write(" <");
                        for (i, a) in assocs.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.write(&a.node);
                        }
                        self.write(">");
                    }
                    self.write(" & ");
                    self.format_refinement_fields(fields);
                }
            }

            if multiline {
                self.writeln();
                self.last_pos = elem.span.end;
            } else if i < elems.len() - 1 {
                self.write(", ");
            }
        }

        if multiline {
            self.indent -= 1;
            self.write_indent();
        }

        self.write("]");
    }

    fn format_refinement_fields(&mut self, fields: &[S<InstanceField>]) {
        self.write("{");
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&field.node.name.node);
            if field.node.optional {
                self.write("?");
            }
            if !field.node.assocs.is_empty() {
                self.write(" <");
                for (j, a) in field.node.assocs.iter().enumerate() {
                    if j > 0 {
                        self.write(", ");
                    }
                    self.write(&a.node);
                }
                self.write(">");
            }
            self.write(" ");
            self.format_value(&field.node.value);
            self.format_field_origin(&field.node.origin);
        }
        self.write("}");
    }
}

pub fn format(file: &File, source: &str) -> String {
    Formatter::format(file, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use std::path::Path;

    fn roundtrip(src: &str) -> String {
        let file = parser::parse(src, Path::new("test.ilk")).unwrap();
        format(&file, src)
    }

    #[test]
    fn test_format_simple_type() {
        let src = "type Foo = String\n";
        let out = roundtrip(src);
        assert_eq!(out, src);
    }

    #[test]
    fn test_format_preserves_comment() {
        let src = "// comment\ntype Foo = String\n";
        let out = roundtrip(src);
        assert!(out.contains("// comment"));
        assert!(out.contains("type Foo = String"));
    }

    #[test]
    fn test_format_struct() {
        let src = "type Foo = {x! Int, y! String}\n";
        let out = roundtrip(src);
        assert!(out.contains("x!"));
        assert!(out.contains("y!"));
    }

    #[test]
    fn test_comment_between_items() {
        let src = "type A = Int\n// middle comment\ntype B = String\n";
        let out = roundtrip(src);
        assert!(out.contains("// middle comment"));
        assert!(out.contains("type A = Int"));
        assert!(out.contains("type B = String"));
    }

    #[test]
    fn test_comment_at_end() {
        let src = "type A = Int\n// trailing comment\n";
        let out = roundtrip(src);
        assert!(out.contains("// trailing comment"));
    }

    #[test]
    fn test_instance_with_assocs() {
        let src = "foo = Bar<a, b> {x Int}\n";
        let out = roundtrip(src);
        assert!(out.contains("Bar<a, b>"));
    }

    #[test]
    fn test_import() {
        let src = "import \"./base.ilk\"\n";
        let out = roundtrip(src);
        assert_eq!(out, src);
    }

    #[test]
    fn test_import_with_alias() {
        let src = "import \"./base.ilk\" as base\n";
        let out = roundtrip(src);
        assert_eq!(out, src);
    }
}
