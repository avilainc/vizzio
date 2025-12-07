//! JSON serialization and deserialization
//!
//! Native JSON implementation without serde.

use std::collections::HashMap;
use std::fmt;

/// JSON value representation
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Creates a JSON object from key-value pairs
    pub fn object(pairs: Vec<(&str, JsonValue)>) -> Self {
        let mut map = HashMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v);
        }
        JsonValue::Object(map)
    }

    /// Creates a JSON array
    pub fn array(values: Vec<JsonValue>) -> Self {
        JsonValue::Array(values)
    }

    /// Converts to JSON string
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => {
                if n.is_finite() {
                    if n.fract() == 0.0 {
                        format!("{:.0}", n)
                    } else {
                        n.to_string()
                    }
                } else {
                    "null".to_string()
                }
            }
            JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_json_string(k), v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }

    /// Gets a field from an object
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    /// Gets an array element
    pub fn index(&self, idx: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(idx),
            _ => None,
        }
    }

    /// Converts to string if possible
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Converts to number if possible
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Converts to bool if possible
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

impl From<f64> for JsonValue {
    fn from(n: f64) -> Self {
        JsonValue::Number(n)
    }
}

impl From<i32> for JsonValue {
    fn from(n: i32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<u32> for JsonValue {
    fn from(n: u32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<usize> for JsonValue {
    fn from(n: usize) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl<T: Into<JsonValue>> From<Vec<T>> for JsonValue {
    fn from(vec: Vec<T>) -> Self {
        JsonValue::Array(vec.into_iter().map(|v| v.into()).collect())
    }
}

/// Escape JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => result.push_str(&format!("\\u{:04x}", c as u32)),
            c => result.push(c),
        }
    }
    result
}

/// JSON builder macro helper
#[macro_export]
macro_rules! json {
    (null) => {
        $crate::json::JsonValue::Null
    };
    (true) => {
        $crate::json::JsonValue::Bool(true)
    };
    (false) => {
        $crate::json::JsonValue::Bool(false)
    };
    ($n:expr) => {
        $crate::json::JsonValue::from($n)
    };
    ([$($elem:tt),* $(,)?]) => {
        $crate::json::JsonValue::array(vec![$(json!($elem)),*])
    };
    ({$($key:tt : $value:tt),* $(,)?}) => {
        $crate::json::JsonValue::object(vec![$(($key, json!($value))),*])
    };
}

/// Simple JSON parser
pub struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Err("Unexpected end of input".to_string());
        }

        match self.current_char() {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            _ => Err(format!("Unexpected character: {}", self.current_char())),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        if self.match_str("null") {
            Ok(JsonValue::Null)
        } else {
            Err("Expected 'null'".to_string())
        }
    }

    fn parse_bool(&mut self) -> Result<JsonValue, String> {
        if self.match_str("true") {
            Ok(JsonValue::Bool(true))
        } else if self.match_str("false") {
            Ok(JsonValue::Bool(false))
        } else {
            Err("Expected 'true' or 'false'".to_string())
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;

        if self.current_char() == '-' {
            self.advance();
        }

        while self.pos < self.input.len() && self.current_char().is_numeric() {
            self.advance();
        }

        if self.pos < self.input.len() && self.current_char() == '.' {
            self.advance();
            while self.pos < self.input.len() && self.current_char().is_numeric() {
                self.advance();
            }
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        num_str
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| "Invalid number".to_string())
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '"' {
            return Err("Expected '\"'".to_string());
        }
        self.advance();

        let mut result = String::new();
        while self.pos < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.pos >= self.input.len() {
                    return Err("Unexpected end of string".to_string());
                }
                match self.current_char() {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    _ => result.push(self.current_char()),
                }
            } else {
                result.push(self.current_char());
            }
            self.advance();
        }

        if self.pos >= self.input.len() {
            return Err("Unterminated string".to_string());
        }

        self.advance(); // Skip closing quote
        Ok(JsonValue::String(result))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '[' {
            return Err("Expected '['".to_string());
        }
        self.advance();
        self.skip_whitespace();

        let mut elements = Vec::new();

        if self.current_char() == ']' {
            self.advance();
            return Ok(JsonValue::Array(elements));
        }

        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();

            if self.current_char() == ']' {
                self.advance();
                break;
            }

            if self.current_char() != ',' {
                return Err("Expected ',' or ']'".to_string());
            }
            self.advance();
            self.skip_whitespace();
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        if self.current_char() != '{' {
            return Err("Expected '{'".to_string());
        }
        self.advance();
        self.skip_whitespace();

        let mut map = HashMap::new();

        if self.current_char() == '}' {
            self.advance();
            return Ok(JsonValue::Object(map));
        }

        loop {
            self.skip_whitespace();
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err("Expected string key".to_string()),
            };

            self.skip_whitespace();
            if self.current_char() != ':' {
                return Err("Expected ':'".to_string());
            }
            self.advance();

            let value = self.parse_value()?;
            map.insert(key, value);

            self.skip_whitespace();
            if self.current_char() == '}' {
                self.advance();
                break;
            }

            if self.current_char() != ',' {
                return Err("Expected ',' or '}'".to_string());
            }
            self.advance();
        }

        Ok(JsonValue::Object(map))
    }

    fn current_char(&self) -> char {
        if self.pos < self.input.len() {
            self.input[self.pos]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn match_str(&mut self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if self.pos + chars.len() > self.input.len() {
            return false;
        }

        for (i, ch) in chars.iter().enumerate() {
            if self.input[self.pos + i] != *ch {
                return false;
            }
        }

        self.pos += chars.len();
        true
    }
}

/// Parse JSON string
pub fn parse(input: &str) -> Result<JsonValue, String> {
    JsonParser::new(input).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_to_string() {
        assert_eq!(JsonValue::Null.to_string(), "null");
        assert_eq!(JsonValue::Bool(true).to_string(), "true");
        assert_eq!(JsonValue::Number(42.0).to_string(), "42");
        assert_eq!(JsonValue::String("hello".to_string()).to_string(), "\"hello\"");
    }

    #[test]
    fn test_json_array() {
        let arr = JsonValue::array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(arr.to_string(), "[1,2,3]");
    }

    #[test]
    fn test_json_object() {
        let obj = JsonValue::object(vec![
            ("name", JsonValue::String("test".to_string())),
            ("value", JsonValue::Number(42.0)),
        ]);
        let json_str = obj.to_string();
        assert!(json_str.contains("\"name\":\"test\""));
        assert!(json_str.contains("\"value\":42"));
    }

    #[test]
    fn test_parse_null() {
        let result = parse("null").unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(parse("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(parse("3.14").unwrap(), JsonValue::Number(3.14));
        assert_eq!(parse("-10").unwrap(), JsonValue::Number(-10.0));
    }

    #[test]
    fn test_parse_string() {
        let result = parse(r#""hello""#).unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_array() {
        let result = parse("[1, 2, 3]").unwrap();
        match result {
            JsonValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], JsonValue::Number(1.0));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_object() {
        let result = parse(r#"{"name": "test", "value": 42}"#).unwrap();
        match result {
            JsonValue::Object(obj) => {
                assert_eq!(obj.len(), 2);
                assert_eq!(obj.get("name"), Some(&JsonValue::String("test".to_string())));
            }
            _ => panic!("Expected object"),
        }
    }
}
