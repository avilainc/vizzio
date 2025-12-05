// Progress bar component
use super::Component;

pub struct ProgressBar {
    pub current: usize,
    pub total: usize,
    pub width: usize,
    pub label: String,
}

impl ProgressBar {
    pub fn new(total: usize, width: usize, label: String) -> Self {
        Self {
            current: 0,
            total,
            width,
            label,
        }
    }

    pub fn update(&mut self, current: usize) {
        self.current = current.min(self.total);
    }

    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.current as f32 / self.total as f32) * 100.0
        }
    }
}

impl Component for ProgressBar {
    fn render(&self) -> String {
        let filled = (self.width as f32 * self.current as f32 / self.total as f32) as usize;
        let empty = self.width - filled;

        format!(
            "{} [{}{}] {:.1}% ({}/{})",
            self.label,
            "=".repeat(filled),
            " ".repeat(empty),
            self.percentage(),
            self.current,
            self.total
        )
    }

    fn handle_input(&mut self, _key: char) {
        // Progress bar doesn't handle input
    }
}
