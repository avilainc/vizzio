//! Builders for primitive arrays
//!
//! Provides efficient builders for constructing primitive arrays with optional null values.

use crate::array::*;
use crate::error::{Result, ArrowError};

/// Builder for Int32Array
pub struct Int32Builder {
    values: Vec<i32>,
    nulls: Vec<bool>,
}

impl Int32Builder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: i32) {
        self.values.push(value);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(0);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<i32>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn append_slice(&mut self, values: &[i32]) {
        self.values.extend_from_slice(values);
        self.nulls.extend(std::iter::repeat(true).take(values.len()));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<Int32Array> {
        Int32Array::with_nulls(self.values, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

/// Builder for Int64Array
pub struct Int64Builder {
    values: Vec<i64>,
    nulls: Vec<bool>,
}

impl Int64Builder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: i64) {
        self.values.push(value);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(0);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<i64>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn append_slice(&mut self, values: &[i64]) {
        self.values.extend_from_slice(values);
        self.nulls.extend(std::iter::repeat(true).take(values.len()));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<Int64Array> {
        Int64Array::with_nulls(self.values, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

/// Builder for Float32Array
pub struct Float32Builder {
    values: Vec<f32>,
    nulls: Vec<bool>,
}

impl Float32Builder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: f32) {
        self.values.push(value);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(0.0);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<f32>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn append_slice(&mut self, values: &[f32]) {
        self.values.extend_from_slice(values);
        self.nulls.extend(std::iter::repeat(true).take(values.len()));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<Float32Array> {
        Float32Array::with_nulls(self.values, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

/// Builder for Float64Array
pub struct Float64Builder {
    values: Vec<f64>,
    nulls: Vec<bool>,
}

impl Float64Builder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: f64) {
        self.values.push(value);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(0.0);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<f64>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn append_slice(&mut self, values: &[f64]) {
        self.values.extend_from_slice(values);
        self.nulls.extend(std::iter::repeat(true).take(values.len()));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<Float64Array> {
        Float64Array::with_nulls(self.values, self.nulls)
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.nulls.clear();
    }
}

/// Builder for BooleanArray
pub struct BooleanBuilder {
    values: Vec<bool>,
    nulls: Vec<bool>,
}

impl BooleanBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            nulls: Vec::with_capacity(capacity),
        }
    }

    pub fn append_value(&mut self, value: bool) {
        self.values.push(value);
        self.nulls.push(true);
    }

    pub fn append_null(&mut self) {
        self.values.push(false);
        self.nulls.push(false);
    }

    pub fn append_option(&mut self, value: Option<bool>) {
        match value {
            Some(v) => self.append_value(v),
            None => self.append_null(),
        }
    }

    pub fn append_slice(&mut self, values: &[bool]) {
        self.values.extend_from_slice(values);
        self.nulls.extend(std::iter::repeat(true).take(values.len()));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn finish(self) -> Result<BooleanArray> {
        BooleanArray::with_nulls(self.values, self.nulls)
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
    fn test_int32_builder() {
        let mut builder = Int32Builder::new(10);
        builder.append_value(1);
        builder.append_value(2);
        builder.append_null();
        builder.append_value(4);

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 4);
        assert_eq!(array.value(0), Some(1));
        assert_eq!(array.value(1), Some(2));
        assert_eq!(array.value(2), None);
        assert_eq!(array.value(3), Some(4));
    }

    #[test]
    fn test_float64_builder() {
        let mut builder = Float64Builder::new(10);
        builder.append_value(1.5);
        builder.append_null();
        builder.append_value(3.14);

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array.value(0), Some(1.5));
        assert_eq!(array.value(1), None);
        assert_eq!(array.value(2), Some(3.14));
    }

    #[test]
    fn test_boolean_builder() {
        let mut builder = BooleanBuilder::new(10);
        builder.append_value(true);
        builder.append_value(false);
        builder.append_null();

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array.value(0), Some(true));
        assert_eq!(array.value(1), Some(false));
        assert_eq!(array.value(2), None);
    }

    #[test]
    fn test_append_slice() {
        let mut builder = Int32Builder::new(10);
        builder.append_slice(&[1, 2, 3, 4, 5]);

        let array = builder.finish().unwrap();
        assert_eq!(array.len(), 5);
        assert_eq!(array.value(0), Some(1));
        assert_eq!(array.value(4), Some(5));
    }
}
        Int32Array { data: self.data }
    }
}

/// Float64 array builder
#[derive(Debug, Default)]
pub struct Float64Builder {
    data: Vec<f64>,
}

impl Float64Builder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn append(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn finish(self) -> Float64Array {
        Float64Array { data: self.data }
    }
}
