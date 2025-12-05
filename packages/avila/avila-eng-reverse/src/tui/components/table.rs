// Table component
use super::Component;

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub selected_row: usize,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
            selected_row: 0,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn select_next(&mut self) {
        if self.selected_row < self.rows.len().saturating_sub(1) {
            self.selected_row += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        }
    }
}

impl Component for Table {
    fn render(&self) -> String {
        let mut output = String::new();

        // Headers
        output.push_str(&self.headers.join(" | "));
        output.push('\n');
        output.push_str(&"-".repeat(self.headers.len() * 15));
        output.push('\n');

        // Rows
        for (i, row) in self.rows.iter().enumerate() {
            let marker = if i == self.selected_row { ">" } else { " " };
            output.push_str(&format!("{} {}\n", marker, row.join(" | ")));
        }

        output
    }

    fn handle_input(&mut self, key: char) {
        match key {
            'j' | 'J' => self.select_next(),
            'k' | 'K' => self.select_prev(),
            _ => {}
        }
    }
}
