//! Schema and Field definitions

use std::sync::Arc;

/// Schema representing a collection of fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    fields: Vec<Arc<Field>>,
    metadata: Option<Vec<(String, String)>>,
}

impl Schema {
    pub fn new(fields: Vec<Arc<Field>>) -> Self {
        Self {
            fields,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: Vec<(String, String)>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn fields(&self) -> &[Arc<Field>] {
        &self.fields
    }

    pub fn metadata(&self) -> Option<&[(String, String)]> {
        self.metadata.as_deref()
    }

    pub fn field(&self, index: usize) -> Option<&Arc<Field>> {
        self.fields.get(index)
    }

    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }
}

/// Field representing a named column with a data type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    name: String,
    nullable: bool,
    metadata: Option<Vec<(String, String)>>,
}

impl Field {
    pub fn new(name: impl Into<String>, nullable: bool) -> Self {
        Self {
            name: name.into(),
            nullable,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: Vec<(String, String)>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn metadata(&self) -> Option<&[(String, String)]> {
        self.metadata.as_deref()
    }
}
