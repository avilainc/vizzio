// Reusable TUI components
pub mod progress_bar;
pub mod table;
pub mod menu;

pub use progress_bar::ProgressBar;
pub use table::Table;
pub use menu::Menu;

// Component trait
pub trait Component {
    fn render(&self) -> String;
    fn handle_input(&mut self, key: char);
}
