// Control Flow Graph viewer
use std::error::Error;

/// CFG visualization component
pub struct CfgView {
    nodes: Vec<CfgNode>,
    selected_node: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    pub id: usize,
    pub address: u64,
    pub instructions: Vec<String>,
    pub connections: Vec<usize>,
}

impl CfgView {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            selected_node: None,
        }
    }

    /// Add node to CFG
    pub fn add_node(&mut self, node: CfgNode) {
        self.nodes.push(node);
    }

    /// Select node
    pub fn select_node(&mut self, id: usize) {
        self.selected_node = Some(id);
    }

    /// Navigate to next node
    pub fn next_node(&mut self) {
        if let Some(current) = self.selected_node {
            if current + 1 < self.nodes.len() {
                self.selected_node = Some(current + 1);
            }
        }
    }

    /// Navigate to previous node
    pub fn prev_node(&mut self) {
        if let Some(current) = self.selected_node {
            if current > 0 {
                self.selected_node = Some(current - 1);
            }
        }
    }

    /// Render CFG
    pub fn render(&self) -> Result<String, Box<dyn Error>> {
        let mut output = String::new();

        for (i, node) in self.nodes.iter().enumerate() {
            let marker = if Some(i) == self.selected_node { ">" } else { " " };
            output.push_str(&format!("{} Node {}: 0x{:x}\n", marker, i, node.address));

            for instr in &node.instructions {
                output.push_str(&format!("    {}\n", instr));
            }

            if !node.connections.is_empty() {
                output.push_str(&format!("    -> {:?}\n", node.connections));
            }
            output.push('\n');
        }

        Ok(output)
    }
}
