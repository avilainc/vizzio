// Log viewer component
use std::collections::VecDeque;

/// Log viewer for analysis output
pub struct LogView {
    logs: VecDeque<LogEntry>,
    max_entries: usize,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl LogView {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    /// Add log entry
    pub fn add_log(&mut self, entry: LogEntry) {
        if self.logs.len() >= self.max_entries {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
    }

    /// Clear logs
    pub fn clear(&mut self) {
        self.logs.clear();
    }

    /// Filter by level
    pub fn filter_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter().filter(|e| e.level == level).collect()
    }

    /// Render log view
    pub fn render(&self, lines: usize) -> String {
        let start = self.logs.len().saturating_sub(lines);
        let mut output = String::new();

        for entry in self.logs.iter().skip(start) {
            let level_str = match entry.level {
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARN",
                LogLevel::Error => "ERR ",
                LogLevel::Debug => "DBG ",
            };

            output.push_str(&format!("[{}] {} {}\n",
                entry.timestamp, level_str, entry.message));
        }

        output
    }
}
