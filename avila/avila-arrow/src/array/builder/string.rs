//! Builders for string and binary arrays

use crate::array::*;
use crate::error::{Result, ArrowError};

/// Builder for StringArray
pub struct StringBuilder {
    values: Vec<String>,
    nulls: Vec<bool>,
}

impl StringBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: impl Into<String>) {
        self.values.push(value.into());
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(String::new());
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<impl Into<String>>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<StringArray> {
        StringArray::with_nulls(self.values, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

/// Builder for BinaryArray
pub struct BinaryBuilder {
    values: Vec<Vec<u8>>,
    nulls: Vec<bool>,
}

impl BinaryBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: &[u8]) {
        self.values.push(value.to_vec());
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(Vec::new());
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<&[u8]>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Vec<Vec<u8>> {
        self.values
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_builder() {
        let mut builder = StringBuilder::new(10);
        builder.append_value("hello");
        builder.append_value("world");
        builder.append_null();
        builder.append_value("rust");

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 4);
        assert_eq!(array.value(0), Some("hello"));
        assert_eq!(array.value(1), Some("world"));
        assert_eq!(array.value(2), None);
        assert_eq!(array.value(3), Some("rust"));
    }

    #[test]
    fn test_string_builder_option() {
        let mut builder = StringBuilder::new(5);
        builder.append_option(Some("test"));
        builder.append_option(None::<String>);
        builder.append_option(Some("data"));

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array.value(0), Some("test"));
        assert_eq!(array.value(1), None);
        assert_eq!(array.value(2), Some("data"));
    }

    #[test]
    fn test_binary_builder() {
        let mut builder = BinaryBuilder::new(10);
        builder.append_value(&[1, 2, 3]);
        builder.append_value(&[4, 5]);
        builder.append_null();

        let result = builder.finish();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec![1, 2, 3]);
        assert_eq!(result[1], vec![4, 5]);
        assert_eq!(result[2], vec![]);
    }
}
