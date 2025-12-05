//! Builders for nested arrays (List, Struct)

use crate::error::{Result, ArrowError};
use std::collections::HashMap;

/// Builder for ListArray
pub struct ListBuilder<T> {
    values: Vec<T>,
    offsets: Vec<i32>,
    nulls: Vec<bool>,
}

impl<T> ListBuilder<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            offsets: vec![0],
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, values: Vec<T>) {
        let new_offset = self.offsets.last().unwrap() + values.len() as i32;
        self.values.extend(values);
        self.offsets.push(new_offset);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        let last_offset = *self.offsets.last().unwrap();
        self.offsets.push(last_offset);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, values: Option<Vec<T>>) {
        match values {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.len() <= 1
    }

    pub fn finish(self) -> (Vec<T>, Vec<i32>, Vec<bool>) {
        (self.values, self.offsets, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.offsets.clear();
        self.offsets.push(0);
        self.nulls.clear();
    }
}

/// Builder for StructArray
pub struct StructBuilder {
    fields: Vec<String>,
    columns: HashMap<String, Vec<Box<dyn std::any::Any>>>,
    nulls: Vec<bool>,
    len: usize,
}

impl StructBuilder {
    pub fn new(fields: Vec<String>) -> Self {
        let mut columns = HashMap::new();
        for field in &fields {
            columns.insert(field.clone(), Vec::new());
        }

        Self {
            fields,
            columns,
            nulls: Vec::new(),
            len: 0,
        }
    }

    pub fn append_null(&mut self) {
        self.nulls.push(false);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        for values in self.columns.values_mut() {
            values.clear();
        }
        self.nulls.clear();
        self.len = 0;
    }
}

/// Builder for MapArray (key-value pairs)
pub struct MapBuilder<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    offsets: Vec<i32>,
    nulls: Vec<bool>,
}

impl<K, V> MapBuilder<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            offsets: vec![0],
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, entries: Vec<(K, V)>) {
        let new_offset = self.offsets.last().unwrap() + entries.len() as i32;
        for (key, value) in entries {
            self.keys.push(key);
            self.values.push(value);
        }
        self.offsets.push(new_offset);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        let last_offset = *self.offsets.last().unwrap();
        self.offsets.push(last_offset);
        self.nulls.push(false);
    }

    pub fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.len() <= 1
    }

    pub fn finish(self) -> (Vec<K>, Vec<V>, Vec<i32>, Vec<bool>) {
        (self.keys, self.values, self.offsets, self.nulls)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.offsets.clear();
        self.offsets.push(0);
        self.nulls.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_builder() {
        let mut builder = ListBuilder::new(10);
        builder.append_value(vec![1, 2, 3]);
        builder.append_value(vec![4, 5]);
        builder.append_null();
        builder.append_value(vec![6]);

        let (values, offsets, nulls) = builder.finish();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(offsets, vec![0, 3, 5, 5, 6]);
        assert_eq!(nulls, vec![true, true, false, true]);
    }

    #[test]
    fn test_list_builder_strings() {
        let mut builder = ListBuilder::new(5);
        builder.append_value(vec!["a".to_string(), "b".to_string()]);
        builder.append_null();
        builder.append_value(vec!["c".to_string()]);

        let (values, offsets, nulls) = builder.finish();
        assert_eq!(values.len(), 3);
        assert_eq!(offsets, vec![0, 2, 2, 3]);
        assert_eq!(nulls, vec![true, false, true]);
    }

    #[test]
    fn test_map_builder() {
        let mut builder = MapBuilder::new(10);
        builder.append_value(vec![("a", 1), ("b", 2)]);
        builder.append_value(vec![("c", 3)]);
        builder.append_null();

        let (keys, values, offsets, nulls) = builder.finish();
        assert_eq!(keys, vec!["a", "b", "c"]);
        assert_eq!(values, vec![1, 2, 3]);
        assert_eq!(offsets, vec![0, 2, 3, 3]);
        assert_eq!(nulls, vec![true, true, false]);
    }

    #[test]
    fn test_struct_builder() {
        let mut builder = StructBuilder::new(vec![
            "id".to_string(),
            "name".to_string(),
        ]);
        
        builder.append_null();
        assert_eq!(builder.len(), 1);
        assert_eq!(builder.nulls, vec![false]);
    }
}
