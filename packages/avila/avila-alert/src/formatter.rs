use crate::types::Alert;

/// Trait for formatting alerts
pub trait AlertFormatter {
    fn format(&self, alert: &Alert) -> String;
}

/// Simple text formatter
#[derive(Debug, Default, Clone, Copy)]
pub struct SimpleFormatter;

impl AlertFormatter for SimpleFormatter {
    fn format(&self, alert: &Alert) -> String {
        format!("[{}] {}", alert.level, alert.message)
    }
}

/// Detailed formatter with emoji and metadata
#[derive(Debug, Clone, Copy)]
pub struct DetailedFormatter {
    pub include_emoji: bool,
    pub include_timestamp: bool,
    pub include_tags: bool,
    pub include_context: bool,
}

impl Default for DetailedFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DetailedFormatter {
    pub fn new() -> Self {
        Self {
            include_emoji: true,
            include_timestamp: true,
            include_tags: true,
            include_context: true,
        }
    }

    pub fn with_emoji(mut self, include: bool) -> Self {
        self.include_emoji = include;
        self
    }

    pub fn with_timestamp(mut self, include: bool) -> Self {
        self.include_timestamp = include;
        self
    }

    pub fn with_tags(mut self, include: bool) -> Self {
        self.include_tags = include;
        self
    }

    pub fn with_context(mut self, include: bool) -> Self {
        self.include_context = include;
        self
    }
}

impl AlertFormatter for DetailedFormatter {
    fn format(&self, alert: &Alert) -> String {
        let mut parts = Vec::new();

        // Level with optional emoji
        if self.include_emoji {
            parts.push(format!("{} [{}]", alert.level.emoji(), alert.level));
        } else {
            parts.push(format!("[{}]", alert.level));
        }

        // Timestamp
        if self.include_timestamp {
            if let Some(timestamp) = alert.timestamp {
                parts.push(format!("[{}]", timestamp));
            }
        }

        // Message
        parts.push(alert.message.clone());

        // Tags
        if self.include_tags && !alert.tags.is_empty() {
            parts.push(format!("(tags: {})", alert.tags.join(", ")));
        }

        // Context
        if self.include_context && !alert.context.is_empty() {
            let context_str: Vec<String> = alert
                .context
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            parts.push(format!("{{context: {}}}", context_str.join(", ")));
        }

        parts.join(" ")
    }
}

/// JSON-like formatter (manual JSON serialization)
#[derive(Debug, Default, Clone, Copy)]
pub struct JsonFormatter {
    pub pretty: bool,
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    fn escape_json_string(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                c if c.is_control() => format!("\\u{:04x}", c as u32),
                c => c.to_string(),
            })
            .collect()
    }
}

impl AlertFormatter for JsonFormatter {
    fn format(&self, alert: &Alert) -> String {
        let indent = if self.pretty { "  " } else { "" };
        let nl = if self.pretty { "\n" } else { "" };
        let space = if self.pretty { " " } else { "" };

        let mut json = String::new();
        json.push('{');
        json.push_str(nl);

        // Level
        json.push_str(indent);
        json.push_str(&format!(
            "\"level\":{}\"{}\"",
            space,
            Self::escape_json_string(alert.level.as_str())
        ));
        json.push(',');
        json.push_str(nl);

        // Message
        json.push_str(indent);
        json.push_str(&format!(
            "\"message\":{}\"{}\"",
            space,
            Self::escape_json_string(&alert.message)
        ));

        // Timestamp
        if let Some(timestamp) = alert.timestamp {
            json.push(',');
            json.push_str(nl);
            json.push_str(indent);
            json.push_str(&format!(
                "\"timestamp\":{}\"{}\"",
                space,
                Self::escape_json_string(&timestamp.to_string())
            ));
        }

        // Tags
        if !alert.tags.is_empty() {
            json.push(',');
            json.push_str(nl);
            json.push_str(indent);
            json.push_str(&format!("\"tags\":{}", space));
            json.push('[');
            for (i, tag) in alert.tags.iter().enumerate() {
                if i > 0 {
                    json.push(',');
                    json.push_str(space);
                }
                json.push_str(&format!("\"{}\"", Self::escape_json_string(tag)));
            }
            json.push(']');
        }

        // Context
        if !alert.context.is_empty() {
            json.push(',');
            json.push_str(nl);
            json.push_str(indent);
            json.push_str(&format!("\"context\":{}", space));
            json.push('{');
            
            let mut first = true;
            for (key, value) in &alert.context {
                if !first {
                    json.push(',');
                    json.push_str(space);
                }
                first = false;
                json.push_str(&format!(
                    "\"{}\":{}\"{}\"",
                    Self::escape_json_string(key),
                    space,
                    Self::escape_json_string(value)
                ));
            }
            json.push('}');
        }

        json.push_str(nl);
        json.push('}');
        json
    }
}

/// Compact formatter - single line without extras
#[derive(Debug, Default, Clone, Copy)]
pub struct CompactFormatter;

impl AlertFormatter for CompactFormatter {
    fn format(&self, alert: &Alert) -> String {
        format!("{}: {}", alert.level.as_str(), alert.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AlertLevel;

    #[test]
    fn test_simple_formatter() {
        let alert = Alert::info("Test message");
        let formatter = SimpleFormatter;
        let output = formatter.format(&alert);
        
        assert!(output.contains("INFO"));
        assert!(output.contains("Test message"));
    }

    #[test]
    fn test_detailed_formatter() {
        let alert = Alert::warning("Test")
            .with_tag("test")
            .with_context("user", "john");
        
        let formatter = DetailedFormatter::new();
        let output = formatter.format(&alert);
        
        assert!(output.contains("WARNING"));
        assert!(output.contains("test"));
        assert!(output.contains("user=john"));
    }

    #[test]
    fn test_detailed_formatter_no_emoji() {
        let alert = Alert::error("Test");
        let formatter = DetailedFormatter::new().with_emoji(false);
        let output = formatter.format(&alert);
        
        assert!(!output.contains("❌"));
        assert!(output.contains("[ERROR]"));
    }

    #[test]
    fn test_json_formatter() {
        let alert = Alert::info("Test").with_tag("json");
        let formatter = JsonFormatter::new();
        let output = formatter.format(&alert);
        
        assert!(output.contains("\"level\""));
        assert!(output.contains("\"message\""));
        assert!(output.contains("\"tags\""));
    }

    #[test]
    fn test_json_escape() {
        let alert = Alert::info("Test \"quoted\" string\nwith newline");
        let formatter = JsonFormatter::new();
        let output = formatter.format(&alert);
        
        assert!(output.contains("\\\""));
        assert!(output.contains("\\n"));
    }

    #[test]
    fn test_compact_formatter() {
        let alert = Alert::warning("Simple warning");
        let formatter = CompactFormatter;
        let output = formatter.format(&alert);
        
        assert_eq!(output, "WARNING: Simple warning");
    }
}