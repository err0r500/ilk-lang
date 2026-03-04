use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Annotation {
    Unique,
    Required,
    Source(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    Primitive(String),
    RefTo(String),
    ListOf(Box<SchemaType>),
    Object(FieldsDef),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldsDef {
    Fixed(HashMap<String, FieldDef>),
    Wildcard(Box<FieldDef>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    pub typ: SchemaType,
    pub optional: bool,
    pub generated: bool,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDef {
    pub name: String,
    pub extends: Option<String>,
    pub fields: FieldsDef,
    pub nested_blocks: HashMap<String, BlockDef>,
    pub annotations: Vec<Annotation>,
}

impl BlockDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            extends: None,
            fields: FieldsDef::Fixed(HashMap::new()),
            nested_blocks: HashMap::new(),
            annotations: vec![],
        }
    }

    pub fn with_extends(mut self, parent: impl Into<String>) -> Self {
        self.extends = Some(parent.into());
        self
    }

    pub fn with_field(mut self, name: impl Into<String>, field: FieldDef) -> Self {
        if let FieldsDef::Fixed(ref mut map) = self.fields {
            map.insert(name.into(), field);
        }
        self
    }

    pub fn with_wildcard_field(mut self, field: FieldDef) -> Self {
        self.fields = FieldsDef::Wildcard(Box::new(field));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Schema {
    pub block_defs: HashMap<String, BlockDef>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_block(&mut self, def: BlockDef) {
        self.block_defs.insert(def.name.clone(), def);
    }

    pub fn get_block(&self, name: &str) -> Option<&BlockDef> {
        self.block_defs.get(name)
    }

    pub fn resolve_extends(&self, name: &str) -> Vec<&BlockDef> {
        let mut chain = vec![];
        let mut current = name;
        while let Some(def) = self.get_block(current) {
            chain.push(def);
            match &def.extends {
                Some(parent) => current = parent,
                None => break,
            }
        }
        chain
    }
}

impl FieldDef {
    pub fn new(typ: SchemaType) -> Self {
        Self {
            typ,
            optional: false,
            generated: false,
            annotations: vec![],
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn generated(mut self) -> Self {
        self.generated = true;
        self
    }

    pub fn with_annotation(mut self, ann: Annotation) -> Self {
        self.annotations.push(ann);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_builder() {
        let mut schema = Schema::new();

        let event = BlockDef::new("event")
            .with_field(
                "name",
                FieldDef::new(SchemaType::Primitive("string".into())),
            )
            .with_field(
                "timestamp",
                FieldDef::new(SchemaType::Primitive("int".into())),
            );

        let user_event = BlockDef::new("userEvent").with_extends("event").with_field(
            "userId",
            FieldDef::new(SchemaType::Primitive("string".into()))
                .with_annotation(Annotation::Required),
        );

        schema.add_block(event);
        schema.add_block(user_event);

        assert!(schema.get_block("event").is_some());
        assert!(schema.get_block("userEvent").is_some());

        let chain = schema.resolve_extends("userEvent");
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].name, "userEvent");
        assert_eq!(chain[1].name, "event");
    }
}
