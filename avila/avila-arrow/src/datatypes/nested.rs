//! Nested data types: List, Struct, Map, Union

use std::sync::Arc;

/// List type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListType {
    pub field: Arc<Field>,
}

/// Large list type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeListType {
    pub field: Arc<Field>,
}

/// Struct type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub fields: Vec<Arc<Field>>,
}

/// Map type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapType {
    pub key_field: Arc<Field>,
    pub value_field: Arc<Field>,
    pub sorted: bool,
}

/// Union type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionType {
    Sparse(Vec<Arc<Field>>),
    Dense(Vec<Arc<Field>>),
}

// Placeholder for Field type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
}
