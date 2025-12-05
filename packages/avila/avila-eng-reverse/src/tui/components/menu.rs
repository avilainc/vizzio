// Menu component
use super::Component;

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub key: char,
    pub enabled: bool,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            selected: 0,
        }
    }

    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn select_prev(&mut self) {
        self.selected = if self.selected == 0 {
            self.items.len() - 1
        } else {
            self.selected - 1
        };
    }

    pub fn get_selected(&self) -> Option<&MenuItem> {
        self.items.get(self.selected)
    }
}

impl Component for Menu {
    fn render(&self) -> String {
        let mut output = String::new();

        for (i, item) in self.items.iter().enumerate() {
            let marker = if i == self.selected { ">" } else { " " };
            let enabled = if item.enabled { "" } else { " (disabled)" };
            output.push_str(&format!(
                "{} [{}] {}{}\n",
                marker, item.key, item.label, enabled
            ));
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
