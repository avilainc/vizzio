// Main TUI application
use std::error::Error;

/// TUI application
pub struct TuiApp {
    running: bool,
    current_view: View,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Dashboard,
    HexView,
    CfgView,
    LogView,
    Help,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            running: false,
            current_view: View::Dashboard,
        }
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = true;

        // TODO: Initialize terminal (ratatui/crossterm)
        // TODO: Main event loop
        // TODO: Handle keyboard input
        // TODO: Render views

        Ok(())
    }

    /// Switch to view
    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
    }

    /// Handle keyboard input
    pub fn handle_input(&mut self, key: char) -> Result<(), Box<dyn Error>> {
        match key {
            'q' => self.running = false,
            '1' => self.switch_view(View::Dashboard),
            '2' => self.switch_view(View::HexView),
            '3' => self.switch_view(View::CfgView),
            '4' => self.switch_view(View::LogView),
            '?' => self.switch_view(View::Help),
            _ => {}
        }
        Ok(())
    }

    /// Render current view
    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Render based on current_view
        Ok(())
    }

    /// Cleanup and exit
    pub fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = false;
        // TODO: Restore terminal
        Ok(())
    }
}
